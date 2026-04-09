# DCF Analysis Example

This example demonstrates how to build a Discounted Cash Flow (DCF) model using the TradingView MCP server.

## Overview

A DCF model values a company based on its projected future cash flows, discounted back to present value. This workflow uses TradingView data for the key inputs.

## Step 1: Get Company Profile

First, understand the business you're valuing.

```json
{
  "symbol": "NASDAQ:AAPL"
}
```

**Tool:** `get_company_profile`

This returns sector, industry, and business description to inform your revenue growth assumptions.

## Step 2: Get Financial Statements

Fetch historical financial data to build your projections.

```json
{
  "symbol": "NASDAQ:AAPL",
  "statement_type": "income"
}
```

**Tool:** `get_financial_statements`

Key fields for DCF:

- `revenue_fy_h` - Historical annual revenue
- `ebitda_fy_h` - Historical EBITDA
- `operating_income_fy_h` - Historical EBIT
- `net_income_fy_h` - Historical net income
- `free_cash_flow_fy_h` - Historical FCF

## Step 3: Get Current Quote

Get the current stock price for comparison with your calculated intrinsic value.

```json
{
  "symbol": "NASDAQ:AAPL"
}
```

**Tool:** `get_quote`

## Step 4: Get Debt Maturity Schedule

Understand the company's debt obligations for the WACC calculation.

```json
{
  "symbol": "AAPL"
}
```

**Tool:** `get_debt_maturity`

This returns debt maturities by year from SEC EDGAR filings.

## Step 5: Build the DCF Model

### 5.1 Revenue Projections

Use historical revenue growth rates to project future revenue:

```
Year 1 Revenue = Historical Revenue * (1 + Growth Rate)
Year 2 Revenue = Year 1 Revenue * (1 + Growth Rate)
...
```

### 5.2 Calculate Unlevered Free Cash Flow

For each projected year:

```
EBIT = Revenue * EBIT Margin
Tax = EBIT * Tax Rate
NOPAT = EBIT - Tax
FCF = NOPAT + D&A - CapEx - Change in Working Capital
```

Use historical margins from the financial statements.

### 5.3 Calculate WACC

```
Cost of Equity = Risk-Free Rate + Beta * Equity Risk Premium
Cost of Debt = Interest Rate * (1 - Tax Rate)

WACC = (E/V * Cost of Equity) + (D/V * Cost of Debt)
```

Get beta from `get_fundamentals`.

### 5.4 Discount Cash Flows

```
PV of Year N FCF = FCF / (1 + WACC)^N
```

### 5.5 Terminal Value

```
Terminal Value = FCF_N * (1 + g) / (WACC - g)
PV of Terminal Value = Terminal Value / (1 + WACC)^N
```

Where `g` is the perpetual growth rate (typically 2-3%).

### 5.6 Enterprise Value and Equity Value

```
Enterprise Value = Sum of PV of FCFs + PV of Terminal Value
Equity Value = Enterprise Value - Net Debt
Intrinsic Value per Share = Equity Value / Shares Outstanding
```

## Step 6: Sensitivity Analysis

Test different assumptions:

- Revenue growth rates (base, bull, bear cases)
- WACC (range +/- 2%)
- Terminal growth rate (1-4%)

## Complete Example Workflow

```
1. get_company_profile {"symbol": "NASDAQ:AAPL"}
2. get_financial_statements {"symbol": "NASDAQ:AAPL", "statement_type": "income"}
3. get_quote {"symbol": "NASDAQ:AAPL"}
4. get_fundamentals {"symbol": "NASDAQ:AAPL"}
5. get_debt_maturity {"symbol": "AAPL"}
6. Build Excel model with projected FCFs
7. Calculate WACC and discount rate
8. Compute enterprise and equity value
9. Compare intrinsic value to current price
```

## Output

Your DCF model should include:

- 5-10 year revenue and FCF projections
- WACC calculation with supporting assumptions
- Sensitivity table showing value range
- Upside/downside to current price
- Key risks and assumptions
