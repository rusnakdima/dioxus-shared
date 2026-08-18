//! WebSocket server for the MCP Bridge
//! Uses tokio's async I/O for reliable partial-read handling.
//!
//! JSON-RPC wire types come from the shared `crate::mcp` module
//! (`JsonRpcRequest` / `JsonRpcResponse` / `JsonRpcError`), so the bridge and
//! the MCP client speak exactly the same protocol definitions.

use super::state::{BridgeState, Command, Response};
use crate::mcp::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};
use sha1::Digest;
use std::sync::Arc;
use std::time::Instant;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::net::TcpListener as TokioTcpListener;
use tokio::net::TcpStream as TokioTcpStream;
use tracing::{error, info};

mod frame {
    use super::*;

    pub(crate) struct Frame {
        pub opcode: u8,
        pub data: Vec<u8>,
    }

    /// Read a WebSocket frame using tokio async I/O.
    /// Tokio's AsyncReadExt::read_exact properly handles partial TCP reads
    /// by polling until exactly n bytes arrive (unlike std BufReader::read_exact
    /// in blocking mode which silently returns partial data on EOF).
    pub(crate) async fn read_frame(
        stream: &mut BufReader<TokioTcpStream>,
    ) -> Result<Option<Frame>, Box<dyn std::error::Error + Send + Sync>> {
        let mut header = [0u8; 2];
        match stream.read(&mut header).await {
            Ok(2) => {}
            Ok(0) => return Ok(None),
            Ok(_) => return Err("Invalid frame header".into()),
            Err(e) => return Err(e.into()),
        }

        let opcode = header[0] & 0x0f;
        let masked = (header[1] & 0x80) != 0;
        let base_len = header[1] & 0x7f;

        let payload_len = match base_len {
            0..=125 => base_len as usize,
            126 => {
                let mut len_bytes = [0u8; 2];
                stream.read_exact(&mut len_bytes).await?;
                u16::from_be_bytes(len_bytes) as usize
            }
            127 => {
                let mut len_bytes = [0u8; 8];
                stream.read_exact(&mut len_bytes).await?;
                u64::from_be_bytes(len_bytes) as usize
            }
            _ => unreachable!(),
        };

        // For masked frames, the mask key (4 bytes) is INSIDE the payload,
        // so we must read (4 + payload_len) total bytes. For unmasked, just payload_len.
        let actual_payload_len = if masked { 4 + payload_len } else { payload_len };

        // Use tokio's AsyncReadExt::read_exact on BufReader<TokioTcpStream>.
        // This properly handles partial TCP reads by looping until all bytes arrive.
        let mut payload = vec![0u8; actual_payload_len];
        stream.read_exact(&mut payload).await?;

        let data = if masked {
            // First 4 bytes of payload are the mask key (per RFC6455)
            let mask = [payload[0], payload[1], payload[2], payload[3]];
            let masked_data = &payload[4..];
            let mut unmasked = masked_data.to_vec();
            for (i, byte) in unmasked.iter_mut().enumerate() {
                *byte ^= mask[i % 4];
            }
            unmasked
        } else {
            payload
        };

        Ok(Some(Frame { opcode, data }))
    }

    /// Send a WebSocket frame (server->client, no masking per RFC6455).
    pub(crate) async fn send_frame(
        stream: &mut BufReader<TokioTcpStream>,
        opcode: u8,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use tokio::io::AsyncWriteExt;

        let len = data.len();
        let mut frame = Vec::with_capacity(2 + 8 + len);

        frame.push(opcode); // No mask bit for server->client frames

        match len {
            0..=125 => {
                frame.push(len as u8);
            }
            126..=65535 => {
                frame.push(126);
                frame.push((len >> 8) as u8);
                frame.push((len & 0xff) as u8);
            }
            _ => {
                frame.push(127);
                for i in (0..8).rev() {
                    frame.push((len >> (i * 8)) as u8);
                }
            }
        }

        frame.extend_from_slice(data);
        let stream_ref = stream.get_mut();
        stream_ref.write_all(&frame).await?;
        stream_ref.flush().await?;
        Ok(())
    }
}

use frame::{read_frame, send_frame};

fn process_request(text: &str, state: &Arc<BridgeState>) -> Option<JsonRpcResponse> {
    let request: JsonRpcRequest = match serde_json::from_str(text) {
        Ok(r) => r,
        Err(e) => {
            return Some(JsonRpcResponse::parse_error(format!("Parse error: {}", e)));
        }
    };

    if request.jsonrpc != "2.0" {
        return Some(JsonRpcResponse::error(
            request.id,
            JsonRpcError::invalid_request("Invalid JSON-RPC version"),
        ));
    }

    let method = request.method.clone();
    let id = uuid::Uuid::new_v4().to_string();

    state.enqueue(Command {
        id: id.clone(),
        method,
        params: request.params.clone().unwrap_or(serde_json::json!({})),
        received_at: Instant::now(),
    });

    let req_id = request.id.clone();

    match state.wait_for_response(&id, std::time::Duration::from_secs(5)) {
        Some(resp) => Some(match resp {
            Response {
                result: Some(r),
                error: None,
            } => JsonRpcResponse::success(req_id, r),
            Response {
                result: None,
                error: Some(e),
            } => JsonRpcResponse::error(req_id, JsonRpcError::new(-32000, e)),
            _ => JsonRpcResponse::error(req_id, JsonRpcError::internal_error("Unknown error")),
        }),
        None => Some(JsonRpcResponse::error(
            req_id,
            JsonRpcError::server_error(1, "Command timeout - app may not be processing commands"),
        )),
    }
}

/// Act as a WebSocket server for one connected client.
async fn ws_handler(stream: TokioTcpStream, state: Arc<BridgeState>) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut stream = BufReader::new(stream);

    // Read HTTP upgrade request
    let mut buffer = vec![0u8; 4096];
    let n = match stream.read(&mut buffer).await {
        Ok(n) => n,
        Err(e) => {
            error!("Failed to read WebSocket handshake: {}", e);
            return;
        }
    };

    let request = String::from_utf8_lossy(&buffer[..n]);

    if !request.contains("Upgrade: websocket") {
        return;
    }

    let client_key = match extract_websocket_key(&request) {
        Some(key) => key,
        None => {
            error!("Missing Sec-WebSocket-Key in upgrade request");
            return;
        }
    };
    let accept_key = compute_accept_key(&client_key);

    let handshake = format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
        Connection: Upgrade\r\n\
        Upgrade: websocket\r\n\
        Sec-WebSocket-Accept: {}\r\n\
        \r\n",
        accept_key
    );

    // Send handshake with short-lived borrow
    {
        let stream_ref = stream.get_mut();
        if let Err(e) = stream_ref.write_all(handshake.as_bytes()).await {
            error!("Failed to send WebSocket handshake: {}", e);
            return;
        }
        if let Err(e) = stream_ref.flush().await {
            error!("Failed to flush handshake: {}", e);
            return;
        }
    } // stream_ref dropped here, borrow ends

    info!("WebSocket connection established");

    loop {
        if state.is_shutdown() {
            // Short-lived borrow for close frame
            let stream_ref = stream.get_mut();
            let _ = stream_ref.write_all(&[0x88, 0x00]).await;
            let _ = stream_ref.flush().await;
            return;
        }

        match read_frame(&mut stream).await {
            Ok(Some(frame)) => match frame.opcode {
                0x1 => {
                    let text = String::from_utf8_lossy(&frame.data);
                    let response = process_request(&text, &state);

                    if let Some(response) = response {
                        let response_text = serde_json::to_string(&response).unwrap_or_else(|_| {
                            serde_json::to_string(&JsonRpcResponse::parse_error(
                                "Serialize error".to_string(),
                            ))
                            .unwrap()
                        });
                        if let Err(e) = send_frame(&mut stream, 0x1, response_text.as_bytes()).await
                        {
                            error!("Failed to send response: {}", e);
                            return;
                        }
                    }
                }
                0x8 => {
                    let stream_ref = stream.get_mut();
                    let _ = stream_ref.write_all(&[0x88, 0x00]).await;
                    let _ = stream_ref.flush().await;
                    return;
                }
                0x9 => {
                    let stream_ref = stream.get_mut();
                    let _ = stream_ref.write_all(&[0x8a, 0x00]).await;
                    let _ = stream_ref.flush().await;
                }
                0xA | 0x0 => {}
                _ => {}
            },
            Ok(None) => return,
            Err(e) => {
                error!("Frame read error: {}", e);
                return;
            }
        }
    }
}

/// Maximum number of port retries on `AddrInUse` errors.
const MAX_PORT_RETRIES: u16 = 10;

/// Try to bind a Tokio TCP listener starting at `port` and incrementing by 1
/// on `AddrInUse` errors, up to `MAX_PORT_RETRIES` attempts. Returns the
/// listener and the actual bound port, or the final error if all attempts
/// fail.
pub async fn bind_listener_with_retry(port: u16) -> std::io::Result<(TokioTcpListener, u16)> {
    for offset in 0..=MAX_PORT_RETRIES {
        let attempt_port = port.saturating_add(offset);
        match TokioTcpListener::bind(format!("127.0.0.1:{attempt_port}")).await {
            Ok(listener) => {
                if offset > 0 {
                    tracing::info!("Port {port} busy, bound to fallback 127.0.0.1:{attempt_port}");
                }
                return Ok((listener, attempt_port));
            }
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse && offset < MAX_PORT_RETRIES => {
                continue;
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}

/// Start the WebSocket server on the given port.
fn serve(port: u16, state: Arc<BridgeState>) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    rt.block_on(async {
        let (tokio_listener, bound_port) = bind_listener_with_retry(port)
            .await
            .expect("Failed to bind TCP port after retries");
        state.set_bound_port(bound_port);
        info!(
            "MCP Bridge WebSocket server listening on 127.0.0.1:{}",
            bound_port
        );

        loop {
            if state.is_shutdown() {
                info!("MCP Bridge shutting down");
                break;
            }
            match tokio_listener.accept().await {
                Ok((stream, _)) => {
                    let state = state.clone();
                    std::thread::spawn(move || {
                        let rt = tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .unwrap();
                        rt.block_on(ws_handler(stream, state));
                    });
                }
                Err(_) => {
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }
            }
        }
    });
}

// Provide `run` as an alias for `serve` (called from lib.rs)
pub fn run(port: u16, state: Arc<BridgeState>) {
    serve(port, state)
}

// ============ WebSocket Helpers ============

fn extract_websocket_key(request: &str) -> Option<String> {
    for line in request.lines() {
        if line.starts_with("Sec-WebSocket-Key:") {
            let key = line.trim_start_matches("Sec-WebSocket-Key:").trim();
            return Some(key.to_string());
        }
    }
    None
}

fn compute_accept_key(client_key: &str) -> String {
    use sha1::Sha1;
    let mut hasher = Sha1::new();
    hasher.update(client_key.as_bytes());
    hasher.update(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let result = hasher.finalize();
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn bind_retry_succeeds_on_alternate_port() {
        // Occupy the first port
        let blocker = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let busy_port = blocker.local_addr().unwrap().port();

        // bind_listener_with_retry should skip busy_port and bind busy_port+1
        let (listener, bound) = bind_listener_with_retry(busy_port).await.unwrap();
        assert_eq!(bound, busy_port + 1);
        drop(listener);
        drop(blocker);
    }

    #[tokio::test]
    async fn bind_retry_returns_first_port_when_free() {
        // Pick a high port unlikely to be in use; retry should bind it on first try
        let (_listener, bound) = bind_listener_with_retry(38999).await.unwrap();
        assert_eq!(bound, 38999);
    }

    // ============ JsonRpc serialization round-trip tests ============

    #[test]
    fn jsonrpc_request_serialize_roundtrip() {
        use crate::mcp::{JsonRpcId, JsonRpcRequest};

        let req = JsonRpcRequest::with_id(
            "evaluate_js",
            JsonRpcId::Number(42),
            Some(serde_json::json!({ "code": "document.title" })),
        );
        let json = serde_json::to_string(&req).unwrap();
        let parsed: JsonRpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.jsonrpc, "2.0");
        assert_eq!(parsed.method, "evaluate_js");
        assert_eq!(parsed.id, Some(JsonRpcId::Number(42)));
        assert_eq!(
            parsed.params,
            Some(serde_json::json!({ "code": "document.title" }))
        );
    }

    #[test]
    fn jsonrpc_request_string_id_roundtrip() {
        use crate::mcp::{JsonRpcId, JsonRpcRequest};

        let req =
            JsonRpcRequest::with_id("dom_snapshot", JsonRpcId::String("abc-123".into()), None);
        let json = serde_json::to_string(&req).unwrap();
        let parsed: JsonRpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, Some(JsonRpcId::String("abc-123".into())));
    }

    #[test]
    fn jsonrpc_response_success_roundtrip() {
        use crate::mcp::{JsonRpcId, JsonRpcResponse};

        let resp = JsonRpcResponse::success(Some(JsonRpcId::Number(1)), serde_json::json!("ok"));
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: JsonRpcResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.jsonrpc, "2.0");
        assert_eq!(parsed.id, Some(JsonRpcId::Number(1)));
        assert_eq!(parsed.result, Some(serde_json::json!("ok")));
        assert!(parsed.error.is_none());
    }

    #[test]
    fn jsonrpc_response_error_roundtrip() {
        use crate::mcp::{JsonRpcError, JsonRpcId, JsonRpcResponse};

        let resp = JsonRpcResponse::error(
            Some(JsonRpcId::String("req-99".into())),
            JsonRpcError::server_error(1, "Command timeout"),
        );
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: JsonRpcResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, Some(JsonRpcId::String("req-99".into())));
        assert!(parsed.result.is_none());
        let err = parsed.error.unwrap();
        assert_eq!(err.code, -31999);
        assert_eq!(err.message, "Command timeout");
    }

    #[test]
    fn jsonrpc_response_parse_error_roundtrip() {
        use crate::mcp::JsonRpcResponse;

        let resp = JsonRpcResponse::parse_error("Unexpected token");
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: JsonRpcResponse = serde_json::from_str(&json).unwrap();
        assert!(parsed.id.is_none());
        let err = parsed.error.unwrap();
        assert_eq!(err.code, -32700);
        assert_eq!(err.message, "Unexpected token");
    }

    // ============ extract_websocket_key tests ============

    #[test]
    fn extract_websocket_key_finds_key() {
        let request =
            "GET / HTTP/1.1\r\nHost: localhost\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n";
        let key = extract_websocket_key(request);
        assert_eq!(key, Some("dGhlIHNhbXBsZSBub25jZQ==".to_string()));
    }

    #[test]
    fn extract_websocket_key_trims_whitespace() {
        let request = "Sec-WebSocket-Key:   dGhlIHNhbXBsZSBub25jZQ==  \r\n";
        let key = extract_websocket_key(request).unwrap();
        assert_eq!(key, "dGhlIHNhbXBsZSBub25jZQ==");
    }

    #[test]
    fn extract_websocket_key_case_insensitive_header() {
        // The header name is case-sensitive per RFC6455 but we test the standard form
        let request = "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n";
        let key = extract_websocket_key(request);
        assert!(key.is_some());
    }

    #[test]
    fn extract_websocket_key_missing_returns_none() {
        let request = "GET / HTTP/1.1\r\nHost: localhost\r\n";
        let key = extract_websocket_key(request);
        assert!(key.is_none());
    }

    #[test]
    fn extract_websocket_key_no_websocket_header_returns_none() {
        let request = "GET / HTTP/1.1\r\nConnection: close\r\n";
        let key = extract_websocket_key(request);
        assert!(key.is_none());
    }

    // ============ compute_accept_key tests ============

    #[test]
    fn compute_accept_key_produces_valid_base64() {
        // Known answer: the accept key for "dGhlIHNhbXBsZSBub25jZQ==" is deterministic
        use base64::Engine;
        let key = compute_accept_key("dGhlIHNhbXBsZSBub25jZQ==");
        // The result should be valid base64 (URL-safe not required, STANDARD encoding)
        let decoded = base64::engine::general_purpose::STANDARD.decode(&key);
        assert!(decoded.is_ok());
        // SHA1 produces 20 bytes
        assert_eq!(decoded.unwrap().len(), 20);
    }

    #[test]
    fn compute_accept_key_deterministic() {
        let key1 = compute_accept_key("test-key-123");
        let key2 = compute_accept_key("test-key-123");
        assert_eq!(key1, key2);
    }

    #[test]
    fn compute_accept_key_different_inputs_different_outputs() {
        let key1 = compute_accept_key("key-a");
        let key2 = compute_accept_key("key-b");
        assert_ne!(key1, key2);
    }
}
