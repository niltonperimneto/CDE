//! ToolTalk D-Bus message broker.
//!
//! `ToolTalkBroker` replaces the old signal-only `ToolTalkManager` with a
//! proper request–reply D-Bus interface:
//!
//! * Senders call `org.cde.ToolTalk.SendMessage` and receive a synchronous
//!   `TT_OK (0)` / `TT_ERR_* (<0)` status reply.
//! * The broker dispatches matching ptypes (auto-starting processes as needed)
//!   and broadcasts a `MessageDelivered` signal so observers can react.
//!
//! The broker is activated on-demand by systemd via `Type=dbus` — it starts
//! the first time any process calls `SendMessage` and runs until the session ends.

use log::{error, info, warn};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;
use zbus::object_server::SignalEmitter;

use crate::types::{Action, Ptype, Scope};

// ---------------------------------------------------------------------------
// TT_OK / TT_ERR_* constants (mirror tt_c.h; kept local to avoid a shared dep)
// ---------------------------------------------------------------------------
const TT_OK: i32 = 0;
const TT_ERR_NOMP: i32 = -1; // no message protocol daemon

// ---------------------------------------------------------------------------
// ToolTalkBroker
// ---------------------------------------------------------------------------

/// Central broker for CDE ToolTalk IPC over D-Bus.
///
/// One instance is served at `/org/cde/ToolTalk` under the well-known name
/// `org.cde.ToolTalk`.  It is registered with zbus via
/// `connection::Builder::serve_at()`.
pub struct ToolTalkBroker {
    ptypes: Arc<Mutex<HashMap<String, Ptype>>>,
}

impl ToolTalkBroker {
    pub fn new() -> Self {
        ToolTalkBroker {
            ptypes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Load ptype JSON files from `db_path` directory asynchronously.
    ///
    /// Each `.json` file must deserialize as a [`Ptype`].  Malformed or
    /// unreadable files are logged and skipped; they do not abort loading.
    pub async fn load_ptypes(&self, db_path: &str) {
        info!("Loading ptypes from: {}", db_path);
        let path = Path::new(db_path);
        if !path.is_dir() {
            warn!("Ptype path does not exist or is not a directory: {}", db_path);
            return;
        }

        // Use std::fs for directory listing (one-time startup; acceptable to block briefly).
        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(e) => {
                error!("Cannot read ptype directory {}: {}", db_path, e);
                return;
            }
        };

        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().map_or(false, |ext| ext == "json") {
                match tokio::fs::read_to_string(&p).await {
                    Ok(content) => match serde_json::from_str::<Ptype>(&content) {
                        Ok(ptype) => {
                            info!("Loaded ptype: {}", ptype.name);
                            self.ptypes.lock().await.insert(ptype.name.clone(), ptype);
                        }
                        Err(e) => error!("Failed to parse {:?}: {}", p, e),
                    },
                    Err(e) => error!("Cannot read {:?}: {}", p, e),
                }
            }
        }
    }

    /// Return `true` if the signature's scope matches the message's scope integer.
    ///
    /// D-Bus wire encoding: 1 = session, 2 = file.
    fn scope_matches(sig_scope: Scope, msg_scope: i32) -> bool {
        match sig_scope {
            Scope::Session => msg_scope == 1,
            Scope::File => msg_scope == 2,
            Scope::Both => true,
        }
    }
}

// ---------------------------------------------------------------------------
// D-Bus interface implementation
// ---------------------------------------------------------------------------

#[zbus::interface(name = "org.cde.ToolTalk")]
impl ToolTalkBroker {
    /// Route a ToolTalk message from the caller to all matching ptypes.
    ///
    /// # Arguments
    /// * `op`    — operation name (e.g. `"DtEditor_Quit"`)
    /// * `args`  — encoded arguments as `(mode, vtype, value)` triples
    ///             (`a(sss)` in D-Bus notation)
    /// * `scope` — 1 = session scope, 2 = file scope
    /// * `file`  — file path for file-scope messages; empty string otherwise
    ///
    /// # Returns
    /// `TT_OK (0)` on success, or a negative `TT_ERR_*` code on failure.
    ///
    /// # Side effects
    /// * Spawns matching ptypes that have `Action::Start`.
    /// * Emits `MessageDelivered` on the session bus for observers.
    async fn send_message(
        &self,
        op: String,
        args: Vec<(String, String, String)>,
        scope: i32,
        file: String,
        #[zbus(header)] hdr: zbus::message::Header<'_>,
        #[zbus(signal_context)] ctx: SignalEmitter<'_>,
    ) -> i32 {
        // Extract the D-Bus unique name of the calling process for the signal.
        let sender = hdr
            .sender()
            .map(|s| s.as_str().to_owned())
            .unwrap_or_else(|| "unknown".to_owned());

        info!(
            "SendMessage: op='{}' scope={} file='{}' from='{}'",
            op, scope, file, sender
        );

        // Snapshot the ptype table so we release the lock before doing any I/O.
        let ptypes_snapshot: Vec<Ptype> = {
            let db = self.ptypes.lock().await;
            db.values().cloned().collect()
        };

        // Dispatch: find matching signatures and auto-start ptypes as required.
        for ptype in &ptypes_snapshot {
            for sig in &ptype.signatures {
                if sig.op != op {
                    continue;
                }
                if !Self::scope_matches(sig.scope, scope) {
                    continue;
                }

                info!(
                    "Match: ptype='{}' action={:?}",
                    ptype.name, sig.action
                );

                if sig.action == Action::Start {
                    match &ptype.start_string {
                        Some(cmd) => {
                            info!("Auto-starting ptype '{}': {}", ptype.name, cmd);
                            if let Err(e) = Command::new("sh").arg("-c").arg(cmd).spawn() {
                                error!("Failed to spawn '{}': {}", cmd, e);
                            }
                        }
                        None => warn!(
                            "Ptype '{}' has Start action but no start_string",
                            ptype.name
                        ),
                    }
                }
            }
        }

        // Broadcast MessageDelivered so all registered observers receive the message.
        if let Err(e) =
            Self::message_delivered(&ctx, &op, &args, scope, &file, &sender).await
        {
            error!("Failed to emit MessageDelivered for op='{}': {}", op, e);
            return TT_ERR_NOMP;
        }

        TT_OK
    }

    /// Signal emitted after a message has been accepted and dispatched.
    ///
    /// All processes that have subscribed to `org.cde.ToolTalk.MessageDelivered`
    /// (typically via a D-Bus match rule registered in their listen loop) will
    /// receive this signal.
    ///
    /// # Signal body (`a(sss)` args)
    /// Each element of `args` is a `(mode, vtype, value)` triple:
    ///
    /// * `mode`  — `"in"`, `"out"`, or `"inout"` (from CDE `Tt_mode`)
    /// * `vtype` — `"int"`, `"string"`, or `"bytes"`
    /// * `value` — decimal integer, UTF-8 string, or lowercase hex bytes
    #[zbus(signal)]
    async fn message_delivered(
        ctx: &SignalEmitter<'_>,
        op: &str,
        args: &[(String, String, String)],
        scope: i32,
        file: &str,
        sender: &str,
    ) -> zbus::Result<()>;

    /// Health-check endpoint — returns `"Pong"`.
    ///
    /// Used by monitoring tools and integration tests to verify the broker
    /// is alive without sending a real ToolTalk message.
    fn ping(&self) -> &'static str {
        info!("Ping received");
        "Pong"
    }

    /// List all currently loaded process type names.
    ///
    /// Useful for diagnostics: `busctl --user call org.cde.ToolTalk /org/cde/ToolTalk org.cde.ToolTalk ListPtypes`
    async fn list_ptypes(&self) -> Vec<String> {
        self.ptypes.lock().await.keys().cloned().collect()
    }
}
