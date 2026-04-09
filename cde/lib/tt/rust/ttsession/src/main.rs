// ttsession is pure Rust (tokio + zbus); no FFI obligations.
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]

use anyhow::Result;
use log::info;
use std::future::pending;

use crate::broker::ToolTalkBroker;

mod broker;
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
    info!("Starting ttsession (Rust D-Bus method-call broker)...");

    let broker = ToolTalkBroker::new();
    let db_path = ptype_db_path();
    broker.load_ptypes(&db_path).await;

    // Register the broker on the session bus and acquire the well-known name.
    // zbus 5: builder is at zbus::connection::Builder.
    //
    // The connection is stored in `_conn` to keep it alive for the duration of
    // the process.  Dropping it would release the bus name and unregister the
    // object.  All dispatch happens inside ToolTalkBroker::send_message(); no
    // separate monitor task is needed.
    let _conn = zbus::connection::Builder::session()?
        .name("org.cde.ToolTalk")?
        .serve_at("/org/cde/ToolTalk", broker)?
        .build()
        .await?;

    info!("ttsession started — serving org.cde.ToolTalk at /org/cde/ToolTalk");

    // Run forever; the tokio runtime drives zbus method dispatch.
    pending::<()>().await;

    Ok(())
}
