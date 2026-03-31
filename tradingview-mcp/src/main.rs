use axum::{
    routing::{get, post},
    Router,
    Json,
    response::Response,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

use tradingview_mcp::McpServer;
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

        Ok(Config {
            tvdata_username,
            tvdata_password,
            port,
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

/// MCP JSON-RPC request
#[derive(Deserialize, Debug)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    method: String,
    #[serde(default)]
    params: Option<serde_json::Value>,
}

/// MCP JSON-RPC response
#[derive(Serialize, Debug)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

/// MCP JSON-RPC error
#[derive(Serialize, Debug)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

impl JsonRpcResponse {
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
            error: Some(JsonRpcError {
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
    axum::extract::Json(request): axum::extract::Json<JsonRpcRequest>,
) -> Json<JsonRpcResponse> {
    tracing::debug!("MCP request: {:?}", request);

    if request.jsonrpc != "2.0" {
        return Json(JsonRpcResponse::error(
            request.id,
            -32600,
            "Invalid Request: jsonrpc must be '2.0'".to_string(),
        ));
    }

    let server = state.mcp_server.read().await;
    let response = server.handle_request(tradingview_mcp::JsonRpcRequest {
        jsonrpc: request.jsonrpc,
        id: request.id,
        method: request.method,
        params: request.params,
    }).await;

    Json(JsonRpcResponse {
        jsonrpc: response.jsonrpc,
        id: response.id,
        result: response.result,
        error: response.error.map(|e| JsonRpcError {
            code: e.code,
            message: e.message,
            data: e.data,
        }),
    })
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
fn create_mcp_server() -> McpServer {
    let mut server = McpServer::new();

    server.register_handler("get_quote", handle_get_quote);
    server.register_handler("search_symbols", handle_search_symbols);
    server.register_handler("get_historical", handle_get_historical);
    server.register_handler("get_fundamentals", handle_get_fundamentals);
    server.register_handler("get_financial_statements", handle_get_financial_statements);
    server.register_handler("get_credit_ratings", handle_get_credit_ratings);
    server.register_handler("scan_stocks", handle_scan_stocks);
    server.register_handler("get_earnings_calendar", handle_get_earnings_calendar);
    server.register_handler("get_dividend_calendar", handle_get_dividend_calendar);
    server.register_handler("calculate_rsi", handle_calculate_rsi);
    server.register_handler("calculate_macd", handle_calculate_macd);
    server.register_handler("get_debt_maturity", handle_get_debt_maturity);
    server.register_handler("compute_macd_signal", handle_compute_macd_signal);

    server
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("Starting TradingView MCP server...");

    let config = Config::from_env()?;
    info!("Configuration loaded successfully");

    info!("Server will listen on port {}", config.port);

    let mcp_server = create_mcp_server();

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
