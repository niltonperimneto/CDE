//! Parallel directory loader for `.dt` files.
//!
//! The legacy C loader (`ActionDb.c`) walks `~/.dt/types/`, `/etc/dt/types`,
//! and `$DTDATABASESEARCHPATH` sequentially — hundreds of small files, opened
//! one by one during CDE startup. This module replaces that with a `rayon`
//! + `walkdir` parallel traversal, merging per-thread databases at the end.

use crate::ast::Database;
use crate::parser::parse_file_lossy;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Recursively load every `.dt` file under `root` in parallel.
///
/// Returns a populated [`Database`]. Parse errors are collected on
/// `Database::errors` instead of aborting the whole load, matching the
/// "best-effort" semantics of the legacy loader.
pub fn load_dir(root: &Path) -> Database {
    if !root.exists() {
        return Database::new();
    }

    let files: Vec<PathBuf> = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("dt"))
                .unwrap_or(false)
        })
        .map(|e| e.into_path())
        .collect();

    // Parse every file in parallel; each task produces a tiny Database.
    let partials: Vec<Database> = files
        .par_iter()
        .map(|path| {
            let (records, err) = parse_file_lossy(path);
            let mut db = Database::new();
            if let Some(e) = err {
                db.errors.push(e);
            }
            for r in records {
                db.insert(r);
            }
            db
        })
        .collect();

    let mut out = Database::new();
    for partial in partials {
        out.merge(partial);
    }
    out
}

/// Load from the caller-supplied list of search-path roots, in order.
/// Later roots override earlier ones (same semantics as the legacy loader:
/// user config shadows system config).
pub fn load_search_path(paths: &[PathBuf]) -> Database {
    let mut out = Database::new();
    for p in paths {
        out.merge(load_dir(p));
    }
    out
}
