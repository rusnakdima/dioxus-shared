//! Bridge state shared between MCP WebSocket server and Dioxus app
//!
//! Architecture:
//! - WebSocket server thread enqueues Commands and waits for Responses
//! - Dioxus app polls for Commands and sets Responses
//! - For evaluate_js/dom_snapshot, the App component's eval loop picks up
//!   pending commands from pending_eval_requests, calls webview, stores results

use std::collections::{HashMap, VecDeque};
use std::sync::{
    atomic::{AtomicBool, AtomicU16, Ordering},
    Arc, Condvar, Mutex,
};
use std::time::Instant;

/// An eval request (evaluate_js, dom_snapshot) to be processed by the Dioxus main thread
#[derive(Debug, Clone)]
pub struct EvalRequest {
    pub id: String,
    pub method: String,
    pub params: serde_json::Value,
}

/// A command received from the MCP client (via WebSocket)
#[derive(Debug, Clone)]
pub struct Command {
    pub id: String,
    pub method: String,
    pub params: serde_json::Value,
    pub received_at: Instant,
}

/// A response to return to the MCP client
#[derive(Debug, Clone)]
pub struct Response {
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// An IPC event for queuing (ipc_emit_event/ipc_monitor)
#[derive(Debug, Clone)]
pub struct IpcEvent {
    pub event: String,
    pub payload: serde_json::Value,
}

/// A keyboard request for webview_keyboard
#[derive(Debug, Clone)]
pub struct KeyboardRequest {
    pub id: String,
    pub key: String,
    pub modifiers: Option<serde_json::Value>,
    pub action: String,
}

/// A window request for manage_window
#[derive(Debug, Clone)]
pub struct WindowRequest {
    pub id: String,
    pub action: String,
    pub window_id: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub x: Option<i32>,
    pub y: Option<i32>,
}

/// Thread-safe bridge state shared between WebSocket thread and Dioxus app
#[derive(Debug)]
pub struct BridgeState {
    /// Pending commands to process (WebSocket thread -> Dioxus app)
    pending: Mutex<VecDeque<Command>>,
    /// Responses from Dioxus app (Dioxus app -> WebSocket thread)
    responses: Mutex<HashMap<String, Response>>,
    /// Pending JS evaluation results (webview callback thread -> polling thread)
    /// (command_id, result_json_string)
    pending_js_results: Mutex<VecDeque<(String, String)>>,
    /// Pending eval requests (evaluate_js, dom_snapshot) — consumed by Dioxus main thread
    pending_eval_requests: Mutex<VecDeque<EvalRequest>>,
    /// Pending eval request IDs awaiting webview callback (Dioxus main thread polls this)
    pending_eval_ids: Mutex<VecDeque<String>>,
    /// In-memory log buffer (circular, max 10_000 lines)
    log_buffer: Mutex<Vec<String>>,
    /// Flag to signal shutdown
    shutdown: AtomicBool,
    /// Condvar for efficient response wait (WebSocket thread waits here)
    response_condvar: Arc<(Mutex<()>, Condvar)>,
    /// Actual port the bridge bound to (may differ from requested if retry succeeded)
    bound_port: AtomicU16,
    /// IPC event buffer (max 1000 events, FIFO)
    ipc_event_buffer: Mutex<VecDeque<IpcEvent>>,
    /// Pending keyboard requests for webview_keyboard
    pending_keyboard_requests: Mutex<VecDeque<KeyboardRequest>>,
    /// Pending window requests for manage_window
    pending_window_requests: Mutex<VecDeque<WindowRequest>>,
}

impl BridgeState {
    /// Create new bridge state
    pub fn new() -> Self {
        Self {
            pending: Mutex::new(VecDeque::new()),
            responses: Mutex::new(HashMap::new()),
            pending_js_results: Mutex::new(VecDeque::new()),
            pending_eval_requests: Mutex::new(VecDeque::new()),
            pending_eval_ids: Mutex::new(VecDeque::new()),
            log_buffer: Mutex::new(Vec::with_capacity(10_000)),
            shutdown: AtomicBool::new(false),
            response_condvar: Arc::new((Mutex::new(()), Condvar::new())),
            bound_port: AtomicU16::new(0),
            ipc_event_buffer: Mutex::new(VecDeque::new()),
            pending_keyboard_requests: Mutex::new(VecDeque::new()),
            pending_window_requests: Mutex::new(VecDeque::new()),
        }
    }

    /// Record the actual port the bridge bound to (used after retry-based bind).
    pub fn set_bound_port(&self, port: u16) {
        self.bound_port.store(port, Ordering::SeqCst);
    }

    /// Get the actual port the bridge bound to (0 if not yet bound).
    pub fn bound_port(&self) -> u16 {
        self.bound_port.load(Ordering::SeqCst)
    }

    /// Check if shutdown was requested
    pub fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::SeqCst)
    }

    /// Request shutdown
    pub fn request_shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
    }

    /// Enqueue a command from the WebSocket server
    pub fn enqueue(&self, cmd: Command) {
        match self.pending.lock() {
            Ok(mut guard) => guard.push_back(cmd),
            Err(poisoned) => {
                let mut guard = poisoned.into_inner();
                guard.push_back(cmd);
            }
        }
    }

    /// Dequeue all pending commands
    pub fn dequeue_all(&self) -> Vec<Command> {
        let mut pending = match self.pending.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        pending.drain(..).collect()
    }

    /// Check if there are pending commands
    pub fn has_pending(&self) -> bool {
        !match self.pending.lock() {
            Ok(guard) => guard.is_empty(),
            Err(poisoned) => poisoned.into_inner().is_empty(),
        }
    }

    /// Set a response for a command
    pub fn set_response(&self, id: String, resp: Response) {
        {
            let mut responses = match self.responses.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            responses.insert(id, resp);
        }
        self.notify_response();
    }

    /// Get and remove a response for a command ID
    pub fn get_response(&self, id: &str) -> Option<Response> {
        match self.responses.lock() {
            Ok(mut guard) => guard.remove(id),
            Err(poisoned) => poisoned.into_inner().remove(id),
        }
    }

    /// Wait for a response with timeout using Condvar.
    /// Returns the response if available before timeout, else None.
    pub fn wait_for_response(&self, id: &str, timeout: std::time::Duration) -> Option<Response> {
        let (lock, cvar) = &*self.response_condvar;
        let mut guard = match lock.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        let start = std::time::Instant::now();
        loop {
            if let Some(resp) = match self.responses.lock() {
                Ok(mut guard) => guard.remove(id),
                Err(poisoned) => poisoned.into_inner().remove(id),
            } {
                return Some(resp);
            }
            let elapsed = start.elapsed();
            if elapsed >= timeout {
                return None;
            }
            let remaining = timeout - elapsed;
            let (new_guard, _timeout_result) = match cvar.wait_timeout(guard, remaining) {
                Ok((g, r)) => (g, r),
                Err(_poisoned) => {
                    // Thread was poisoned — recover and re-acquire the lock
                    guard = match lock.lock() {
                        Ok(g) => g,
                        Err(poisoned) => poisoned.into_inner(),
                    };
                    continue;
                }
            };
            guard = new_guard;
            // Check responses again after wakeup before re-waiting
            if let Some(resp) = match self.responses.lock() {
                Ok(mut guard) => guard.remove(id),
                Err(poisoned) => poisoned.into_inner().remove(id),
            } {
                return Some(resp);
            }
        }
    }

    /// Notify the waiters that a response may be available.
    pub fn notify_response(&self) {
        let (_, cvar) = &*self.response_condvar;
        cvar.notify_all();
    }

    /// Get the count of pending commands
    pub fn pending_count(&self) -> usize {
        match self.pending.lock() {
            Ok(guard) => guard.len(),
            Err(poisoned) => poisoned.into_inner().len(),
        }
    }

    /// Enqueue a JS evaluation result (called from webview callback thread)
    pub fn enqueue_js_result(&self, id: String, result: String) {
        match self.pending_js_results.lock() {
            Ok(mut guard) => guard.push_back((id, result)),
            Err(poisoned) => {
                let mut guard = poisoned.into_inner();
                guard.push_back((id, result));
            }
        }
    }

    /// Drain all pending JS results and return them as a vector
    pub fn dequeue_js_results(&self) -> Vec<(String, String)> {
        let mut results = match self.pending_js_results.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        results.drain(..).collect()
    }

    /// Enqueue an eval request (evaluate_js, dom_snapshot) for the Dioxus main thread
    pub fn enqueue_eval_request(&self, request: EvalRequest) {
        match self.pending_eval_requests.lock() {
            Ok(mut guard) => guard.push_back(request),
            Err(poisoned) => {
                let mut guard = poisoned.into_inner();
                guard.push_back(request);
            }
        }
    }

    /// Dequeue all pending eval requests
    pub fn dequeue_eval_requests(&self) -> Vec<EvalRequest> {
        let mut requests = match self.pending_eval_requests.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        requests.drain(..).collect()
    }

    /// Store an eval request ID awaiting webview callback (called from main thread)
    pub fn store_pending_eval(&self, id: String) {
        match self.pending_eval_ids.lock() {
            Ok(mut guard) => guard.push_back(id),
            Err(poisoned) => {
                let mut guard = poisoned.into_inner();
                guard.push_back(id);
            }
        }
    }

    /// Take one pending eval ID if available (called from main thread after dequeue)
    pub fn take_pending_eval_id(&self) -> Option<String> {
        match self.pending_eval_ids.lock() {
            Ok(mut guard) => guard.pop_front(),
            Err(poisoned) => poisoned.into_inner().pop_front(),
        }
    }

    /// Append a log line to the in-memory buffer (max 10_000 lines, FIFO)
    pub fn append_log(&self, line: String) {
        let mut buffer = match self.log_buffer.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if buffer.len() >= 10_000 {
            buffer.remove(0);
        }
        buffer.push(line);
    }

    /// Get all log lines (for logs_read command)
    pub fn get_logs(&self) -> Vec<String> {
        match self.log_buffer.lock() {
            Ok(guard) => guard.clone(),
            Err(poisoned) => poisoned.into_inner().clone(),
        }
    }

    /// Enqueue an IPC event (max 1000 events, FIFO)
    pub fn enqueue_ipc_event(&self, event: IpcEvent) {
        let mut buffer = match self.ipc_event_buffer.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if buffer.len() >= 1000 {
            buffer.pop_front();
        }
        buffer.push_back(event);
    }

    /// Dequeue all IPC events and return them
    pub fn dequeue_ipc_events(&self) -> Vec<IpcEvent> {
        let mut buffer = match self.ipc_event_buffer.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        buffer.drain(..).collect()
    }

    /// Get a clone of all IPC events (for ipc_get_captured)
    pub fn get_ipc_events(&self) -> Vec<IpcEvent> {
        match self.ipc_event_buffer.lock() {
            Ok(guard) => guard.iter().cloned().collect(),
            Err(poisoned) => poisoned.into_inner().iter().cloned().collect(),
        }
    }

    /// Clear all IPC events
    pub fn clear_ipc_events(&self) {
        if let Ok(mut buffer) = self.ipc_event_buffer.lock() {
            buffer.clear();
        }
    }

    /// Enqueue a keyboard request
    pub fn enqueue_keyboard_request(&self, request: KeyboardRequest) {
        match self.pending_keyboard_requests.lock() {
            Ok(mut guard) => guard.push_back(request),
            Err(poisoned) => {
                let mut guard = poisoned.into_inner();
                guard.push_back(request);
            }
        }
    }

    /// Dequeue all pending keyboard requests
    pub fn dequeue_keyboard_requests(&self) -> Vec<KeyboardRequest> {
        let mut requests = match self.pending_keyboard_requests.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        requests.drain(..).collect()
    }

    /// Enqueue a window request
    pub fn enqueue_window_request(&self, request: WindowRequest) {
        match self.pending_window_requests.lock() {
            Ok(mut guard) => guard.push_back(request),
            Err(poisoned) => {
                let mut guard = poisoned.into_inner();
                guard.push_back(request);
            }
        }
    }

    /// Dequeue all pending window requests
    pub fn dequeue_window_requests(&self) -> Vec<WindowRequest> {
        let mut requests = match self.pending_window_requests.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        requests.drain(..).collect()
    }
}

impl Default for BridgeState {
    fn default() -> Self {
        Self::new()
    }
}

/// Newtype wrapper for Arc<BridgeState> that implements PartialEq
/// Uses pointer equality since BridgeState is a singleton context
#[derive(Debug, Clone)]
pub struct BridgeStateHandle(pub Arc<BridgeState>);

impl PartialEq for BridgeStateHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl BridgeStateHandle {
    /// Create a new BridgeStateHandle wrapping a fresh BridgeState
    pub fn new() -> Self {
        BridgeStateHandle(Arc::new(BridgeState::new()))
    }

    /// Get the underlying BridgeState
    pub fn get_state(&self) -> Arc<BridgeState> {
        self.0.clone()
    }
}

impl Default for BridgeStateHandle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wait_for_response_returns_none_on_timeout() {
        let state = BridgeState::new();
        let result =
            state.wait_for_response("nonexistent-id", std::time::Duration::from_millis(50));
        assert!(result.is_none());
    }

    #[test]
    fn wait_for_response_returns_response_when_set() {
        let state = BridgeState::new();
        state.set_response(
            "test-id".to_string(),
            Response {
                result: Some(serde_json::json!({"ok": true})),
                error: None,
            },
        );
        let result = state.wait_for_response("test-id", std::time::Duration::from_secs(1));
        assert!(result.is_some());
        let resp = result.unwrap();
        assert!(resp.result.is_some());
        assert_eq!(resp.result.unwrap(), serde_json::json!({"ok": true}));
        assert!(resp.error.is_none());
    }

    #[test]
    fn wait_for_response_returns_error_response() {
        let state = BridgeState::new();
        state.set_response(
            "err-id".to_string(),
            Response {
                result: None,
                error: Some("Something went wrong".into()),
            },
        );
        let result = state.wait_for_response("err-id", std::time::Duration::from_secs(1));
        assert!(result.is_some());
        let resp = result.unwrap();
        assert!(resp.result.is_none());
        assert_eq!(resp.error, Some("Something went wrong".into()));
    }

    #[test]
    fn wait_for_response_idempotent_after_timeout() {
        let state = BridgeState::new();
        // First call times out
        let result1 = state.wait_for_response("dead-id", std::time::Duration::from_millis(30));
        assert!(result1.is_none());
        // Setting a response after timeout should still be findable via get_response
        state.set_response(
            "dead-id".to_string(),
            Response {
                result: Some(serde_json::json!("late")),
                error: None,
            },
        );
        let result2 = state.get_response("dead-id");
        assert!(result2.is_some());
    }

    #[test]
    fn set_response_then_wait_returns_immediately() {
        let state = BridgeState::new();
        // Set response before waiting
        state.set_response(
            "fast-id".to_string(),
            Response {
                result: Some(serde_json::json!({"fast": true})),
                error: None,
            },
        );
        // wait should return instantly since response is already present
        let result = state.wait_for_response("fast-id", std::time::Duration::from_secs(5));
        assert!(result.is_some());
    }

    #[test]
    fn enqueue_and_dequeue_command() {
        let state = BridgeState::new();
        let cmd = Command {
            id: "cmd-1".to_string(),
            method: "evaluate_js".to_string(),
            params: serde_json::json!({"code": "1+1"}),
            received_at: Instant::now(),
        };
        state.enqueue(cmd.clone());
        let pending = state.dequeue_all();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, "cmd-1");
        assert_eq!(pending[0].method, "evaluate_js");
    }

    #[test]
    fn dequeue_all_is_empty_after_drain() {
        let state = BridgeState::new();
        state.enqueue(Command {
            id: "c1".to_string(),
            method: "m".to_string(),
            params: serde_json::json!({}),
            received_at: Instant::now(),
        });
        state.enqueue(Command {
            id: "c2".to_string(),
            method: "m".to_string(),
            params: serde_json::json!({}),
            received_at: Instant::now(),
        });
        assert_eq!(state.pending_count(), 2);
        let first = state.dequeue_all();
        assert_eq!(first.len(), 2);
        assert_eq!(state.pending_count(), 0);
    }

    #[test]
    fn append_log_and_get_logs() {
        let state = BridgeState::new();
        state.append_log("line 1".to_string());
        state.append_log("line 2".to_string());
        let logs = state.get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0], "line 1");
        assert_eq!(logs[1], "line 2");
    }

    #[test]
    fn enqueue_and_dequeue_js_results() {
        let state = BridgeState::new();
        state.enqueue_js_result("js-1".to_string(), r#""result-1""#.to_string());
        state.enqueue_js_result("js-2".to_string(), r#""result-2""#.to_string());
        let results = state.dequeue_js_results();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "js-1");
        assert_eq!(results[1].0, "js-2");
        // Second drain should be empty
        assert_eq!(state.dequeue_js_results().len(), 0);
    }

    #[test]
    fn enqueue_and_dequeue_eval_requests() {
        let state = BridgeState::new();
        let req = EvalRequest {
            id: "eval-1".to_string(),
            method: "evaluate_js".to_string(),
            params: serde_json::json!({"code": "2+2"}),
        };
        state.enqueue_eval_request(req.clone());
        let requests = state.dequeue_eval_requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].id, "eval-1");
        assert_eq!(requests[0].method, "evaluate_js");
    }

    #[test]
    fn bridge_state_handle_new_and_get_state() {
        let handle = BridgeStateHandle::new();
        let state1 = handle.get_state();
        let state2 = handle.get_state();
        // Same Arc
        assert!(Arc::ptr_eq(&state1, &state2));
    }

    #[test]
    fn bridge_state_handle_partial_eq() {
        let handle1 = BridgeStateHandle::new();
        let handle2 = BridgeStateHandle::new();
        // Different handles wrap different Arcs
        assert_ne!(handle1, handle2);
        let state = handle1.get_state();
        let handle3 = BridgeStateHandle(state);
        // But same underlying Arc makes them equal
        assert_eq!(handle1, handle3);
    }

    #[test]
    fn shutdown_flag() {
        let state = BridgeState::new();
        assert!(!state.is_shutdown());
        state.request_shutdown();
        assert!(state.is_shutdown());
    }

    #[test]
    fn bound_port_default_zero() {
        let state = BridgeState::new();
        assert_eq!(state.bound_port(), 0);
    }

    #[test]
    fn set_and_get_bound_port() {
        let state = BridgeState::new();
        state.set_bound_port(12345);
        assert_eq!(state.bound_port(), 12345);
    }

    #[test]
    fn enqueue_and_dequeue_ipc_events() {
        let state = BridgeState::new();
        let event1 = IpcEvent {
            event: "click".to_string(),
            payload: serde_json::json!({"x": 1, "y": 2}),
        };
        let event2 = IpcEvent {
            event: "keydown".to_string(),
            payload: serde_json::json!({"key": "a"}),
        };
        state.enqueue_ipc_event(event1.clone());
        state.enqueue_ipc_event(event2.clone());
        let events = state.dequeue_ipc_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event, "click");
        assert_eq!(events[1].event, "keydown");
        // Second drain should be empty
        assert_eq!(state.dequeue_ipc_events().len(), 0);
    }

    #[test]
    fn ipc_event_buffer_max_1000_fifo() {
        let state = BridgeState::new();
        for i in 0..1500 {
            state.enqueue_ipc_event(IpcEvent {
                event: format!("event-{}", i),
                payload: serde_json::json!({"n": i}),
            });
        }
        let events = state.dequeue_ipc_events();
        assert_eq!(events.len(), 1000);
        assert_eq!(events[0].event, "event-500");
        assert_eq!(events[999].event, "event-1499");
    }

    #[test]
    fn get_ipc_events_clones() {
        let state = BridgeState::new();
        state.enqueue_ipc_event(IpcEvent {
            event: "test".to_string(),
            payload: serde_json::json!({"val": 1}),
        });
        let events1 = state.get_ipc_events();
        let events2 = state.get_ipc_events();
        assert_eq!(events1.len(), 1);
        assert_eq!(events2.len(), 1);
        // Original buffer should still have the event
        assert_eq!(state.get_ipc_events().len(), 1);
    }

    #[test]
    fn clear_ipc_events() {
        let state = BridgeState::new();
        state.enqueue_ipc_event(IpcEvent {
            event: "test".to_string(),
            payload: serde_json::json!({}),
        });
        assert_eq!(state.get_ipc_events().len(), 1);
        state.clear_ipc_events();
        assert_eq!(state.get_ipc_events().len(), 0);
    }

    #[test]
    fn enqueue_and_dequeue_keyboard_requests() {
        let state = BridgeState::new();
        let req1 = KeyboardRequest {
            id: "kb-1".to_string(),
            key: "a".to_string(),
            modifiers: Some(serde_json::json!(["ctrl"])),
            action: "press".to_string(),
        };
        let req2 = KeyboardRequest {
            id: "kb-2".to_string(),
            key: "b".to_string(),
            modifiers: None,
            action: "release".to_string(),
        };
        state.enqueue_keyboard_request(req1.clone());
        state.enqueue_keyboard_request(req2.clone());
        let requests = state.dequeue_keyboard_requests();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].id, "kb-1");
        assert_eq!(requests[1].id, "kb-2");
        // Second drain should be empty
        assert_eq!(state.dequeue_keyboard_requests().len(), 0);
    }

    #[test]
    fn enqueue_and_dequeue_window_requests() {
        let state = BridgeState::new();
        let req1 = WindowRequest {
            id: "win-1".to_string(),
            action: "create".to_string(),
            window_id: None,
            width: Some(800),
            height: Some(600),
            x: None,
            y: None,
        };
        let req2 = WindowRequest {
            id: "win-2".to_string(),
            action: "move".to_string(),
            window_id: Some("main".to_string()),
            width: None,
            height: None,
            x: Some(100),
            y: Some(200),
        };
        state.enqueue_window_request(req1.clone());
        state.enqueue_window_request(req2.clone());
        let requests = state.dequeue_window_requests();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].id, "win-1");
        assert_eq!(requests[1].id, "win-2");
        // Second drain should be empty
        assert_eq!(state.dequeue_window_requests().len(), 0);
    }
}
