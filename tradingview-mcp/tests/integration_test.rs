//! End-to-end integration tests for TradingView MCP server
//!
//! These tests verify:
//! - MCP handshake (initialize, notifications/initialized)
//! - All 15 tools return proper responses
//! - Response structures are valid and don't contain mock data where real data is expected

use serde_json::json;
use std::sync::Arc;
use tradingview_mcp::{
    client::{TradingViewMcpClient, ClientConfig},
    handlers::*,
    tools::*,
    JsonRpcRequest, JsonRpcResponse, McpServer, ToolCallResult, error_codes,
};

// ============================================================================
// MCP Handshake Tests
// ============================================================================

#[tokio::test]
async fn test_mcp_initialize() {
    let server = McpServer::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: Some(json!({
            "protocolVersion": "2025-03-26",
            "capabilities": {},
            "clientInfo": {"name": "test-client", "version": "1.0.0"}
        })),
    };

    let response = server.handle_request(request).await;

    // Verify response structure
    assert_eq!(response.jsonrpc, "2.0");
    assert!(response.error.is_none(), "Expected no error, got: {:?}", response.error);
    assert!(response.result.is_some(), "Expected result in response");

    let result = response.result.unwrap();
    assert_eq!(result["protocolVersion"], "2025-03-26");
    assert_eq!(result["serverInfo"]["name"], "tradingview-mcp");
    assert!(result["serverInfo"]["version"].as_str().is_some());
    assert!(result["capabilities"]["tools"]["listChanged"].as_bool().unwrap_or(false));
}

#[tokio::test]
async fn test_mcp_initialize_default_version() {
    let server = McpServer::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: None,
    };

    let response = server.handle_request(request).await;
    assert!(response.error.is_none());
    assert!(response.result.is_some());
}

#[tokio::test]
async fn test_mcp_notifications_initialized() {
    let server = McpServer::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: None, // Notifications have no id
        method: "notifications/initialized".to_string(),
        params: None,
    };

    let response = server.handle_request(request).await;

    // Notifications should return empty response with no error
    assert!(response.error.is_none(), "Notification should not return error");
    assert!(response.result.is_none(), "Notification should not return result");
    assert_eq!(response.id, None, "Notification response should have no id");
    assert_eq!(response.jsonrpc, "2.0");
}

#[tokio::test]
async fn test_mcp_tools_list() {
    let server = McpServer::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "tools/list".to_string(),
        params: None,
    };

    let response = server.handle_request(request).await;

    assert!(response.error.is_none(), "Expected no error");
    assert!(response.result.is_some(), "Expected result");

    let result = response.result.unwrap();
    let tools = result["tools"].as_array().expect("tools should be an array");
    assert_eq!(tools.len(), 15, "Expected 15 tools");

    // Verify all expected tools are present
    let tool_names: Vec<String> = tools
        .iter()
        .map(|t| t["name"].as_str().unwrap().to_string())
        .collect();

    let expected_tools = vec![
        "get_quote",
        "search_symbols",
        "get_historical",
        "get_fundamentals",
        "get_financial_statements",
        "get_credit_ratings",
        "scan_stocks",
        "get_earnings_calendar",
        "get_dividend_calendar",
        "calculate_rsi",
        "calculate_macd",
        "get_debt_maturity",
        "compute_macd_signal",
        "get_company_profile",
        "get_market_news",
    ];

    for expected in &expected_tools {
        assert!(
            tool_names.contains(&expected.to_string()),
            "Missing tool: {}",
            expected
        );
    }
}

#[tokio::test]
async fn test_mcp_invalid_jsonrpc_version() {
    let server = McpServer::new();
    let request = JsonRpcRequest {
        jsonrpc: "1.0".to_string(),
        id: Some(json!(1)),
        method: "tools/list".to_string(),
        params: None,
    };

    let response = server.handle_request(request).await;

    assert!(response.error.is_some(), "Expected error for invalid JSON-RPC version");
    let error = response.error.unwrap();
    assert_eq!(error.code, error_codes::INVALID_REQUEST);
}

#[tokio::test]
async fn test_mcp_method_not_found() {
    let server = McpServer::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "unknown/method".to_string(),
        params: None,
    };

    let response = server.handle_request(request).await;

    assert!(response.error.is_some(), "Expected error for unknown method");
    let error = response.error.unwrap();
    assert_eq!(error.code, error_codes::METHOD_NOT_FOUND);
}

#[tokio::test]
async fn test_mcp_full_handshake_sequence() {
    let server = McpServer::new();

    // Step 1: Initialize
    let init_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: Some(json!({
            "protocolVersion": "2025-03-26",
            "capabilities": {},
            "clientInfo": {"name": "test", "version": "1.0"}
        })),
    };
    let init_response = server.handle_request(init_request).await;
    assert!(init_response.error.is_none(), "Initialize failed");
    assert!(init_response.result.is_some());

    // Step 2: Notification
    let notif_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: None,
        method: "notifications/initialized".to_string(),
        params: None,
    };
    let notif_response = server.handle_request(notif_request).await;
    assert!(notif_response.error.is_none());
    assert!(notif_response.result.is_none());

    // Step 3: Tools list still works
    let tools_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(2)),
        method: "tools/list".to_string(),
        params: None,
    };
    let tools_response = server.handle_request(tools_request).await;
    assert!(tools_response.error.is_none());
    let result = tools_response.result.unwrap();
    let tools = result["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 15);
}

// ============================================================================
// Tool Handler Tests - Parameter Validation
// ============================================================================

#[tokio::test]
async fn test_tool_call_unknown_tool() {
    let server = McpServer::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": "unknown_tool",
            "arguments": {}
        })),
    };

    let response = server.handle_request(request).await;

    assert!(response.error.is_some(), "Expected error for unknown tool");
    let error = response.error.unwrap();
    assert_eq!(error.code, error_codes::METHOD_NOT_FOUND);
    assert!(error.message.contains("unknown_tool"));
}

#[tokio::test]
async fn test_tool_call_missing_params() {
    let server = McpServer::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "tools/call".to_string(),
        params: None,
    };

    let response = server.handle_request(request).await;

    assert!(response.error.is_some(), "Expected error for missing params");
    let error = response.error.unwrap();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
}

// ============================================================================
// Handler Direct Tests - Input Validation
// ============================================================================

#[tokio::test]
async fn test_validate_symbol_valid() {
    // Test valid symbols through handler calls
    let client = Arc::new(create_test_client().await);

    // Valid symbol with exchange prefix
    let args = Some(json!({"symbol": "NASDAQ:AAPL"}));
    let result = handle_get_quote(client.clone(), args).await;
    // Should not fail on validation, may fail on API call

    // Valid symbol without exchange prefix
    let args = Some(json!({"symbol": "MSFT"}));
    let _ = handle_get_quote(client.clone(), args).await;
}

#[tokio::test]
async fn test_validate_symbol_empty() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({"symbol": ""}));
    let result = handle_get_quote(client, args).await;

    assert!(result.is_err(), "Expected error for empty symbol");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
    assert!(error.message.contains("empty"));
}

#[tokio::test]
async fn test_validate_symbol_invalid_format() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({"symbol": "EX:SYM:BOL"}));
    let result = handle_get_quote(client, args).await;

    assert!(result.is_err(), "Expected error for invalid symbol format");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
}

#[tokio::test]
async fn test_search_symbols_empty_query() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({"query": ""}));
    let result = handle_search_symbols(client, args).await;

    assert!(result.is_err(), "Expected error for empty query");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
}

#[tokio::test]
async fn test_get_historical_invalid_interval() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({
        "symbol": "NASDAQ:AAPL",
        "interval": "invalid",
        "count": 100
    }));
    let result = handle_get_historical(client, args).await;

    assert!(result.is_err(), "Expected error for invalid interval");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
    assert!(error.message.contains("interval"));
}

#[tokio::test]
async fn test_get_financial_statements_invalid_period() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({
        "symbol": "NASDAQ:AAPL",
        "period": "monthly"
    }));
    let result = handle_get_financial_statements(client, args).await;

    assert!(result.is_err(), "Expected error for invalid period");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
}

#[tokio::test]
async fn test_get_earnings_calendar_invalid_days() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({"days_ahead": -1}));
    let result = handle_get_earnings_calendar(client.clone(), args).await;

    assert!(result.is_err(), "Expected error for negative days");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);

    let args = Some(json!({"days_ahead": 91}));
    let result = handle_get_earnings_calendar(client, args).await;

    assert!(result.is_err(), "Expected error for days > 90");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
}

#[tokio::test]
async fn test_get_dividend_calendar_empty_exchange() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({"exchange": ""}));
    let result = handle_get_dividend_calendar(client, args).await;

    assert!(result.is_err(), "Expected error for empty exchange");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
}

#[tokio::test]
async fn test_calculate_rsi_invalid_period() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({
        "symbol": "NASDAQ:AAPL",
        "period": 1
    }));
    let result = handle_calculate_rsi(client.clone(), args).await;

    assert!(result.is_err(), "Expected error for period < 2");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);

    let args = Some(json!({
        "symbol": "NASDAQ:AAPL",
        "period": 101
    }));
    let result = handle_calculate_rsi(client, args).await;

    assert!(result.is_err(), "Expected error for period > 100");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
}

#[tokio::test]
async fn test_calculate_macd_invalid_periods() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({
        "symbol": "NASDAQ:AAPL",
        "fast": 20,
        "slow": 10,
        "signal": 9
    }));
    let result = handle_calculate_macd(client, args).await;

    assert!(result.is_err(), "Expected error when fast >= slow");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
}

#[tokio::test]
async fn test_compute_macd_signal_invalid_windows() {
    let client = Arc::new(create_test_client().await);

    // Test short_window >= long_window
    let args = Some(json!({
        "symbol": "NASDAQ:AAPL",
        "short_window": 20,
        "long_window": 10,
        "signal_window": 9
    }));
    let result = handle_compute_macd_signal(client.clone(), args).await;

    assert!(result.is_err(), "Expected error when short >= long");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);

    // Test signal_window = 0
    let args = Some(json!({
        "symbol": "NASDAQ:AAPL",
        "short_window": 12,
        "long_window": 26,
        "signal_window": 0
    }));
    let result = handle_compute_macd_signal(client, args).await;

    assert!(result.is_err(), "Expected error when signal_window = 0");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
}

#[tokio::test]
async fn test_get_market_news_invalid_limit() {
    let client = Arc::new(create_test_client().await);

    let args = Some(json!({
        "symbol": "NASDAQ:AAPL",
        "limit": 0
    }));
    let result = handle_get_market_news(client.clone(), args).await;

    assert!(result.is_err(), "Expected error for limit = 0");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);

    let args = Some(json!({
        "symbol": "NASDAQ:AAPL",
        "limit": 101
    }));
    let result = handle_get_market_news(client, args).await;

    assert!(result.is_err(), "Expected error for limit > 100");
    let error = result.unwrap_err();
    assert_eq!(error.code, error_codes::INVALID_PARAMS);
}

// ============================================================================
// Tool Response Structure Tests
// ============================================================================

#[tokio::test]
async fn test_scan_stocks_response_structure() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({
        "filters": {},
        "limit": 10
    }));
    let result = handle_scan_stocks(client, args).await;

    assert!(result.is_ok(), "Expected successful response");
    let response = result.unwrap();
    assert_eq!(response.is_error, Some(false));
    assert_eq!(response.content.len(), 1);

    // Parse the content
    let content_text = &response.content[0].text;
    let parsed: serde_json::Value = serde_json::from_str(content_text)
        .expect("Response should be valid JSON");

    assert!(parsed["results"].is_array(), "Results should be an array");
    let results = parsed["results"].as_array().unwrap();
    assert!(!results.is_empty(), "Should have at least one result");

    // Verify structure of first result
    let first = &results[0];
    assert!(first["symbol"].is_string());
    assert!(first["name"].is_string());
    assert!(first["exchange"].is_string());
    assert!(first["price"].is_number());
}

#[tokio::test]
async fn test_get_debt_maturity_response_structure() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({"symbol": "NASDAQ:AAPL"}));
    let result = handle_get_debt_maturity(client, args).await;

    assert!(result.is_ok(), "Expected successful response");
    let response = result.unwrap();
    assert_eq!(response.is_error, Some(false));

    let content_text = &response.content[0].text;
    let parsed: serde_json::Value = serde_json::from_str(content_text)
        .expect("Response should be valid JSON");

    assert!(parsed["maturity"].is_array(), "Maturity should be an array");
    let maturity = parsed["maturity"].as_array().unwrap();

    for item in maturity {
        assert!(item["due_date"].is_string(), "Due date should be a string");
        assert!(item["amount"].is_number(), "Amount should be a number");
    }
}

#[tokio::test]
async fn test_get_company_profile_response_structure() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({"symbol": "NASDAQ:AAPL"}));
    let result = handle_get_company_profile(client, args).await;

    assert!(result.is_ok(), "Expected successful response");
    let response = result.unwrap();
    assert_eq!(response.is_error, Some(false));

    let content_text = &response.content[0].text;
    let parsed: serde_json::Value = serde_json::from_str(content_text)
        .expect("Response should be valid JSON");

    assert!(parsed["overview"].is_object(), "Overview should be an object");
    let overview = parsed["overview"].as_object().unwrap();

    // Check expected fields
    assert!(overview.contains_key("name"));
    assert!(overview.contains_key("sector"));
    assert!(overview.contains_key("industry"));
}

#[tokio::test]
async fn test_get_market_news_response_structure() {
    let client = Arc::new(create_test_client().await);
    let args = Some(json!({
        "symbol": "NASDAQ:AAPL",
        "limit": 5
    }));
    let result = handle_get_market_news(client, args).await;

    assert!(result.is_ok(), "Expected successful response");
    let response = result.unwrap();
    assert_eq!(response.is_error, Some(false));

    let content_text = &response.content[0].text;
    let parsed: serde_json::Value = serde_json::from_str(content_text)
        .expect("Response should be valid JSON");

    assert!(parsed["articles"].is_array(), "Articles should be an array");
    let articles = parsed["articles"].as_array().unwrap();
    assert!(!articles.is_empty(), "Should have at least one article");

    for article in articles {
        assert!(article["title"].is_string(), "Title should be a string");
        assert!(article["url"].is_string(), "URL should be a string");
        assert!(article["published_at"].is_string(), "Published_at should be a string");
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

async fn create_test_client() -> TradingViewMcpClient {
    // Create a client with default config for testing
    // This won't make actual API calls in most tests
    let config = ClientConfig::default();
    TradingViewMcpClient::with_config(config).await
        .expect("Failed to create test client")
}

// ============================================================================
// Integration Test Suite - All Tools
// ============================================================================

/// Test that verifies all 15 tools are registered and have proper schemas
#[tokio::test]
async fn test_all_tools_registered() {
    let server = McpServer::new();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "tools/list".to_string(),
        params: None,
    };

    let response = server.handle_request(request).await;
    assert!(response.error.is_none());

    let result = response.result.unwrap();
    let tools = result["tools"].as_array().unwrap();

    assert_eq!(tools.len(), 15, "Expected exactly 15 tools");

    // Verify each tool has required fields
    for tool in tools {
        assert!(tool["name"].is_string(), "Tool should have a name");
        assert!(tool["description"].is_string(), "Tool should have a description");
        assert!(tool["inputSchema"].is_object(), "Tool should have inputSchema");

        let name = tool["name"].as_str().unwrap();
        let input_schema = tool["inputSchema"].as_object().unwrap();

        // All tools should have type: object in their schema
        assert_eq!(
            input_schema.get("type").and_then(|v| v.as_str()),
            Some("object"),
            "Tool {} should have type: object",
            name
        );

        // All tools should have properties
        assert!(
            input_schema.contains_key("properties"),
            "Tool {} should have properties",
            name
        );
    }
}

/// Test complete MCP lifecycle
#[tokio::test]
async fn test_complete_mcp_lifecycle() {
    let server = McpServer::new();

    // 1. Initialize
    let init_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "initialize".to_string(),
        params: Some(json!({
            "protocolVersion": "2025-03-26",
            "capabilities": {},
            "clientInfo": {"name": "integration-test", "version": "1.0.0"}
        })),
    };
    let init_response = server.handle_request(init_request).await;
    assert!(init_response.error.is_none(), "Initialize should succeed");
    assert!(init_response.result.is_some());

    let result = init_response.result.unwrap();
    assert_eq!(result["protocolVersion"], "2025-03-26");
    assert_eq!(result["serverInfo"]["name"], "tradingview-mcp");

    // 2. Send initialized notification
    let notif_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: None,
        method: "notifications/initialized".to_string(),
        params: None,
    };
    let notif_response = server.handle_request(notif_request).await;
    assert!(notif_response.error.is_none());
    assert!(notif_response.result.is_none());
    assert_eq!(notif_response.id, None);

    // 3. List tools
    let tools_request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(2)),
        method: "tools/list".to_string(),
        params: None,
    };
    let tools_response = server.handle_request(tools_request).await;
    assert!(tools_response.error.is_none());

    let tools_result = tools_response.result.unwrap();
    let tools = tools_result["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 15);

    // 4. Verify tool schemas are valid
    for tool in tools {
        let name = tool["name"].as_str().unwrap();
        let schema = &tool["inputSchema"];

        // Verify schema structure
        assert!(schema["type"] == "object", "{}: schema type should be object", name);
        assert!(schema["properties"].is_object(), "{}: should have properties", name);

        // Verify no $schema or title fields (per MCP spec cleanup)
        assert!(!schema.as_object().unwrap().contains_key("$schema"),
            "{}: should not have $schema", name);
    }
}

/// Test error handling for malformed requests
#[tokio::test]
async fn test_error_handling() {
    let server = McpServer::new();

    // Test missing method
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: Some(json!(1)),
        method: "".to_string(),
        params: None,
    };
    let response = server.handle_request(request).await;
    assert!(response.error.is_some());

    // Test invalid JSON-RPC version
    let request = JsonRpcRequest {
        jsonrpc: "invalid".to_string(),
        id: Some(json!(1)),
        method: "tools/list".to_string(),
        params: None,
    };
    let response = server.handle_request(request).await;
    assert!(response.error.is_some());
    let error = response.error.unwrap();
    assert_eq!(error.code, error_codes::INVALID_REQUEST);
}

/// Test that verifies response content types
#[tokio::test]
async fn test_response_content_types() {
    let client = Arc::new(create_test_client().await);

    // Test scan_stocks returns proper content
    let args = Some(json!({
        "filters": {"market_cap": {"gt": 1000000000}},
        "limit": 5
    }));
    let result = handle_scan_stocks(client.clone(), args).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.is_error, Some(false));
    assert!(!response.content.is_empty());

    // Verify content type
    let content = &response.content[0];
    assert_eq!(content.content_type, "text");

    // Verify content is valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&content.text)
        .expect("Content should be valid JSON");
    assert!(parsed.is_object() || parsed.is_array());
}

/// Test batch-like sequence of operations
#[tokio::test]
async fn test_handler_response_consistency() {
    let client = Arc::new(create_test_client().await);

    // Test that handlers return consistent response structures
    let test_cases = vec![
        ("scan_stocks", json!({"filters": {}, "limit": 3})),
        ("get_debt_maturity", json!({"symbol": "NASDAQ:AAPL"})),
        ("get_company_profile", json!({"symbol": "NASDAQ:AAPL"})),
        ("get_market_news", json!({"symbol": "NASDAQ:AAPL", "limit": 2})),
    ];

    for (tool_name, params) in test_cases {
        let args = Some(params);
        let result = match tool_name {
            "scan_stocks" => handle_scan_stocks(client.clone(), args).await,
            "get_debt_maturity" => handle_get_debt_maturity(client.clone(), args).await,
            "get_company_profile" => handle_get_company_profile(client.clone(), args).await,
            "get_market_news" => handle_get_market_news(client.clone(), args).await,
            _ => continue,
        };

        assert!(result.is_ok(), "{} should succeed", tool_name);
        let response = result.unwrap();

        // Verify standard response structure
        assert_eq!(response.is_error, Some(false), "{}: is_error should be false", tool_name);
        assert!(!response.content.is_empty(), "{}: should have content", tool_name);

        // Verify content is valid JSON
        for content in &response.content {
            assert_eq!(content.content_type, "text", "{}: content type should be text", tool_name);
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(&content.text);
            assert!(parsed.is_ok(), "{}: content should be valid JSON", tool_name);
        }
    }
}
