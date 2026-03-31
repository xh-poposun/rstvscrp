use crate::tools::*;
use crate::{JsonRpcError, ToolCallResult, ToolContent};
use serde_json::Value;

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

/// Handler for get_quote tool
pub async fn handle_get_quote(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetQuoteParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    // Mock quote data
    let response = GetQuoteResponse {
        price: 150.25,
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
pub async fn handle_search_symbols(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: SearchSymbolsParams = parse_args(args)?;

    if params.query.is_empty() {
        return Err(JsonRpcError::invalid_params("Query cannot be empty"));
    }

    // Mock search results
    let response = SearchSymbolsResponse {
        results: vec![
            SymbolInfo {
                symbol: "NASDAQ:AAPL".to_string(),
                name: "Apple Inc.".to_string(),
                exchange: "NASDAQ".to_string(),
            },
            SymbolInfo {
                symbol: "NASDAQ:MSFT".to_string(),
                name: "Microsoft Corporation".to_string(),
                exchange: "NASDAQ".to_string(),
            },
            SymbolInfo {
                symbol: "NASDAQ:GOOGL".to_string(),
                name: "Alphabet Inc.".to_string(),
                exchange: "NASDAQ".to_string(),
            },
        ],
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
pub async fn handle_get_historical(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetHistoricalParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    // Validate interval
    let valid_intervals = ["1m", "5m", "15m", "1h", "4h", "1d", "1w", "1M"];
    if !valid_intervals.contains(&params.interval.as_str()) {
        return Err(JsonRpcError::invalid_params(
            format!("Invalid interval '{}'. Valid intervals: {:?}", params.interval, valid_intervals)
        ));
    }

    // Mock historical data
    let mut points = Vec::new();
    let base_price = 150.0;
    for i in 0..params.count.min(100) {
        let offset = i as f64 * 0.5;
        points.push(HistoricalPoint {
            timestamp: chrono::Utc::now()
                .checked_sub_days(chrono::Days::new((params.count - i) as u64))
                .unwrap_or_else(|| chrono::Utc::now())
                .to_rfc3339(),
            open: base_price + offset,
            high: base_price + offset + 1.0,
            low: base_price + offset - 0.5,
            close: base_price + offset + 0.25,
            volume: 1000000.0 + (i as f64 * 10000.0),
        });
    }

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
pub async fn handle_get_fundamentals(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetFundamentalsParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    // Mock fundamentals data
    let response = GetFundamentalsResponse {
        market_cap: Some(2500000000000.0),
        pe_ratio: Some(28.5),
        eps: Some(6.05),
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
pub async fn handle_get_financial_statements(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetFinancialStatementsParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    // Validate period
    if !["annual", "quarterly"].contains(&params.period.as_str()) {
        return Err(JsonRpcError::invalid_params(
            "Period must be 'annual' or 'quarterly'"
        ));
    }

    // Mock financial statements
    let items = serde_json::json!({
        "revenue": 394328000000.0,
        "net_income": 96995000000.0,
        "total_assets": 352755000000.0,
        "total_liabilities": 290437000000.0,
        "shareholders_equity": 62318000000.0,
    });

    let response = GetFinancialStatementsResponse {
        statements: vec![
            FinancialStatement {
                date: "2024-09-28".to_string(),
                statement_type: "income_statement".to_string(),
                items: items.clone(),
            },
            FinancialStatement {
                date: "2024-09-28".to_string(),
                statement_type: "balance_sheet".to_string(),
                items: items.clone(),
            },
        ],
    };

    let content = ToolContent::text(
        serde_json::to_string(&response).map_err(|e| JsonRpcError::internal_error(e.to_string()))?
    );

    Ok(ToolCallResult {
        content: vec![content],
        is_error: Some(false),
    })
}

/// Handler for get_credit_ratings tool
pub async fn handle_get_credit_ratings(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetCreditRatingsParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    // Mock credit ratings
    let response = GetCreditRatingsResponse {
        rating: Some("AA+".to_string()),
        outlook: Some("Stable".to_string()),
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
pub async fn handle_scan_stocks(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: ScanStocksParams = parse_args(args)?;

    // Mock stock screener results
    let response = ScanStocksResponse {
        results: vec![
            StockInfo {
                symbol: "NASDAQ:AAPL".to_string(),
                name: "Apple Inc.".to_string(),
                exchange: "NASDAQ".to_string(),
                price: 150.25,
            },
            StockInfo {
                symbol: "NASDAQ:MSFT".to_string(),
                name: "Microsoft Corporation".to_string(),
                exchange: "NASDAQ".to_string(),
                price: 420.50,
            },
            StockInfo {
                symbol: "NASDAQ:GOOGL".to_string(),
                name: "Alphabet Inc.".to_string(),
                exchange: "NASDAQ".to_string(),
                price: 175.80,
            },
        ],
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
pub async fn handle_get_earnings_calendar(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetEarningsCalendarParams = parse_args(args)?;

    if params.days_ahead < 0 || params.days_ahead > 90 {
        return Err(JsonRpcError::invalid_params("days_ahead must be between 0 and 90"));
    }

    // Mock earnings calendar
    let response = GetEarningsCalendarResponse {
        events: vec![
            EarningsEvent {
                symbol: "NASDAQ:AAPL".to_string(),
                date: chrono::Utc::now()
                    .checked_add_days(chrono::Days::new(7))
                    .unwrap_or_else(|| chrono::Utc::now())
                    .format("%Y-%m-%d")
                    .to_string(),
                estimate: 1.50,
            },
            EarningsEvent {
                symbol: "NASDAQ:MSFT".to_string(),
                date: chrono::Utc::now()
                    .checked_add_days(chrono::Days::new(14))
                    .unwrap_or_else(|| chrono::Utc::now())
                    .format("%Y-%m-%d")
                    .to_string(),
                estimate: 2.80,
            },
        ],
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
pub async fn handle_get_dividend_calendar(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetDividendCalendarParams = parse_args(args)?;

    if params.exchange.is_empty() {
        return Err(JsonRpcError::invalid_params("Exchange cannot be empty"));
    }

    // Mock dividend calendar
    let response = GetDividendCalendarResponse {
        dividends: vec![
            DividendEvent {
                symbol: "NASDAQ:AAPL".to_string(),
                ex_date: chrono::Utc::now()
                    .checked_add_days(chrono::Days::new(30))
                    .unwrap_or_else(|| chrono::Utc::now())
                    .format("%Y-%m-%d")
                    .to_string(),
                amount: 0.25,
            },
            DividendEvent {
                symbol: "NASDAQ:MSFT".to_string(),
                ex_date: chrono::Utc::now()
                    .checked_add_days(chrono::Days::new(45))
                    .unwrap_or_else(|| chrono::Utc::now())
                    .format("%Y-%m-%d")
                    .to_string(),
                amount: 0.75,
            },
        ],
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
pub async fn handle_calculate_rsi(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: CalculateRsiParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    if params.period < 2 || params.period > 100 {
        return Err(JsonRpcError::invalid_params("RSI period must be between 2 and 100"));
    }

    // Mock RSI calculation
    let response = CalculateRsiResponse {
        rsi: 65.5,
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
pub async fn handle_calculate_macd(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: CalculateMacdParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    if params.fast >= params.slow {
        return Err(JsonRpcError::invalid_params("Fast period must be less than slow period"));
    }

    // Mock MACD calculation
    let response = CalculateMacdResponse {
        macd: 1.25,
        signal: 0.75,
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
pub async fn handle_get_debt_maturity(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetDebtMaturityParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    // Mock debt maturity schedule (will integrate with SEC EDGAR later)
    let response = GetDebtMaturityResponse {
        maturity: vec![
            DebtInstrument {
                due_date: "2025-06-15".to_string(),
                amount: 5000000000.0,
            },
            DebtInstrument {
                due_date: "2026-09-20".to_string(),
                amount: 7500000000.0,
            },
            DebtInstrument {
                due_date: "2028-02-10".to_string(),
                amount: 10000000000.0,
            },
        ],
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
pub async fn handle_get_company_profile(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetCompanyProfileParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    // Mock company profile
    let overview = serde_json::json!({
        "name": "Apple Inc.",
        "sector": "Technology",
        "industry": "Consumer Electronics",
        "employees": 161000,
        "website": "https://www.apple.com",
        "description": "Apple Inc. designs, manufactures, and markets smartphones, personal computers, tablets, wearables, and accessories worldwide.",
        "ceo": "Tim Cook",
        "headquarters": "Cupertino, California",
        "founded": "1976",
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
pub async fn handle_get_market_news(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: GetMarketNewsParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    if params.limit == 0 || params.limit > 100 {
        return Err(JsonRpcError::invalid_params("Limit must be between 1 and 100"));
    }

    // Mock market news
    let response = GetMarketNewsResponse {
        articles: vec![
            NewsItem {
                title: format!("{} Reports Strong Quarterly Earnings", params.symbol),
                url: "https://example.com/news/1".to_string(),
                published_at: chrono::Utc::now().to_rfc3339(),
            },
            NewsItem {
                title: format!("{} Announces New Product Line", params.symbol),
                url: "https://example.com/news/2".to_string(),
                published_at: (chrono::Utc::now() - chrono::Duration::days(1)).to_rfc3339(),
            },
        ],
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
pub async fn handle_compute_macd_signal(args: Option<Value>) -> Result<ToolCallResult, JsonRpcError> {
    let params: ComputeMacdSignalParams = parse_args(args)?;
    validate_symbol(&params.symbol)?;

    if params.short_window >= params.long_window {
        return Err(JsonRpcError::invalid_params("Short window must be less than long window"));
    }

    if params.signal_window == 0 {
        return Err(JsonRpcError::invalid_params("Signal window must be greater than 0"));
    }

    // Mock MACD signal computation
    let response = ComputeMacdSignalResponse {
        macd: 1.45,
        signal: 0.95,
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
    }

    #[tokio::test]
    async fn test_handle_get_quote() {
        let args = Some(serde_json::json!({"symbol": "NASDAQ:AAPL"}));
        let result = handle_get_quote(args).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.is_error, Some(false));
    }

    #[tokio::test]
    async fn test_handle_get_quote_invalid_symbol() {
        let args = Some(serde_json::json!({"symbol": ""}));
        let result = handle_get_quote(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_search_symbols() {
        let args = Some(serde_json::json!({"query": "Apple"}));
        let result = handle_search_symbols(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_historical() {
        let args = Some(serde_json::json!({
            "symbol": "NASDAQ:AAPL",
            "interval": "1d",
            "count": 10
        }));
        let result = handle_get_historical(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_historical_invalid_interval() {
        let args = Some(serde_json::json!({
            "symbol": "NASDAQ:AAPL",
            "interval": "invalid",
            "count": 10
        }));
        let result = handle_get_historical(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_get_fundamentals() {
        let args = Some(serde_json::json!({"symbol": "NASDAQ:AAPL"}));
        let result = handle_get_fundamentals(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_financial_statements() {
        let args = Some(serde_json::json!({
            "symbol": "NASDAQ:AAPL",
            "period": "annual"
        }));
        let result = handle_get_financial_statements(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_credit_ratings() {
        let args = Some(serde_json::json!({"symbol": "NASDAQ:AAPL"}));
        let result = handle_get_credit_ratings(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_scan_stocks() {
        let args = Some(serde_json::json!({
            "filters": {},
            "limit": 10
        }));
        let result = handle_scan_stocks(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_earnings_calendar() {
        let args = Some(serde_json::json!({"days_ahead": 30}));
        let result = handle_get_earnings_calendar(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_dividend_calendar() {
        let args = Some(serde_json::json!({"exchange": "NASDAQ"}));
        let result = handle_get_dividend_calendar(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_calculate_rsi() {
        let args = Some(serde_json::json!({
            "symbol": "NASDAQ:AAPL",
            "period": 14
        }));
        let result = handle_calculate_rsi(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_calculate_rsi_invalid_period() {
        let args = Some(serde_json::json!({
            "symbol": "NASDAQ:AAPL",
            "period": 200
        }));
        let result = handle_calculate_rsi(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_calculate_macd() {
        let args = Some(serde_json::json!({
            "symbol": "NASDAQ:AAPL",
            "fast": 12,
            "slow": 26,
            "signal": 9
        }));
        let result = handle_calculate_macd(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_calculate_macd_invalid_params() {
        let args = Some(serde_json::json!({
            "symbol": "NASDAQ:AAPL",
            "fast": 26,
            "slow": 12,
            "signal": 9
        }));
        let result = handle_calculate_macd(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_handle_get_debt_maturity() {
        let args = Some(serde_json::json!({"symbol": "NASDAQ:AAPL"}));
        let result = handle_get_debt_maturity(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_company_profile() {
        let args = Some(serde_json::json!({"symbol": "NASDAQ:AAPL"}));
        let result = handle_get_company_profile(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_get_market_news() {
        let args = Some(serde_json::json!({
            "symbol": "NASDAQ:AAPL",
            "limit": 5
        }));
        let result = handle_get_market_news(args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_compute_macd_signal() {
        let args = Some(serde_json::json!({
            "symbol": "NASDAQ:AAPL",
            "short_window": 12,
            "long_window": 26,
            "signal_window": 9
        }));
        let result = handle_compute_macd_signal(args).await;
        assert!(result.is_ok());
    }
}
