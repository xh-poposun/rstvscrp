use crate::client::{TradingViewMcpClient, ClientError};
use crate::tools::{
    GetFundamentalsParams, GetFundamentalsResponse,
    GetFinancialStatementsParams, GetFinancialStatementsResponse,
    GetCreditRatingsParams, GetCreditRatingsResponse,
};

/// Get TTM fundamentals (market cap, P/E, EPS, etc.)
pub async fn handle_get_fundamentals(
    client: &TradingViewMcpClient,
    params: GetFundamentalsParams,
) -> Result<GetFundamentalsResponse, ClientError> {
    let fundamentals = client.get_fundamentals(&params.symbol).await?;

    Ok(GetFundamentalsResponse {
        symbol: params.symbol,
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
        buyback_yield: fundamentals.buyback_yield,
        share_buyback_ratio_fq: fundamentals.share_buyback_ratio_fq,
        share_buyback_ratio_fy: fundamentals.share_buyback_ratio_fy,
        total_shares_outstanding: fundamentals.total_shares_outstanding,
        total_shares_outstanding_current: fundamentals.total_shares_outstanding_current,
        diluted_shares_outstanding_fq: fundamentals.diluted_shares_outstanding_fq,
        float_shares_outstanding: fundamentals.float_shares_outstanding,
        shares_outstanding: fundamentals.shares_outstanding,
        total_shares_outstanding_calculated: fundamentals.total_shares_outstanding_calculated,
    })
}

/// Get detailed financial statements with 200+ fields
pub async fn handle_get_financial_statements(
    client: &TradingViewMcpClient,
    params: GetFinancialStatementsParams,
) -> Result<GetFinancialStatementsResponse, ClientError> {
    let detail = client.get_financial_statements_detail(&params.symbol).await?;

    Ok(GetFinancialStatementsResponse {
        symbol: params.symbol,
        statements_present: true,
        // Income Statement
        revenue_fy: detail.revenue_fy,
        revenue_fq: detail.revenue_fq,
        revenue_ttm: detail.revenue_ttm,
        gross_profit_fy: detail.gross_profit_fy,
        gross_profit_fq: detail.gross_profit_fq,
        gross_profit_ttm: detail.gross_profit_ttm,
        operating_income_fy: detail.operating_income_fy,
        operating_income_fq: detail.operating_income_fq,
        operating_income_ttm: detail.operating_income_ttm,
        ebitda_fy: detail.ebitda_fy,
        ebitda_fq: detail.ebitda_fq,
        ebitda_ttm: detail.ebitda_ttm,
        ebit_fy: detail.ebit_fy,
        ebit_fq: detail.ebit_fq,
        net_income_fy: detail.net_income_fy,
        net_income_fq: detail.net_income_fq,
        net_income_ttm: detail.net_income_ttm,
        eps_basic_fy: detail.eps_basic_fy,
        eps_basic_fq: detail.eps_basic_fq,
        eps_basic_ttm: detail.eps_basic_ttm,
        eps_diluted_fq: detail.eps_diluted_fq,
        eps_diluted_ttm: detail.eps_diluted_ttm,
        cost_of_goods_fy: detail.cost_of_goods_fy,
        operating_expenses_fq: detail.operating_expenses_fq,
        operating_expenses_fy: detail.operating_expenses_fy,
        // Balance Sheet
        total_assets_fq: detail.total_assets_fq,
        total_assets_fy: detail.total_assets_fy,
        current_assets_fq: detail.current_assets_fq,
        cash_fq: detail.cash_fq,
        cash_fy: detail.cash_fy,
        receivables_fq: detail.receivables_fq,
        inventory_fq: detail.inventory_fq,
        ppe_net_fy: detail.ppe_net_fy,
        goodwill_fy: detail.goodwill_fy,
        intangibles_net_fq: detail.intangibles_net_fq,
        intangibles_net_fy: detail.intangibles_net_fy,
        total_liabilities_fq: detail.total_liabilities_fq,
        total_liabilities_fy: detail.total_liabilities_fy,
        current_liabilities_fq: detail.current_liabilities_fq,
        accounts_payable_fy: detail.accounts_payable_fy,
        long_term_debt_fq: detail.long_term_debt_fq,
        short_term_debt_fq: detail.short_term_debt_fq,
        total_equity_fq: detail.total_equity_fq,
        common_equity_total_fy: detail.common_equity_total_fy,
        // Cash Flow
        operating_cash_flow_fy: detail.operating_cash_flow_fy,
        operating_cash_flow_ttm: detail.operating_cash_flow_ttm,
        free_cash_flow_fy: detail.free_cash_flow_fy,
        free_cash_flow_ttm: detail.free_cash_flow_ttm,
        capex_fq: detail.capex_fq,
        capex_fy: detail.capex_fy,
        investing_cash_flow_fy: detail.investing_cash_flow_fy,
        financing_cash_flow_fy: detail.financing_cash_flow_fy,
        financing_cash_flow_fq: detail.financing_cash_flow_fq,
        // Historical data
        revenue_fq_h: detail.revenue_fq_h,
        revenue_fy_h: detail.revenue_fy_h,
        gross_profit_fq_h: detail.gross_profit_fq_h,
        gross_profit_fy_h: detail.gross_profit_fy_h,
        ebitda_fy_h: detail.ebitda_fy_h,
        ebit_fq_h: detail.ebit_fq_h,
        ebit_fy_h: detail.ebit_fy_h,
        net_income_fq_h: detail.net_income_fq_h,
        net_income_fy_h: detail.net_income_fy_h,
        eps_basic_fq_h: detail.eps_basic_fq_h,
        total_assets_fq_h: detail.total_assets_fq_h,
        total_assets_fy_h: detail.total_assets_fy_h,
        total_liabilities_fq_h: detail.total_liabilities_fq_h,
        total_liabilities_fy_h: detail.total_liabilities_fy_h,
        receivables_fy_h: detail.receivables_fy_h,
        inventory_fq_h: detail.inventory_fq_h,
        ppe_net_fy_h: detail.ppe_net_fy_h,
        goodwill_fy_h: detail.goodwill_fy_h,
        intangibles_net_fq_h: detail.intangibles_net_fq_h,
        long_term_debt_fy_h: detail.long_term_debt_fy_h,
        short_term_debt_fq_h: detail.short_term_debt_fq_h,
        total_equity_fy_h: detail.total_equity_fy_h,
        common_equity_total_fq_h: detail.common_equity_total_fq_h,
        operating_cash_flow_fy_h: detail.operating_cash_flow_fy_h,
        free_cash_flow_fy_h: detail.free_cash_flow_fy_h,
        capex_fq_h: detail.capex_fq_h,
        capex_fy_h: detail.capex_fy_h,
    })
}

/// Get Fitch and S&P credit ratings
pub async fn handle_get_credit_ratings(
    client: &TradingViewMcpClient,
    params: GetCreditRatingsParams,
) -> Result<GetCreditRatingsResponse, ClientError> {
    let ratings = client.get_credit_ratings(&params.symbol).await?;

    Ok(GetCreditRatingsResponse {
        symbol: params.symbol,
        fitch_rating: ratings.fitch_rating_lt.map(map_credit_rating),
        fitch_rating_st: ratings.fitch_rating_st.map(map_credit_rating),
        fitch_outlook: ratings.fitch_outlook_lt.map(map_outlook),
        snp_rating: ratings.snp_rating_lt.map(map_credit_rating),
        agency: params.agency,
    })
}

/// Map numeric credit rating code to human-readable string
/// Based on tvdata-rs decode.rs mapping
fn map_credit_rating(code: i32) -> String {
    match code {
        100..=199 => format!("AAA{}", map_rating_suffix(code % 100)),
        200..=299 => format!("AA{}", map_rating_suffix(code % 100)),
        300..=399 => format!("A{}", map_rating_suffix(code % 100)),
        400..=499 => format!("BBB{}", map_rating_suffix(code % 100)),
        500..=599 => format!("BB{}", map_rating_suffix(code % 100)),
        600..=699 => format!("B{}", map_rating_suffix(code % 100)),
        700..=799 => format!("CCC{}", map_rating_suffix(code % 100)),
        800..=899 => format!("CC{}", map_rating_suffix(code % 100)),
        900..=999 => format!("C{}", map_rating_suffix(code % 100)),
        1000..=1099 => format!("D{}", map_rating_suffix(code % 100)),
        _ => "Unknown".to_string(),
    }
}

/// Map rating suffix code to + or - modifier
fn map_rating_suffix(suffix: i32) -> &'static str {
    match suffix {
        0 => "",
        10 => "+",
        20 => "-",
        _ => "",
    }
}

/// Map outlook code to human-readable string
fn map_outlook(code: i32) -> String {
    match code {
        1 => "Positive".to_string(),
        2 => "Stable".to_string(),
        3 => "Negative".to_string(),
        4 => "Developing".to_string(),
        _ => "Unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_credit_rating() {
        assert_eq!(map_credit_rating(100), "AAA");
        assert_eq!(map_credit_rating(110), "AAA+");
        assert_eq!(map_credit_rating(120), "AAA-");
        assert_eq!(map_credit_rating(500), "BB");
        assert_eq!(map_credit_rating(510), "BB+");
        assert_eq!(map_credit_rating(520), "BB-");
        assert_eq!(map_credit_rating(700), "CCC");
        assert_eq!(map_credit_rating(999), "C");
        assert_eq!(map_credit_rating(1000), "D");
        assert_eq!(map_credit_rating(0), "Unknown");
    }

    #[test]
    fn test_map_outlook() {
        assert_eq!(map_outlook(1), "Positive");
        assert_eq!(map_outlook(2), "Stable");
        assert_eq!(map_outlook(3), "Negative");
        assert_eq!(map_outlook(4), "Developing");
        assert_eq!(map_outlook(0), "Unknown");
    }
}
