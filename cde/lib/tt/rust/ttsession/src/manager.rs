use zbus::{dbus_interface, Connection, MessageStream, MessageType};
use futures::StreamExt;
use log::{info, error, warn};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::types::Ptype;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ToolTalkManager {
    // Store loaded ptypes: name -> Ptype
    ptypes: Arc<Mutex<HashMap<String, Ptype>>>,
}

impl ToolTalkManager {
// Imports moved to top

    pub fn new() -> Self {
        let ptypes = HashMap::new();
        
        ToolTalkManager {
            ptypes: Arc::new(Mutex::new(ptypes)),
        }
    }

    pub fn load_ptypes(&self, db_path: &str) {
         info!("Loading Ptypes from directory: {}", db_path);
         let path = Path::new(db_path);
         if !path.exists() || !path.is_dir() {
             warn!("Ptype database path does not exist or is not a directory: {}", db_path);
             return;
         }

         if let Ok(entries) = fs::read_dir(path) {
             for entry in entries.flatten() {
                 let path = entry.path();
                 if path.extension().map_or(false, |ext| ext == "json") {
                     info!("Reading ptype file: {:?}", path);
                     // Read content
                     if let Ok(content) = fs::read_to_string(&path) {
                         // Parse JSON
                         match serde_json::from_str::<Ptype>(&content) {
                             Ok(ptype) => {
                                 info!("Loaded Ptype: {}", ptype.name);
                                 if let Ok(mut db) = self.ptypes.lock() {
                                     db.insert(ptype.name.clone(), ptype);
                                 }
                             }
                             Err(e) => error!("Failed to parse JSON {:?}: {}", path, e),
                         }
                     }
                 }
             }
         }
    }

    pub async fn spawn_monitor(&self, conn: Connection) {
        let ptypes = self.ptypes.clone();
        tokio::spawn(async move {
            info!("Starting Signal Monitor...");
            
            // To add a match rule, we must call the DBus daemon method `AddMatch`.
            // zbus provides a proxy for the 'org.freedesktop.DBus' service.
            // In zbus 3.x, zbus::fdo::DBusProxy is the way.
            
            let rule = "type='signal',interface='org.cde.ToolTalk'";
            
            // Create proxy for the bus driver
            match zbus::fdo::DBusProxy::new(&conn).await {
                Ok(proxy) => {
                     match proxy.add_match(rule).await {
                        Ok(_) => info!("Match rule added: {}", rule),
                        Err(e) => {
                            error!("Failed to add match rule via DBusProxy: {}", e);
                            return;
                        }
                     }
                }
                Err(e) => {
                    error!("Failed to create DBusProxy: {}", e);
                    return;
                }
            }
            
            let mut stream = MessageStream::from(conn);
            
            while let Some(msg_res) = stream.next().await {
                match msg_res {
                    Ok(msg) => {
                        // msg.header() returns zbus::Result<MessageHeader>
                        match msg.header() {
                            Ok(header) => {
                                // header.message_type() returns MessageType (Result? No, typically Copy enum)
                                // Only look at signals
                                if let Ok(mtype) = header.message_type() {
                                    if mtype == MessageType::Signal {
                                        // header.interface() returns Result<Option<&InterfaceName>>
                                        if let Ok(Some(iface)) = header.interface() {
                                            if iface.as_str() == "org.cde.ToolTalk" {
                                                
                                                let member_str = match header.member() {
                                                    Ok(Some(m)) => m.as_str().to_string(),
                                                    _ => "?".to_string(),
                                                };
                                                
                                                let sender_str = match header.sender() {
                                                    Ok(Some(s)) => s.as_str().to_string(),
                                                    _ => "?".to_string(),
                                                };
                                                
                                                info!(">> BROKER SIGNAL: '{}' from {}", member_str, sender_str);

                                                // Routing Logic:
                                                // 1. Lock ptypes
                                                // 2. Iterate signatures
                                                // 3. Match 'op' == member_str
                                                // 4. If action == Start, launch proc (placeholder)
                                                
                                                if let Ok(db) = ptypes.lock() {
                                                    for (_, ptype) in db.iter() {
                                                        for sig in &ptype.signatures {
                                                            if sig.op == member_str {
                                                                info!("MATCH! Ptype='{}', Action={:?}", ptype.name, sig.action);
                                                                
                                                                if sig.action == crate::types::Action::Start {
                                                                    if let Some(cmd_str) = &ptype.start_string {
                                                                        info!("Auto-Starting: {}", cmd_str);
                                                                        // Execute command string using shell
                                                                        let _ = Command::new("sh")
                                                                            .arg("-c")
                                                                            .arg(cmd_str)
                                                                            .spawn()
                                                                            .map_err(|e| error!("Failed to spawn '{}': {}", cmd_str, e));
                                                                    } else {
                                                                        warn!("Start action requested but no start_string for Ptype '{}'", ptype.name);
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => warn!("Failed to parse message header: {}", e),
                        }
                    }
                    Err(e) => warn!("Bus stream error: {}", e),
                }
            }
        });
    }
}

#[dbus_interface(name = "org.cde.ToolTalk")]
impl ToolTalkManager {
    fn ping(&self) -> String {
        info!("Ping received");
        "Pong".to_string()
    }
}
