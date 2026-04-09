use futures::StreamExt;
use log::{error, info, warn};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};
use zbus::{message::Type as MessageType, Connection, MessageStream};

use crate::types::Ptype;

#[derive(Clone)]
pub struct ToolTalkManager {
    ptypes: Arc<Mutex<HashMap<String, Ptype>>>,
}

impl ToolTalkManager {
    pub fn new() -> Self {
        ToolTalkManager {
            ptypes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn load_ptypes(&self, db_path: &str) {
        info!("Loading ptypes from: {}", db_path);
        let path = Path::new(db_path);
        if !path.is_dir() {
            warn!("Ptype path does not exist or is not a directory: {}", db_path);
            return;
        }
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
                match fs::read_to_string(&p) {
                    Ok(content) => match serde_json::from_str::<Ptype>(&content) {
                        Ok(ptype) => {
                            info!("Loaded ptype: {}", ptype.name);
                            if let Ok(mut db) = self.ptypes.lock() {
                                db.insert(ptype.name.clone(), ptype);
                            }
                        }
                        Err(e) => error!("Failed to parse {:?}: {}", p, e),
                    },
                    Err(e) => error!("Cannot read {:?}: {}", p, e),
                }
            }
        }
    }

    /// Spawn a background task that monitors D-Bus for `org.cde.ToolTalk`
    /// signals and dispatches them according to loaded ptypes.
    pub async fn spawn_monitor(&self, conn: Connection) {
        let ptypes = self.ptypes.clone();
        tokio::spawn(async move {
            info!("ToolTalk signal monitor starting");

            // zbus 5: add a match rule via the bus driver proxy so the daemon
            // delivers org.cde.ToolTalk signals to us.
            let rule = zbus::MatchRule::builder()
                .msg_type(MessageType::Signal)
                .interface("org.cde.ToolTalk")
                .expect("hard-coded interface name is valid")
                .build();

            match zbus::fdo::DBusProxy::new(&conn).await {
                Ok(proxy) => {
                    if let Err(e) = proxy.add_match_rule(rule).await {
                        error!("Failed to add match rule: {}", e);
                        return;
                    }
                    info!("Match rule registered for org.cde.ToolTalk signals");
                }
                Err(e) => {
                    error!("Failed to create DBusProxy: {}", e);
                    return;
                }
            }

            let mut stream = MessageStream::from(&conn);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(msg) => {
                        // zbus 5: header() returns &MessageHeader directly (no Result).
                        let header = msg.header();

                        if header.message_type() != MessageType::Signal {
                            continue;
                        }

                        // zbus 5: interface() / member() / sender() return Option
                        // directly, not Result<Option<...>>.
                        let iface = match header.interface() {
                            Some(i) => i.as_str().to_owned(),
                            None => continue,
                        };
                        if iface != "org.cde.ToolTalk" {
                            continue;
                        }

                        let member = header
                            .member()
                            .map(|m| m.as_str().to_owned())
                            .unwrap_or_default();
                        let sender = header
                            .sender()
                            .map(|s| s.as_str().to_owned())
                            .unwrap_or_else(|| "?".to_owned());

                        info!("SIGNAL '{}' from {}", member, sender);

                        if let Ok(db) = ptypes.lock() {
                            for ptype in db.values() {
                                for sig in &ptype.signatures {
                                    if sig.op == member {
                                        info!(
                                            "Match: ptype='{}' action={:?}",
                                            ptype.name, sig.action
                                        );
                                        if sig.action == crate::types::Action::Start {
                                            match &ptype.start_string {
                                                Some(cmd) => {
                                                    info!("Auto-starting: {}", cmd);
                                                    if let Err(e) = Command::new("sh")
                                                        .arg("-c")
                                                        .arg(cmd)
                                                        .spawn()
                                                    {
                                                        error!(
                                                            "Failed to spawn '{}': {}",
                                                            cmd, e
                                                        );
                                                    }
                                                }
                                                None => warn!(
                                                    "Start action but no start_string for '{}'",
                                                    ptype.name
                                                ),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => warn!("Bus stream error: {}", e),
                }
            }

            info!("ToolTalk signal monitor exited");
        });
    }
}

// zbus 5: #[interface] replaces the deprecated #[dbus_interface] macro.
#[zbus::interface(name = "org.cde.ToolTalk")]
impl ToolTalkManager {
    fn ping(&self) -> String {
        info!("Ping received");
        "Pong".to_string()
    }

    fn list_ptypes(&self) -> Vec<String> {
        self.ptypes
            .lock()
            .map(|db| db.keys().cloned().collect())
            .unwrap_or_default()
    }
}
