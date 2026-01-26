mod ast;
mod parser;

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "tt_type_comp")]
#[command(about = "ToolTalk Type Compiler (Rust Rewrite)", long_about = None)]
struct Cli {
    /// Output file (prints to stdout if not specified)
    #[arg(short = 'o', long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Merge into database (ignored in this modernization, just for compatibility)
    #[arg(short = 'd', long)]
    merge_db: Option<String>,

    /// Input file (.ptype or .otype)
    input: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let content = fs::read_to_string(&cli.input)
        .with_context(|| format!("Failed to read input file: {:?}", cli.input))?;

    let ptype = parser::parse_ptype(&content)?;

    let json = serde_json::to_string_pretty(&ptype)?;

    if let Some(out_path) = cli.output {
        fs::write(&out_path, json)
            .with_context(|| format!("Failed to write output file: {:?}", out_path))?;
    } else {
        println!("{}", json);
    }

    Ok(())
}
