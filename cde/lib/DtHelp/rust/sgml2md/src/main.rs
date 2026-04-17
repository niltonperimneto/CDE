//! `sgml2md` — Convert CDE's SGML/SDL help sources into Markdown.
//!
//! This utility walks an input tree of SGML files (extensions `.sgm`, `.sgml`,
//! `.sdl`) and emits an equivalent `.md` file per input into the output
//! directory.  It is a best-effort converter for the legacy CDE help corpus
//! and deliberately does not aim for full DocBook fidelity.
//!
//! # XML parser version
//!
//! Pinned to `quick-xml = "0.39"`.  The 0.30 -> 0.39 migration moved reader
//! configuration onto [`Reader::config_mut`] and removed the short-hand
//! `unescape()` method on `BytesText`; we now decode via [`BytesText::decode`]
//! and swallow the (expected) unknown-entity errors common in legacy SGML.

// This crate has no FFI obligations; forbid all unsafe code.
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use clap::Parser;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input directory or file (SGML/SDL)
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for Markdown files
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.input.is_file() {
        convert_file(&args.input, &args.output)?;
    } else if args.input.is_dir() {
        for entry in WalkDir::new(&args.input) {
            let entry = entry?;
            let path = entry.path();
            if path
                .extension()
                .map_or(false, |ext| ext == "sgm" || ext == "sgml" || ext == "sdl")
            {
                // simple output path mapping
                // For now, flatten structure or mirror it? Let's flatten for simplicity in test
                // Actually mirroring is better if we want to organize.
                // But for now, let's just output to the output dir.
                let file_name = path.file_stem().unwrap();
                let mut out_path = args.output.clone();
                out_path.push(file_name);
                out_path.set_extension("md");

                println!("Converting {:?} -> {:?}", path, out_path);
                if let Err(e) = convert_file(path, &out_path) {
                    eprintln!("Failed to convert {:?}: {}", path, e);
                }
            }
        }
    }

    Ok(())
}

fn convert_file(input: &std::path::Path, output: &std::path::Path) -> anyhow::Result<()> {
    let mut reader = Reader::from_file(input)?;
    // quick-xml 0.39: configuration is set through `config_mut()`.  Legacy
    // SGML commonly has mismatched end-tag names (e.g. `<Para>` closed by
    // `</para>`) and stray DOCTYPE comments; be permissive about both.
    {
        let cfg = reader.config_mut();
        cfg.trim_text(true);
        cfg.check_end_names = false;
        cfg.check_comments = false;
    }

    let mut buf = Vec::new();
    let mut markdown = String::new();

    // State to track simple nesting
    let mut title_level = 0; // 0 = none, 1 = RefEntryTitle, 2 = RefSect1, 3 = RefSect2

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name();
                match name.as_ref() {
                    b"RefEntryTitle" => {
                        title_level = 1;
                        markdown.push_str("# ");
                    }
                    b"RefSect1" => {
                        title_level = 2;
                    }
                    b"RefSect2" => {
                        title_level = 3;
                    }
                    b"Title" => {
                        match title_level {
                            2 => markdown.push_str("\n## "),
                            3 => markdown.push_str("\n### "),
                            _ => markdown.push_str("\n# "), // Default fallback
                        }
                    }
                    b"Para" => markdown.push_str("\n\n"),
                    b"Literal" | b"Symbol" | b"Classname" | b"SystemItem" | b"Function" => {
                        markdown.push('`');
                    }
                    b"VariableList" => {
                        markdown.push_str("\n");
                    }
                    b"VarListEntry" => {}                   // Start of an entry
                    b"Term" => markdown.push_str("\n* **"), // Bullet point for term
                    b"ListItem" => {}
                    _ => (),
                }
            }
            Ok(Event::Text(e)) => {
                // quick-xml 0.39 no longer exposes `BytesText::unescape`; we
                // decode to UTF-8 and fall back to a lossy raw view if the
                // buffer is not valid UTF-8 (rare but possible in the legacy
                // ISO-8859-1 SGML sources).
                match e.decode() {
                    Ok(text) => markdown.push_str(text.as_ref()),
                    Err(_) => {
                        let text = String::from_utf8_lossy(e.as_ref());
                        markdown.push_str(&text);
                    }
                }
            }
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"RefEntryTitle" | b"Title" => {
                    markdown.push('\n');
                }
                b"Literal" | b"Symbol" | b"Classname" | b"SystemItem" | b"Function" => {
                    markdown.push('`');
                }
                b"VariableList" => {
                    markdown.push('\n');
                }
                b"Term" => markdown.push_str("** "),
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(_) => {
                // Ignore errors for now to survive bad SGML
            }
            _ => (),
        }
        buf.clear();
    }

    // Ensure output dir exists
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(output, markdown)?;
    Ok(())
}
