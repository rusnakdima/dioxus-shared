//! Dioxus MCP Bridge - GUI inspection for Dioxus Desktop
//!
//! This library adds WebSocket-based GUI inspection to Dioxus Desktop apps.
//! Uses blocking I/O and std threads to avoid tokio runtime conflicts.
//!
//! ## Architecture
//!
//! ```text
//! Bridge Thread          Dioxus App (main thread)
//!     │                        │
//!     │ command queue ─────────►│
//!     │◄─── response queue ────│
//!     │                        │
//! ```
//!
//! The bridge puts commands in a queue. The Dioxus app polls this queue,
//! evaluates JS via the webview, and puts results in the response queue.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use dioxus_shared::mcp::bridge::{McpBridge, BridgeState};
//! use std::sync::Arc;
//!
//! fn main() {
//!     let (bridge, state) = McpBridge::new(9223);
//!
//!     // Run bridge in background thread
//!     let bridge_state = state.clone();
//!     std::thread::spawn(move || {
//!         bridge.run();
//!     });
//!
//!     // In Dioxus app component, use use_effect to poll:
//!     let state = state.clone();
//!     use_effect(move || {
//!         let state = state.clone();
//!         std::thread::spawn(move || {
//!             loop {
//!                 let commands = state.dequeue_all();
//!                 for cmd in commands {
//!                     // Process command via webview
//!                 }
//!                 std::thread::sleep(std::time::Duration::from_millis(10));
//!             }
//!         });
//!     });
//! }
//! ```

mod commands;
pub mod state;
mod websocket;

#[cfg(feature = "dioxus-desktop")]
pub mod webview_integration;

#[cfg(feature = "dioxus-desktop")]
pub use webview_integration::{deliver_eval_results, handle_eval_command};

use std::sync::Arc;

pub use commands::AppInfo;
pub use state::{BridgeState, BridgeStateHandle, Command, EvalRequest, Response};

/// MCP Bridge for Dioxus Desktop applications
pub struct McpBridge {
    port: u16,
    state: Arc<BridgeState>,
}

impl McpBridge {
    /// Create a new MCP Bridge on the specified port
    /// Returns the bridge and shared state
    pub fn new(port: u16) -> (Self, Arc<BridgeState>) {
        let state = Arc::new(BridgeState::new());
        (
            Self {
                port,
                state: state.clone(),
            },
            state,
        )
    }

    /// Get the shared state
    pub fn state(&self) -> Arc<BridgeState> {
        self.state.clone()
    }

    /// Start the WebSocket server (blocking)
    pub fn run(&self) {
        websocket::run(self.port, self.state.clone());
    }
}
