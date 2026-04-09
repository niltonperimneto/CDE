// This crate has no FFI obligations; forbid all unsafe code.
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

/*
 * dtterm_shim — CDE Terminal Emulator Wrapper
 *
 * Translates CDE dtterm(1) arguments into an Alacritty invocation,
 * applying the active CDE colour theme.
 *
 * Copyright (c) 2026 CDE Modernization Project
 * Licensed under GPL v3.0
 */

use anyhow::Result;
use clap::Parser;
use std::os::unix::process::CommandExt;
use std::process::Command;

mod cde_theme;

#[derive(Parser, Debug)]
#[command(
    name = "dtterm",
    about = "CDE Terminal Emulator Wrapper",
    disable_help_flag = true
)]
struct Args {
    #[arg(short = 'e', allow_hyphen_values = true, num_args = 1..)]
    command: Option<Vec<String>>,

    #[arg(long = "title", short = 't')]
    title: Option<String>,

    #[arg(long = "geometry", short = 'g')]
    geometry: Option<String>,

    #[arg(long = "display", short = 'd')]
    display: Option<String>,

    #[arg(long = "name")]
    name: Option<String>,

    #[arg(short = 'l', long = "ls")]
    login_shell: bool,

    // CDE-specific flags we accept but do not forward
    #[arg(long = "C")]
    console_mode: bool,

    #[arg(long = "map")]
    map: bool,

    #[arg(long = "xr", allow_hyphen_values = true)]
    xr: Option<String>,

    // Absorb any remaining unrecognised flags so clap doesn't error out
    #[arg(allow_hyphen_values = true)]
    rest: Vec<String>,
}

/// Parsed result of a standard X11 geometry string: `[WxH][+X+Y]`
struct Geometry {
    cols: Option<u32>,
    rows: Option<u32>,
    /// Position is accepted syntactically but ignored at runtime because
    /// Alacritty does not support window positioning on Wayland.
    _x: Option<i32>,
    _y: Option<i32>,
}

/// Parse a CDE/X11 geometry string of the form `[C]x[R][+X+Y]` or
/// `[C]x[R][-X-Y]`.  The `x` separator between columns and rows is required;
/// the column, row, and position fields are individually optional.
/// Position-only strings like `+100+200` are **not** supported.
/// Returns `None` if no `x`/`X` separator is found.
///
/// Examples: `80x24`, `80x24+100+200`, `132x50-0-0`
fn parse_geometry(s: &str) -> Option<Geometry> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // Locate the 'x'/'X' separator directly in the original string.
    // Using `to_lowercase().find()` would give a byte offset into the
    // lowercased copy; for non-ASCII input Unicode lowercasing can change
    // string length, making that offset invalid for slicing `s` (panic).
    let x_pos = s.find(|c| c == 'x' || c == 'X')?;
    let cols_str = &s[..x_pos];
    let rest = &s[x_pos + 1..];

    // The rows part ends at the first '+' or '-' that introduces position
    let pos_start = rest.find(|c| c == '+' || c == '-');
    let rows_str = pos_start.map(|i| &rest[..i]).unwrap_or(rest);
    let pos_str = pos_start.map(|i| &rest[i..]).unwrap_or("");

    let cols: Option<u32> = cols_str.parse().ok();
    let rows: Option<u32> = rows_str.parse().ok();

    // Parse optional +X+Y or -X-Y (sign is part of the token)
    let (x, y) = if pos_str.is_empty() {
        (None, None)
    } else {
        // pos_str looks like "+100+200" or "-0+0" etc.
        // Split at the second sign character after the first.
        let second = pos_str[1..].find(|c| c == '+' || c == '-').map(|i| i + 1);
        match second {
            Some(idx) => {
                let xv: Option<i32> = pos_str[..idx].parse().ok();
                let yv: Option<i32> = pos_str[idx..].parse().ok();
                (xv, yv)
            }
            None => (pos_str.parse().ok(), None),
        }
    };

    Some(Geometry {
        cols,
        rows,
        _x: x,
        _y: y,
    })
}

fn main() -> Result<()> {
    // 1. Build Alacritty theme config from the active CDE colour scheme
    let theme = cde_theme::CdeTheme::get_current().unwrap_or_else(|e| {
        eprintln!("[dtterm_shim] Warning: failed to load CDE theme: {}", e);
        cde_theme::CdeTheme {
            background: "#aaaaaa".into(),
            foreground: "#000000".into(),
            cursor: "#000000".into(),
            palette: Default::default(),
        }
    });

    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| "/tmp".into())
        .join("dtterm_shim");
    std::fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("alacritty.toml");
    theme.generate_alacritty_config(&config_path)?;

    // 2. Parse CDE arguments
    let args = Args::parse();

    // 3. Build the Alacritty command
    let mut cmd = Command::new("alacritty");
    cmd.arg("--config-file").arg(&config_path);

    if let Some(title) = args.title {
        cmd.arg("--title").arg(title);
    }

    // Window class for dtwm integration (WM_CLASS instance,general)
    let instance = args.name.as_deref().unwrap_or("dtterm");
    cmd.arg("--class").arg(format!("{},Dtterm", instance));

    // P2-8: geometry translation.
    // Alacritty accepts `--option window.dimensions.columns=N` style args
    // in recent versions, but the most portable approach for size is the
    // `--dimensions C R` flag available since Alacritty 0.12.
    // Window *position* is intentionally not forwarded: Alacritty ignores
    // --position on Wayland compositors, and it is rarely meaningful on X11
    // either (most WMs re-place windows anyway).
    if let Some(ref geo_str) = args.geometry {
        match parse_geometry(geo_str) {
            Some(geo) => {
                if let (Some(cols), Some(rows)) = (geo.cols, geo.rows) {
                    cmd.arg("--dimensions")
                        .arg(cols.to_string())
                        .arg(rows.to_string());
                }
                if geo._x.is_some() || geo._y.is_some() {
                    // Log but do not error — position is cosmetic
                    eprintln!(
                        "[dtterm_shim] Note: window position in geometry '{}' is not \
                         forwarded (Alacritty does not support positioning on Wayland)",
                        geo_str
                    );
                }
            }
            None => eprintln!(
                "[dtterm_shim] Warning: could not parse geometry '{}', ignoring",
                geo_str
            ),
        }
    }

    // Explicit $DISPLAY forwarding (X11 only; skip on pure Wayland sessions)
    if let Some(display) = args.display {
        if std::env::var_os("WAYLAND_DISPLAY").is_none() {
            cmd.env("DISPLAY", display);
        } else {
            eprintln!(
                "[dtterm_shim] Note: --display ignored under Wayland \
                 ($WAYLAND_DISPLAY is set)"
            );
        }
    }

    if let Some(command_args) = args.command {
        cmd.arg("-e").args(command_args);
    }

    // 4. Replace this process with Alacritty (exec-family call — no return)
    Err(cmd.exec().into())
}
