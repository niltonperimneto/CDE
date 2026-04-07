use anyhow::Result;
use log::info;
use std::future::pending;

use crate::manager::ToolTalkManager;

mod manager;
mod types;

/// Resolve the directory that holds compiled ptype JSON files.
///
/// Priority:
///   1. `$DTDIR/appconfig/types/C/tt/compiled`  (standard CDE install tree)
///   2. `$CDE_CONFIGURATION_TOP/dt/tt/types`     (packager override)
///   3. `/etc/dt/tt/types`                        (hard-coded fallback)
fn ptype_db_path() -> String {
    if let Ok(dtdir) = std::env::var("DTDIR") {
        return format!("{}/appconfig/types/C/tt/compiled", dtdir);
    }
    if let Ok(cfg_top) = std::env::var("CDE_CONFIGURATION_TOP") {
        return format!("{}/dt/tt/types", cfg_top);
    }
    "/etc/dt/tt/types".to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting ttsession (Rust D-Bus Broker)...");

    let manager = ToolTalkManager::new();
    let db_path = ptype_db_path();
    manager.load_ptypes(&db_path);

    // zbus 5: builder is at zbus::connection::Builder.
    // Clone manager before handing ownership to serve_at so we can call
    // spawn_monitor afterwards.  ToolTalkManager derives Clone (its only
    // field is Arc<Mutex<...>>).
    let conn = zbus::connection::Builder::session()?
        .name("org.cde.ToolTalk")?
        .serve_at("/org/cde/ToolTalk", manager.clone())?
        .build()
        .await?;

    info!("ttsession started on D-Bus: org.cde.ToolTalk");

    manager.spawn_monitor(conn).await;

    // Run forever — the signal monitor task drives the event loop.
    pending::<()>().await;

    Ok(())
}
