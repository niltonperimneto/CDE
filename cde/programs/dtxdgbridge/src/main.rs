// SPDX-License-Identifier: LGPL-2.0-or-later
//
// dtxdgbridge — XDG .desktop → CDE Action (.dt) bridge daemon.
//
// This daemon monitors the standard Freedesktop application directories
// (`/usr/share/applications/` and `~/.local/share/applications/`) for `.desktop`
// files and automatically generates native CDE Action files in `~/.dt/types/`.
//
// Modern applications (Firefox, VS Code, Discord, etc.) appear in the CDE
// Application Manager and Front Panel without modifying any CDE C code.
//
// ## Architecture
//
// - `desktop_entry` — Parser for XDG `.desktop` files.
// - `mime_map`      — MIME type → file extension mapper (via globs2).
// - `dt_writer`     — CDE `.dt` file generator with atomic writes.
// - `watcher`       — inotify-backed filesystem watcher (via `notify` crate).
// - `notifier`      — Debounced D-Bus `DtTypesReloaded` signal emitter.
//
// All modules are `#![forbid(unsafe_code)]`.

mod desktop_entry;
mod dt_writer;
mod mime_map;
mod notifier;
mod watcher;

use anyhow::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize structured logging.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    log::info!("dtxdgbridge v{} starting", env!("CARGO_PKG_VERSION"));

    // 1. Load the system MIME → extension mapping.
    let mime_map = mime_map::MimeMap::load();

    // 2. Initial scan: translate all existing .desktop files.
    let count = watcher::initial_scan(&mime_map)?;
    log::info!("initial scan: translated {count} .desktop files");

    // 3. Start the debounced D-Bus notifier.
    let notifier = Arc::new(notifier::DebouncedNotifier::new());
    let notifier_handle = {
        let n = Arc::clone(&notifier);
        tokio::spawn(async move { n.run().await })
    };

    // Fire one reload signal after the initial scan so CDE picks up everything.
    notifier.request();

    // 4. Start the filesystem watcher (runs forever).
    log::info!("entering watch mode");
    tokio::select! {
        result = watcher::run(mime_map, Arc::clone(&notifier)) => {
            if let Err(e) = result {
                log::error!("watcher error: {e:#}");
            }
        }
        _ = notifier_handle => {
            log::error!("notifier task exited unexpectedly");
        }
        _ = tokio::signal::ctrl_c() => {
            log::info!("received SIGINT, shutting down");
        }
    }

    Ok(())
}
