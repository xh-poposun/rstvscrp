use crate::market_data::{
    classification_columns, identity_columns, merge_columns, quote_columns, technical_columns,
};
use crate::scanner::fields::{analyst, financials, fundamentals, price};
use crate::scanner::Column;

pub(crate) fn fundamentals_columns() -> Vec<Column> {
    merge_columns([
        equity_identity_columns(),
        vec![
            fundamentals::MARKET_CAP_BASIC,
            fundamentals::PRICE_EARNINGS_TTM,
            fundamentals::PRICE_TO_BOOK_FQ,
            fundamentals::PRICE_TO_SALES_CURRENT,
            fundamentals::TOTAL_REVENUE_TTM,
            fundamentals::NET_INCOME_TTM,
            fundamentals::EPS_TTM,
            fundamentals::DIVIDEND_YIELD_RECENT,
            fundamentals::RETURN_ON_EQUITY_TTM,
            fundamentals::RETURN_ON_ASSETS_TTM,
            fundamentals::DEBT_TO_EQUITY_MRQ,
            fundamentals::CURRENT_RATIO_MRQ,
            fundamentals::FREE_CASH_FLOW_TTM,
            fundamentals::EBITDA_TTM,
        ],
    ])
}

pub(crate) fn analyst_recommendation_columns() -> Vec<Column> {
    vec![
        analyst::RECOMMENDATION_BUY,
        analyst::RECOMMENDATION_SELL,
        analyst::RECOMMENDATION_HOLD,
        analyst::RECOMMENDATION_OVER,
        analyst::RECOMMENDATION_UNDER,
        analyst::RECOMMENDATION_TOTAL,
        analyst::RECOMMENDATION_MARK,
    ]
}

pub(crate) fn analyst_price_target_columns() -> Vec<Column> {
    vec![
        analyst::PRICE_TARGET_AVERAGE,
        analyst::PRICE_TARGET_HIGH,
        analyst::PRICE_TARGET_LOW,
        analyst::PRICE_TARGET_MEDIAN,
        analyst::PRICE_TARGET_1Y,
        analyst::PRICE_TARGET_1Y_DELTA,
    ]
}

pub(crate) fn analyst_forecast_columns() -> Vec<Column> {
    vec![
        analyst::REVENUE_FORECAST_FQ,
        analyst::REVENUE_FORECAST_NEXT_FQ,
        analyst::REVENUE_FORECAST_NEXT_FH,
        analyst::REVENUE_FORECAST_NEXT_FY,
        analyst::EPS_FORECAST_FQ,
        analyst::EPS_FORECAST_NEXT_FQ,
        analyst::EPS_FORECAST_NEXT_FH,
        analyst::EPS_FORECAST_NEXT_FY,
        analyst::EPS_SURPRISE_FQ,
        analyst::EPS_SURPRISE_PERCENT_FQ,
        analyst::FORWARD_PE_NON_GAAP_NEXT_FY,
        analyst::FORWARD_PE_FY,
    ]
}

pub(crate) fn earnings_calendar_columns() -> Vec<Column> {
    vec![
        analyst::EARNINGS_RELEASE_DATE,
        analyst::EARNINGS_RELEASE_NEXT_DATE,
        analyst::EARNINGS_RELEASE_CALENDAR_DATE,
        analyst::EARNINGS_RELEASE_NEXT_CALENDAR_DATE,
        analyst::EARNINGS_RELEASE_TRADING_DATE_FQ,
        analyst::EARNINGS_RELEASE_NEXT_TRADING_DATE_FQ,
        analyst::EARNINGS_RELEASE_NEXT_TRADING_DATE_FY,
        analyst::EARNINGS_RELEASE_TIME,
        analyst::EARNINGS_RELEASE_NEXT_TIME,
        analyst::EARNINGS_PUBLICATION_TYPE_FQ,
        analyst::EARNINGS_PUBLICATION_TYPE_NEXT_FQ,
    ]
}

pub(crate) fn analyst_fx_rate_columns() -> Vec<Column> {
    vec![
        analyst::RATES_CURRENT,
        analyst::RATES_TIME_SERIES,
        analyst::RATES_FQ,
        analyst::RATES_FH,
        analyst::RATES_FY,
        analyst::RATES_TTM,
        analyst::RATES_CF,
        analyst::RATES_PT,
        analyst::RATES_MC,
        analyst::RATES_EARNINGS_FQ,
        analyst::RATES_EARNINGS_NEXT_FQ,
        analyst::RATES_DIVIDEND_RECENT,
        analyst::RATES_DIVIDEND_UPCOMING,
    ]
}

pub(crate) fn analyst_columns() -> Vec<Column> {
    merge_columns([
        equity_identity_columns(),
        vec![price::CLOSE],
        analyst_recommendation_columns(),
        analyst_price_target_columns(),
        analyst_forecast_columns(),
        earnings_calendar_columns(),
        analyst_fx_rate_columns(),
    ])
}

pub(crate) fn overview_columns() -> Vec<Column> {
    merge_columns([
        equity_quote_columns(),
        fundamentals_columns(),
        analyst_columns(),
        equity_technical_columns(),
        buyback_columns(),
    ])
}

pub(crate) fn equity_identity_columns() -> Vec<Column> {
    merge_columns([identity_columns(), classification_columns()])
}

pub(crate) fn equity_quote_columns() -> Vec<Column> {
    merge_columns([equity_identity_columns(), quote_columns()])
}

pub(crate) fn equity_technical_columns() -> Vec<Column> {
    merge_columns([equity_identity_columns(), technical_columns()])
}

pub(crate) fn financial_statements_detail_columns() -> Vec<Column> {
    merge_columns([
        equity_identity_columns(),
        vec![
            financials::REVENUE_FQ,
            financials::REVENUE_FY,
            financials::GROSS_PROFIT_FQ,
            financials::GROSS_PROFIT_FY,
            financials::OPERATING_INCOME_FQ,
            financials::OPERATING_INCOME_FY,
            financials::PRETAX_INCOME_FQ,
            financials::PRETAX_INCOME_FY,
            financials::NET_INCOME_FQ,
            financials::NET_INCOME_FY,
            financials::EBITDA_FQ,
            financials::EBITDA_FY,
            financials::EBIT_FQ,
            financials::EBIT_FY,
            financials::EPS_DILUTED_FQ,
            financials::EPS_DILUTED_FY,
            financials::EPS_BASIC_FQ,
            financials::EPS_BASIC_FY,
            financials::TOTAL_ASSETS_FQ,
            financials::TOTAL_ASSETS_FY,
            financials::TOTAL_LIABILITIES_FQ,
            financials::TOTAL_LIABILITIES_FY,
            financials::TOTAL_EQUITY_FQ,
            financials::TOTAL_EQUITY_FY,
            financials::TOTAL_CURRENT_ASSETS_FQ,
            financials::TOTAL_CURRENT_ASSETS_FY,
            financials::TOTAL_CURRENT_LIABILITIES_FQ,
            financials::TOTAL_CURRENT_LIABILITIES_FY,
            financials::CASH_AND_EQUIVALENTS_FQ,
            financials::CASH_AND_EQUIVALENTS_FY,
            financials::LONG_TERM_DEBT_FQ,
            financials::LONG_TERM_DEBT_FY,
            financials::SHORT_TERM_DEBT_FQ,
            financials::SHORT_TERM_DEBT_FY,
            financials::INVENTORY_FQ,
            financials::INVENTORY_FY,
            financials::ACCOUNTS_RECEIVABLE_FQ,
            financials::ACCOUNTS_RECEIVABLE_FY,
            financials::ACCOUNTS_PAYABLE_FQ,
            financials::ACCOUNTS_PAYABLE_FY,
            financials::PROPERTY_PLANT_EQUIPMENT_FQ,
            financials::PROPERTY_PLANT_EQUIPMENT_FY,
            financials::GOODWILL_FQ,
            financials::GOODWILL_FY,
            financials::INTANGIBLE_ASSETS_FQ,
            financials::INTANGIBLE_ASSETS_FY,
            financials::CASH_FROM_OPERATIONS_FQ,
            financials::CASH_FROM_OPERATIONS_FY,
            financials::CASH_FROM_INVESTING_FQ,
            financials::CASH_FROM_INVESTING_FY,
            financials::CASH_FROM_FINANCING_FQ,
            financials::CASH_FROM_FINANCING_FY,
            financials::CAPITAL_EXPENDITURES_FQ,
            financials::CAPITAL_EXPENDITURES_FY,
            financials::FREE_CASH_FLOW_FQ,
            financials::FREE_CASH_FLOW_FY,
            financials::DIVIDENDS_PAID_FQ,
            financials::DIVIDENDS_PAID_FY,
        ],
    ])
}

pub(crate) fn debt_detail_columns() -> Vec<Column> {
    merge_columns([
        equity_identity_columns(),
        vec![
            financials::TOTAL_DEBT_FQ,
            financials::TOTAL_DEBT_FY,
            financials::NET_DEBT_FQ,
            financials::NET_DEBT_FY,
            financials::LONG_TERM_DEBT_FQ,
            financials::LONG_TERM_DEBT_FY,
            financials::SHORT_TERM_DEBT_FQ,
            financials::SHORT_TERM_DEBT_FY,
            financials::INTEREST_COVERAGE_FQ,
            financials::INTEREST_COVERAGE_FY,
            financials::DEBT_TO_EQUITY_FQ,
            financials::DEBT_TO_EQUITY_FY,
            financials::DEBT_TO_ASSETS_FQ,
            financials::DEBT_TO_ASSETS_FY,
            financials::DEBT_TO_EBITDA_FQ,
            financials::DEBT_TO_EBITDA_FY,
        ],
    ])
}

pub(crate) fn credit_rating_columns() -> Vec<Column> {
    merge_columns([
        equity_identity_columns(),
        vec![
            financials::ISSUER_FITCH_RATING_LT,
            financials::ISSUER_FITCH_RATING_ST,
            financials::ISSUER_FITCH_OUTLOOK_LT,
            financials::ISSUER_SNP_RATING_LT,
            financials::ISSUER_SNP_RATING_ST,
            financials::ISSUER_SNP_OUTLOOK_LT,
            financials::ISSUER_MOODY_RATING_LT,
            financials::ISSUER_MOODY_RATING_ST,
            financials::ISSUER_MOODY_OUTLOOK_LT,
        ],
    ])
}

pub(crate) fn valuation_metrics_columns() -> Vec<Column> {
    merge_columns([
        equity_identity_columns(),
        vec![
            financials::ENTERPRISE_VALUE_FQ,
            financials::ENTERPRISE_VALUE_FY,
            financials::EV_TO_EBITDA_FQ,
            financials::EV_TO_EBITDA_FY,
            financials::EV_TO_SALES_FQ,
            financials::EV_TO_SALES_FY,
            financials::PRICE_TO_CASH_FLOW_FQ,
            financials::PRICE_TO_CASH_FLOW_FY,
            financials::PRICE_TO_FREE_CASH_FLOW_FQ,
            financials::PRICE_TO_FREE_CASH_FLOW_FY,
            financials::PRICE_TO_TANGIBLE_BOOK_FQ,
            financials::PRICE_TO_TANGIBLE_BOOK_FY,
            financials::BOOK_VALUE_PER_SHARE_FQ,
            financials::BOOK_VALUE_PER_SHARE_FY,
            financials::TANGIBLE_BOOK_VALUE_PER_SHARE_FQ,
            financials::TANGIBLE_BOOK_VALUE_PER_SHARE_FY,
            financials::CASH_PER_SHARE_FQ,
            financials::CASH_PER_SHARE_FY,
            financials::FREE_CASH_FLOW_PER_SHARE_FQ,
            financials::FREE_CASH_FLOW_PER_SHARE_FY,
            financials::GROSS_MARGIN_FQ,
            financials::GROSS_MARGIN_FY,
            financials::OPERATING_MARGIN_FQ,
            financials::OPERATING_MARGIN_FY,
            financials::NET_MARGIN_FQ,
            financials::NET_MARGIN_FY,
            financials::EBITDA_MARGIN_FQ,
            financials::EBITDA_MARGIN_FY,
            financials::EBIT_MARGIN_FQ,
            financials::EBIT_MARGIN_FY,
            financials::RETURN_ON_EQUITY_FQ,
            financials::RETURN_ON_EQUITY_FY,
            financials::RETURN_ON_ASSETS_FQ,
            financials::RETURN_ON_ASSETS_FY,
            financials::RETURN_ON_INVESTED_CAPITAL_FQ,
            financials::RETURN_ON_INVESTED_CAPITAL_FY,
        ],
    ])
}

pub(crate) fn buyback_columns() -> Vec<Column> {
    merge_columns([
        equity_identity_columns(),
        vec![
            financials::BUYBACK_YIELD,
            financials::SHARE_BUYBACK_RATIO_FQ,
            financials::SHARE_BUYBACK_RATIO_FY,
        ],
    ])
}
