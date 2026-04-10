// SPDX-License-Identifier: LGPL-2.0-or-later
//
// desktop_entry.rs — Parse XDG .desktop files into a structured representation.
//
// Handles the `[Desktop Entry]` group from the Freedesktop Desktop Entry Spec.
// Only `Type=Application` entries are relevant; everything else is skipped.

use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;

/// A parsed XDG `.desktop` entry with only the fields CDE needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopEntry {
    /// The `Name` field (unlocalized).
    pub name: String,
    /// The `Exec` command line.
    pub exec: String,
    /// The `Icon` name (no path, no extension).
    pub icon: String,
    /// The `Comment` field (unlocalized), used as CDE DESCRIPTION.
    pub comment: String,
    /// Whether `Terminal=true` was set.
    pub terminal: bool,
    /// Semicolon-separated MIME types from the `MimeType` field.
    pub mime_types: Vec<String>,
    /// Whether `NoDisplay=true` was set (we skip these).
    pub no_display: bool,
    /// Whether `Hidden=true` was set (we skip these).
    pub hidden: bool,
    /// Basename of the `.desktop` file (without extension), used as the action
    /// identifier.
    pub id: String,
}

impl DesktopEntry {
    /// Returns `true` if this entry should be translated to a CDE action.
    pub fn is_actionable(&self) -> bool {
        !self.no_display && !self.hidden && !self.name.is_empty() && !self.exec.is_empty()
    }
}

/// Parse a `.desktop` file at `path`.
///
/// Returns `Ok(None)` if the file is not `Type=Application` or is otherwise
/// not translatable.  Returns `Err` only on I/O failures.
pub fn parse(path: &Path) -> io::Result<Option<DesktopEntry>> {
    let file = std::fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut in_desktop_entry = false;
    let mut fields: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        // Skip blanks and comments.
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Group header.
        if trimmed.starts_with('[') {
            if in_desktop_entry {
                // We've left the [Desktop Entry] group — stop.
                break;
            }
            if trimmed == "[Desktop Entry]" {
                in_desktop_entry = true;
            }
            continue;
        }

        if !in_desktop_entry {
            continue;
        }

        // key=value (first '=' wins).
        if let Some((key, value)) = trimmed.split_once('=') {
            // Only store unlocalized keys (no brackets).
            let key = key.trim();
            if !key.contains('[') {
                fields.insert(key.to_owned(), value.trim().to_owned());
            }
        }
    }

    // Only process Application entries.
    match fields.get("Type").map(String::as_str) {
        Some("Application") => {}
        _ => return Ok(None),
    }

    let id = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();

    let mime_types: Vec<String> = fields
        .get("MimeType")
        .map(|v| {
            v.split(';')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    let entry = DesktopEntry {
        name: fields.get("Name").cloned().unwrap_or_default(),
        exec: fields.get("Exec").cloned().unwrap_or_default(),
        icon: fields.get("Icon").cloned().unwrap_or_default(),
        comment: fields.get("Comment").cloned().unwrap_or_default(),
        terminal: fields.get("Terminal").is_some_and(|v| v == "true"),
        mime_types,
        no_display: fields
            .get("NoDisplay")
            .is_some_and(|v| v == "true"),
        hidden: fields
            .get("Hidden")
            .is_some_and(|v| v == "true"),
        id,
    };

    Ok(Some(entry))
}

/// Translate an XDG `Exec` string to a CDE `EXEC_STRING`.
///
/// XDG field codes:
///   %f → single file  → `%Arg_1%`
///   %F → file list     → `%Arg_1%`
///   %u → single URL    → `%Arg_1%`
///   %U → URL list      → `%Arg_1%`
///   %i → `--icon <Icon>` (dropped — CDE has its own icon field)
///   %c → translated Name (dropped)
///   %k → desktop file path (dropped)
///   %% → literal %
///
/// If none of the file/URL codes are present, we append `%Arg_1%` so CDE can
/// pass a file argument via drag-and-drop.
pub fn translate_exec(exec: &str) -> String {
    let mut result = String::with_capacity(exec.len() + 16);
    let mut has_arg = false;
    let mut chars = exec.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            match chars.peek() {
                Some('f' | 'F' | 'u' | 'U') => {
                    chars.next();
                    if !has_arg {
                        result.push_str("\"%Arg_1%\"");
                        has_arg = true;
                    }
                }
                Some('i' | 'c' | 'k') => {
                    chars.next(); // drop
                }
                Some('%') => {
                    chars.next();
                    result.push('%');
                }
                _ => {
                    result.push('%');
                }
            }
        } else {
            result.push(ch);
        }
    }

    // Trim trailing whitespace left by removed field codes.
    let result = result.trim_end().to_string();

    // If there's no arg placeholder yet, append one so drag-and-drop works.
    if has_arg {
        result
    } else {
        format!("{result} \"%Arg_1%\"")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_desktop(content: &str) -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        f.write_all(content.as_bytes()).unwrap();
        f
    }

    #[test]
    fn parse_firefox_style() {
        let f = write_desktop(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=Firefox\n\
             Exec=/usr/lib/firefox/firefox %u\n\
             Icon=firefox\n\
             Terminal=false\n\
             Comment=Browse the web\n\
             MimeType=text/html;application/xhtml+xml;\n",
        );
        let entry = parse(f.path()).unwrap().unwrap();
        assert_eq!(entry.name, "Firefox");
        assert_eq!(entry.icon, "firefox");
        assert!(!entry.terminal);
        assert_eq!(entry.mime_types, vec!["text/html", "application/xhtml+xml"]);
        assert!(entry.is_actionable());
    }

    #[test]
    fn skip_non_application() {
        let f = write_desktop(
            "[Desktop Entry]\n\
             Type=Link\n\
             Name=Something\n\
             URL=https://example.com\n",
        );
        assert!(parse(f.path()).unwrap().is_none());
    }

    #[test]
    fn skip_hidden() {
        let f = write_desktop(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=Hidden\n\
             Exec=hidden\n\
             Hidden=true\n",
        );
        let entry = parse(f.path()).unwrap().unwrap();
        assert!(!entry.is_actionable());
    }

    #[test]
    fn exec_translation() {
        assert_eq!(
            translate_exec("/usr/lib/firefox/firefox %u"),
            "/usr/lib/firefox/firefox \"%Arg_1%\""
        );
        assert_eq!(
            translate_exec("code --new-window %F"),
            "code --new-window \"%Arg_1%\""
        );
        // No arg code → append one.
        assert_eq!(
            translate_exec("supertux2"),
            "supertux2 \"%Arg_1%\""
        );
        // Literal %%.
        assert_eq!(
            translate_exec("echo 100%%"),
            "echo 100% \"%Arg_1%\""
        );
    }
}
