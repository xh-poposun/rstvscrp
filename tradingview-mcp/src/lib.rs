use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub mod client;
pub mod handlers;
pub mod sec_edgar;
pub mod tools;

pub const MCP_VERSION: &str = "2024-11-05";

pub mod error_codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
    pub const SERVER_ERROR: i32 = -32000;
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn with_data(mut self, data: Value) -> Self {
        self.data = Some(data);
        self
    }

    pub fn method_not_found(method: &str) -> Self {
        Self::new(
            error_codes::METHOD_NOT_FOUND,
            format!("Method '{}' not found", method),
        )
    }

    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self::new(error_codes::INVALID_PARAMS, message)
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::new(error_codes::INTERNAL_ERROR, message)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ToolCallParams {
    pub name: String,
    pub arguments: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolCallResult {
    pub content: Vec<ToolContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

impl ToolContent {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content_type: "text".to_string(),
            text: text.into(),
        }
    }
}

pub type ToolHandlerFn = Box<
    dyn Fn(Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>>
        + Send
        + Sync,
>;

pub struct McpServer {
    tool_registry: Vec<tools::Tool>,
    handlers: HashMap<String, ToolHandlerFn>,
}

impl McpServer {
    pub fn new() -> Self {
        let tool_registry = tools::registry();
        let handlers: HashMap<String, ToolHandlerFn> = HashMap::new();

        Self {
            tool_registry,
            handlers,
        }
    }

    pub fn register_handler<F, Fut>(&mut self, name: impl Into<String>, handler: F)
    where
        F: Fn(Option<Value>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<ToolCallResult, JsonRpcError>> + Send + 'static,
    {
        let boxed_handler: ToolHandlerFn =
            Box::new(move |args| Box::pin(handler(args)));
        self.handlers.insert(name.into(), boxed_handler);
    }

    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        if request.jsonrpc != "2.0" {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(JsonRpcError::new(
                    error_codes::INVALID_REQUEST,
                    "Invalid JSON-RPC version, expected '2.0'",
                )),
            };
        }

        let result = match request.method.as_str() {
            "initialize" => self.handle_initialize(request.params).await,
            "tools/list" => self.handle_tools_list().await,
            "tools/call" => self.handle_tools_call(request.params).await,
            _ => Err(JsonRpcError::method_not_found(&request.method)),
        };

        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(value),
                error: None,
            },
            Err(error) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(error),
            },
        }
    }

    async fn handle_initialize(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        let result = serde_json::json!({
            "protocolVersion": MCP_VERSION,
            "capabilities": {
                "tools": {
                    "listChanged": true
                }
            },
            "serverInfo": {
                "name": "tradingview-mcp",
                "version": env!("CARGO_PKG_VERSION")
            }
        });
        Ok(result)
    }

    async fn handle_tools_list(&self) -> Result<Value, JsonRpcError> {
        let tools: Vec<Value> = self
            .tool_registry
            .iter()
            .map(|tool| {
                serde_json::json!({
                    "name": tool.name,
                    "description": tool.description,
                    "inputSchema": tool.input_schema,
                })
            })
            .collect();

        Ok(serde_json::json!({ "tools": tools }))
    }

    async fn handle_tools_call(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| {
            JsonRpcError::invalid_params("Missing 'params' field in tools/call request")
        })?;

        let tool_params: ToolCallParams = serde_json::from_value(params).map_err(|e| {
            JsonRpcError::invalid_params(format!("Invalid tool call params: {}", e))
        })?;

        let tool_exists = self
            .tool_registry
            .iter()
            .any(|t| t.name == tool_params.name);

        if !tool_exists {
            return Err(JsonRpcError::new(
                error_codes::METHOD_NOT_FOUND,
                format!("Tool '{}' not found", tool_params.name),
            ));
        }

        if let Some(handler) = self.handlers.get(&tool_params.name) {
            let result = handler(tool_params.arguments).await?;
            Ok(serde_json::to_value(result).map_err(|e| {
                JsonRpcError::internal_error(format!("Failed to serialize result: {}", e))
            })?)
        } else {
            let result = ToolCallResult {
                content: vec![ToolContent::text(format!(
                    "Tool '{}' called successfully (handler not yet implemented)",
                    tool_params.name
                ))],
                is_error: Some(false),
            };
            Ok(serde_json::to_value(result).map_err(|e| {
                JsonRpcError::internal_error(format!("Failed to serialize result: {}", e))
            })?)
        }
    }
}

impl Default for McpServer {
    fn default() -> Self {
        Self::new()
    }
}

pub fn success_response(id: Option<Value>, result: Value) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: Some(result),
        error: None,
    }
}

pub fn error_response(id: Option<Value>, code: i32, message: impl Into<String>) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result: None,
        error: Some(JsonRpcError::new(code, message)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize() {
        let server = McpServer::new();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(1.into()),
            method: "initialize".to_string(),
            params: None,
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        assert!(response.result.is_some());

        let result = response.result.unwrap();
        assert_eq!(result["protocolVersion"], MCP_VERSION);
        assert_eq!(result["serverInfo"]["name"], "tradingview-mcp");
    }

    #[tokio::test]
    async fn test_tools_list() {
        let server = McpServer::new();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(1.into()),
            method: "tools/list".to_string(),
            params: None,
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        assert!(response.result.is_some());

        let result = response.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 15);
    }

    #[tokio::test]
    async fn test_tools_call_unknown_tool() {
        let server = McpServer::new();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(1.into()),
            method: "tools/call".to_string(),
            params: Some(serde_json::json!({
                "name": "unknown_tool",
                "arguments": {}
            })),
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, error_codes::METHOD_NOT_FOUND);
    }

    #[tokio::test]
    async fn test_method_not_found() {
        let server = McpServer::new();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(1.into()),
            method: "unknown/method".to_string(),
            params: None,
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, error_codes::METHOD_NOT_FOUND);
    }

    #[tokio::test]
    async fn test_invalid_jsonrpc_version() {
        let server = McpServer::new();
        let request = JsonRpcRequest {
            jsonrpc: "1.0".to_string(),
            id: Some(1.into()),
            method: "tools/list".to_string(),
            params: None,
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert_eq!(error.code, error_codes::INVALID_REQUEST);
    }
}
