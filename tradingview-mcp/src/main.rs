use axum::{
    routing::{get, post},
    Router,
    Json,
    response::Response,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};
use tracing_subscriber::{fmt, EnvFilter};

use tradingview_mcp::McpServer;
use tradingview_mcp::ToolCallResult;
use tradingview_mcp::JsonRpcError;
use tradingview_mcp::JsonRpcRequest as McpJsonRpcRequest;
use tradingview_mcp::client::TradingViewMcpClient;
use tradingview_mcp::handlers::{
    handle_get_quote,
    handle_search_symbols,
    handle_get_historical,
    handle_get_fundamentals,
    handle_get_financial_statements,
    handle_get_credit_ratings,
    handle_scan_stocks,
    handle_get_earnings_calendar,
    handle_get_dividend_calendar,
    handle_calculate_rsi,
    handle_calculate_macd,
    handle_compute_macd_signal,
    handle_get_debt_maturity,
    handle_get_company_profile,
    handle_get_market_news,
};

/// Application configuration loaded from environment variables
#[derive(Debug, Clone)]
struct Config {
    /// TradingView username for authentication
    tvdata_username: String,
    /// TradingView password for authentication
    tvdata_password: String,
    /// Server port (default: 3000)
    port: u16,
    /// Log level for tracing (default: info)
    log_level: String,
}

impl Config {
    /// Load configuration from environment variables
    fn from_env() -> Result<Self, ConfigError> {
        let tvdata_username = std::env::var("TVDATA_USERNAME")
            .unwrap_or_else(|_| "demo".to_string());

        let tvdata_password = std::env::var("TVDATA_PASSWORD")
            .unwrap_or_else(|_| "demo".to_string());

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);

        let log_level = std::env::var("RUST_LOG")
            .unwrap_or_else(|_| "info".to_string());

        Ok(Config {
            tvdata_username,
            tvdata_password,
            port,
            log_level,
        })
    }
}

/// Configuration loading errors
#[derive(Debug)]
enum ConfigError {
    MissingEnvVar(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingEnvVar(var) => {
                write!(f, "Missing required environment variable: {}", var)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Health check response
#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

/// MCP JSON-RPC request (HTTP API version)
#[derive(Serialize, Deserialize, Debug)]
struct HttpJsonRpcRequest {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    method: String,
    #[serde(default)]
    params: Option<serde_json::Value>,
}

/// MCP JSON-RPC response (HTTP API version)
#[derive(Serialize, Debug)]
struct HttpJsonRpcResponse {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<HttpJsonRpcError>,
}

/// MCP JSON-RPC error (HTTP API version)
#[derive(Serialize, Debug)]
struct HttpJsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

impl HttpJsonRpcResponse {
    fn success(id: Option<serde_json::Value>, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    fn error(id: Option<serde_json::Value>, code: i32, message: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(HttpJsonRpcError {
                code,
                message,
                data: None,
            }),
        }
    }
}

/// Application state shared across handlers
struct AppState {
    config: Config,
    mcp_server: RwLock<McpServer>,
}

/// Health check handler
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

/// MCP endpoint handler
async fn mcp_handler(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    axum::extract::Json(request): axum::extract::Json<HttpJsonRpcRequest>,
) -> Json<HttpJsonRpcResponse> {
    let request_id = request.id.clone();
    let request_method = request.method.clone();

    // Access log - INFO level
    tracing::info!(
        method = %request_method,
        id = ?request_id,
        "MCP request received"
    );

    // Debug log - full request body
    tracing::debug!(
        request = %serde_json::to_string(&request).unwrap_or_default(),
        "MCP request body"
    );

    if request.jsonrpc != "2.0" {
        let error_response = HttpJsonRpcResponse::error(
            request.id,
            -32600,
            "Invalid Request: jsonrpc must be '2.0'".to_string(),
        );
        tracing::info!(
            method = %request_method,
            id = ?request_id,
            has_error = true,
            "MCP request completed"
        );
        return Json(error_response);
    }

    let server = state.mcp_server.read().await;
    let response = server.handle_request(McpJsonRpcRequest {
        jsonrpc: request.jsonrpc,
        id: request.id,
        method: request.method,
        params: request.params,
    }).await;

    // Check if this is a notification (no id means no response should be sent)
    if response.id.is_none() {
        // Access log - completion for notification
        tracing::info!(
            method = %request_method,
            "MCP notification processed"
        );
        // Return 202 Accepted with empty body for notifications (no JSON-RPC response)
        return Json(HttpJsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: None,
            result: None,
            error: None,
        });
    }

    // Debug log - full response body
    let json_response = HttpJsonRpcResponse {
        jsonrpc: response.jsonrpc,
        id: response.id,
        result: response.result,
        error: response.error.map(|e| HttpJsonRpcError {
            code: e.code,
            message: e.message,
            data: e.data,
        }),
    };
    tracing::debug!(
        response = %serde_json::to_string(&json_response).unwrap_or_default(),
        "MCP response body"
    );

    // Access log - completion
    let response_id = json_response.id.clone();
    let has_error = json_response.error.is_some();
    tracing::info!(
        method = %request_method,
        id = ?response_id,
        has_error = has_error,
        "MCP request completed"
    );

    Json(json_response)
}

/// Global error handler
async fn handle_error(err: axum::BoxError) -> Response {
    error!("Unhandled error: {}", err);

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body("Internal Server Error".into())
        .unwrap()
}

/// Create and configure the MCP server with all handlers
async fn create_mcp_server(username: &str, password: &str) -> McpServer {
    let config = tradingview_mcp::client::ClientConfig {
        username: Some(username.to_string()),
        password: Some(password.to_string()),
        ..Default::default()
    };
    let client = Arc::new(TradingViewMcpClient::with_config(config).await.expect("Failed to create TradingViewMcpClient"));
    let mut server = McpServer::with_client(client);

    // Register all handlers using function pointers
    server.register_handler("get_quote", handle_get_quote_wrapper);
    server.register_handler("search_symbols", handle_search_symbols_wrapper);
    server.register_handler("get_historical", handle_get_historical_wrapper);
    server.register_handler("get_fundamentals", handle_get_fundamentals_wrapper);
    server.register_handler("get_financial_statements", handle_get_financial_statements_wrapper);
    server.register_handler("get_credit_ratings", handle_get_credit_ratings_wrapper);
    server.register_handler("scan_stocks", handle_scan_stocks_wrapper);
    server.register_handler("get_earnings_calendar", handle_get_earnings_calendar_wrapper);
    server.register_handler("get_dividend_calendar", handle_get_dividend_calendar_wrapper);
    server.register_handler("calculate_rsi", handle_calculate_rsi_wrapper);
    server.register_handler("calculate_macd", handle_calculate_macd_wrapper);
    server.register_handler("get_debt_maturity", handle_get_debt_maturity_wrapper);
    server.register_handler("compute_macd_signal", handle_compute_macd_signal_wrapper);
    server.register_handler("get_company_profile", handle_get_company_profile_wrapper);
    server.register_handler("get_market_news", handle_get_market_news_wrapper);

    server
}

// Wrapper functions to avoid closure lifetime issues
fn handle_get_quote_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_get_quote(client, args))
}

fn handle_search_symbols_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_search_symbols(client, args))
}

fn handle_get_historical_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_get_historical(client, args))
}

fn handle_get_fundamentals_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_get_fundamentals(client, args))
}

fn handle_get_financial_statements_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_get_financial_statements(client, args))
}

fn handle_get_credit_ratings_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_get_credit_ratings(client, args))
}

fn handle_scan_stocks_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_scan_stocks(client, args))
}

fn handle_get_earnings_calendar_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_get_earnings_calendar(client, args))
}

fn handle_get_dividend_calendar_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_get_dividend_calendar(client, args))
}

fn handle_calculate_rsi_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_calculate_rsi(client, args))
}

fn handle_calculate_macd_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_calculate_macd(client, args))
}

fn handle_get_debt_maturity_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_get_debt_maturity(client, args))
}

fn handle_compute_macd_signal_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_compute_macd_signal(client, args))
}

fn handle_get_company_profile_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_get_company_profile(client, args))
}

fn handle_get_market_news_wrapper(client: Arc<TradingViewMcpClient>, args: Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>> {
    Box::pin(handle_get_market_news(client, args))
}

/// Initialize logging using EnvFilter and fmt layer
fn init_logging(log_level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_level(true)
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    init_logging(&config.log_level);
    info!("Starting TradingView MCP server...");
    info!("Configuration loaded successfully");

    info!("Server will listen on port {}", config.port);

    let mcp_server = create_mcp_server(&config.tvdata_username, &config.tvdata_password).await;

    let app_state = Arc::new(AppState {
        config: config.clone(),
        mcp_server: RwLock::new(mcp_server),
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/mcp", post(mcp_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shutdown complete");
    Ok(())
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, starting graceful shutdown...");
        }
        _ = terminate => {
            info!("Received SIGTERM, starting graceful shutdown...");
        }
    }
}
