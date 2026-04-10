// SPDX-License-Identifier: LGPL-2.0-or-later
//
// watcher.rs — Monitor XDG application directories with inotify and translate
// new/changed/removed `.desktop` files into CDE Actions.
//
// Uses the `notify` crate for cross-platform filesystem events backed by
// inotify on Linux.  Events are processed asynchronously via a
// `tokio::sync::mpsc` channel so the filesystem watch thread never blocks.

use crate::desktop_entry;
use crate::dt_writer;
use crate::mime_map::MimeMap;
use crate::notifier::DebouncedNotifier;

use anyhow::{Context, Result};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Directories to monitor for `.desktop` files.
fn watch_dirs() -> Vec<PathBuf> {
    let mut dirs = vec![PathBuf::from("/usr/share/applications")];

    if let Ok(home) = std::env::var("HOME") {
        dirs.push(PathBuf::from(home).join(".local/share/applications"));
    }

    // Also respect $XDG_DATA_DIRS if set.
    if let Ok(xdg_dirs) = std::env::var("XDG_DATA_DIRS") {
        for dir in xdg_dirs.split(':') {
            let app_dir = PathBuf::from(dir).join("applications");
            if !dirs.contains(&app_dir) {
                dirs.push(app_dir);
            }
        }
    }

    dirs
}

/// The CDE output directory for user-local actions.
fn output_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".dt/types")
    } else {
        PathBuf::from("/tmp/.dt/types")
    }
}

/// Perform an initial scan of all watch directories, translating every
/// existing `.desktop` file.  This ensures that apps installed *before*
/// the daemon started are also available in CDE.
pub fn initial_scan(mime_map: &MimeMap) -> Result<usize> {
    let out = output_dir();
    let mut count = 0usize;

    for dir in watch_dirs() {
        if !dir.is_dir() {
            log::debug!("skipping non-existent directory: {}", dir.display());
            continue;
        }

        let entries = std::fs::read_dir(&dir)
            .with_context(|| format!("reading {}", dir.display()))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                continue;
            }

            match process_desktop_file(&path, mime_map, &out) {
                Ok(true) => count += 1,
                Ok(false) => {}
                Err(e) => {
                    log::warn!("failed to process {}: {e}", path.display());
                }
            }
        }
    }

    Ok(count)
}

/// Start the filesystem watcher.
///
/// This spawns two tasks:
/// 1. A blocking thread running `notify::RecommendedWatcher` that pushes
///    events into a channel.
/// 2. An async task that reads events, translates `.desktop` files, and
///    requests reload notifications.
pub async fn run(mime_map: MimeMap, notifier: Arc<DebouncedNotifier>) -> Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Event>(256);
    let out = output_dir();

    // Create the watcher on a blocking thread because `notify` uses
    // synchronous I/O internally.
    let watcher_dirs = watch_dirs();
    let _watcher = tokio::task::spawn_blocking(move || -> Result<RecommendedWatcher> {
        let mut watcher = RecommendedWatcher::new(
            move |result: notify::Result<Event>| {
                if let Ok(event) = result {
                    // Best-effort send; if the channel is full we drop the
                    // event — the next modification will pick it up.
                    let _ = tx.blocking_send(event);
                }
            },
            Config::default(),
        )?;

        for dir in &watcher_dirs {
            if dir.is_dir() {
                watcher
                    .watch(dir, RecursiveMode::NonRecursive)
                    .with_context(|| format!("watching {}", dir.display()))?;
                log::info!("watching {}", dir.display());
            } else {
                log::debug!("skipping non-existent: {}", dir.display());
            }
        }

        // Park the thread so the watcher stays alive.
        loop {
            std::thread::park();
        }
    });

    // Event processing loop.
    while let Some(event) = rx.recv().await {
        match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) => {
                for path in &event.paths {
                    if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                        continue;
                    }
                    match process_desktop_file(path, &mime_map, &out) {
                        Ok(true) => notifier.request(),
                        Ok(false) => {}
                        Err(e) => log::warn!("error processing {}: {e}", path.display()),
                    }
                }
            }
            EventKind::Remove(_) => {
                for path in &event.paths {
                    if path.extension().and_then(|e| e.to_str()) != Some("desktop") {
                        continue;
                    }
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        match dt_writer::remove(stem, &out) {
                            Ok(true) => notifier.request(),
                            Ok(false) => {}
                            Err(e) => log::warn!("error removing {stem}.dt: {e}"),
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}

/// Process a single `.desktop` file: parse → generate `.dt`.
///
/// Returns `Ok(true)` if a `.dt` file was written, `Ok(false)` if skipped.
fn process_desktop_file(path: &Path, mime_map: &MimeMap, output_dir: &Path) -> Result<bool> {
    let entry = desktop_entry::parse(path)
        .with_context(|| format!("parsing {}", path.display()))?;

    let entry = match entry {
        Some(e) if e.is_actionable() => e,
        _ => return Ok(false),
    };

    dt_writer::generate(&entry, mime_map, output_dir)
        .with_context(|| format!("generating .dt for {}", entry.id))?;

    Ok(true)
}
