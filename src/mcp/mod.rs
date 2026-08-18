//! JSON-RPC 2.0 types shared between dioxus-mcp and dioxus-plugin-mcp-bridge.

#[cfg(feature = "dioxus-desktop")]
pub mod bridge;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcId {
    Number(u64),
    String(String),
    Null,
}

impl JsonRpcId {
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            JsonRpcId::Number(n) => Some(*n),
            _ => None,
        }
    }
}

impl PartialEq for JsonRpcId {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JsonRpcId::Number(a), JsonRpcId::Number(b)) => a == b,
            (JsonRpcId::String(a), JsonRpcId::String(b)) => a == b,
            (JsonRpcId::Null, JsonRpcId::Null) => true,
            _ => false,
        }
    }
}

impl Eq for JsonRpcId {}

impl std::hash::Hash for JsonRpcId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            JsonRpcId::Number(n) => {
                0u8.hash(state);
                n.hash(state);
            }
            JsonRpcId::String(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            JsonRpcId::Null => {
                2u8.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    #[serde(default = "jsonrpc_version")]
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Option<JsonRpcId>,
    pub method: String,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
}

fn jsonrpc_version() -> String {
    "2.0".to_string()
}

impl JsonRpcRequest {
    pub fn new(method: impl Into<String>, params: Option<serde_json::Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: None,
            method: method.into(),
            params,
        }
    }

    pub fn with_id(
        method: impl Into<String>,
        id: JsonRpcId,
        params: Option<serde_json::Value>,
    ) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            method: method.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self::new(-32700, msg)
    }

    pub fn invalid_request(msg: impl Into<String>) -> Self {
        Self::new(-32600, msg)
    }

    pub fn method_not_found(msg: impl Into<String>) -> Self {
        Self::new(-32601, msg)
    }

    pub fn internal_error(msg: impl Into<String>) -> Self {
        Self::new(-32603, msg)
    }

    pub fn server_error(code: i32, msg: impl Into<String>) -> Self {
        Self::new(-32000 + code, msg)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    #[serde(default = "jsonrpc_version")]
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Option<JsonRpcId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

impl JsonRpcResponse {
    pub fn success(id: Option<JsonRpcId>, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: Option<JsonRpcId>, err: JsonRpcError) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(err),
        }
    }

    pub fn parse_error(msg: impl Into<String>) -> Self {
        Self::error(None, JsonRpcError::parse_error(msg))
    }

    pub fn invalid_request(msg: impl Into<String>) -> Self {
        Self::error(None, JsonRpcError::invalid_request(msg))
    }
}
