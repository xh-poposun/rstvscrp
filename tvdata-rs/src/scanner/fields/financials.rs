use crate::scanner::field::Column;

// Income Statement Fields
pub const REVENUE_FQ: Column = Column::from_static("revenue_fq");
pub const REVENUE_FY: Column = Column::from_static("revenue_fy");
pub const GROSS_PROFIT_FQ: Column = Column::from_static("gross_profit_fq");
pub const GROSS_PROFIT_FY: Column = Column::from_static("gross_profit_fy");
pub const OPERATING_INCOME_FQ: Column = Column::from_static("operating_income_fq");
pub const OPERATING_INCOME_FY: Column = Column::from_static("operating_income_fy");
pub const PRETAX_INCOME_FQ: Column = Column::from_static("pretax_income_fq");
pub const PRETAX_INCOME_FY: Column = Column::from_static("pretax_income_fy");
pub const NET_INCOME_FQ: Column = Column::from_static("net_income_fq");
pub const NET_INCOME_FY: Column = Column::from_static("net_income_fy");
pub const EBITDA_FQ: Column = Column::from_static("ebitda_fq");
pub const EBITDA_FY: Column = Column::from_static("ebitda_fy");
pub const EBIT_FQ: Column = Column::from_static("ebit_fq");
pub const EBIT_FY: Column = Column::from_static("ebit_fy");
pub const EPS_DILUTED_FQ: Column = Column::from_static("earnings_per_share_diluted_fq");
pub const EPS_DILUTED_FY: Column = Column::from_static("earnings_per_share_diluted_fy");
pub const EPS_BASIC_FQ: Column = Column::from_static("earnings_per_share_basic_fq");
pub const EPS_BASIC_FY: Column = Column::from_static("earnings_per_share_basic_fy");

// Balance Sheet Fields
pub const TOTAL_ASSETS_FQ: Column = Column::from_static("total_assets_fq");
pub const TOTAL_ASSETS_FY: Column = Column::from_static("total_assets_fy");
pub const TOTAL_LIABILITIES_FQ: Column = Column::from_static("total_liabilities_fq");
pub const TOTAL_LIABILITIES_FY: Column = Column::from_static("total_liabilities_fy");
pub const TOTAL_EQUITY_FQ: Column = Column::from_static("total_equity_fq");
pub const TOTAL_EQUITY_FY: Column = Column::from_static("total_equity_fy");
pub const TOTAL_CURRENT_ASSETS_FQ: Column = Column::from_static("total_current_assets_fq");
pub const TOTAL_CURRENT_ASSETS_FY: Column = Column::from_static("total_current_assets_fy");
pub const TOTAL_CURRENT_LIABILITIES_FQ: Column =
    Column::from_static("total_current_liabilities_fq");
pub const TOTAL_CURRENT_LIABILITIES_FY: Column =
    Column::from_static("total_current_liabilities_fy");
pub const CASH_AND_EQUIVALENTS_FQ: Column = Column::from_static("cash_n_short_term_invest_fq");
pub const CASH_AND_EQUIVALENTS_FY: Column = Column::from_static("cash_n_short_term_invest_fy");
pub const LONG_TERM_DEBT_FQ: Column = Column::from_static("long_term_debt_fq");
pub const LONG_TERM_DEBT_FY: Column = Column::from_static("long_term_debt_fy");
pub const SHORT_TERM_DEBT_FQ: Column = Column::from_static("short_term_debt_fq");
pub const SHORT_TERM_DEBT_FY: Column = Column::from_static("short_term_debt_fy");
pub const INVENTORY_FQ: Column = Column::from_static("inventory_fq");
pub const INVENTORY_FY: Column = Column::from_static("inventory_fy");
pub const ACCOUNTS_RECEIVABLE_FQ: Column = Column::from_static("accounts_receivable_fq");
pub const ACCOUNTS_RECEIVABLE_FY: Column = Column::from_static("accounts_receivable_fy");
pub const ACCOUNTS_PAYABLE_FQ: Column = Column::from_static("accounts_payable_fq");
pub const ACCOUNTS_PAYABLE_FY: Column = Column::from_static("accounts_payable_fy");
pub const PROPERTY_PLANT_EQUIPMENT_FQ: Column = Column::from_static("property_plant_equipment_fq");
pub const PROPERTY_PLANT_EQUIPMENT_FY: Column = Column::from_static("property_plant_equipment_fy");
pub const GOODWILL_FQ: Column = Column::from_static("goodwill_fq");
pub const GOODWILL_FY: Column = Column::from_static("goodwill_fy");
pub const INTANGIBLE_ASSETS_FQ: Column = Column::from_static("intangible_assets_fq");
pub const INTANGIBLE_ASSETS_FY: Column = Column::from_static("intangible_assets_fy");

// Cash Flow Fields
pub const CASH_FROM_OPERATIONS_FQ: Column = Column::from_static("cash_f_operating_activities_fq");
pub const CASH_FROM_OPERATIONS_FY: Column = Column::from_static("cash_f_operating_activities_fy");
pub const CASH_FROM_INVESTING_FQ: Column = Column::from_static("cash_f_investing_activities_fq");
pub const CASH_FROM_INVESTING_FY: Column = Column::from_static("cash_f_investing_activities_fy");
pub const CASH_FROM_FINANCING_FQ: Column = Column::from_static("cash_f_financing_activities_fq");
pub const CASH_FROM_FINANCING_FY: Column = Column::from_static("cash_f_financing_activities_fy");
pub const CAPITAL_EXPENDITURES_FQ: Column = Column::from_static("capital_expenditures_fq");
pub const CAPITAL_EXPENDITURES_FY: Column = Column::from_static("capital_expenditures_fy");
pub const FREE_CASH_FLOW_FQ: Column = Column::from_static("free_cash_flow_fq");
pub const FREE_CASH_FLOW_FY: Column = Column::from_static("free_cash_flow_fy");
pub const DIVIDENDS_PAID_FQ: Column = Column::from_static("dividends_paid_fq");
pub const DIVIDENDS_PAID_FY: Column = Column::from_static("dividends_paid_fy");

// Debt & Coverage Fields
pub const TOTAL_DEBT_FQ: Column = Column::from_static("total_debt_fq");
pub const TOTAL_DEBT_FY: Column = Column::from_static("total_debt_fy");
pub const NET_DEBT_FQ: Column = Column::from_static("net_debt_fq");
pub const NET_DEBT_FY: Column = Column::from_static("net_debt_fy");
pub const INTEREST_COVERAGE_FQ: Column = Column::from_static("interest_coverage_fq");
pub const INTEREST_COVERAGE_FY: Column = Column::from_static("interest_coverage_fy");
pub const DEBT_TO_ASSETS_FQ: Column = Column::from_static("debt_to_assets_fq");
pub const DEBT_TO_ASSETS_FY: Column = Column::from_static("debt_to_assets_fy");
pub const DEBT_TO_EBITDA_FQ: Column = Column::from_static("debt_to_ebitda_fq");
pub const DEBT_TO_EBITDA_FY: Column = Column::from_static("debt_to_ebitda_fy");

// Credit Rating Fields
pub const ISSUER_FITCH_RATING_LT: Column = Column::from_static("issuer_fitch_rating_lt");
pub const ISSUER_FITCH_RATING_ST: Column = Column::from_static("issuer_fitch_rating_st");
pub const ISSUER_FITCH_OUTLOOK_LT: Column = Column::from_static("issuer_fitch_outlook_lt");
pub const ISSUER_SNP_RATING_LT: Column = Column::from_static("issuer_snp_rating_lt");
pub const ISSUER_SNP_RATING_ST: Column = Column::from_static("issuer_snp_rating_st");
pub const ISSUER_SNP_OUTLOOK_LT: Column = Column::from_static("issuer_snp_outlook_lt");
pub const ISSUER_MOODY_RATING_LT: Column = Column::from_static("issuer_moody_rating_lt");
pub const ISSUER_MOODY_RATING_ST: Column = Column::from_static("issuer_moody_rating_st");
pub const ISSUER_MOODY_OUTLOOK_LT: Column = Column::from_static("issuer_moody_outlook_lt");

// Extended Valuation Fields
pub const ENTERPRISE_VALUE_FQ: Column = Column::from_static("enterprise_value_fq");
pub const ENTERPRISE_VALUE_FY: Column = Column::from_static("enterprise_value_fy");
pub const EV_TO_EBITDA_FQ: Column = Column::from_static("enterprise_value_ebitda_fq");
pub const EV_TO_EBITDA_FY: Column = Column::from_static("enterprise_value_ebitda_fy");
pub const EV_TO_SALES_FQ: Column = Column::from_static("enterprise_value_revenue_fq");
pub const EV_TO_SALES_FY: Column = Column::from_static("enterprise_value_revenue_fy");
pub const PRICE_TO_CASH_FLOW_FQ: Column = Column::from_static("price_cash_flow_fq");
pub const PRICE_TO_CASH_FLOW_FY: Column = Column::from_static("price_cash_flow_fy");
pub const PRICE_TO_FREE_CASH_FLOW_FQ: Column = Column::from_static("price_free_cash_flow_fq");
pub const PRICE_TO_FREE_CASH_FLOW_FY: Column = Column::from_static("price_free_cash_flow_fy");
pub const PRICE_TO_TANGIBLE_BOOK_FQ: Column = Column::from_static("price_tangible_book_fq");
pub const PRICE_TO_TANGIBLE_BOOK_FY: Column = Column::from_static("price_tangible_book_fy");

// Per Share Fields
pub const BOOK_VALUE_PER_SHARE_FQ: Column = Column::from_static("book_value_per_share_fq");
pub const BOOK_VALUE_PER_SHARE_FY: Column = Column::from_static("book_value_per_share_fy");
pub const TANGIBLE_BOOK_VALUE_PER_SHARE_FQ: Column =
    Column::from_static("tangible_book_value_per_share_fq");
pub const TANGIBLE_BOOK_VALUE_PER_SHARE_FY: Column =
    Column::from_static("tangible_book_value_per_share_fy");
pub const CASH_PER_SHARE_FQ: Column = Column::from_static("cash_per_share_fq");
pub const CASH_PER_SHARE_FY: Column = Column::from_static("cash_per_share_fy");
pub const FREE_CASH_FLOW_PER_SHARE_FQ: Column = Column::from_static("free_cash_flow_per_share_fq");
pub const FREE_CASH_FLOW_PER_SHARE_FY: Column = Column::from_static("free_cash_flow_per_share_fy");

// Margin Fields
pub const GROSS_MARGIN_FQ: Column = Column::from_static("gross_margin_fq");
pub const GROSS_MARGIN_FY: Column = Column::from_static("gross_margin_fy");
pub const OPERATING_MARGIN_FQ: Column = Column::from_static("operating_margin_fq");
pub const OPERATING_MARGIN_FY: Column = Column::from_static("operating_margin_fy");
pub const NET_MARGIN_FQ: Column = Column::from_static("net_margin_fq");
pub const NET_MARGIN_FY: Column = Column::from_static("net_margin_fy");
pub const EBITDA_MARGIN_FQ: Column = Column::from_static("ebitda_margin_fq");
pub const EBITDA_MARGIN_FY: Column = Column::from_static("ebitda_margin_fy");
pub const EBIT_MARGIN_FQ: Column = Column::from_static("ebit_margin_fq");
pub const EBIT_MARGIN_FY: Column = Column::from_static("ebit_margin_fy");

// Return Fields
pub const RETURN_ON_EQUITY_FQ: Column = Column::from_static("return_on_equity_fq");
pub const RETURN_ON_EQUITY_FY: Column = Column::from_static("return_on_equity_fy");
pub const RETURN_ON_ASSETS_FQ: Column = Column::from_static("return_on_assets_fq");
pub const RETURN_ON_ASSETS_FY: Column = Column::from_static("return_on_assets_fy");
pub const RETURN_ON_INVESTED_CAPITAL_FQ: Column =
    Column::from_static("return_on_invested_capital_fq");
pub const RETURN_ON_INVESTED_CAPITAL_FY: Column =
    Column::from_static("return_on_invested_capital_fy");

// Efficiency Fields
pub const ASSET_TURNOVER_FQ: Column = Column::from_static("asset_turnover_fq");
pub const ASSET_TURNOVER_FY: Column = Column::from_static("asset_turnover_fy");
pub const INVENTORY_TURNOVER_FQ: Column = Column::from_static("inventory_turnover_fq");
pub const INVENTORY_TURNOVER_FY: Column = Column::from_static("inventory_turnover_fy");
pub const RECEIVABLES_TURNOVER_FQ: Column = Column::from_static("receivables_turnover_fq");
pub const RECEIVABLES_TURNOVER_FY: Column = Column::from_static("receivables_turnover_fy");
pub const DAYS_SALES_OUTSTANDING_FQ: Column = Column::from_static("days_sales_outstanding_fq");
pub const DAYS_SALES_OUTSTANDING_FY: Column = Column::from_static("days_sales_outstanding_fy");
pub const DAYS_PAYABLE_OUTSTANDING_FQ: Column = Column::from_static("days_payable_outstanding_fq");
pub const DAYS_PAYABLE_OUTSTANDING_FY: Column = Column::from_static("days_payable_outstanding_fy");
pub const DAYS_INVENTORY_OUTSTANDING_FQ: Column =
    Column::from_static("days_inventory_outstanding_fq");
pub const DAYS_INVENTORY_OUTSTANDING_FY: Column =
    Column::from_static("days_inventory_outstanding_fy");
pub const CASH_CONVERSION_CYCLE_FQ: Column = Column::from_static("cash_conversion_cycle_fq");
pub const CASH_CONVERSION_CYCLE_FY: Column = Column::from_static("cash_conversion_cycle_fy");

// Liquidity Fields
pub const CURRENT_RATIO_FQ: Column = Column::from_static("current_ratio_fq");
pub const CURRENT_RATIO_FY: Column = Column::from_static("current_ratio_fy");
pub const QUICK_RATIO_FQ: Column = Column::from_static("quick_ratio_fq");
pub const QUICK_RATIO_FY: Column = Column::from_static("quick_ratio_fy");
pub const CASH_RATIO_FQ: Column = Column::from_static("cash_ratio_fq");
pub const CASH_RATIO_FY: Column = Column::from_static("cash_ratio_fy");

// Growth Fields
pub const REVENUE_GROWTH_FQ: Column = Column::from_static("revenue_growth_fq");
pub const REVENUE_GROWTH_FY: Column = Column::from_static("revenue_growth_fy");
pub const EPS_GROWTH_FQ: Column = Column::from_static("earnings_per_share_diluted_growth_fq");
pub const EPS_GROWTH_FY: Column = Column::from_static("earnings_per_share_diluted_growth_fy");
pub const NET_INCOME_GROWTH_FQ: Column = Column::from_static("net_income_growth_fq");
pub const NET_INCOME_GROWTH_FY: Column = Column::from_static("net_income_growth_fy");
pub const EBITDA_GROWTH_FQ: Column = Column::from_static("ebitda_growth_fq");
pub const EBITDA_GROWTH_FY: Column = Column::from_static("ebitda_growth_fy");
pub const EBIT_GROWTH_FQ: Column = Column::from_static("ebit_growth_fq");
pub const EBIT_GROWTH_FY: Column = Column::from_static("ebit_growth_fy");

// Dividend Fields
pub const DIVIDEND_RATE_FQ: Column = Column::from_static("dividend_rate_fq");
pub const DIVIDEND_RATE_FY: Column = Column::from_static("dividend_rate_fy");
pub const DIVIDEND_YIELD_FQ: Column = Column::from_static("dividend_yield_fq");
pub const DIVIDEND_YIELD_FY: Column = Column::from_static("dividend_yield_fy");
pub const DIVIDEND_YIELD_TTM: Column = Column::from_static("dividend_yield_ttm");
pub const DIVIDEND_YIELD_CURRENT: Column = Column::from_static("dividend_yield_current");
pub const PAYOUT_RATIO_FQ: Column = Column::from_static("payout_ratio_fq");
pub const PAYOUT_RATIO_FY: Column = Column::from_static("payout_ratio_fy");
pub const PAYOUT_RATIO_TTM: Column = Column::from_static("payout_ratio_ttm");

// Additional Balance Sheet Fields
pub const RETAINED_EARNINGS_FQ: Column = Column::from_static("retained_earnings_fq");
pub const RETAINED_EARNINGS_FY: Column = Column::from_static("retained_earnings_fy");
pub const RETAINED_EARNINGS_FQ_H: Column = Column::from_static("retained_earnings_fq_h");
pub const RETAINED_EARNINGS_FY_H: Column = Column::from_static("retained_earnings_fy_h");
pub const WORKING_CAPITAL_FQ: Column = Column::from_static("working_capital_fq");
pub const WORKING_CAPITAL_FY: Column = Column::from_static("working_capital_fy");
pub const WORKING_CAPITAL_FQ_H: Column = Column::from_static("working_capital_fq_h");
pub const WORKING_CAPITAL_FY_H: Column = Column::from_static("working_capital_fy_h");

// Additional Cash Flow Fields
pub const OPERATING_CASH_FLOW_FQ: Column = Column::from_static("operating_cash_flow_fq");
pub const OPERATING_CASH_FLOW_FY: Column = Column::from_static("operating_cash_flow_fy");
pub const OPERATING_CASH_FLOW_FQ_H: Column = Column::from_static("operating_cash_flow_fq_h");
pub const OPERATING_CASH_FLOW_FY_H: Column = Column::from_static("operating_cash_flow_fy_h");
pub const FREE_CASH_FLOW_TTM: Column = Column::from_static("free_cash_flow_ttm");
pub const FREE_CASH_FLOW_MARGIN_FQ: Column = Column::from_static("free_cash_flow_margin_fq");
pub const FREE_CASH_FLOW_MARGIN_FY: Column = Column::from_static("free_cash_flow_margin_fy");

// Additional Valuation Fields
pub const PRICE_TO_SALES_TTM: Column = Column::from_static("price_sales_ttm");
pub const PRICE_TO_EARNINGS_GROWTH_FQ: Column = Column::from_static("price_earnings_growth_fq");
pub const PRICE_TO_EARNINGS_GROWTH_FY: Column = Column::from_static("price_earnings_growth_fy");
pub const ENTERPRISE_VALUE_FQ_H: Column = Column::from_static("enterprise_value_fq_h");
pub const ENTERPRISE_VALUE_FY_H: Column = Column::from_static("enterprise_value_fy_h");
pub const EV_TO_EBITDA_FQ_H: Column = Column::from_static("enterprise_value_ebitda_fq_h");
pub const EV_TO_EBITDA_FY_H: Column = Column::from_static("enterprise_value_ebitda_fy_h");
pub const EV_TO_SALES_FQ_H: Column = Column::from_static("enterprise_value_revenue_fq_h");
pub const EV_TO_SALES_FY_H: Column = Column::from_static("enterprise_value_revenue_fy_h");

// Beta Fields
pub const BETA_1_YEAR: Column = Column::from_static("beta_1_year");
pub const BETA_3_YEAR: Column = Column::from_static("beta_3_year");
pub const BETA_5_YEAR: Column = Column::from_static("beta_5_year");
pub const BETA_1_YEAR_H: Column = Column::from_static("beta_1_year_h");
pub const BETA_3_YEAR_H: Column = Column::from_static("beta_3_year_h");
pub const BETA_5_YEAR_H: Column = Column::from_static("beta_5_year_h");

// Forecast Fields
pub const EARNINGS_PER_SHARE_FORECAST_FQ: Column =
    Column::from_static("earnings_per_share_forecast_fq");
pub const EARNINGS_PER_SHARE_FORECAST_FY: Column =
    Column::from_static("earnings_per_share_forecast_fy");
pub const EARNINGS_PER_SHARE_FORECAST_NEXT_FQ: Column =
    Column::from_static("earnings_per_share_forecast_next_fq");
pub const EARNINGS_PER_SHARE_FORECAST_NEXT_FY: Column =
    Column::from_static("earnings_per_share_forecast_next_fy");
pub const EARNINGS_PER_SHARE_FORECAST_NEXT_FH: Column =
    Column::from_static("earnings_per_share_forecast_next_fh");
pub const REVENUE_FORECAST_FQ: Column = Column::from_static("revenue_forecast_fq");
pub const REVENUE_FORECAST_FY: Column = Column::from_static("revenue_forecast_fy");
pub const REVENUE_FORECAST_NEXT_FQ: Column = Column::from_static("revenue_forecast_next_fq");
pub const REVENUE_FORECAST_NEXT_FY: Column = Column::from_static("revenue_forecast_next_fy");

// Fiscal Period Fields
pub const FISCAL_PERIOD_FQ: Column = Column::from_static("fiscal_period_fq");
pub const FISCAL_PERIOD_FY: Column = Column::from_static("fiscal_period_fy");
pub const FISCAL_PERIOD_FQ_H: Column = Column::from_static("fiscal_period_fq_h");
pub const FISCAL_PERIOD_FY_H: Column = Column::from_static("fiscal_period_fy_h");
pub const EARNINGS_FISCAL_PERIOD_FQ: Column = Column::from_static("earnings_fiscal_period_fq");
pub const EARNINGS_FISCAL_PERIOD_FY: Column = Column::from_static("earnings_fiscal_period_fy");
pub const NEXT_EARNINGS_FISCAL_PERIOD_FQ: Column =
    Column::from_static("next_earnings_fiscal_period_fq");
pub const NEXT_EARNINGS_FISCAL_PERIOD_FY: Column =
    Column::from_static("next_earnings_fiscal_period_fy");

// Additional Historical Fields
pub const REVENUE_FQ_H: Column = Column::from_static("revenue_fq_h");
pub const REVENUE_FY_H: Column = Column::from_static("revenue_fy_h");
pub const GROSS_PROFIT_FQ_H: Column = Column::from_static("gross_profit_fq_h");
pub const GROSS_PROFIT_FY_H: Column = Column::from_static("gross_profit_fy_h");
pub const OPERATING_INCOME_FQ_H: Column = Column::from_static("operating_income_fq_h");
pub const OPERATING_INCOME_FY_H: Column = Column::from_static("operating_income_fy_h");
pub const EBITDA_FQ_H: Column = Column::from_static("ebitda_fq_h");
pub const EBITDA_FY_H: Column = Column::from_static("ebitda_fy_h");
pub const EBIT_FQ_H: Column = Column::from_static("ebit_fq_h");
pub const EBIT_FY_H: Column = Column::from_static("ebit_fy_h");
pub const NET_INCOME_FQ_H: Column = Column::from_static("net_income_fq_h");
pub const NET_INCOME_FY_H: Column = Column::from_static("net_income_fy_h");
pub const EARNINGS_PER_SHARE_BASIC_FQ_H: Column =
    Column::from_static("earnings_per_share_basic_fq_h");
pub const EARNINGS_PER_SHARE_BASIC_FY_H: Column =
    Column::from_static("earnings_per_share_basic_fy_h");
pub const EARNINGS_PER_SHARE_DILUTED_FQ_H: Column =
    Column::from_static("earnings_per_share_diluted_fq_h");
pub const EARNINGS_PER_SHARE_DILUTED_FY_H: Column =
    Column::from_static("earnings_per_share_diluted_fy_h");

// Additional Per Share Fields
pub const REVENUE_PER_SHARE_FQ: Column = Column::from_static("revenue_per_share_fq");
pub const REVENUE_PER_SHARE_FY: Column = Column::from_static("revenue_per_share_fy");
pub const REVENUE_PER_SHARE_FQ_H: Column = Column::from_static("revenue_per_share_fq_h");
pub const REVENUE_PER_SHARE_FY_H: Column = Column::from_static("revenue_per_share_fy_h");
pub const EBITDA_PER_SHARE_FQ: Column = Column::from_static("ebitda_per_share_fq");
pub const EBITDA_PER_SHARE_FY: Column = Column::from_static("ebitda_per_share_fy");
pub const EBIT_PER_SHARE_FQ: Column = Column::from_static("ebit_per_share_fq");
pub const EBIT_PER_SHARE_FY: Column = Column::from_static("ebit_per_share_fy");
pub const CAPEX_PER_SHARE_FQ: Column = Column::from_static("capex_per_share_fq");
pub const CAPEX_PER_SHARE_FY: Column = Column::from_static("capex_per_share_fy");

// Additional Margin Fields
pub const GROSS_MARGIN_TTM: Column = Column::from_static("gross_margin_ttm");
pub const OPERATING_MARGIN_TTM: Column = Column::from_static("operating_margin_ttm");
pub const EBITDA_MARGIN_TTM: Column = Column::from_static("ebitda_margin_ttm");
pub const EBIT_MARGIN_TTM: Column = Column::from_static("ebit_margin_ttm");
pub const NET_MARGIN_TTM: Column = Column::from_static("net_margin_ttm");
pub const PRE_TAX_MARGIN_FQ: Column = Column::from_static("pre_tax_margin_fq");
pub const PRE_TAX_MARGIN_FY: Column = Column::from_static("pre_tax_margin_fy");
pub const PRE_TAX_MARGIN_TTM: Column = Column::from_static("pre_tax_margin_ttm");

// Additional Return Fields
pub const RETURN_ON_EQUITY_TTM: Column = Column::from_static("return_on_equity_ttm");
pub const RETURN_ON_ASSETS_TTM: Column = Column::from_static("return_on_assets_ttm");
pub const RETURN_ON_INVESTED_CAPITAL_TTM: Column =
    Column::from_static("return_on_invested_capital_ttm");

// Additional Debt Fields
pub const DEBT_TO_EQUITY_FQ: Column = Column::from_static("debt_to_equity_fq");
pub const DEBT_TO_EQUITY_FY: Column = Column::from_static("debt_to_equity_fy");
pub const DEBT_TO_EQUITY_TTM: Column = Column::from_static("debt_to_equity_ttm");
pub const INTEREST_COVERAGE_TTM: Column = Column::from_static("interest_coverage_ttm");

// Additional Price Multiple Historical Fields
pub const PRICE_EARNINGS_FQ_H: Column = Column::from_static("price_earnings_fq_h");
pub const PRICE_EARNINGS_FY_H: Column = Column::from_static("price_earnings_fy_h");
pub const PRICE_BOOK_FQ_H: Column = Column::from_static("price_book_fq_h");
pub const PRICE_BOOK_FY_H: Column = Column::from_static("price_book_fy_h");
pub const PRICE_SALES_FQ_H: Column = Column::from_static("price_sales_fq_h");
pub const PRICE_SALES_FY_H: Column = Column::from_static("price_sales_fy_h");
pub const PRICE_CASH_FLOW_FQ_H: Column = Column::from_static("price_cash_flow_fq_h");
pub const PRICE_CASH_FLOW_FY_H: Column = Column::from_static("price_cash_flow_fy_h");
pub const PRICE_FREE_CASH_FLOW_FQ_H: Column = Column::from_static("price_free_cash_flow_fq_h");
pub const PRICE_FREE_CASH_FLOW_FY_H: Column = Column::from_static("price_free_cash_flow_fy_h");

// Additional Book Value Historical Fields
pub const BOOK_VALUE_PER_SHARE_FQ_H: Column = Column::from_static("book_value_per_share_fq_h");
pub const BOOK_VALUE_PER_SHARE_FY_H: Column = Column::from_static("book_value_per_share_fy_h");
pub const TANGIBLE_BOOK_VALUE_PER_SHARE_FQ_H: Column =
    Column::from_static("tangible_book_value_per_share_fq_h");
pub const TANGIBLE_BOOK_VALUE_PER_SHARE_FY_H: Column =
    Column::from_static("tangible_book_value_per_share_fy_h");

// Additional Liquidity Fields
pub const CURRENT_RATIO_FQ_H: Column = Column::from_static("current_ratio_fq_h");
pub const CURRENT_RATIO_FY_H: Column = Column::from_static("current_ratio_fy_h");
pub const QUICK_RATIO_FQ_H: Column = Column::from_static("quick_ratio_fq_h");
pub const QUICK_RATIO_FY_H: Column = Column::from_static("quick_ratio_fy_h");

// Additional Efficiency Historical Fields
pub const ASSET_TURNOVER_FQ_H: Column = Column::from_static("asset_turnover_fq_h");
pub const ASSET_TURNOVER_FY_H: Column = Column::from_static("asset_turnover_fy_h");
pub const INVENTORY_TURNOVER_FQ_H: Column = Column::from_static("inventory_turnover_fq_h");
pub const INVENTORY_TURNOVER_FY_H: Column = Column::from_static("inventory_turnover_fy_h");
pub const RECEIVABLES_TURNOVER_FQ_H: Column = Column::from_static("receivables_turnover_fq_h");
pub const RECEIVABLES_TURNOVER_FY_H: Column = Column::from_static("receivables_turnover_fy_h");

// Additional Return Historical Fields
pub const RETURN_ON_EQUITY_FQ_H: Column = Column::from_static("return_on_equity_fq_h");
pub const RETURN_ON_EQUITY_FY_H: Column = Column::from_static("return_on_equity_fy_h");
pub const RETURN_ON_ASSETS_FQ_H: Column = Column::from_static("return_on_assets_fq_h");
pub const RETURN_ON_ASSETS_FY_H: Column = Column::from_static("return_on_assets_fy_h");
pub const RETURN_ON_INVESTED_CAPITAL_FQ_H: Column =
    Column::from_static("return_on_invested_capital_fq_h");
pub const RETURN_ON_INVESTED_CAPITAL_FY_H: Column =
    Column::from_static("return_on_invested_capital_fy_h");

// Additional Dividend Historical Fields
pub const DIVIDEND_RATE_FQ_H: Column = Column::from_static("dividend_rate_fq_h");
pub const DIVIDEND_RATE_FY_H: Column = Column::from_static("dividend_rate_fy_h");
pub const DIVIDEND_YIELD_FQ_H: Column = Column::from_static("dividend_yield_fq_h");
pub const DIVIDEND_YIELD_FY_H: Column = Column::from_static("dividend_yield_fy_h");
pub const PAYOUT_RATIO_FQ_H: Column = Column::from_static("payout_ratio_fq_h");
pub const PAYOUT_RATIO_FY_H: Column = Column::from_static("payout_ratio_fy_h");

// Additional Growth Historical Fields
pub const REVENUE_GROWTH_FQ_H: Column = Column::from_static("revenue_growth_fq_h");
pub const REVENUE_GROWTH_FY_H: Column = Column::from_static("revenue_growth_fy_h");
pub const EPS_GROWTH_FQ_H: Column = Column::from_static("earnings_per_share_diluted_growth_fq_h");
pub const EPS_GROWTH_FY_H: Column = Column::from_static("earnings_per_share_diluted_growth_fy_h");
pub const NET_INCOME_GROWTH_FQ_H: Column = Column::from_static("net_income_growth_fq_h");
pub const NET_INCOME_GROWTH_FY_H: Column = Column::from_static("net_income_growth_fy_h");
pub const EBITDA_GROWTH_FQ_H: Column = Column::from_static("ebitda_growth_fq_h");
pub const EBITDA_GROWTH_FY_H: Column = Column::from_static("ebitda_growth_fy_h");
pub const EBIT_GROWTH_FQ_H: Column = Column::from_static("ebit_growth_fq_h");
pub const EBIT_GROWTH_FY_H: Column = Column::from_static("ebit_growth_fy_h");

// Buyback Fields
pub const BUYBACK_YIELD: Column = Column::from_static("buyback_yield");
pub const SHARE_BUYBACK_RATIO_FQ: Column = Column::from_static("share_buyback_ratio_fq");
pub const SHARE_BUYBACK_RATIO_FY: Column = Column::from_static("share_buyback_ratio_fy");

// Shares Outstanding Fields
pub const TOTAL_SHARES_OUTSTANDING: Column =
    Column::from_static("total_shares_outstanding_fundamental");
pub const TOTAL_SHARES_OUTSTANDING_CURRENT: Column =
    Column::from_static("total_shares_outstanding_current");
pub const DILUTED_SHARES_OUTSTANDING_FQ: Column =
    Column::from_static("diluted_shares_outstanding_fq");
pub const FLOAT_SHARES_OUTSTANDING: Column = Column::from_static("float_shares_outstanding");
pub const SHARES_OUTSTANDING: Column = Column::from_static("shares_outstanding");
pub const TOTAL_SHARES_OUTSTANDING_CALC: Column =
    Column::from_static("total_shares_outstanding_calculated");
