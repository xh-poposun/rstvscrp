use crate::market_data::{decode_quote, decode_technical, RowDecoder};
use crate::scanner::fields::{analyst, financials, fundamentals, price};

use super::types::{
    AnalystForecasts, AnalystFxRates, AnalystPriceTargets, AnalystRecommendations, AnalystSummary,
    BuybackMetrics, CreditRatingHistoryPoint, DebtDetail, EarningsCalendar, EquityOverview,
    FinancialStatementsDetail, FundamentalsSnapshot,
};

pub(crate) fn decode_fundamentals(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> FundamentalsSnapshot {
    FundamentalsSnapshot {
        instrument: decoder.identity(row),
        market_cap: decoder.number(row, fundamentals::MARKET_CAP_BASIC.as_str()),
        price_earnings_ttm: decoder.number(row, fundamentals::PRICE_EARNINGS_TTM.as_str()),
        price_to_book_fq: decoder.number(row, fundamentals::PRICE_TO_BOOK_FQ.as_str()),
        price_to_sales_current: decoder.number(row, fundamentals::PRICE_TO_SALES_CURRENT.as_str()),
        total_revenue_ttm: decoder.number(row, fundamentals::TOTAL_REVENUE_TTM.as_str()),
        net_income_ttm: decoder.number(row, fundamentals::NET_INCOME_TTM.as_str()),
        eps_ttm: decoder.number(row, fundamentals::EPS_TTM.as_str()),
        dividend_yield_recent: decoder.number(row, fundamentals::DIVIDEND_YIELD_RECENT.as_str()),
        return_on_equity_ttm: decoder.number(row, fundamentals::RETURN_ON_EQUITY_TTM.as_str()),
        return_on_assets_ttm: decoder.number(row, fundamentals::RETURN_ON_ASSETS_TTM.as_str()),
        debt_to_equity_mrq: decoder.number(row, fundamentals::DEBT_TO_EQUITY_MRQ.as_str()),
        current_ratio_mrq: decoder.number(row, fundamentals::CURRENT_RATIO_MRQ.as_str()),
        free_cash_flow_ttm: decoder.number(row, fundamentals::FREE_CASH_FLOW_TTM.as_str()),
        ebitda_ttm: decoder.number(row, fundamentals::EBITDA_TTM.as_str()),
    }
}

pub(crate) fn decode_buyback(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> BuybackMetrics {
    BuybackMetrics {
        buyback_yield: decoder.number(row, financials::BUYBACK_YIELD.as_str()),
        share_buyback_ratio_fq: decoder.number(row, financials::SHARE_BUYBACK_RATIO_FQ.as_str()),
        share_buyback_ratio_fy: decoder.number(row, financials::SHARE_BUYBACK_RATIO_FY.as_str()),
        total_shares_outstanding: decoder
            .number(row, financials::TOTAL_SHARES_OUTSTANDING.as_str()),
        total_shares_outstanding_current: decoder
            .number(row, financials::TOTAL_SHARES_OUTSTANDING_CURRENT.as_str()),
        diluted_shares_outstanding_fq: decoder
            .number(row, financials::DILUTED_SHARES_OUTSTANDING_FQ.as_str()),
        float_shares_outstanding: decoder
            .number(row, financials::FLOAT_SHARES_OUTSTANDING.as_str()),
        shares_outstanding: decoder.number(row, financials::SHARES_OUTSTANDING.as_str()),
        total_shares_outstanding_calculated: decoder
            .number(row, financials::TOTAL_SHARES_OUTSTANDING_CALC.as_str()),
    }
}

pub(crate) fn decode_analyst_recommendations(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> AnalystRecommendations {
    AnalystRecommendations {
        buy: decoder.whole_number(row, analyst::RECOMMENDATION_BUY.as_str()),
        sell: decoder.whole_number(row, analyst::RECOMMENDATION_SELL.as_str()),
        hold: decoder.whole_number(row, analyst::RECOMMENDATION_HOLD.as_str()),
        outperform: decoder.whole_number(row, analyst::RECOMMENDATION_OVER.as_str()),
        underperform: decoder.whole_number(row, analyst::RECOMMENDATION_UNDER.as_str()),
        total: decoder.whole_number(row, analyst::RECOMMENDATION_TOTAL.as_str()),
        rating: decoder.number(row, analyst::RECOMMENDATION_MARK.as_str()),
    }
}

pub(crate) fn decode_analyst_price_targets(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> AnalystPriceTargets {
    AnalystPriceTargets {
        average: decoder.number(row, analyst::PRICE_TARGET_AVERAGE.as_str()),
        high: decoder.number(row, analyst::PRICE_TARGET_HIGH.as_str()),
        low: decoder.number(row, analyst::PRICE_TARGET_LOW.as_str()),
        median: decoder.number(row, analyst::PRICE_TARGET_MEDIAN.as_str()),
        one_year: decoder.number(row, analyst::PRICE_TARGET_1Y.as_str()),
        one_year_delta_percent: decoder.number(row, analyst::PRICE_TARGET_1Y_DELTA.as_str()),
    }
}

pub(crate) fn decode_analyst_forecasts(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> AnalystForecasts {
    AnalystForecasts {
        revenue_current_quarter: decoder.number(row, analyst::REVENUE_FORECAST_FQ.as_str()),
        revenue_next_quarter: decoder.number(row, analyst::REVENUE_FORECAST_NEXT_FQ.as_str()),
        revenue_next_half_year: decoder.number(row, analyst::REVENUE_FORECAST_NEXT_FH.as_str()),
        revenue_next_fiscal_year: decoder.number(row, analyst::REVENUE_FORECAST_NEXT_FY.as_str()),
        eps_current_quarter: decoder.number(row, analyst::EPS_FORECAST_FQ.as_str()),
        eps_next_quarter: decoder.number(row, analyst::EPS_FORECAST_NEXT_FQ.as_str()),
        eps_next_half_year: decoder.number(row, analyst::EPS_FORECAST_NEXT_FH.as_str()),
        eps_next_fiscal_year: decoder.number(row, analyst::EPS_FORECAST_NEXT_FY.as_str()),
        eps_surprise_recent_quarter: decoder.number(row, analyst::EPS_SURPRISE_FQ.as_str()),
        eps_surprise_percent_recent_quarter: decoder
            .number(row, analyst::EPS_SURPRISE_PERCENT_FQ.as_str()),
        forward_non_gaap_price_earnings: decoder
            .number(row, analyst::FORWARD_PE_NON_GAAP_NEXT_FY.as_str()),
        forward_price_earnings_fiscal_year: decoder.number(row, analyst::FORWARD_PE_FY.as_str()),
    }
}

pub(crate) fn decode_earnings_calendar(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> EarningsCalendar {
    EarningsCalendar {
        recent_release_at: decoder.timestamp(row, analyst::EARNINGS_RELEASE_DATE.as_str()),
        next_release_at: decoder.timestamp(row, analyst::EARNINGS_RELEASE_NEXT_DATE.as_str()),
        recent_calendar_date: decoder
            .timestamp(row, analyst::EARNINGS_RELEASE_CALENDAR_DATE.as_str()),
        next_calendar_date: decoder
            .timestamp(row, analyst::EARNINGS_RELEASE_NEXT_CALENDAR_DATE.as_str()),
        current_quarter_trading_date: decoder
            .timestamp(row, analyst::EARNINGS_RELEASE_TRADING_DATE_FQ.as_str()),
        next_quarter_trading_date: decoder
            .timestamp(row, analyst::EARNINGS_RELEASE_NEXT_TRADING_DATE_FQ.as_str()),
        fiscal_year_trading_date: decoder
            .timestamp(row, analyst::EARNINGS_RELEASE_NEXT_TRADING_DATE_FY.as_str()),
        recent_release_time_code: decoder
            .whole_number(row, analyst::EARNINGS_RELEASE_TIME.as_str()),
        next_release_time_code: decoder
            .whole_number(row, analyst::EARNINGS_RELEASE_NEXT_TIME.as_str()),
        current_quarter_publication_type_code: decoder
            .whole_number(row, analyst::EARNINGS_PUBLICATION_TYPE_FQ.as_str()),
        next_quarter_publication_type_code: decoder
            .whole_number(row, analyst::EARNINGS_PUBLICATION_TYPE_NEXT_FQ.as_str()),
    }
}

pub(crate) fn decode_analyst_fx_rates(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> AnalystFxRates {
    AnalystFxRates {
        current: decoder.conversion_rates(row, analyst::RATES_CURRENT.as_str()),
        time_series: decoder.conversion_rates(row, analyst::RATES_TIME_SERIES.as_str()),
        revenue_current_quarter: decoder.conversion_rates(row, analyst::RATES_FQ.as_str()),
        revenue_next_half_year: decoder.conversion_rates(row, analyst::RATES_FH.as_str()),
        revenue_next_fiscal_year: decoder.conversion_rates(row, analyst::RATES_FY.as_str()),
        trailing_twelve_months: decoder.conversion_rates(row, analyst::RATES_TTM.as_str()),
        cash_flow: decoder.conversion_rates(row, analyst::RATES_CF.as_str()),
        price_target: decoder.conversion_rates(row, analyst::RATES_PT.as_str()),
        market_cap: decoder.conversion_rates(row, analyst::RATES_MC.as_str()),
        earnings_current_quarter: decoder
            .conversion_rates(row, analyst::RATES_EARNINGS_FQ.as_str()),
        earnings_next_quarter: decoder
            .conversion_rates(row, analyst::RATES_EARNINGS_NEXT_FQ.as_str()),
        dividend_recent: decoder.conversion_rates(row, analyst::RATES_DIVIDEND_RECENT.as_str()),
        dividend_upcoming: decoder.conversion_rates(row, analyst::RATES_DIVIDEND_UPCOMING.as_str()),
    }
}

pub(crate) fn decode_analyst(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> AnalystSummary {
    AnalystSummary {
        instrument: decoder.identity(row),
        close: decoder.number(row, price::CLOSE.as_str()),
        recommendations: decode_analyst_recommendations(decoder, row),
        price_targets: decode_analyst_price_targets(decoder, row),
        forecasts: decode_analyst_forecasts(decoder, row),
        earnings: decode_earnings_calendar(decoder, row),
        fx_rates: decode_analyst_fx_rates(decoder, row),
    }
}

pub(crate) fn decode_overview(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> EquityOverview {
    EquityOverview {
        quote: decode_quote(decoder, row),
        fundamentals: decode_fundamentals(decoder, row),
        analyst: decode_analyst(decoder, row),
        technicals: decode_technical(decoder, row),
        buyback: decode_buyback(decoder, row),
    }
}

pub(crate) fn decode_financial_statements_detail(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> FinancialStatementsDetail {
    use crate::scanner::fields::financials;

    FinancialStatementsDetail {
        revenue_fy: decoder.number(row, financials::REVENUE_FY.as_str()),
        revenue_fq: decoder.number(row, financials::REVENUE_FQ.as_str()),
        revenue_ttm: None,
        revenue_fq_h: None,
        revenue_fy_h: None,
        gross_profit_fy: decoder.number(row, financials::GROSS_PROFIT_FY.as_str()),
        gross_profit_fq: decoder.number(row, financials::GROSS_PROFIT_FQ.as_str()),
        gross_profit_ttm: None,
        gross_profit_fq_h: None,
        gross_profit_fy_h: None,
        operating_income_fy: decoder.number(row, financials::OPERATING_INCOME_FY.as_str()),
        operating_income_fq: decoder.number(row, financials::OPERATING_INCOME_FQ.as_str()),
        operating_income_ttm: None,
        ebitda_fy: decoder.number(row, financials::EBITDA_FY.as_str()),
        ebitda_fq: decoder.number(row, financials::EBITDA_FQ.as_str()),
        ebitda_ttm: None,
        ebitda_fy_h: None,
        ebit_fy: decoder.number(row, financials::EBIT_FY.as_str()),
        ebit_fq: decoder.number(row, financials::EBIT_FQ.as_str()),
        ebit_fq_h: None,
        ebit_fy_h: None,
        net_income_fy: decoder.number(row, financials::NET_INCOME_FY.as_str()),
        net_income_fq: decoder.number(row, financials::NET_INCOME_FQ.as_str()),
        net_income_ttm: None,
        net_income_fq_h: None,
        net_income_fy_h: None,
        eps_basic_fy: decoder.number(row, financials::EPS_BASIC_FY.as_str()),
        eps_basic_fq: decoder.number(row, financials::EPS_BASIC_FQ.as_str()),
        eps_basic_ttm: None,
        eps_basic_fq_h: None,
        eps_diluted_fq: decoder.number(row, financials::EPS_DILUTED_FQ.as_str()),
        eps_diluted_ttm: None,
        cost_of_goods_fy: None,
        cost_of_goods_fy_h: None,
        operating_expenses_fq: None,
        operating_expenses_fy: None,
        operating_expenses_fq_h: None,
        operating_expenses_fy_h: None,
        total_assets_fq: decoder.number(row, financials::TOTAL_ASSETS_FQ.as_str()),
        total_assets_fy: decoder.number(row, financials::TOTAL_ASSETS_FY.as_str()),
        total_assets_fq_h: None,
        total_assets_fy_h: None,
        current_assets_fq: decoder.number(row, financials::TOTAL_CURRENT_ASSETS_FQ.as_str()),
        cash_fq: decoder.number(row, financials::CASH_AND_EQUIVALENTS_FQ.as_str()),
        cash_fy: decoder.number(row, financials::CASH_AND_EQUIVALENTS_FY.as_str()),
        receivables_fq: decoder.number(row, financials::ACCOUNTS_RECEIVABLE_FQ.as_str()),
        receivables_fy_h: None,
        inventory_fq: decoder.number(row, financials::INVENTORY_FQ.as_str()),
        inventory_fq_h: None,
        ppe_net_fy: decoder.number(row, financials::PROPERTY_PLANT_EQUIPMENT_FY.as_str()),
        ppe_net_fy_h: None,
        goodwill_fy: decoder.number(row, financials::GOODWILL_FY.as_str()),
        goodwill_fy_h: None,
        intangibles_net_fq: decoder.number(row, financials::INTANGIBLE_ASSETS_FQ.as_str()),
        intangibles_net_fy: decoder.number(row, financials::INTANGIBLE_ASSETS_FY.as_str()),
        intangibles_net_fq_h: None,
        total_liabilities_fq: decoder.number(row, financials::TOTAL_LIABILITIES_FQ.as_str()),
        total_liabilities_fy: decoder.number(row, financials::TOTAL_LIABILITIES_FY.as_str()),
        total_liabilities_fq_h: None,
        total_liabilities_fy_h: None,
        current_liabilities_fq: decoder
            .number(row, financials::TOTAL_CURRENT_LIABILITIES_FQ.as_str()),
        accounts_payable_fy: decoder.number(row, financials::ACCOUNTS_PAYABLE_FY.as_str()),
        long_term_debt_fq: decoder.number(row, financials::LONG_TERM_DEBT_FQ.as_str()),
        long_term_debt_fy_h: None,
        short_term_debt_fq: decoder.number(row, financials::SHORT_TERM_DEBT_FQ.as_str()),
        short_term_debt_fq_h: None,
        total_equity_fq: decoder.number(row, financials::TOTAL_EQUITY_FQ.as_str()),
        total_equity_fy_h: None,
        common_equity_total_fy: None,
        common_equity_total_fq_h: None,
        operating_cash_flow_fy: decoder.number(row, financials::CASH_FROM_OPERATIONS_FY.as_str()),
        operating_cash_flow_ttm: None,
        operating_cash_flow_fy_h: None,
        free_cash_flow_fy: decoder.number(row, financials::FREE_CASH_FLOW_FY.as_str()),
        free_cash_flow_ttm: None,
        free_cash_flow_fy_h: None,
        capex_fq: decoder.number(row, financials::CAPITAL_EXPENDITURES_FQ.as_str()),
        capex_fy: decoder.number(row, financials::CAPITAL_EXPENDITURES_FY.as_str()),
        capex_fq_h: None,
        capex_fy_h: None,
        investing_cash_flow_fy: decoder.number(row, financials::CASH_FROM_INVESTING_FY.as_str()),
        financing_cash_flow_fy: decoder.number(row, financials::CASH_FROM_FINANCING_FY.as_str()),
        financing_cash_flow_fq: decoder.number(row, financials::CASH_FROM_FINANCING_FQ.as_str()),
    }
}

pub(crate) fn decode_debt_detail(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> DebtDetail {
    use crate::scanner::fields::financials;

    DebtDetail {
        total_debt: decoder.number(row, financials::TOTAL_DEBT_FQ.as_str()),
        long_term_debt: decoder.number(row, financials::LONG_TERM_DEBT_FQ.as_str()),
        long_term_debt_excl_capital_lease: None,
        short_term_debt: decoder.number(row, financials::SHORT_TERM_DEBT_FQ.as_str()),
        net_debt: decoder.number(row, financials::NET_DEBT_FQ.as_str()),
        net_debt_fq_h: None,
        net_debt_fy_h: None,
        debt_to_equity: decoder.number(row, financials::DEBT_TO_EQUITY_FQ.as_str()),
        debt_to_equity_fq_h: None,
        debt_to_assets: decoder.number(row, financials::DEBT_TO_ASSETS_FQ.as_str()),
        long_term_debt_to_assets: None,
        long_term_debt_to_assets_fq_h: None,
        long_term_debt_to_assets_fy_h: None,
        net_debt_to_ebitda: decoder.number(row, financials::DEBT_TO_EBITDA_FQ.as_str()),
        interest_expense_fq: None,
        interest_expense_fy: None,
        interest_coverage: decoder.number(row, financials::INTEREST_COVERAGE_FQ.as_str()),
        ebitda_interest_cover_fy: None,
    }
}

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

fn map_rating_suffix(suffix: i32) -> &'static str {
    match suffix {
        0 => "",
        10 => "+",
        20 => "-",
        _ => "",
    }
}

pub(crate) fn decode_credit_rating_snapshot(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> super::types::CreditRatingSnapshot {
    use crate::scanner::fields::financials;

    super::types::CreditRatingSnapshot {
        fitch_rating_lt: decoder
            .number(row, financials::ISSUER_FITCH_RATING_LT.as_str())
            .map(|v| v as i32),
        fitch_rating_st: decoder
            .number(row, financials::ISSUER_FITCH_RATING_ST.as_str())
            .map(|v| v as i32),
        fitch_outlook_lt: decoder
            .number(row, financials::ISSUER_FITCH_OUTLOOK_LT.as_str())
            .map(|v| v as i32),
        fitch_rating_st_h: None,
        snp_rating_lt: decoder
            .number(row, financials::ISSUER_SNP_RATING_LT.as_str())
            .map(|v| v as i32),
    }
}

pub(crate) fn decode_valuation_metrics(
    decoder: &RowDecoder,
    row: &crate::scanner::ScanRow,
) -> super::types::ValuationMetrics {
    use crate::scanner::fields::financials;

    super::types::ValuationMetrics {
        price_earnings: None,
        price_book: None,
        price_book_fq_h: None,
        price_book_fy_h: None,
        price_sales: None,
        price_sales_fq_h: None,
        price_cash_flow: decoder.number(row, financials::PRICE_TO_CASH_FLOW_FQ.as_str()),
        enterprise_value: decoder.number(row, financials::ENTERPRISE_VALUE_FQ.as_str()),
        enterprise_value_fy_h: None,
        ev_ebitda: decoder.number(row, financials::EV_TO_EBITDA_FQ.as_str()),
        ev_ebitda_fy: decoder.number(row, financials::EV_TO_EBITDA_FY.as_str()),
        beta_3_year: None,
        beta_5_year: None,
    }
}
