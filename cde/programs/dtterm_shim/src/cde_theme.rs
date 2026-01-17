use std::collections::HashMap;
use std::process::Command;
use anyhow::{Result, Context};
use std::io::Write;

pub struct CdeTheme {
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub palette: [String; 16], // ANSI colors
}

impl CdeTheme {
    /// Queries X resources using `xrdb -query` and extracts CDE colors
    pub fn get_current() -> Result<Self> {
        let output = Command::new("xrdb")
            .arg("-query")
            .output()
            .context("Failed to execute xrdb")?;

        let resources_str = String::from_utf8_lossy(&output.stdout);
        let resources: HashMap<&str, &str> = resources_str
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, ":\t");
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    Some((key.trim(), value.trim()))
                } else {
                    None
                }
            })
            .collect();

        // Defaults (CDE-like)
        let bg = resources.get("*background").unwrap_or(&"#a0a0a0").to_string();
        let fg = resources.get("*foreground").unwrap_or(&"#000000").to_string();
        
        // Map CDE palette to ANSI if possible.
        // CDE uses *0.primaryColor, *1.primaryColor etc for different palette entries sometimes
        // But simpler strategy is to just use standard xterm colors or map specifically if CDE resources exist.
        // For now, let's just ensure BG/FG are correct as that's 90% of the look.
        
        Ok(CdeTheme {
            background: bg.clone(),
            foreground: fg.clone(),
            cursor: fg, // Cursor usually matches FG in high contrast
            palette: [
                "#000000".into(), "#c00000".into(), "#00c000".into(), "#c0c000".into(),
                "#0000c0".into(), "#c000c0".into(), "#00c0c0".into(), "#c0c0c0".into(),
                "#808080".into(), "#ff0000".into(), "#00ff00".into(), "#ffff00".into(),
                "#0000ff".into(), "#ff00ff".into(), "#00ffff".into(), "#ffffff".into(),
            ],
        })
    }

    pub fn generate_alacritty_config(&self, path: &std::path::Path) -> Result<()> {
        let toml_content = format!(
            r#"
[window]
padding = {{ x = 2, y = 2 }}
decorations = "full"
dynamic_title = true

[colors.primary]
background = "{}"
foreground = "{}"

[colors.cursor]
text = "{}"
cursor = "{}"

[font]
size = 12.0
normal = {{ family = "Monospace", style = "Regular" }}
"#,
            self.background, self.foreground, self.background, self.cursor
        );

        let mut file = std::fs::File::create(path)?;
        file.write_all(toml_content.as_bytes())?;
        Ok(())
    }
}
