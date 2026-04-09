---
name: data-fetchers
description: TradingView MCP data fetching patterns and field mappings for financial modeling
---

# TradingView Data Fetching Guide

## Overview

This document provides reusable patterns for fetching financial data from TradingView MCP server for use in DCF, Comps, LBO, and 3-statement models.

## Environment Setup

```bash
# Required environment variables
export MCP_SERVER_URL="http://localhost:3000"
export TVDATA_USERNAME="your_username"
export TVDATA_PASSWORD="your_password"

# Health check
curl http://localhost:3000/health
```

## Available MCP Tools

### 1. get_quote
**Purpose**: Current market data (price, volume, market cap)

**Parameters**:
- `symbol` (string): Trading symbol (e.g., "NASDAQ:AAPL", "NYSE:MSFT")

**Returns**: Current price, change, volume, market cap

**Example**:
```json
{
  "symbol": "NASDAQ:AAPL",
  "price": 175.50,
  "change": 2.30,
  "change_percent": 1.33,
  "volume": 45678900,
  "market_cap": 2750000000000
}
```

### 2. get_fundamentals
**Purpose**: Comprehensive financial metrics (200+ fields)

**Parameters**:
- `symbol` (string): Trading symbol
- `fields` (array, optional): Specific fields to retrieve

**Key Fields for Financial Models**:

| Field | Description | Model Usage |
|-------|-------------|-------------|
| `total_revenue_fq_h` | Total revenue (quarterly, historical) | DCF, Comps, 3-Statement |
| `total_revenue_ttm` | Total revenue (TTM) | Comps |
| `ebitda_fq` | EBITDA (quarterly) | DCF, Comps, LBO |
| `ebitda_ttm` | EBITDA (TTM) | Comps |
| `net_income_fq` | Net income (quarterly) | 3-Statement |
| `net_income_ttm` | Net income (TTM) | Comps |
| `eps_fq` | EPS (quarterly) | Comps |
| `eps_ttm` | EPS (TTM) | Comps |
| `market_cap_basic` | Market capitalization | Comps, DCF |
| `beta_3_year` | 3-year beta | DCF (WACC) |
| `total_debt_fq` | Total debt | DCF, LBO |
| `cash_n_short_term_invest_fq` | Cash & equivalents | DCF, LBO |
| `net_debt_fq` | Net debt | LBO |
| `capital_expenditures_fq_h` | CapEx (historical) | DCF, 3-Statement |
| `depreciation_fy_h` | Depreciation (historical) | DCF |
| `cash_f_operating_activities_fq_h` | CFO (historical) | 3-Statement |
| `free_cash_flow_ttm` | Free cash flow (TTM) | LBO |
| `price_earnings_ttm` | P/E ratio | Comps |
| `price_book_fq_h` | P/B ratio | Comps |
| `enterprise_value_ebitda_current` | EV/EBITDA | Comps |
| `return_on_equity_fy` | ROE | Comps |
| `debt_to_equity_current` | Debt/Equity | LBO |
| `ebitda_interst_cover_fy` | Interest coverage | LBO |
| `cost_of_goods_fy_h` | COGS (historical) | 3-Statement |
| `operating_expenses_fq` | Operating expenses | 3-Statement |
| `income_tax_fq` | Income tax | DCF (tax rate calc) |
| `pretax_income_fq` | Pre-tax income | DCF (tax rate calc) |

### 3. get_financial_statements
**Purpose**: Full income statement, balance sheet, cash flow

**Parameters**:
- `symbol` (string): Trading symbol
- `statement` (string): "income", "balance", "cash"
- `period` (string): "annual" or "quarterly"

**Returns**: Complete financial statements with historical data

### 4. get_credit_ratings
**Purpose**: Credit ratings from Fitch, S&P, Moody's

**Parameters**:
- `symbol` (string): Trading symbol

**Returns**: Current credit ratings

**Example**:
```json
{
  "fitch": "AA-",
  "sp": "AA",
  "moodys": "Aa3"
}
```

### 5. get_debt_maturity
**Purpose**: Debt maturity schedule from SEC EDGAR

**Parameters**:
- `symbol` (string): Trading symbol

**Returns**: Debt maturity profile

### 6. scan_stocks
**Purpose**: Find comparable companies

**Parameters**:
- `filters` (object): Filter criteria
  - `market_cap_min` / `market_cap_max`
  - `sector`
  - `exchange`
  - `pe_min` / `pe_max`
  - `ebitda_margin_min` / `ebitda_margin_max`

**Returns**: List of matching symbols

**Example**:
```json
{
  "filters": {
    "sector": "Technology",
    "market_cap_min": 10000000000,
    "market_cap_max": 1000000000000
  }
}
```

### 7. get_earnings_calendar
**Purpose**: Earnings dates and estimates

**Parameters**:
- `symbol` (string): Trading symbol
- `start_date` (string): Start date (YYYY-MM-DD)
- `end_date` (string): End date (YYYY-MM-DD)

**Returns**: Earnings dates and analyst estimates

## Field Mappings by Model Type

### DCF Model Data Mapping

| Required Field | TradingView Source | Coverage | Gap Handling |
|---------------|-------------------|----------|--------------|
| Revenue history | `get_fundamentals` → `total_revenue_fq_h` | 100% | Direct use |
| EBITDA | `get_fundamentals` → `ebitda_fq` | 100% | Direct use |
| CapEx | `get_fundamentals` → `capital_expenditures_fq_h` | 100% | Direct use |
| Depreciation | `get_fundamentals` → `depreciation_fy_h` | 100% | Direct use |
| Net Working Capital | Calculate from current assets/liabilities | 100% | Calculate |
| Beta | `get_fundamentals` → `beta_3_year` | 100% | Direct use |
| Tax Rate | Calculate from `income_tax_fq` / `pretax_income_fq` | 100% | Calculate |
| Cost of Debt | `get_fundamentals` → `interest_expense_on_debt_fq` | 90% | Prompt if missing |
| Credit Rating | `get_credit_ratings` | 90% | Prompt for outlook |
| Risk-Free Rate | User input | 0% | **PROMPT USER** |
| Market Risk Premium | User input | 0% | **PROMPT USER** |
| Terminal Growth Rate | User assumption | 0% | **PROMPT USER** |

**Coverage: 92%**

### Comps Analysis Data Mapping

| Required Field | TradingView Source | Coverage | Gap Handling |
|---------------|-------------------|----------|--------------|
| Market Cap | `get_fundamentals` → `market_cap_basic` | 100% | Direct use |
| Enterprise Value | Calculate from `market_cap + net_debt` | 100% | Calculate |
| EV/EBITDA | `get_fundamentals` → `enterprise_value_ebitda_current` | 95% | Prompt if missing |
| P/E Ratio | `get_fundamentals` → `price_earnings_ttm` | 95% | Prompt if missing |
| P/B Ratio | `get_fundamentals` → `price_book_fq_h` | 95% | Prompt if missing |
| Revenue Growth | Calculate from historical revenue | 100% | Calculate |
| EBITDA Margin | Calculate from `ebitda / revenue` | 100% | Calculate |
| ROE | `get_fundamentals` → `return_on_equity_fy` | 90% | Prompt if missing |
| Revenue TTM | `get_fundamentals` → `total_revenue_ttm` | 100% | Direct use |
| EBITDA TTM | `get_fundamentals` → `ebitda_ttm` | 100% | Direct use |
| Net Income TTM | `get_fundamentals` → `net_income_ttm` | 100% | Direct use |

**Coverage: 95%**

### LBO Model Data Mapping

| Required Field | TradingView Source | Coverage | Gap Handling |
|---------------|-------------------|----------|--------------|
| Net Debt | `get_fundamentals` → `net_debt_fq` | 90% | Prompt if missing |
| EBITDA | `get_fundamentals` → `ebitda` | 100% | Direct use |
| Interest Coverage | `get_fundamentals` → `ebitda_interst_cover_fy` | 85% | Prompt if missing |
| Debt/Equity | `get_fundamentals` → `debt_to_equity_current` | 90% | Prompt if missing |
| Free Cash Flow | `get_fundamentals` → `free_cash_flow_ttm` | 100% | Direct use |
| Debt Maturity | `get_debt_maturity` (SEC EDGAR) | 80% | Prompt if missing |
| Purchase Price | User input (LBO assumption) | 0% | **PROMPT USER** |
| Debt Financing Terms | User input | 0% | **PROMPT USER** |
| Exit Multiple | User assumption | 0% | **PROMPT USER** |

**Coverage: 75%**

### 3-Statement Model Data Mapping

| Required Field | TradingView Source | Coverage | Gap Handling |
|---------------|-------------------|----------|--------------|
| Revenue | `get_financial_statements` | 100% | Direct use |
| Cost of Goods | `get_fundamentals` → `cost_of_goods_fy_h` | 95% | Prompt if missing |
| Operating Expenses | `get_fundamentals` → `operating_expenses_fq` | 90% | Prompt if missing |
| Total Assets | `get_financial_statements` | 100% | Direct use |
| Total Liabilities | `get_financial_statements` | 100% | Direct use |
| Cash | `get_fundamentals` → `cash_n_short_term_invest_fq` | 100% | Direct use |
| Debt (Short + Long) | `get_fundamentals` | 100% | Direct use |
| CFO | `get_fundamentals` → `cash_f_operating_activities_fq_h` | 100% | Direct use |
| CapEx | `get_fundamentals` → `capital_expenditures_fq_h` | 100% | Direct use |
| Dividends | `get_financial_statements` or calendar | 90% | Prompt if missing |

**Coverage: 85%**

## Error Handling Patterns

### Pattern 1: Missing Data Response
```python
# When TradingView returns null/undefined for a field
def get_field_with_fallback(symbol, field, prompt_message):
    data = fetch_from_tradingview(symbol, field)
    if data is None or data == 0:
        # Prompt user for missing data
        return prompt_user(prompt_message)
    return data
```

### Pattern 2: Calculate Derived Metrics
```python
# When TradingView doesn't provide the exact metric
def calculate_working_capital(symbol):
    current_assets = fetch_fundamentals(symbol, "total_current_assets_fq")
    current_liabilities = fetch_fundamentals(symbol, "total_current_liabilities_fq")
    return current_assets - current_liabilities
```

### Pattern 3: Historical Data Handling
```python
# When fetching historical data
def get_historical_revenue(symbol, years=5):
    data = fetch_fundamentals(symbol, "total_revenue_fq_h")
    # TradingView returns array of historical values
    return data[-years:] if len(data) >= years else data
```

## User Prompt Templates

### Missing Risk-Free Rate
```markdown
## ⚠️ Data Gap: Risk-Free Rate

TradingView does not provide risk-free rate data. For DCF WACC calculation:

**Options:**
1. **Use 10-Year Treasury Yield**: ~4.2% (as of 2026)
2. **Use your company-specific rate**
3. **Skip and use default assumption** (4.0%)

**Enter your choice or custom value:**
```

### Missing Market Risk Premium
```markdown
## ⚠️ Data Gap: Market Risk Premium

TradingView does not provide market risk premium data.

**Standard Values:**
1. **Conservative**: 5.0%
2. **Moderate**: 5.5% (most common)
3. **Aggressive**: 6.0%

**Enter your choice or custom value:**
```

### Missing Terminal Growth Rate
```markdown
## ⚠️ Data Gap: Terminal Growth Rate

TradingView does not provide terminal growth assumptions.

**Guidelines:**
- **Conservative**: 2.0-2.5% (GDP growth)
- **Moderate**: 2.5-3.0%
- **Aggressive**: 3.0-3.5% (market leaders)

**Critical Constraint**: Must be LESS than WACC

**Enter your assumption:**
```

## Code Examples

### Example 1: Fetch DCF Inputs
```python
def fetch_dcf_inputs(symbol):
    """Fetch all required data for DCF model"""
    fundamentals = mcp_call("get_fundamentals", {"symbol": symbol})
    
    return {
        "revenue_history": fundamentals.get("total_revenue_fq_h", []),
        "ebitda": fundamentals.get("ebitda_fq"),
        "capex": fundamentals.get("capital_expenditures_fq_h", []),
        "depreciation": fundamentals.get("depreciation_fy_h", []),
        "beta": fundamentals.get("beta_3_year"),
        "market_cap": fundamentals.get("market_cap_basic"),
        "net_debt": fundamentals.get("net_debt_fq"),
        "tax_rate": calculate_tax_rate(fundamentals),
        # User prompts required for:
        # - risk_free_rate
        # - market_risk_premium
        # - terminal_growth
    }
```

### Example 2: Fetch Comps Data
```python
def fetch_comps_data(symbols):
    """Fetch data for comparable company analysis"""
    comps_data = []
    
    for symbol in symbols:
        fundamentals = mcp_call("get_fundamentals", {"symbol": symbol})
        quote = mcp_call("get_quote", {"symbol": symbol})
        
        comps_data.append({
            "symbol": symbol,
            "market_cap": quote.get("market_cap"),
            "revenue_ttm": fundamentals.get("total_revenue_ttm"),
            "ebitda_ttm": fundamentals.get("ebitda_ttm"),
            "net_income_ttm": fundamentals.get("net_income_ttm"),
            "pe_ratio": fundamentals.get("price_earnings_ttm"),
            "pb_ratio": fundamentals.get("price_book_fq_h"),
            "ev_ebitda": fundamentals.get("enterprise_value_ebitda_current"),
            "roe": fundamentals.get("return_on_equity_fy"),
        })
    
    return comps_data
```

### Example 3: Find Comparable Companies
```python
def find_comparable_companies(target_symbol):
    """Use screener to find comparable companies"""
    target = mcp_call("get_fundamentals", {"symbol": target_symbol})
    
    # Get sector and market cap range
    sector = target.get("sector")
    market_cap = target.get("market_cap_basic")
    
    # Screen for peers
    peers = mcp_call("scan_stocks", {
        "filters": {
            "sector": sector,
            "market_cap_min": market_cap * 0.3,
            "market_cap_max": market_cap * 3.0
        }
    })
    
    return peers
```

## Data Quality Checks

### Before Using Data:
1. **Verify non-zero values**: Check for 0 or null in critical fields
2. **Check date alignment**: Ensure historical data covers expected period
3. **Validate calculations**: Cross-check derived metrics (e.g., EV = Market Cap + Net Debt)
4. **Confirm currency**: All values should be in same currency

### Red Flags:
- Negative revenue or EBITDA (data error)
- Beta outside 0.2-3.0 range (unusual)
- Tax rate outside 15-35% (verify calculation)
- Terminal growth > WACC (infinite value error)

## Buyback Data Fields

TradingView provides several fields for buyback analysis and share count tracking:

### Buyback Fields

| Field | Description | Model Usage |
|-------|-------------|-------------|
| `buyback_yield` | Annual buyback yield as percentage of market cap | Total Shareholder Yield, Comps |
| `share_buyback_ratio_fq` | Quarterly share buyback ratio | LBO, cash flow analysis |
| `share_buyback_ratio_fy` | Annual share buyback ratio | LBO, capital allocation |

### Shares Outstanding Fields

| Field | Description | Model Usage |
|-------|-------------|-------------|
| `total_shares_outstanding` | Total shares outstanding | Comps, per-share metrics |
| `diluted_shares_outstanding_fq` | Diluted shares (quarterly) | EPS calculations, LBO |
| `diluted_shares_outstanding_fy` | Diluted shares (annual) | EPS calculations |
| `common_shares_outstanding` | Common shares outstanding | Balance sheet |
| `weighted_avg_shares_outstanding_fq` | Weighted avg shares (quarterly) | EPS |
| `weighted_avg_diluted_shares_fy` | Weighted avg diluted shares (annual) | EPS |

### Data Retrieval
```python
def fetch_buyback_data(symbol):
    fundamentals = mcp_call("get_fundamentals", {"symbol": symbol})
    return {
        "buyback_yield": fundamentals.get("buyback_yield"),
        "share_buyback_ratio_fq": fundamentals.get("share_buyback_ratio_fq"),
        "share_buyback_ratio_fy": fundamentals.get("share_buyback_ratio_fy"),
        "total_shares": fundamentals.get("total_shares_outstanding"),
        "diluted_shares_fq": fundamentals.get("diluted_shares_outstanding_fq"),
    }
```

## Normalized ROE Formula

The standard ROE calculation can be distorted by share buybacks that reduce equity. The normalized ROE adjusts for this by adding back the equity reduction from buybacks.

### Full Formula

```
Normalized Equity = total_equity_fq + (market_cap × buyback_yield / 100)
Normalized ROE = net_income_fy / Normalized Equity
```

Where:
- `total_equity_fq` = Total shareholders equity (quarterly)
- `market_cap` = Market capitalization
- `buyback_yield` = Annual buyback yield as percentage
- `net_income_fy` = Net income (annual)

### Simplified Formula

When market cap is not available, use shares outstanding:

```
Normalized Equity = total_equity_fq + (share_buyback_ratio_fy × market_cap)
Normalized ROE = net_income_fy / Normalized Equity
```

### Interpretation

- **Standard ROE**: May appear inflated if company reduces equity through buybacks
- **Normalized ROE**: Shows true return on actual capital employed, excluding buyback effects

Use normalized ROE when comparing companies with different capital return policies.

## Total Shareholder Yield

Total shareholder yield measures the total cash returned to shareholders through both dividends and buybacks, expressed as a percentage of market cap.

### Formula

```
Total Shareholder Yield = dividend_yield_recent + buyback_yield
```

### Components

| Component | Source Field | Description |
|-----------|--------------|-------------|
| Dividend Yield | `dividend_yield_recent` | Annual dividend yield |
| Buyback Yield | `buyback_yield` | Annual buyback yield |

### Example Calculation
```python
def calculate_total_shareholder_yield(symbol):
    fundamentals = mcp_call("get_fundamentals", {"symbol": symbol})
    dividend_yield = fundamentals.get("dividend_yield_recent", 0)
    buyback_yield = fundamentals.get("buyback_yield", 0)
    return dividend_yield + buyback_yield
```

### Usage in Models

- **Comps Analysis**: Compare total shareholder yield across peers
- **Valuation**: High yield may indicate undervaluation or special situation
- **Capital Allocation**: Assess company's total return policy

## Summary

TradingView MCP provides **92% coverage** for DCF, **95% for Comps**, **75% for LBO**, and **85% for 3-Statement models**. The remaining gaps require user input for:

1. **Macro assumptions**: Risk-free rate, market risk premium
2. **Model-specific assumptions**: Terminal growth, exit multiples
3. **Transaction details**: Purchase price, debt terms (LBO)

Always prompt users transparently when data gaps exist, providing context and standard values as options.
