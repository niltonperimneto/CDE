// SPDX-License-Identifier: LGPL-2.0-or-later
//
// notifier.rs — Send the `DtTypes_Reloaded` D-Bus signal to notify CDE
// applications that the action/datatype database has changed.
//
// Historically this was a ToolTalk `TT_NOTICE` message observed by
// `DtDbReloadNotify()` in `Action.c`.  Since the migration replaces ToolTalk
// with D-Bus, we emit an equivalent session-bus signal that the Rust
// `libtt_shim` bridge will relay into ToolTalk for legacy clients.

use anyhow::Result;

/// D-Bus well-known name for the CDE types-reload service.
const BUS_NAME: &str = "org.cde.TypesReload";

/// D-Bus object path.
const OBJECT_PATH: &str = "/org/cde/TypesReload";

/// D-Bus interface.
const INTERFACE: &str = "org.cde.TypesReload";

/// D-Bus signal name (mirrors the `ToolTalk` op `DtTypes_Reloaded`).
const SIGNAL_NAME: &str = "DtTypesReloaded";

/// Emit the `DtTypesReloaded` signal on the session bus.
///
/// This uses a one-shot connection — it connects, sends, and disconnects.
/// The overhead is negligible because reload events are rare (order of
/// seconds at most).
pub async fn send_reload_signal() -> Result<()> {
    let connection = zbus::Connection::session().await?;

    // Request the well-known name so listeners can match on sender.
    // WellKnownName::try_from validates the D-Bus naming rules.
    let name = zbus::names::WellKnownName::try_from(BUS_NAME)?;
    connection.request_name(name).await?;

    // Build and emit the signal using the zbus 5 Message::signal builder.
    let signal = zbus::message::Message::signal(OBJECT_PATH, INTERFACE, SIGNAL_NAME)?
        .build(&())?;

    connection.send(&signal).await?;

    log::info!("emitted D-Bus signal {INTERFACE}.{SIGNAL_NAME}");
    Ok(())
}

/// A debounced reload notifier.
///
/// When multiple `.desktop` files change in rapid succession (e.g. a package
/// manager installing 50 apps), we coalesce the reload signals into a single
/// emission after a quiet period.
///
/// This uses a `tokio::sync::Notify` + timer approach:
///
/// 1. Every translation triggers `request()`.
/// 2. A background task waits for `DEBOUNCE_MS` of quiet time.
/// 3. After the quiet period, it sends one D-Bus signal.
pub struct DebouncedNotifier {
    trigger: tokio::sync::Notify,
}

impl DebouncedNotifier {
    /// Debounce window in milliseconds.
    const DEBOUNCE_MS: u64 = 500;

    pub fn new() -> Self {
        Self {
            trigger: tokio::sync::Notify::new(),
        }
    }

    /// Request a reload signal.  Non-blocking; returns immediately.
    pub fn request(&self) {
        self.trigger.notify_one();
    }

    /// Background loop — runs forever, sending debounced reload signals.
    ///
    /// Cancel by dropping the `JoinHandle`.
    pub async fn run(&self) {
        loop {
            // Wait for at least one request.
            self.trigger.notified().await;

            // Drain any additional requests during the debounce window.
            loop {
                let delay = tokio::time::sleep(
                    std::time::Duration::from_millis(Self::DEBOUNCE_MS),
                );
                tokio::select! {
                    () = self.trigger.notified() => {
                        // Another request came in — restart the timer.
                        continue;
                    }
                    () = delay => {
                        // Quiet period elapsed — send the signal.
                        break;
                    }
                }
            }

            if let Err(e) = send_reload_signal().await {
                log::warn!("failed to send reload signal: {e}");
            }
        }
    }
}
