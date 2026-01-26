use anyhow::Result;
use log::info;
use zbus::ConnectionBuilder;
use std::future::pending;
use crate::manager::ToolTalkManager;

mod manager;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting ttsession (Rust D-Bus Broker)...");

    let manager = ToolTalkManager::new();
    
    // Default location for compiled ptypes (JSON)
    // In production, this might be /usr/dt/appconfig/types/C/tt/compiled or similar.
    // We'll use a relative path or CDE_CONFIGURATION_TOP based one.
    // For now: /etc/dt/tt/types
    manager.load_ptypes("/etc/dt/tt/types");

    let conn = ConnectionBuilder::session()?
        .name("org.cde.ToolTalk")?
        .serve_at("/org/cde/ToolTalk", manager.clone())? // ToolTalkManager needs to be Clone? It has Arc.
        .build()
        .await?;

    info!("ttsession started on D-Bus: org.cde.ToolTalk");
    
    // Spawn signal monitor to sniff/route messages
    // We need access to the manager instance.
    // Connection::serve_at takes ownership or specific type? zbus 3.x usually takes Clone or moves.
    // If we move manager into serve_at, we can't use it.
    // Let's check if ToolTalkManager derives Clone.
    
    // Actually, serve_at takes the interface implementation.
    // If I want to use it afterwards, I should implement Clone (easy, it just has Arc).
    
    // But `serve_at` consumes it unless it's a clone.
    
    // Wait, let's clone it before serving.
    // But I haven't implemented Clone for ToolTalkManager yet.
    
    // Alternative: spawn monitor BEFORE serving? No, monitor needs connection.
    // But connection builder needs manager.
    
    // I need to implement Clone for ToolTalkManager in manager.rs first.
    
    // For now, let's update main.rs assuming Clone or looking for solution.
    // If I change `spawn_monitor` to not need self, but `ptypes`, I'm back to before.
    // But instance method is cleaner.
    
    // Providing 'manager' to serve_at consumes it.
    // I will implement Clone for ToolTalkManager (manual or derive).
    
    manager.spawn_monitor(conn.clone()).await;

    // Do not exit
    pending::<()>().await;
    
    Ok(())
}
