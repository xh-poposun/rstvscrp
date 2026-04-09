use crate::market_data::{
    ConversionRatesSnapshot, InstrumentIdentity, QuoteSnapshot, TechnicalSummary,
};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq)]
pub struct FundamentalsSnapshot {
    pub instrument: InstrumentIdentity,
    pub market_cap: Option<f64>,
    pub price_earnings_ttm: Option<f64>,
    pub price_to_book_fq: Option<f64>,
    pub price_to_sales_current: Option<f64>,
    pub total_revenue_ttm: Option<f64>,
    pub net_income_ttm: Option<f64>,
    pub eps_ttm: Option<f64>,
    pub dividend_yield_recent: Option<f64>,
    pub return_on_equity_ttm: Option<f64>,
    pub return_on_assets_ttm: Option<f64>,
    pub debt_to_equity_mrq: Option<f64>,
    pub current_ratio_mrq: Option<f64>,
    pub free_cash_flow_ttm: Option<f64>,
    pub ebitda_ttm: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnalystRecommendations {
    pub buy: Option<u32>,
    pub sell: Option<u32>,
    pub hold: Option<u32>,
    pub outperform: Option<u32>,
    pub underperform: Option<u32>,
    pub total: Option<u32>,
    pub rating: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnalystPriceTargets {
    pub average: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub median: Option<f64>,
    pub one_year: Option<f64>,
    pub one_year_delta_percent: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnalystForecasts {
    pub revenue_current_quarter: Option<f64>,
    pub revenue_next_quarter: Option<f64>,
    pub revenue_next_half_year: Option<f64>,
    pub revenue_next_fiscal_year: Option<f64>,
    pub eps_current_quarter: Option<f64>,
    pub eps_next_quarter: Option<f64>,
    pub eps_next_half_year: Option<f64>,
    pub eps_next_fiscal_year: Option<f64>,
    pub eps_surprise_recent_quarter: Option<f64>,
    pub eps_surprise_percent_recent_quarter: Option<f64>,
    pub forward_non_gaap_price_earnings: Option<f64>,
    pub forward_price_earnings_fiscal_year: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EarningsCalendar {
    pub recent_release_at: Option<OffsetDateTime>,
    pub next_release_at: Option<OffsetDateTime>,
    pub recent_calendar_date: Option<OffsetDateTime>,
    pub next_calendar_date: Option<OffsetDateTime>,
    pub current_quarter_trading_date: Option<OffsetDateTime>,
    pub next_quarter_trading_date: Option<OffsetDateTime>,
    pub fiscal_year_trading_date: Option<OffsetDateTime>,
    pub recent_release_time_code: Option<u32>,
    pub next_release_time_code: Option<u32>,
    pub current_quarter_publication_type_code: Option<u32>,
    pub next_quarter_publication_type_code: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnalystFxRates {
    pub current: Option<ConversionRatesSnapshot>,
    pub time_series: Option<ConversionRatesSnapshot>,
    pub revenue_current_quarter: Option<ConversionRatesSnapshot>,
    pub revenue_next_half_year: Option<ConversionRatesSnapshot>,
    pub revenue_next_fiscal_year: Option<ConversionRatesSnapshot>,
    pub trailing_twelve_months: Option<ConversionRatesSnapshot>,
    pub cash_flow: Option<ConversionRatesSnapshot>,
    pub price_target: Option<ConversionRatesSnapshot>,
    pub market_cap: Option<ConversionRatesSnapshot>,
    pub earnings_current_quarter: Option<ConversionRatesSnapshot>,
    pub earnings_next_quarter: Option<ConversionRatesSnapshot>,
    pub dividend_recent: Option<ConversionRatesSnapshot>,
    pub dividend_upcoming: Option<ConversionRatesSnapshot>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnalystSummary {
    pub instrument: InstrumentIdentity,
    pub close: Option<f64>,
    pub recommendations: AnalystRecommendations,
    pub price_targets: AnalystPriceTargets,
    pub forecasts: AnalystForecasts,
    pub earnings: EarningsCalendar,
    pub fx_rates: AnalystFxRates,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EquityOverview {
    pub quote: QuoteSnapshot,
    pub fundamentals: FundamentalsSnapshot,
    pub analyst: AnalystSummary,
    pub technicals: TechnicalSummary,
    pub buyback: BuybackMetrics,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BuybackMetrics {
    pub buyback_yield: Option<f64>,
    pub share_buyback_ratio_fq: Option<f64>,
    pub share_buyback_ratio_fy: Option<f64>,
    pub total_shares_outstanding: Option<f64>,
    pub total_shares_outstanding_current: Option<f64>,
    pub diluted_shares_outstanding_fq: Option<f64>,
    pub float_shares_outstanding: Option<f64>,
    pub shares_outstanding: Option<f64>,
    pub total_shares_outstanding_calculated: Option<f64>,
}

/// Detailed financial statements data including income statement, balance sheet, and cash flow
/// with fiscal year (FY), fiscal quarter (FQ), and trailing twelve months (TTM) variants.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FinancialStatementsDetail {
    // Income Statement
    /// Total revenue for fiscal year
    pub revenue_fy: Option<f64>,
    /// Total revenue for fiscal quarter
    pub revenue_fq: Option<f64>,
    /// Total revenue trailing twelve months
    pub revenue_ttm: Option<f64>,
    /// Revenue historical data (quarterly)
    pub revenue_fq_h: Option<Vec<f64>>,
    /// Revenue historical data (yearly)
    pub revenue_fy_h: Option<Vec<f64>>,
    /// Gross profit for fiscal year
    pub gross_profit_fy: Option<f64>,
    /// Gross profit for fiscal quarter
    pub gross_profit_fq: Option<f64>,
    /// Gross profit trailing twelve months
    pub gross_profit_ttm: Option<f64>,
    /// Gross profit historical data (quarterly)
    pub gross_profit_fq_h: Option<Vec<f64>>,
    /// Gross profit historical data (yearly)
    pub gross_profit_fy_h: Option<Vec<f64>>,
    /// Operating income for fiscal year
    pub operating_income_fy: Option<f64>,
    /// Operating income for fiscal quarter
    pub operating_income_fq: Option<f64>,
    /// Operating income trailing twelve months
    pub operating_income_ttm: Option<f64>,
    /// EBITDA for fiscal year
    pub ebitda_fy: Option<f64>,
    /// EBITDA for fiscal quarter
    pub ebitda_fq: Option<f64>,
    /// EBITDA trailing twelve months
    pub ebitda_ttm: Option<f64>,
    /// EBITDA historical data (yearly)
    pub ebitda_fy_h: Option<Vec<f64>>,
    /// EBIT for fiscal year
    pub ebit_fy: Option<f64>,
    /// EBIT for fiscal quarter
    pub ebit_fq: Option<f64>,
    /// EBIT historical data (quarterly)
    pub ebit_fq_h: Option<Vec<f64>>,
    /// EBIT historical data (yearly)
    pub ebit_fy_h: Option<Vec<f64>>,
    /// Net income for fiscal year
    pub net_income_fy: Option<f64>,
    /// Net income for fiscal quarter
    pub net_income_fq: Option<f64>,
    /// Net income trailing twelve months
    pub net_income_ttm: Option<f64>,
    /// Net income historical data (quarterly)
    pub net_income_fq_h: Option<Vec<f64>>,
    /// Net income historical data (yearly)
    pub net_income_fy_h: Option<Vec<f64>>,
    /// Basic EPS for fiscal year
    pub eps_basic_fy: Option<f64>,
    /// Basic EPS for fiscal quarter
    pub eps_basic_fq: Option<f64>,
    /// Basic EPS trailing twelve months
    pub eps_basic_ttm: Option<f64>,
    /// Basic EPS historical data (quarterly)
    pub eps_basic_fq_h: Option<Vec<f64>>,
    /// Diluted EPS for fiscal quarter
    pub eps_diluted_fq: Option<f64>,
    /// Diluted EPS trailing twelve months
    pub eps_diluted_ttm: Option<f64>,
    /// Cost of goods sold for fiscal year
    pub cost_of_goods_fy: Option<f64>,
    /// Cost of goods sold historical data (yearly)
    pub cost_of_goods_fy_h: Option<Vec<f64>>,
    /// Operating expenses for fiscal quarter
    pub operating_expenses_fq: Option<f64>,
    /// Operating expenses for fiscal year
    pub operating_expenses_fy: Option<f64>,
    /// Operating expenses historical data (quarterly)
    pub operating_expenses_fq_h: Option<Vec<f64>>,
    /// Operating expenses historical data (yearly)
    pub operating_expenses_fy_h: Option<Vec<f64>>,

    // Balance Sheet
    /// Total assets for fiscal quarter
    pub total_assets_fq: Option<f64>,
    /// Total assets for fiscal year
    pub total_assets_fy: Option<f64>,
    /// Total assets historical data (quarterly)
    pub total_assets_fq_h: Option<Vec<f64>>,
    /// Total assets historical data (yearly)
    pub total_assets_fy_h: Option<Vec<f64>>,
    /// Current assets for fiscal quarter
    pub current_assets_fq: Option<f64>,
    /// Cash and cash equivalents for fiscal quarter
    pub cash_fq: Option<f64>,
    /// Cash and cash equivalents for fiscal year
    pub cash_fy: Option<f64>,
    /// Accounts receivables net for fiscal quarter
    pub receivables_fq: Option<f64>,
    /// Accounts receivables net historical data (yearly)
    pub receivables_fy_h: Option<Vec<f64>>,
    /// Inventory for fiscal quarter
    pub inventory_fq: Option<f64>,
    /// Inventory historical data (quarterly)
    pub inventory_fq_h: Option<Vec<f64>>,
    /// Property, plant and equipment net for fiscal year
    pub ppe_net_fy: Option<f64>,
    /// Property, plant and equipment net historical data (yearly)
    pub ppe_net_fy_h: Option<Vec<f64>>,
    /// Goodwill for fiscal year
    pub goodwill_fy: Option<f64>,
    /// Goodwill historical data (yearly)
    pub goodwill_fy_h: Option<Vec<f64>>,
    /// Intangibles net for fiscal quarter
    pub intangibles_net_fq: Option<f64>,
    /// Intangibles net for fiscal year
    pub intangibles_net_fy: Option<f64>,
    /// Intangibles net historical data (quarterly)
    pub intangibles_net_fq_h: Option<Vec<f64>>,
    /// Total liabilities for fiscal quarter
    pub total_liabilities_fq: Option<f64>,
    /// Total liabilities for fiscal year
    pub total_liabilities_fy: Option<f64>,
    /// Total liabilities historical data (quarterly)
    pub total_liabilities_fq_h: Option<Vec<f64>>,
    /// Total liabilities historical data (yearly)
    pub total_liabilities_fy_h: Option<Vec<f64>>,
    /// Current liabilities for fiscal quarter
    pub current_liabilities_fq: Option<f64>,
    /// Accounts payable for fiscal year
    pub accounts_payable_fy: Option<f64>,
    /// Long term debt for fiscal quarter
    pub long_term_debt_fq: Option<f64>,
    /// Long term debt historical data (yearly)
    pub long_term_debt_fy_h: Option<Vec<f64>>,
    /// Short term debt for fiscal quarter
    pub short_term_debt_fq: Option<f64>,
    /// Short term debt historical data (quarterly)
    pub short_term_debt_fq_h: Option<Vec<f64>>,
    /// Total equity for fiscal quarter
    pub total_equity_fq: Option<f64>,
    /// Total equity historical data (yearly)
    pub total_equity_fy_h: Option<Vec<f64>>,
    /// Common equity total for fiscal year
    pub common_equity_total_fy: Option<f64>,
    /// Common equity total historical data (quarterly)
    pub common_equity_total_fq_h: Option<Vec<f64>>,

    // Cash Flow
    /// Operating cash flow for fiscal year
    pub operating_cash_flow_fy: Option<f64>,
    /// Operating cash flow trailing twelve months
    pub operating_cash_flow_ttm: Option<f64>,
    /// Operating cash flow historical data (yearly)
    pub operating_cash_flow_fy_h: Option<Vec<f64>>,
    /// Free cash flow for fiscal year
    pub free_cash_flow_fy: Option<f64>,
    /// Free cash flow trailing twelve months
    pub free_cash_flow_ttm: Option<f64>,
    /// Free cash flow historical data (yearly)
    pub free_cash_flow_fy_h: Option<Vec<f64>>,
    /// Capital expenditures for fiscal quarter
    pub capex_fq: Option<f64>,
    /// Capital expenditures for fiscal year
    pub capex_fy: Option<f64>,
    /// Capital expenditures historical data (quarterly)
    pub capex_fq_h: Option<Vec<f64>>,
    /// Capital expenditures historical data (yearly)
    pub capex_fy_h: Option<Vec<f64>>,
    /// Investing cash flow for fiscal year
    pub investing_cash_flow_fy: Option<f64>,
    /// Financing cash flow for fiscal year
    pub financing_cash_flow_fy: Option<f64>,
    /// Financing cash flow for fiscal quarter
    pub financing_cash_flow_fq: Option<f64>,
}

/// Debt structure and coverage metrics
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DebtDetail {
    /// Total debt
    pub total_debt: Option<f64>,
    /// Long term debt
    pub long_term_debt: Option<f64>,
    /// Long term debt excluding capital lease
    pub long_term_debt_excl_capital_lease: Option<f64>,
    /// Short term debt
    pub short_term_debt: Option<f64>,
    /// Net debt (total debt minus cash)
    pub net_debt: Option<f64>,
    /// Net debt historical data (quarterly)
    pub net_debt_fq_h: Option<Vec<f64>>,
    /// Net debt historical data (yearly)
    pub net_debt_fy_h: Option<Vec<f64>>,
    /// Debt to equity ratio
    pub debt_to_equity: Option<f64>,
    /// Debt to equity historical data (quarterly)
    pub debt_to_equity_fq_h: Option<Vec<f64>>,
    /// Debt to assets ratio
    pub debt_to_assets: Option<f64>,
    /// Long term debt to assets ratio
    pub long_term_debt_to_assets: Option<f64>,
    /// Long term debt to assets historical data (quarterly)
    pub long_term_debt_to_assets_fq_h: Option<Vec<f64>>,
    /// Long term debt to assets historical data (yearly)
    pub long_term_debt_to_assets_fy_h: Option<Vec<f64>>,
    /// Net debt to EBITDA ratio
    pub net_debt_to_ebitda: Option<f64>,
    /// Interest expense on debt for fiscal quarter
    pub interest_expense_fq: Option<f64>,
    /// Interest expense on debt for fiscal year
    pub interest_expense_fy: Option<f64>,
    /// Interest coverage ratio (EBITDA / interest expense)
    pub interest_coverage: Option<f64>,
    /// EBITDA interest cover for fiscal year
    pub ebitda_interest_cover_fy: Option<f64>,
}

/// Credit ratings from Fitch and S&P
/// Ratings are stored as numeric codes (e.g., 700 = B, 710 = B+)
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CreditRatingSnapshot {
    /// Fitch long term rating (numeric code)
    pub fitch_rating_lt: Option<i32>,
    /// Fitch short term rating (numeric code)
    pub fitch_rating_st: Option<i32>,
    /// Fitch long term outlook (numeric code)
    pub fitch_outlook_lt: Option<i32>,
    /// Fitch rating history
    pub fitch_rating_st_h: Option<Vec<CreditRatingHistoryPoint>>,
    /// S&P long term rating (numeric code)
    pub snp_rating_lt: Option<i32>,
}

/// Single credit rating history point
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CreditRatingHistoryPoint {
    /// Rating date as Unix timestamp
    pub date: i64,
    /// Rating outlook (numeric code or null)
    pub outlook: Option<i32>,
    /// Rating value (numeric code)
    pub rating: i32,
}

/// Valuation multiples and market metrics
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ValuationMetrics {
    /// Price to earnings ratio (TTM)
    pub price_earnings: Option<f64>,
    /// Price to book ratio (FQ)
    pub price_book: Option<f64>,
    /// Price to book historical data (quarterly)
    pub price_book_fq_h: Option<Vec<f64>>,
    /// Price to book historical data (yearly)
    pub price_book_fy_h: Option<Vec<f64>>,
    /// Price to sales ratio (current)
    pub price_sales: Option<f64>,
    /// Price to sales historical data (quarterly)
    pub price_sales_fq_h: Option<Vec<f64>>,
    /// Price to cash flow ratio
    pub price_cash_flow: Option<f64>,
    /// Enterprise value
    pub enterprise_value: Option<f64>,
    /// Enterprise value historical data (yearly)
    pub enterprise_value_fy_h: Option<Vec<f64>>,
    /// EV/EBITDA ratio (current)
    pub ev_ebitda: Option<f64>,
    /// EV/EBITDA ratio for fiscal year
    pub ev_ebitda_fy: Option<f64>,
    /// 3-year beta
    pub beta_3_year: Option<f64>,
    /// 5-year beta
    pub beta_5_year: Option<f64>,
}
