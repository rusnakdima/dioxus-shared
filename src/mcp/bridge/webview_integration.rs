//! Webview integration helpers for the MCP bridge
//!
//! ## Thread Safety Note
//!
//! `DesktopContext` (= `Rc<DesktopService>`) is **not** `Send`. The webview eval
//! **must** run on the Dioxus main thread — use `use_effect` inside a Dioxus
//! component, NOT `std::thread::spawn`.
//!
//! ## Recommended Pattern: use_effect
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_shared::mcp::bridge::{BridgeState, handle_eval_command, deliver_eval_results};
//!
//! fn App() -> Element {
//!     // Get BridgeState from context (provided when you called BridgeStateHandle::new())
//!     let state = use_context::<Arc<BridgeState>>().unwrap();
//!
//!     // use_window() gives DesktopContext (must stay on main thread)
//!     let desktop = dioxus_desktop::use_window();
//!
//!     let cancelled = use_signal(|| false);
//!     use_effect(move || {
//!         if cancelled() { return; }
//!         loop {
//!             if cancelled() { break; }
//!             // 1. Dequeue and handle ALL pending bridge commands
//!             let commands = state.dequeue_all();
//!             for cmd in commands {
//!                 // eval commands: enqueue for webview processing (step 2)
//!                 // sync commands: process immediately
//!                 if !handle_eval_command(&state, &cmd) {
//!                     let resp = process_my_command(&cmd);
//!                     state.set_response(cmd.id, resp);
//!                 }
//!             }
//!
//!             // 2. Process any pending eval requests via the webview
//!             //    (must be on main thread where DesktopContext is valid)
//!             let eval_requests = state.dequeue_eval_requests();
//!             for request in eval_requests {
//!                 let result = process_eval_on_main_thread(&desktop, &request);
//!                 state.enqueue_js_result(request.id, result);
//!             }
//!
//!             // 3. Deliver eval results to waiting bridge clients
//!             deliver_eval_results(&state);
//!
//!             std::thread::sleep(std::time::Duration::from_millis(10));
//!         }
//!     });
//!     on_cleanup(|| cancelled.set(true));
//!
//!     rsx! { ... }
//! }
//!
//! fn process_eval_on_main_thread(
//!     desktop: &dioxus_desktop::DesktopContext,
//!     request: &EvalRequest,
//! ) -> String {
//!     use std::ops::Deref;
//!     let webview = desktop.deref();
//!     // webview is &DesktopService, its webview field is WebView
//!     match request.method.as_str() {
//!         "evaluate_js" => {
//!             let code = request.params.get("code").and_then(|v| v.as_str()).unwrap_or("");
//!             let js = format!(
//!                 "(function(){{try{{return JSON.stringify((function(){{return eval({})}}())}}catch(e){{return JSON.stringify({{error:e.toString()}})}})}})()",
//!                 serde_json::Value::String(code.to_string())
//!             );
//!             match webview.webview.evaluate_script(&js) {
//!                 Ok(v) => v,
//!                 Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
//!             }
//!         }
//!         "dom_snapshot" => {
//!             let selector = request.params.get("selector").and_then(|v| v.as_str());
//!             let js = if let Some(sel) = selector {
//!                 let escaped = sel.replace('\\', "\\\\").replace('\'', "\\'");
//!                 format!("(function(){{var el=document.querySelector('{}');return el?el.outerHTML:null}})()", escaped)
//!             } else {
//!                 "(function(){return document.body.innerHTML})()".to_string()
//!             };
//!             match webview.webview.evaluate_script(&js) {
//!                 Ok(v) => v,
//!                 Err(e) => serde_json::json!({"error": e.to_string()}).to_string(),
//!             }
//!         }
//!         _ => serde_json::json!({"error": "unknown eval method"}).to_string(),
//!     }
//! }
//! ```

use super::state::{
    BridgeState, Command, EvalRequest, IpcEvent, KeyboardRequest, Response, WindowRequest,
};
use std::sync::Arc;

/// Route a bridge command to the eval queue if it's `evaluate_js` or `dom_snapshot`.
/// Returns `true` if the command was handled as an eval command (caller should NOT
/// set response immediately — the eval loop will set it later).
///
/// ```rust,ignore
/// for cmd in state.dequeue_all() {
///     let is_eval = handle_eval_command(&state, &cmd);
///     if !is_eval {
///         let resp = process_my_sync_command(&cmd);
///         state.set_response(cmd.id, resp);
///     }
/// }
/// ```
pub fn handle_eval_command(state: &Arc<BridgeState>, cmd: &Command) -> bool {
    match cmd.method.as_str() {
        "evaluate_js" | "dom_snapshot" => {
            state.enqueue_eval_request(EvalRequest {
                id: cmd.id.clone(),
                method: cmd.method.clone(),
                params: cmd.params.clone(),
            });
            true
        }
        "webview_keyboard" => {
            let key = cmd.params.get("key").and_then(|v| v.as_str()).unwrap_or("");
            let modifiers = cmd.params.get("modifiers").cloned();
            let action = cmd
                .params
                .get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("press");

            state.enqueue_keyboard_request(KeyboardRequest {
                id: cmd.id.clone(),
                key: key.to_string(),
                modifiers,
                action: action.to_string(),
            });
            true
        }
        "manage_window" => {
            let action = cmd
                .params
                .get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("list");
            let window_id = cmd
                .params
                .get("window_id")
                .and_then(|v| v.as_str())
                .map(String::from);
            let width = cmd
                .params
                .get("width")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32);
            let height = cmd
                .params
                .get("height")
                .and_then(|v| v.as_u64())
                .map(|v| v as u32);
            let x = cmd
                .params
                .get("x")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32);
            let y = cmd
                .params
                .get("y")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32);

            state.enqueue_window_request(WindowRequest {
                id: cmd.id.clone(),
                action: action.to_string(),
                window_id,
                width,
                height,
                x,
                y,
            });
            true
        }
        "ipc_emit_event" => {
            let event = cmd
                .params
                .get("event")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let payload = cmd
                .params
                .get("payload")
                .cloned()
                .unwrap_or(serde_json::json!({}));

            state.enqueue_ipc_event(IpcEvent {
                event: event.to_string(),
                payload,
            });

            state.set_response(
                cmd.id.clone(),
                Response {
                    result: Some(serde_json::json!({"ok": true})),
                    error: None,
                },
            );
            true
        }
        "ipc_monitor" => {
            let enabled = cmd
                .params
                .get("enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            // ipc_monitor just enables/disables capturing - we store this in a dedicated field
            // For now, just acknowledge the command
            // The actual IPC event capture would be done by the Dioxus app when it emits events
            state.set_response(
                cmd.id.clone(),
                Response {
                    result: Some(serde_json::json!({"enabled": enabled})),
                    error: None,
                },
            );
            true
        }
        "webview_wait_for" => {
            // webview_wait_for requires main-thread polling via the webview
            // Return a placeholder response — the app's main thread loop drains pending_eval_requests
            // and can implement polling there; BridgeState has no pending_wait_for_requests queue
            state.set_response(cmd.id.clone(), Response {
                result: Some(serde_json::json!({"status": "not_implemented", "message": "webview_wait_for requires main-thread processing"})),
                error: None,
            });
            true
        }
        "webview_select_element" => {
            let mode = cmd
                .params
                .get("mode")
                .and_then(|v| v.as_str())
                .unwrap_or("default");

            state.set_response(
                cmd.id.clone(),
                Response {
                    result: Some(serde_json::json!({"mode": mode, "status": "not_implemented"})),
                    error: None,
                },
            );
            true
        }
        "webview_get_pointed_element" => {
            let x = cmd
                .params
                .get("x")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32);
            let y = cmd
                .params
                .get("y")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32);

            state.set_response(
                cmd.id.clone(),
                Response {
                    result: Some(
                        serde_json::json!({"x": x, "y": y, "element": serde_json::Value::Null}),
                    ),
                    error: None,
                },
            );
            true
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn make_cmd(id: &str, method: &str, params: serde_json::Value) -> Command {
        Command {
            id: id.to_string(),
            method: method.to_string(),
            params,
            received_at: Instant::now(),
        }
    }

    // --- routing: return value ---

    #[test]
    fn handle_eval_command_returns_true_for_evaluate_js() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd("e1", "evaluate_js", serde_json::json!({"code": "1+1"}));
        let handled = handle_eval_command(&state, &cmd);
        assert!(handled, "evaluate_js should return true");
    }

    #[test]
    fn handle_eval_command_returns_true_for_dom_snapshot() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd(
            "d1",
            "dom_snapshot",
            serde_json::json!({"selector": "body"}),
        );
        let handled = handle_eval_command(&state, &cmd);
        assert!(handled, "dom_snapshot should return true");
    }

    #[test]
    fn handle_eval_command_returns_true_for_webview_keyboard() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd(
            "k1",
            "webview_keyboard",
            serde_json::json!({"key": "Enter", "action": "press"}),
        );
        let handled = handle_eval_command(&state, &cmd);
        assert!(handled, "webview_keyboard should return true");
    }

    #[test]
    fn handle_eval_command_returns_false_for_ping() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd("p1", "ping", serde_json::json!({}));
        let handled = handle_eval_command(&state, &cmd);
        assert!(!handled, "ping should return false");
    }

    #[test]
    fn handle_eval_command_returns_false_for_commands_list() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd("c1", "commands_list", serde_json::json!({}));
        let handled = handle_eval_command(&state, &cmd);
        assert!(!handled, "commands_list should return false");
    }

    // --- routing: queue contents ---

    #[test]
    fn handle_eval_command_evaluate_js_enqueues_pending_eval_request() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd(
            "rq-id",
            "evaluate_js",
            serde_json::json!({"code": "document.title"}),
        );
        handle_eval_command(&state, &cmd);

        let pending = state.dequeue_eval_requests();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, "rq-id");
        assert_eq!(pending[0].method, "evaluate_js");
        assert_eq!(pending[0].params["code"], "document.title");
    }

    #[test]
    fn handle_eval_command_dom_snapshot_enqueues_pending_eval_request() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd(
            "ds-id",
            "dom_snapshot",
            serde_json::json!({"selector": "#main"}),
        );
        handle_eval_command(&state, &cmd);

        let pending = state.dequeue_eval_requests();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, "ds-id");
        assert_eq!(pending[0].method, "dom_snapshot");
        assert_eq!(pending[0].params["selector"], "#main");
    }

    #[test]
    fn handle_eval_command_webview_keyboard_enqueues_pending_keyboard_request() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd(
            "kb-id",
            "webview_keyboard",
            serde_json::json!({
                "key": "a",
                "modifiers": ["ctrl"],
                "action": "press"
            }),
        );
        handle_eval_command(&state, &cmd);

        let pending = state.dequeue_keyboard_requests();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, "kb-id");
        assert_eq!(pending[0].key, "a");
        assert_eq!(pending[0].modifiers, Some(serde_json::json!(["ctrl"])));
        assert_eq!(pending[0].action, "press");
    }

    // --- immediate response: ipc_emit_event ---

    #[test]
    fn handle_eval_command_ipc_emit_event_sets_response_immediately() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd(
            "ipc-emit-1",
            "ipc_emit_event",
            serde_json::json!({
                "event": "my-event",
                "payload": {"x": 42}
            }),
        );
        let handled = handle_eval_command(&state, &cmd);
        assert!(handled, "ipc_emit_event should return true");

        // Response must be set immediately
        let resp = state
            .get_response("ipc-emit-1")
            .expect("response should be set");
        assert!(resp.result.is_some(), "result should be Some");
        let result = resp.result.unwrap();
        assert!(result["ok"].as_bool().unwrap_or(false), "ok should be true");
    }

    #[test]
    fn handle_eval_command_ipc_emit_event_also_enqueues_event() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd(
            "ipc-emit-2",
            "ipc_emit_event",
            serde_json::json!({
                "event": "click",
                "payload": {"button": 1}
            }),
        );
        handle_eval_command(&state, &cmd);

        let events = state.dequeue_ipc_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, "click");
        assert_eq!(events[0].payload["button"], 1);
    }

    // --- immediate response: ipc_monitor ---

    #[test]
    fn handle_eval_command_ipc_monitor_sets_response_immediately() {
        let state = Arc::new(BridgeState::new());
        let cmd = make_cmd(
            "ipc-mon-1",
            "ipc_monitor",
            serde_json::json!({"enabled": true}),
        );
        let handled = handle_eval_command(&state, &cmd);
        assert!(handled, "ipc_monitor should return true");

        let resp = state
            .get_response("ipc-mon-1")
            .expect("response should be set");
        assert!(resp.result.is_some());
        assert_eq!(resp.result.unwrap()["enabled"], true);
    }
}

/// Drain `pending_js_results` and deliver each to the bridge response channel.
/// Call this every iteration of the bridge consumer loop.
pub fn deliver_eval_results(state: &Arc<BridgeState>) {
    let results = state.dequeue_js_results();
    for (id, result) in results {
        let resp = Response {
            result: Some(serde_json::json!({ "result": result })),
            error: None,
        };
        state.set_response(id, resp);
    }
}
