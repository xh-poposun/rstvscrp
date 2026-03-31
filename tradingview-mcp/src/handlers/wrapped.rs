use crate::client::TradingViewMcpClient;
use crate::{
    handlers::{
        quote::{handle_get_quote as quote_handler, handle_search_symbols as search_handler},
        historical::handle_get_historical as historical_handler,
        fundamentals::{
            handle_get_fundamentals as fundamentals_handler,
            handle_get_financial_statements as financial_statements_handler,
            handle_get_credit_ratings as credit_ratings_handler,
        },
        screener::handle_scan_stocks as screener_handler,
        calendar::{
            handle_get_earnings_calendar as earnings_calendar_handler,
            handle_get_dividend_calendar as dividend_calendar_handler,
        },
        technical::{
            handle_calculate_rsi as rsi_handler,
            handle_calculate_macd as macd_handler,
            handle_compute_macd_signal as macd_signal_handler,
        },
        sec::handle_get_debt_maturity as debt_maturity_handler,
    },
    tools::{
        GetQuoteParams, GetQuoteResponse,
        SearchSymbolsParams, SearchSymbolsResponse,
        GetHistoricalParams, GetHistoricalResponse,
        GetFundamentalsParams, GetFundamentalsResponse,
        GetFinancialStatementsParams, GetFinancialStatementsResponse,
        GetCreditRatingsParams, GetCreditRatingsResponse,
        ScanStocksParams, ScanStocksResponse,
        GetEarningsCalendarParams, GetEarningsCalendarResponse,
        GetDividendCalendarParams, GetDividendCalendarResponse,
        CalculateRsiParams, CalculateRsiResponse,
        CalculateMacdParams, CalculateMacdResponse,
        ComputeMacdSignalParams, ComputeMacdSignalResponse,
        GetDebtMaturityParams, GetDebtMaturityResponse,
    },
    JsonRpcError, ToolCallResult, ToolContent,
};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

fn parse_args<T: serde::de::DeserializeOwned>(args: Option<Value>) -> Result<T, JsonRpcError> {
    let args = args.ok_or_else(|| JsonRpcError::invalid_params("Missing arguments"))?;
    serde_json::from_value(args).map_err(|e| JsonRpcError::invalid_params(format!("Invalid arguments: {}", e)))
}

fn success_response<T: serde::Serialize>(data: T) -> Result<ToolCallResult, JsonRpcError> {
    let content = ToolContent::text(
        serde_json::to_string(&data).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );
    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

pub async fn handle_get_quote(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetQuoteParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = quote_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    let response = GetQuoteResponse {
        symbol: result.symbol,
        price: result.price,
        change: result.change,
        change_percent: result.change_percent,
        volume: result.volume,
    };
    
    success_response(response)
}

pub async fn handle_search_symbols(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: SearchSymbolsParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = search_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_get_historical(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetHistoricalParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = historical_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_get_fundamentals(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetFundamentalsParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = fundamentals_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_get_financial_statements(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetFinancialStatementsParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = financial_statements_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_get_credit_ratings(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetCreditRatingsParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = credit_ratings_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_scan_stocks(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: ScanStocksParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = screener_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_get_earnings_calendar(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetEarningsCalendarParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = earnings_calendar_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_get_dividend_calendar(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetDividendCalendarParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = dividend_calendar_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_calculate_rsi(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: CalculateRsiParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = rsi_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_calculate_macd(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: CalculateMacdParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = macd_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_compute_macd_signal(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: ComputeMacdSignalParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = macd_signal_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}

pub async fn handle_get_debt_maturity(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetDebtMaturityParams = parse_args(args)?;
    
    let client = TradingViewMcpClient::new()
        .await
        .map_err(|e| JsonRpcError::internal_error(format!("Failed to create client: {}", e)))?;
    
    let result = debt_maturity_handler(&client, params).await
        .map_err(|e| JsonRpcError::internal_error(format!("Handler error: {}", e)))?;
    
    success_response(result)
}
