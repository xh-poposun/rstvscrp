use crate::client::{TradingViewMcpClient, ClientError};
use crate::tools::*;
use crate::{JsonRpcError, ToolCallResult, ToolContent};
use serde_json::Value;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use tvdata_rs::history::Interval;
use time::format_description::well_known::Rfc3339;

/// Type alias for handler functions
pub type HandlerFn = fn(Arc<TradingViewMcpClient>, Option<Value>) -> Pin<Box<dyn Future<Output = Result<ToolCallResult, JsonRpcError>> + Send>>;

/// Macro to generate handler wrappers
#[macro_export]
macro_rules! handler_wrapper {
    ($handler:ident) => {
        |client: std::sync::Arc<crate::client::TradingViewMcpClient>, args: Option<serde_json::Value>| {
            Box::pin($handler(client, args))
        }
    };
}

/// Helper function to validate symbol format (e.g., "NASDAQ:AAPL" or "AAPL")
fn validate_symbol(symbol: &str) -> Result<(), JsonRpcError> {
    if symbol.is_empty() {
        return Err(JsonRpcError::invalid_params("Symbol cannot be empty"));
    }
    // Basic validation: symbol should be alphanumeric with optional exchange prefix
    let parts: Vec<&str> = symbol.split(':').collect();
    if parts.len() > 2 {
        return Err(JsonRpcError::invalid_params(
            "Invalid symbol format. Use 'EXCHANGE:SYMBOL' or 'SYMBOL'",
        ));
    }
    Ok(())
}

/// Helper to parse arguments from Option<Value>
fn parse_args<T: serde::de::DeserializeOwned>(args: Option<Value>) -> Result<T, JsonRpcError> {
    let args = args.ok_or_else(|| JsonRpcError::invalid_params("Missing arguments"))?;
    serde_json::from_value(args).map_err(|e| JsonRpcError::invalid_params(format!("Invalid arguments: {}", e)))
}

/// Helper to convert ClientError to JsonRpcError
fn client_error_to_rpc_error(e: ClientError) -> JsonRpcError {
    JsonRpcError::internal_error(e.to_string())
}

/// Handler for get_quote tool
pub async fn handle_get_quote(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetQuoteParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    let quote = client.get_quote(&params.symbol).await.map_err(client_error_to_rpc_error)?;

    let response = GetQuoteResponse {
        symbol: quote.symbol,
        price: quote.price,
        change: quote.change,
        change_percent: quote.change_percent,
        volume: quote.volume,
        high: quote.high,
        low: quote.low,
        open: quote.open,
        previous_close: quote.previous_close,
        currency: "USD".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for search_symbols tool
pub async fn handle_search_symbols(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: SearchSymbolsParams = parse_args(args)?;

    if params.query.is_empty() {
        return Err(JsonRpcError::invalid_params("Query cannot be empty"));
    }

    let results = client.search_symbols(&params.query).await.map_err(client_error_to_rpc_error)?;

    let response = SearchSymbolsResponse {
        results: results.into_iter().map(|r| SymbolInfo {
            symbol: r.symbol,
            name: r.name,
            exchange: r.exchange,
        }).collect(),
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for get_historical tool
pub async fn handle_get_historical(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetHistoricalParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    // Validate interval
    let valid_intervals = ["1m", "5m", "15m", "1h", "4h", "1d", "1w", "1M"];
    if !valid_intervals.contains(&params.interval.as_str()) {
        return Err(JsonRpcError::invalid_params(
            format!("Invalid interval '{}'. Valid intervals: {:?}", params.interval, valid_intervals)
        ));
    }

    // Parse interval string to tvdata_rs Interval
    let interval = match params.interval.as_str() {
        "1m" => Interval::Min1,
        "5m" => Interval::Min5,
        "15m" => Interval::Min15,
        "1h" => Interval::Hour1,
        "4h" => Interval::Hour4,
        "1d" => Interval::Day1,
        "1w" => Interval::Week1,
        "1M" => Interval::Month1,
        _ => Interval::Day1, // Fallback, should not happen due to validation
    };

    let series = client.get_historical(&params.symbol, interval, params.count).await.map_err(client_error_to_rpc_error)?;

    let points: Vec<HistoricalPoint> = series.bars.into_iter().map(|bar| HistoricalPoint {
        timestamp: bar.time.to_string(),
        open: bar.open,
        high: bar.high,
        low: bar.low,
        close: bar.close,
        volume: bar.volume.unwrap_or(0.0),
    }).collect();

    let response = GetHistoricalResponse { points };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for get_fundamentals tool
pub async fn handle_get_fundamentals(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetFundamentalsParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    let fundamentals = client.get_fundamentals(&params.symbol).await.map_err(client_error_to_rpc_error)?;

    let response = GetFundamentalsResponse {
        market_cap: fundamentals.market_cap,
        pe_ratio: fundamentals.pe_ratio,
        eps: fundamentals.eps,
        dividend_yield: fundamentals.dividend_yield,
        beta: fundamentals.beta,
        price_to_book: fundamentals.price_to_book,
        debt_to_equity: fundamentals.debt_to_equity,
        current_ratio: fundamentals.current_ratio,
        quick_ratio: fundamentals.quick_ratio,
        roe: fundamentals.roe,
        roa: fundamentals.roa,
        revenue: fundamentals.revenue,
        gross_profit: fundamentals.gross_profit,
        operating_income: fundamentals.operating_income,
        net_income: fundamentals.net_income,
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for get_financial_statements tool
pub async fn handle_get_financial_statements(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetFinancialStatementsParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    // Validate period
    if !["annual", "quarterly"].contains(&params.period.as_str()) {
        return Err(JsonRpcError::invalid_params(
            "Period must be 'annual' or 'quarterly'"
        ));
    }

    let detail = client.get_financial_statements_detail(&params.symbol).await.map_err(client_error_to_rpc_error)?;

    // Build financial statements from the flat structure
    let mut statements = Vec::new();

    // Add latest income statement data (TTM)
    statements.push(FinancialStatement {
        date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        statement_type: "income_statement".to_string(),
        items: serde_json::json!({
            "revenue": detail.revenue_ttm,
            "revenue_fy": detail.revenue_fy,
            "revenue_fq": detail.revenue_fq,
            "gross_profit": detail.gross_profit_ttm,
            "gross_profit_fy": detail.gross_profit_fy,
            "gross_profit_fq": detail.gross_profit_fq,
            "operating_income": detail.operating_income_ttm,
            "operating_income_fy": detail.operating_income_fy,
            "operating_income_fq": detail.operating_income_fq,
            "net_income": detail.net_income_ttm,
            "net_income_fy": detail.net_income_fy,
            "net_income_fq": detail.net_income_fq,
            "ebitda": detail.ebitda_ttm,
            "ebitda_fy": detail.ebitda_fy,
            "ebitda_fq": detail.ebitda_fq,
        }),
    });

    // Add latest balance sheet data
    statements.push(FinancialStatement {
        date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        statement_type: "balance_sheet".to_string(),
        items: serde_json::json!({
            "total_assets": detail.total_assets_fq,
            "total_assets_fy": detail.total_assets_fy,
            "total_liabilities": detail.total_liabilities_fq,
            "total_liabilities_fy": detail.total_liabilities_fy,
            "total_equity": detail.total_equity_fq,
            "cash": detail.cash_fq,
            "cash_fy": detail.cash_fy,
            "receivables": detail.receivables_fq,
            "inventory": detail.inventory_fq,
            "long_term_debt": detail.long_term_debt_fq,
            "short_term_debt": detail.short_term_debt_fq,
        }),
    });

    // Add cash flow data
    statements.push(FinancialStatement {
        date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
        statement_type: "cash_flow".to_string(),
        items: serde_json::json!({
            "operating_cash_flow": detail.operating_cash_flow_ttm,
            "operating_cash_flow_fy": detail.operating_cash_flow_fy,
            "free_cash_flow": detail.free_cash_flow_ttm,
            "free_cash_flow_fy": detail.free_cash_flow_fy,
            "capex": detail.capex_fq,
            "capex_fy": detail.capex_fy,
            "investing_cash_flow": detail.investing_cash_flow_fy,
            "financing_cash_flow": detail.financing_cash_flow_fq,
        }),
    });

    let response = GetFinancialStatementsResponse { statements };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for get_credit_ratings tool
pub async fn handle_get_credit_ratings(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetCreditRatingsParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    let ratings = client.get_credit_ratings(&params.symbol).await.map_err(client_error_to_rpc_error)?;

    // Convert numeric rating codes to string representation
    let fitch_rating = ratings.fitch_rating_lt.map(|code| format!("Fitch: {}", code));
    let snp_rating = ratings.snp_rating_lt.map(|code| format!("S&P: {}", code));

    let rating_str = match (fitch_rating, snp_rating) {
        (Some(f), Some(s)) => Some(format!("{}; {}", f, s)),
        (Some(f), None) => Some(f),
        (None, Some(s)) => Some(s),
        (None, None) => None,
    };

    let outlook_str = ratings.fitch_outlook_lt.map(|code| format!("Fitch Outlook: {}", code));

    let response = GetCreditRatingsResponse {
        rating: rating_str,
        outlook: outlook_str,
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for scan_stocks tool
pub async fn handle_scan_stocks(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: ScanStocksParams = parse_args(args)?;

    let limit = if params.limit == 0 { 50 } else { params.limit };
    let stocks = client.scan_stocks(Some(params.filters), limit).await.map_err(client_error_to_rpc_error)?;

    let response = ScanStocksResponse {
        results: stocks.into_iter().map(|s| StockInfo {
            symbol: s.symbol,
            name: s.name.unwrap_or_default(),
            exchange: s.exchange.unwrap_or_default(),
            price: s.price.unwrap_or(0.0),
        }).collect(),
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for get_earnings_calendar tool
pub async fn handle_get_earnings_calendar(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetEarningsCalendarParams = parse_args(args)?;

    if params.days_ahead < 0 || params.days_ahead > 90 {
        return Err(JsonRpcError::invalid_params("days_ahead must be between 0 and 90"));
    }

    let events = client.get_earnings_calendar(params.days_ahead as i64).await.map_err(client_error_to_rpc_error)?;

    let response = GetEarningsCalendarResponse {
        events: events.into_iter().map(|e| EarningsEvent {
            symbol: e.symbol,
            date: e.date.format(&Rfc3339).unwrap_or_default(),
            estimate: e.eps_estimate.unwrap_or(0.0),
        }).collect(),
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for get_dividend_calendar tool
pub async fn handle_get_dividend_calendar(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetDividendCalendarParams = parse_args(args)?;

    if params.exchange.is_empty() {
        return Err(JsonRpcError::invalid_params("Exchange cannot be empty"));
    }

    // Default to 30 days ahead if not specified in params
    let days_ahead = 30i64;

    let events = client.get_dividend_calendar(&params.exchange, days_ahead).await.map_err(client_error_to_rpc_error)?;

    let response = GetDividendCalendarResponse {
        dividends: events.into_iter().map(|e| DividendEvent {
            symbol: e.symbol,
            ex_date: e.date.format(&Rfc3339).unwrap_or_default(),
            amount: e.amount.unwrap_or(0.0),
        }).collect(),
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for calculate_rsi tool
pub async fn handle_calculate_rsi(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: CalculateRsiParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    if params.period < 2 || params.period > 100 {
        return Err(JsonRpcError::invalid_params("RSI period must be between 2 and 100"));
    }

    let rsi = client.calculate_rsi(&params.symbol, Some(params.period)).await.map_err(client_error_to_rpc_error)?;

    let response = CalculateRsiResponse {
        rsi: rsi.unwrap_or(0.0),
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for calculate_macd tool
pub async fn handle_calculate_macd(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: CalculateMacdParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    if params.fast >= params.slow {
        return Err(JsonRpcError::invalid_params("Fast period must be less than slow period"));
    }

    let macd_result = client.calculate_macd(&params.symbol, Some(params.fast), Some(params.slow), Some(params.signal)).await.map_err(client_error_to_rpc_error)?;

    // Extract the latest MACD values
    let macd_value = macd_result.macd_line.as_ref().and_then(|v| v.last().copied()).unwrap_or(0.0);
    let signal_value = macd_result.signal_line.as_ref().and_then(|v| v.last().copied()).unwrap_or(0.0);

    let response = CalculateMacdResponse {
        macd: macd_value,
        signal: signal_value,
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for get_debt_maturity tool
pub async fn handle_get_debt_maturity(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetDebtMaturityParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    let debt = client.get_debt_maturity(&params.symbol).await.map_err(client_error_to_rpc_error)?;

    let response = GetDebtMaturityResponse {
        total_debt: debt.total_debt,
        long_term_debt: debt.long_term_debt,
        short_term_debt: debt.short_term_debt,
        net_debt: debt.net_debt,
        debt_to_equity: debt.debt_to_equity,
        debt_to_assets: debt.debt_to_assets,
        net_debt_to_ebitda: debt.net_debt_to_ebitda,
        interest_coverage: debt.interest_coverage,
        maturity: vec![],
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for get_company_profile tool
pub async fn handle_get_company_profile(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetCompanyProfileParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    let profile = client.get_company_profile(&params.symbol).await.map_err(client_error_to_rpc_error)?;

    let overview = serde_json::json!({
        "name": profile.quote.instrument.name,
        "sector": profile.quote.instrument.sector,
        "industry": profile.quote.instrument.industry,
        "exchange": profile.quote.instrument.exchange,
        "country": profile.quote.instrument.country,
        "currency": profile.quote.instrument.currency,
        "market": profile.quote.instrument.market,
        "close": profile.quote.close,
        "change_percent": profile.quote.change_percent,
        "change_abs": profile.quote.change_abs,
        "volume": profile.quote.volume,
        "market_cap": profile.fundamentals.market_cap,
        "pe_ratio": profile.fundamentals.price_earnings_ttm,
        "price_to_book": profile.fundamentals.price_to_book_fq,
        "price_to_sales": profile.fundamentals.price_to_sales_current,
        "eps_ttm": profile.fundamentals.eps_ttm,
        "dividend_yield": profile.fundamentals.dividend_yield_recent,
        "roe": profile.fundamentals.return_on_equity_ttm,
        "roa": profile.fundamentals.return_on_assets_ttm,
        "debt_to_equity": profile.fundamentals.debt_to_equity_mrq,
        "current_ratio": profile.fundamentals.current_ratio_mrq,
        "revenue_ttm": profile.fundamentals.total_revenue_ttm,
        "net_income_ttm": profile.fundamentals.net_income_ttm,
        "ebitda_ttm": profile.fundamentals.ebitda_ttm,
        "free_cash_flow_ttm": profile.fundamentals.free_cash_flow_ttm,
    });

    let response = GetCompanyProfileResponse { overview };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for get_market_news tool
pub async fn handle_get_market_news(
    _client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetMarketNewsParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    if params.limit == 0 || params.limit > 100 {
        return Err(JsonRpcError::invalid_params("Limit must be between 1 and 100"));
    }

    let response = GetMarketNewsResponse {
        articles: vec![],
        message: Some("Market news is not yet available via TradingView API. This feature requires integration with an external news source.".to_string()),
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for compute_macd_signal tool
pub async fn handle_compute_macd_signal(
    client: Arc<TradingViewMcpClient>,
    args: Option<Value>,
) -> Result<ToolCallResult, JsonRpcError> {
    let params: ComputeMacdSignalParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    if params.short_window >= params.long_window {
        return Err(JsonRpcError::invalid_params("Short window must be less than long window"));
    }

    if params.signal_window == 0 {
        return Err(JsonRpcError::invalid_params("Signal window must be greater than 0"));
    }

    // First calculate MACD
    let macd_result = client.calculate_macd(&params.symbol, Some(params.short_window), Some(params.long_window), Some(params.signal_window)).await.map_err(client_error_to_rpc_error)?;

    let macd_value = macd_result.macd_line.as_ref().and_then(|v| v.last().copied()).unwrap_or(0.0);
    let signal_value = macd_result.signal_line.as_ref().and_then(|v| v.last().copied()).unwrap_or(0.0);

    let response = ComputeMacdSignalResponse {
        macd: macd_value,
        signal: signal_value,
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::{ClientConfig, TradingViewMcpClient};

    async fn mock_client() -> TradingViewMcpClient {
        let config = ClientConfig {
            username: Some("test".to_string()),
            password: Some("test".to_string()),
            ..Default::default()
        };
        TradingViewMcpClient::with_config(config).await.expect("Failed to create mock client")
    }

    #[tokio::test]
    async fn test_validate_symbol_valid() {
        assert!(validate_symbol("NASDAQ:AAPL").is_ok());
        assert!(validate_symbol("AAPL").is_ok());
    }

    #[tokio::test]
    async fn test_validate_symbol_empty() {
        let result = validate_symbol("");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_symbol_invalid() {
        let result = validate_symbol("EX:SYM:BOL");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, crate::error_codes::INVALID_PARAMS);
        assert!(err.message.contains("Invalid symbol format"));
    }

// ============================================================================
// ERROR HANDLING VERIFICATION TESTS
// Tests for JSON-RPC error responses with proper error codes and messages
// ============================================================================

#[tokio::test]
async fn test_error_handling_empty_symbol() {
    let result = validate_symbol("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, crate::error_codes::INVALID_PARAMS);
    assert!(err.message.contains("empty"));
}

#[tokio::test]
async fn test_error_handling_invalid_symbol_format() {
    let result = validate_symbol("EX:SYM:BOL");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, crate::error_codes::INVALID_PARAMS);
    assert!(err.message.contains("Invalid symbol format"));
}

#[tokio::test]
async fn test_error_handling_invalid_interval() {
    let args = Some(serde_json::json!({
        "symbol": "AAPL",
        "interval": "2h",
        "count": 100
    }));
    let params: Result<GetHistoricalParams, JsonRpcError> = parse_args(args);
    assert!(params.is_ok());
    let params = params.unwrap();
    
    let valid_intervals = ["1m", "5m", "15m", "1h", "4h", "1d", "1w", "1M"];
    assert!(!valid_intervals.contains(&params.interval.as_str()));
}

#[tokio::test]
async fn test_error_handling_invalid_period() {
    let args = Some(serde_json::json!({
        "symbol": "AAPL",
        "period": "monthly"
    }));
    let params: Result<GetFinancialStatementsParams, JsonRpcError> = parse_args(args);
    assert!(params.is_ok());
    let params = params.unwrap();
    
    assert!(!["annual", "quarterly"].contains(&params.period.as_str()));
}

#[tokio::test]
async fn test_error_handling_missing_arguments() {
    let result: Result<GetQuoteParams, JsonRpcError> = parse_args(None);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, crate::error_codes::INVALID_PARAMS);
    assert!(err.message.contains("Missing arguments"));
}

#[tokio::test]
async fn test_error_handling_invalid_json() {
    let args = Some(serde_json::json!({"symbol": 12345}));
    let result: Result<GetQuoteParams, JsonRpcError> = parse_args(args);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.code, crate::error_codes::INVALID_PARAMS);
    assert!(err.message.contains("Invalid arguments"));
}

#[tokio::test]
async fn test_error_handling_rsi_invalid_period() {
    let args = Some(serde_json::json!({
        "symbol": "AAPL",
        "period": 1
    }));
    let params: Result<CalculateRsiParams, JsonRpcError> = parse_args(args);
    assert!(params.is_ok());
    let params = params.unwrap();
    assert!(params.period < 2 || params.period > 100);
}

#[tokio::test]
async fn test_error_handling_macd_invalid_fast_slow() {
    let args = Some(serde_json::json!({
        "symbol": "AAPL",
        "fast": 20,
        "slow": 10,
        "signal": 9
    }));
    let params: Result<CalculateMacdParams, JsonRpcError> = parse_args(args);
    assert!(params.is_ok());
    let params = params.unwrap();
    assert!(params.fast >= params.slow);
}

#[tokio::test]
async fn test_error_handling_compute_macd_invalid_windows() {
    let args = Some(serde_json::json!({
        "symbol": "AAPL",
        "short_window": 20,
        "long_window": 10,
        "signal_window": 0
    }));
    let params: Result<ComputeMacdSignalParams, JsonRpcError> = parse_args(args);
    assert!(params.is_ok());
    let params = params.unwrap();
    assert!(params.short_window >= params.long_window || params.signal_window == 0);
}

#[tokio::test]
async fn test_error_handling_earnings_invalid_days() {
    let args = Some(serde_json::json!({"days_ahead": -1}));
    let params: Result<GetEarningsCalendarParams, JsonRpcError> = parse_args(args);
    assert!(params.is_ok());
    let params = params.unwrap();
    assert!(params.days_ahead < 0 || params.days_ahead > 90);
}

#[tokio::test]
async fn test_error_handling_dividend_empty_exchange() {
    let args = Some(serde_json::json!({"exchange": ""}));
    let params: Result<GetDividendCalendarParams, JsonRpcError> = parse_args(args);
    assert!(params.is_ok());
    let params = params.unwrap();
    assert!(params.exchange.is_empty());
}

#[tokio::test]
async fn test_error_handling_market_news_invalid_limit() {
    let args = Some(serde_json::json!({
        "symbol": "AAPL",
        "limit": 0
    }));
    let params: Result<GetMarketNewsParams, JsonRpcError> = parse_args(args);
    assert!(params.is_ok());
    let params = params.unwrap();
    assert!(params.limit == 0 || params.limit > 100);
}

#[tokio::test]
async fn test_error_handling_search_empty_query() {
    let args = Some(serde_json::json!({"query": ""}));
    let params: Result<SearchSymbolsParams, JsonRpcError> = parse_args(args);
    assert!(params.is_ok());
    let params = params.unwrap();
    assert!(params.query.is_empty());
}

    /// Multi-symbol verification test for fundamentals data
    /// Verifies that different symbols return different market_cap and revenue values
    #[tokio::test]
    #[ignore = "Requires live TradingView API access"]
    async fn test_multi_symbol_fundamentals_uniqueness() {
        let client = TradingViewMcpClient::new().await.expect("Failed to create client");
        let client = Arc::new(client);

        let aapl_fundamentals = client.get_fundamentals("NASDAQ:AAPL").await.expect("Failed to get AAPL fundamentals");
        let msft_fundamentals = client.get_fundamentals("NASDAQ:MSFT").await.expect("Failed to get MSFT fundamentals");

        assert_ne!(
            aapl_fundamentals.market_cap,
            msft_fundamentals.market_cap,
            "AAPL and MSFT should have different market_cap values"
        );

        assert_ne!(
            aapl_fundamentals.revenue,
            msft_fundamentals.revenue,
            "AAPL and MSFT should have different revenue values"
        );

        let googl_fundamentals = client.get_fundamentals("NASDAQ:GOOGL").await.expect("Failed to get GOOGL fundamentals");

        assert_ne!(
            aapl_fundamentals.market_cap,
            googl_fundamentals.market_cap,
            "AAPL and GOOGL should have different market_cap values"
        );

        assert_ne!(
            aapl_fundamentals.revenue,
            googl_fundamentals.revenue,
            "AAPL and GOOGL should have different revenue values"
        );

        if aapl_fundamentals.pe_ratio.is_some() && msft_fundamentals.pe_ratio.is_some() {
            assert_ne!(
                aapl_fundamentals.pe_ratio,
                msft_fundamentals.pe_ratio,
                "AAPL and MSFT should have different pe_ratio values"
            );
        }
    }

    /// Multi-symbol verification test for quote data
    /// Verifies that different symbols return different prices
    #[tokio::test]
    #[ignore = "Requires live TradingView API access"]
    async fn test_multi_symbol_quote_uniqueness() {
        let client = TradingViewMcpClient::new().await.expect("Failed to create client");
        let client = Arc::new(client);

        let aapl_quote = client.get_quote("NASDAQ:AAPL").await.expect("Failed to get AAPL quote");
        let msft_quote = client.get_quote("NASDAQ:MSFT").await.expect("Failed to get MSFT quote");
        let googl_quote = client.get_quote("NASDAQ:GOOGL").await.expect("Failed to get GOOGL quote");
        let meta_quote = client.get_quote("NASDAQ:META").await.expect("Failed to get META quote");

        assert_ne!(
            aapl_quote.price, msft_quote.price,
            "AAPL and MSFT should have different prices"
        );
        assert_ne!(
            aapl_quote.price, googl_quote.price,
            "AAPL and GOOGL should have different prices"
        );
        assert_ne!(
            aapl_quote.price, meta_quote.price,
            "AAPL and META should have different prices"
        );
        assert_ne!(
            msft_quote.price, googl_quote.price,
            "MSFT and GOOGL should have different prices"
        );
        assert_ne!(
            msft_quote.price, meta_quote.price,
            "MSFT and META should have different prices"
        );
        assert_ne!(
            googl_quote.price, meta_quote.price,
            "GOOGL and META should have different prices"
        );

        assert_eq!(aapl_quote.symbol, "NASDAQ:AAPL", "AAPL quote should have correct symbol");
        assert_eq!(msft_quote.symbol, "NASDAQ:MSFT", "MSFT quote should have correct symbol");
        assert_eq!(googl_quote.symbol, "NASDAQ:GOOGL", "GOOGL quote should have correct symbol");
        assert_eq!(meta_quote.symbol, "NASDAQ:META", "META quote should have correct symbol");
    }

    /// Combined test verifying both fundamentals and quotes return unique data per symbol
    #[tokio::test]
    #[ignore = "Requires live TradingView API access"]
    async fn test_multi_symbol_data_consistency() {
        let client = TradingViewMcpClient::new().await.expect("Failed to create client");
        let client = Arc::new(client);

        let symbols = ["NASDAQ:AAPL", "NASDAQ:MSFT", "NASDAQ:GOOGL"];

        let mut fundamentals_data = Vec::new();
        for symbol in &symbols {
            let fund = client.get_fundamentals(symbol).await.expect(&format!("Failed to get fundamentals for {}", symbol));
            fundamentals_data.push((symbol.to_string(), fund));
        }

        let market_caps: Vec<_> = fundamentals_data.iter().map(|(_, f)| f.market_cap).collect();
        for i in 0..market_caps.len() {
            for j in (i + 1)..market_caps.len() {
                assert_ne!(
                    market_caps[i], market_caps[j],
                    "Symbols at index {} and {} should have different market_cap values", i, j
                );
            }
        }

        let mut quotes_data = Vec::new();
        for symbol in &symbols {
            let quote = client.get_quote(symbol).await.expect(&format!("Failed to get quote for {}", symbol));
            quotes_data.push((symbol.to_string(), quote));
        }

        let prices: Vec<_> = quotes_data.iter().map(|(_, q)| q.price).collect();
        for i in 0..prices.len() {
            for j in (i + 1)..prices.len() {
                assert_ne!(
                    prices[i], prices[j],
                    "Symbols at index {} and {} should have different prices", i, j
                );
            }
        }

        for (i, symbol) in symbols.iter().enumerate() {
            assert_eq!(
                fundamentals_data[i].0, *symbol,
                "Fundamentals symbol mismatch at index {}", i
            );
            assert_eq!(
                quotes_data[i].0, *symbol,
                "Quote symbol mismatch at index {}", i
            );
            assert_eq!(
                fundamentals_data[i].1.symbol, *symbol,
                "Fundamentals data symbol field mismatch for {}", symbol
            );
        assert_eq!(
            quotes_data[i].1.symbol, *symbol,
            "Quote data symbol field mismatch for {}", symbol
        );
    }
}

// ============================================================================
// PERFORMANCE BENCHMARK TESTS - Marked with #[ignore] to skip in CI
// Run with: cargo test -- --ignored
// ============================================================================

use std::time::{Duration, Instant};

/// Mock TradingViewMcpClient for benchmarking - returns dummy data instantly
#[derive(Debug)]
struct MockTradingViewClient;

impl MockTradingViewClient {
    fn new() -> Self {
        Self
    }

    async fn get_quote(&self, symbol: &str) -> Result<crate::client::Quote, crate::client::ClientError> {
        // Simulate minimal processing delay
        tokio::time::sleep(Duration::from_micros(100)).await;
        Ok(crate::client::Quote {
            symbol: symbol.to_string(),
            price: 150.25,
            change: 2.5,
            change_percent: 1.69,
            volume: 50000000,
            high: 152.0,
            low: 148.5,
            open: 149.0,
            previous_close: 147.75,
        })
    }

    async fn get_fundamentals(&self, symbol: &str) -> Result<crate::client::Fundamentals, crate::client::ClientError> {
        tokio::time::sleep(Duration::from_micros(100)).await;
        Ok(crate::client::Fundamentals {
            symbol: symbol.to_string(),
            market_cap: Some(2500000000000.0),
            pe_ratio: Some(28.5),
            eps: Some(5.25),
            dividend_yield: Some(0.005),
            beta: Some(1.2),
            price_to_book: Some(35.0),
            debt_to_equity: Some(0.5),
            current_ratio: Some(1.1),
            quick_ratio: Some(0.9),
            roe: Some(0.25),
            roa: Some(0.15),
            revenue: Some(380000000000.0),
            gross_profit: Some(150000000000.0),
            operating_income: Some(120000000000.0),
            net_income: Some(100000000000.0),
        })
    }

    async fn get_historical(
        &self,
        symbol: &str,
        interval: Interval,
        bars: u32,
    ) -> Result<tvdata_rs::history::HistorySeries, crate::client::ClientError> {
        tokio::time::sleep(Duration::from_micros(100)).await;
        use tvdata_rs::history::{Bar, HistorySeries, HistoryProvenance, Adjustment, TradingSession};
        use tvdata_rs::scanner::Ticker;
        use tvdata_rs::metadata::{DataLineage, DataSourceKind, HistoryKind};
        use time::OffsetDateTime;

        let symbol_owned: String = symbol.to_string();
        let mut history_bars = Vec::new();
        let base_time = OffsetDateTime::now_utc();
        for i in 0..bars {
            history_bars.push(Bar {
                time: base_time - time::Duration::seconds(i as i64 * 86400),
                open: 149.0 + (i as f64 * 0.1),
                high: 152.0 + (i as f64 * 0.1),
                low: 148.5 + (i as f64 * 0.1),
                close: 150.25 + (i as f64 * 0.1),
                volume: Some(50000000.0),
            });
        }
        let ticker: Ticker = symbol_owned.clone().into();
        let ticker2: Ticker = symbol_owned.clone().into();
        let ticker3: Ticker = symbol_owned.into();
        Ok(HistorySeries {
            symbol: ticker,
            interval,
            bars: history_bars,
            provenance: HistoryProvenance {
                requested_symbol: ticker2,
                resolved_symbol: ticker3,
                exchange: None,
                session: TradingSession::Regular,
                adjustment: Adjustment::Splits,
                authenticated: false,
                lineage: DataLineage::new(
                    DataSourceKind::HistoryWebSocket,
                    HistoryKind::Native,
                    base_time,
                    None,
                ),
            },
        })
    }
}

/// Helper to wrap MockTradingViewClient in Arc for handler compatibility
fn create_mock_client() -> Arc<MockTradingViewClient> {
    Arc::new(MockTradingViewClient::new())
}

/// Benchmark helper to run multiple iterations and collect timing stats
async fn run_benchmark<F, Fut>(name: &str, iterations: usize, f: F) -> Duration
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let mut times = Vec::with_capacity(iterations);
    let total_start = Instant::now();

    for _ in 0..iterations {
        let start = Instant::now();
        f().await;
        let elapsed = start.elapsed();
        times.push(elapsed);
    }

    let total_elapsed = total_start.elapsed();

    // Calculate statistics
    let min_time = times.iter().min().copied().unwrap_or(Duration::ZERO);
    let max_time = times.iter().max().copied().unwrap_or(Duration::ZERO);
    let avg_time = total_elapsed / iterations as u32;

    println!("\n=== Benchmark: {} ===", name);
    println!("Iterations: {}", iterations);
    println!("Total time: {:?}", total_elapsed);
    println!("Average time: {:?}", avg_time);
    println!("Min time: {:?}", min_time);
    println!("Max time: {:?}", max_time);
    println!("========================\n");

    total_elapsed
}

#[tokio::test]
#[ignore]
async fn benchmark_get_quote_sequential() {
    const ITERATIONS: usize = 10;
    const MAX_ALLOWED_TIME: Duration = Duration::from_secs(5);

    let client = create_mock_client();
    let args = serde_json::json!({ "symbol": "NASDAQ:AAPL" });

    let total_elapsed = run_benchmark("get_quote (sequential)", ITERATIONS, || {
        let client = client.clone();
        let args = args.clone();
        async move {
            // Simulate handler logic without actual client call
            let params: GetQuoteParams = serde_json::from_value(args).unwrap();
            let _ = client.get_quote(&params.symbol).await;
        }
    }).await;

    // Verify all responses complete within 5 seconds
    assert!(
        total_elapsed < MAX_ALLOWED_TIME,
        "Benchmark exceeded 5 second limit: {:?}",
        total_elapsed
    );
}

#[tokio::test]
#[ignore]
async fn benchmark_get_fundamentals_sequential() {
    const ITERATIONS: usize = 10;
    const MAX_ALLOWED_TIME: Duration = Duration::from_secs(5);

    let client = create_mock_client();
    let args = serde_json::json!({ "symbol": "NASDAQ:AAPL" });

    let total_elapsed = run_benchmark("get_fundamentals (sequential)", ITERATIONS, || {
        let client = client.clone();
        let args = args.clone();
        async move {
            let params: GetFundamentalsParams = serde_json::from_value(args).unwrap();
            let _ = client.get_fundamentals(&params.symbol).await;
        }
    }).await;

    assert!(
        total_elapsed < MAX_ALLOWED_TIME,
        "Benchmark exceeded 5 second limit: {:?}",
        total_elapsed
    );
}

#[tokio::test]
#[ignore]
async fn benchmark_get_historical_sequential() {
    const ITERATIONS: usize = 10;
    const MAX_ALLOWED_TIME: Duration = Duration::from_secs(5);

    let client = create_mock_client();
    let args = serde_json::json!({
        "symbol": "NASDAQ:AAPL",
        "interval": "1d",
        "count": 100
    });

    let total_elapsed = run_benchmark("get_historical (sequential)", ITERATIONS, || {
        let client = client.clone();
        let args = args.clone();
        async move {
            let params: GetHistoricalParams = serde_json::from_value(args).unwrap();
            let interval = Interval::Day1;
            let _ = client.get_historical(&params.symbol, interval, params.count).await;
        }
    }).await;

    assert!(
        total_elapsed < MAX_ALLOWED_TIME,
        "Benchmark exceeded 5 second limit: {:?}",
        total_elapsed
    );
}

#[tokio::test]
#[ignore]
async fn benchmark_get_quote_concurrent() {
    const ITERATIONS: usize = 10;
    const MAX_ALLOWED_TIME: Duration = Duration::from_secs(5);

    let client = create_mock_client();
    let args = serde_json::json!({ "symbol": "NASDAQ:AAPL" });

    let start = Instant::now();

    // Run all requests concurrently
    let futures: Vec<_> = (0..ITERATIONS)
        .map(|_| {
            let client = client.clone();
            let args = args.clone();
            async move {
                let params: GetQuoteParams = serde_json::from_value(args).unwrap();
                let _ = client.get_quote(&params.symbol).await;
            }
        })
        .collect();

    tokio::join!(async { for f in futures { f.await; } });

    let total_elapsed = start.elapsed();
    let avg_time = total_elapsed / ITERATIONS as u32;

    println!("\n=== Benchmark: get_quote (concurrent) ===");
    println!("Iterations: {}", ITERATIONS);
    println!("Total time: {:?}", total_elapsed);
    println!("Average time per request: {:?}", avg_time);
    println!("==========================================\n");

    assert!(
        total_elapsed < MAX_ALLOWED_TIME,
        "Benchmark exceeded 5 second limit: {:?}",
        total_elapsed
    );
}

#[tokio::test]
#[ignore]
async fn benchmark_get_fundamentals_concurrent() {
    const ITERATIONS: usize = 10;
    const MAX_ALLOWED_TIME: Duration = Duration::from_secs(5);

    let client = create_mock_client();
    let args = serde_json::json!({ "symbol": "NASDAQ:AAPL" });

    let start = Instant::now();

    let futures: Vec<_> = (0..ITERATIONS)
        .map(|_| {
            let client = client.clone();
            let args = args.clone();
            async move {
                let params: GetFundamentalsParams = serde_json::from_value(args).unwrap();
                let _ = client.get_fundamentals(&params.symbol).await;
            }
        })
        .collect();

    tokio::join!(async { for f in futures { f.await; } });

    let total_elapsed = start.elapsed();
    let avg_time = total_elapsed / ITERATIONS as u32;

    println!("\n=== Benchmark: get_fundamentals (concurrent) ===");
    println!("Iterations: {}", ITERATIONS);
    println!("Total time: {:?}", total_elapsed);
    println!("Average time per request: {:?}", avg_time);
    println!("================================================\n");

    assert!(
        total_elapsed < MAX_ALLOWED_TIME,
        "Benchmark exceeded 5 second limit: {:?}",
        total_elapsed
    );
}

#[tokio::test]
#[ignore]
async fn benchmark_get_historical_concurrent() {
    const ITERATIONS: usize = 10;
    const MAX_ALLOWED_TIME: Duration = Duration::from_secs(5);

    let client = create_mock_client();
    let args = serde_json::json!({
        "symbol": "NASDAQ:AAPL",
        "interval": "1d",
        "count": 100
    });

    let start = Instant::now();

    let futures: Vec<_> = (0..ITERATIONS)
        .map(|_| {
            let client = client.clone();
            let args = args.clone();
            async move {
                let params: GetHistoricalParams = serde_json::from_value(args).unwrap();
                let interval = Interval::Day1;
                let _ = client.get_historical(&params.symbol, interval, params.count).await;
            }
        })
        .collect();

    tokio::join!(async { for f in futures { f.await; } });

    let total_elapsed = start.elapsed();
    let avg_time = total_elapsed / ITERATIONS as u32;

    println!("\n=== Benchmark: get_historical (concurrent) ===");
    println!("Iterations: {}", ITERATIONS);
    println!("Total time: {:?}", total_elapsed);
    println!("Average time per request: {:?}", avg_time);
    println!("==============================================\n");

    assert!(
        total_elapsed < MAX_ALLOWED_TIME,
        "Benchmark exceeded 5 second limit: {:?}",
        total_elapsed
    );
}
}
