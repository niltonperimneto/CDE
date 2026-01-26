/*
 * dtdialog - CDE Dialog Utility
 *
 * A replacement for dtksh dialog scripts.
 * Displays Motif dialogs (error, warning, info, working) via command line.
 *
 * Usage:
 *   dtdialog --error   --title "Title" --message "Message"
 *   dtdialog --warning --title "Title" --message "Message"
 *   dtdialog --info    --title "Title" --message "Message"
 *   dtdialog --working --title "Title" --message "Message"
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

    // CDE specific flags to ignore
    #[arg(long = "C")]
    console_mode: bool,

    #[arg(long = "map")]
    map: bool,

    #[arg(long = "xr", allow_hyphen_values = true)]
    xr: Option<String>,

    // Capture remaining args to prevent parsing errors
    #[arg(allow_hyphen_values = true)]
    rest: Vec<String>,
}

fn main() -> Result<()> {
    // 1. Generate Config
    let theme = cde_theme::CdeTheme::get_current().unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load CDE theme: {}", e);
        cde_theme::CdeTheme {
            background: "#ffffff".into(),
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

    // 2. Parse Args manually or via Clap
    // Clap has trouble with -e command args that look like flags, so we might need loose parsing
    // but allow_hyphen_values helps.
    let args = Args::parse();

    // 3. Construct Alacritty Command
    let mut cmd = Command::new("alacritty");
    cmd.arg("--config-file").arg(config_path);

    if let Some(title) = args.title {
        cmd.arg("--title").arg(title);
    }

    // Set Window Class for dtwm integration (Icon, Style)
    // Alacritty format: --class instance,general
    let instance = args.name.as_deref().unwrap_or("dtterm");
    cmd.arg("--class").arg(format!("{},Dtterm", instance));

    // Geometry translation is complex (WxH+X+Y), Alacritty supports --position and --dimensions in older versions or config
    // For now we might ignore geometry or implement partial parsing if critical.

    if args.login_shell {
        // Alacritty config handles shell, or we pass valid shell command
    }

    if let Some(mut command_args) = args.command {
        // -e arg1 arg2 ...
        cmd.arg("-e");

        // If the first arg is a command alias (like 'vi'), alacritty needs it
        cmd.args(command_args);
    }

    // 4. Exec
    Err(cmd.exec().into())
}
