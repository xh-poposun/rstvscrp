use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub output_schema: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetQuoteParams {
    pub symbol: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetQuoteResponse {
    pub price: f64,
    pub currency: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SearchSymbolsParams {
    pub query: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SearchSymbolsResponse {
    pub results: Vec<SymbolInfo>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SymbolInfo {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetHistoricalParams {
    pub symbol: String,
    pub interval: String,
    pub count: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct HistoricalPoint {
    pub timestamp: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetHistoricalResponse {
    pub points: Vec<HistoricalPoint>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetFundamentalsParams {
    pub symbol: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetFundamentalsResponse {
    pub market_cap: Option<f64>,
    pub pe_ratio: Option<f64>,
    pub eps: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetFinancialStatementsParams {
    pub symbol: String,
    pub period: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetFinancialStatementsResponse {
    pub statements: Vec<FinancialStatement>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FinancialStatement {
    pub date: String,
    pub statement_type: String,
    pub items: serde_json::Value,
}
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetCreditRatingsParams {
    pub symbol: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetCreditRatingsResponse {
    pub rating: Option<String>,
    pub outlook: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ScanStocksParams {
    pub filters: serde_json::Value,
    pub limit: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScanStocksResponse {
    pub results: Vec<StockInfo>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StockInfo {
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub price: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetEarningsCalendarParams {
    pub days_ahead: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetEarningsCalendarResponse {
    pub events: Vec<EarningsEvent>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EarningsEvent {
    pub symbol: String,
    pub date: String,
    pub estimate: f64,
}
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetDividendCalendarParams {
    pub exchange: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetDividendCalendarResponse {
    pub dividends: Vec<DividendEvent>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DividendEvent {
    pub symbol: String,
    pub ex_date: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CalculateRsiParams {
    pub symbol: String,
    pub period: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CalculateRsiResponse {
    pub rsi: f64,
}
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct CalculateMacdParams {
    pub symbol: String,
    pub fast: u32,
    pub slow: u32,
    pub signal: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CalculateMacdResponse {
    pub macd: f64,
    pub signal: f64,
}
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetDebtMaturityParams {
    pub symbol: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetDebtMaturityResponse {
    pub maturity: Vec<DebtInstrument>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DebtInstrument {
    pub due_date: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetCompanyProfileParams {
    pub symbol: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetCompanyProfileResponse {
    pub overview: serde_json::Value,
}
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct GetMarketNewsParams {
    pub symbol: String,
    pub limit: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetMarketNewsResponse {
    pub articles: Vec<NewsItem>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NewsItem {
    pub title: String,
    pub url: String,
    pub published_at: String,
}
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct ComputeMacdSignalParams {
    pub symbol: String,
    pub short_window: u32,
    pub long_window: u32,
    pub signal_window: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ComputeMacdSignalResponse {
    pub macd: f64,
    pub signal: f64,
}

pub fn build_tool_registry() -> Vec<Tool> {
    vec![
        Tool {
            name: "get_quote".to_string(),
            description: "Fetch current quote for a symbol".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(GetQuoteParams)).unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(GetQuoteResponse)).unwrap(),
        },
        Tool {
            name: "search_symbols".to_string(),
            description: "Search symbols by query".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(SearchSymbolsParams)).unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(SearchSymbolsResponse))
                .unwrap(),
        },
        Tool {
            name: "get_historical".to_string(),
            description: "Fetch historical price data".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(GetHistoricalParams)).unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(GetHistoricalResponse))
                .unwrap(),
        },
        Tool {
            name: "get_fundamentals".to_string(),
            description: "Fetch basic fundamentals".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(GetFundamentalsParams))
                .unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(GetFundamentalsResponse))
                .unwrap(),
        },
        Tool {
            name: "get_financial_statements".to_string(),
            description: "Fetch financial statements".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(GetFinancialStatementsParams))
                .unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(
                GetFinancialStatementsResponse
            ))
            .unwrap(),
        },
        Tool {
            name: "get_credit_ratings".to_string(),
            description: "Fetch credit ratings".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(GetCreditRatingsParams))
                .unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(GetCreditRatingsResponse))
                .unwrap(),
        },
        Tool {
            name: "scan_stocks".to_string(),
            description: "Screen stocks with filters".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(ScanStocksParams)).unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(ScanStocksResponse)).unwrap(),
        },
        Tool {
            name: "get_earnings_calendar".to_string(),
            description: "Get earnings calendar".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(GetEarningsCalendarParams))
                .unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(GetEarningsCalendarResponse))
                .unwrap(),
        },
        Tool {
            name: "get_dividend_calendar".to_string(),
            description: "Get dividend calendar".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(GetDividendCalendarParams))
                .unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(GetDividendCalendarResponse))
                .unwrap(),
        },
        Tool {
            name: "calculate_rsi".to_string(),
            description: "Calculate RSI".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(CalculateRsiParams)).unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(CalculateRsiResponse))
                .unwrap(),
        },
        Tool {
            name: "calculate_macd".to_string(),
            description: "Calculate MACD".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(CalculateMacdParams)).unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(CalculateMacdResponse))
                .unwrap(),
        },
        Tool {
            name: "get_debt_maturity".to_string(),
            description: "Get debt maturity schedule".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(GetDebtMaturityParams))
                .unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(GetDebtMaturityResponse))
                .unwrap(),
        },
        Tool {
            name: "get_company_profile".to_string(),
            description: "Fetch company profile overview".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(GetCompanyProfileParams))
                .unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(GetCompanyProfileResponse))
                .unwrap(),
        },
        Tool {
            name: "get_market_news".to_string(),
            description: "Fetch market-related news".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(GetMarketNewsParams)).unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(GetMarketNewsResponse))
                .unwrap(),
        },
        Tool {
            name: "compute_macd_signal".to_string(),
            description: "Compute MACD signal values".to_string(),
            input_schema: serde_json::to_value(schemars::schema_for!(ComputeMacdSignalParams))
                .unwrap(),
            output_schema: serde_json::to_value(schemars::schema_for!(ComputeMacdSignalResponse))
                .unwrap(),
        },
    ]
}

pub fn registry() -> Vec<Tool> {
    build_tool_registry()
}
