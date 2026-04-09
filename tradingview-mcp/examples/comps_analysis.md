# Comps Analysis Example

This example demonstrates how to perform Comparable Company Analysis (Comps) using the TradingView MCP server.

## Overview

Comps analysis values a company by comparing it to similar publicly traded companies. You calculate valuation multiples for peers and apply them to your target company's metrics.

## Step 1: Define Peer Universe

Identify companies in the same sector with similar business models.

For a technology company like Apple (AAPL), peers might include:

- MSFT - Microsoft
- GOOGL - Alphabet
- META - Meta Platforms
- AMZN - Amazon
- NVDA - NVIDIA

## Step 2: Screen for Peers

Use the stock screener to find additional peers.

```json
{
  "market": "america",
  "sector": "Technology",
  "market_cap_min": 100000000000,
  "limit": 20
}
```

**Tool:** `scan_stocks`

## Step 3: Get Fundamentals for Each Peer

Fetch key valuation metrics for each peer company.

```json
{
  "symbol": "NASDAQ:MSFT"
}
```

**Tool:** `get_fundamentals`

Repeat for all peers: MSFT, GOOGL, META, AMZN, NVDA

Key metrics to collect:

- `market_cap` - Market capitalization
- `pe_ratio` - Price-to-Earnings ratio
- `price_to_book` - Price-to-Book ratio
- `beta` - Beta for risk comparison

## Step 4: Get Financial Statements

Fetch detailed financial data to calculate enterprise value multiples.

```json
{
  "symbol": "NASDAQ:MSFT",
  "statement_type": "income"
}
```

**Tool:** `get_financial_statements`

Key fields for EV calculations:

- `revenue_ttm` - Trailing twelve months revenue
- `ebitda_ttm` - TTM EBITDA
- `operating_income_ttm` - TTM EBIT
- `net_income_ttm` - TTM Net Income
- `total_liabilities_fq` - Total liabilities
- `cash_fq` - Cash and equivalents

## Step 5: Calculate Enterprise Value

For each peer:

```
Enterprise Value = Market Cap + Total Debt - Cash
```

From the fundamentals data:

```
EV = market_cap + (long_term_debt_fq + short_term_debt_fq) - cash_fq
```

## Step 6: Calculate Valuation Multiples

For each peer, calculate:

| Multiple | Formula |
|----------|---------|
| P/E | Market Cap / Net Income |
| EV/Revenue | EV / Revenue |
| EV/EBITDA | EV / EBITDA |
| EV/EBIT | EV / Operating Income |
| P/B | Market Cap / Book Value |

## Step 7: Build the Comps Table

Create a comparison table with all peers:

| Company | Market Cap | P/E | EV/Revenue | EV/EBITDA | EV/EBIT |
|---------|------------|-----|------------|-----------|---------|
| AAPL | $3.0T | 28x | 7.5x | 22x | 25x |
| MSFT | $2.8T | 32x | 11x | 24x | 28x |
| GOOGL | $1.7T | 24x | 5.5x | 18x | 21x |
| META | $1.2T | 26x | 8x | 16x | 19x |
| AMZN | $1.5T | 58x | 2.5x | 12x | 35x |
| NVDA | $2.0T | 65x | 35x | 55x | 65x |

## Step 8: Calculate Statistics

Compute summary statistics across the peer set:

| Metric | Mean | Median | Low | High |
|--------|------|--------|-----|------|
| P/E | 39x | 28x | 24x | 65x |
| EV/Revenue | 12x | 7.5x | 2.5x | 35x |
| EV/EBITDA | 25x | 22x | 12x | 55x |
| EV/EBIT | 32x | 25x | 19x | 65x |

## Step 9: Apply Multiples to Target

Get the target company's financials:

```json
{
  "symbol": "NASDAQ:AAPL"
}
```

**Tool:** `get_financial_statements`

Apply median multiples to calculate implied valuation:

```
Implied Equity Value (P/E) = Net Income * Median P/E
Implied EV (EV/Revenue) = Revenue * Median EV/Revenue
Implied EV (EV/EBITDA) = EBITDA * Median EV/EBITDA
Implied EV (EV/EBIT) = EBIT * Median EV/EBIT
```

## Step 10: Calculate Implied Share Price

Convert EV to equity value:

```
Implied Equity Value = Implied EV - Net Debt
Implied Share Price = Implied Equity Value / Shares Outstanding
```

Compare to current price from `get_quote`.

## Step 11: Premium/Discount Analysis

Calculate where the target trades relative to peers:

```
Premium to Median = (Target Multiple / Median Multiple) - 1
```

Example:

- If AAPL trades at 28x P/E and median is 28x, it's at parity
- If AAPL trades at 35x P/E and median is 28x, it's at 25% premium

## Complete Example Workflow

```
1. Define peer universe (5-10 companies)
2. scan_stocks {"market": "america", "sector": "Technology"}
3. For each peer:
   - get_fundamentals {"symbol": "NASDAQ:PEER"}
   - get_financial_statements {"symbol": "NASDAQ:PEER"}
4. Calculate EV for each peer
5. Calculate multiples (P/E, EV/Revenue, EV/EBITDA, EV/EBIT)
6. Build comps table with statistics
7. get_financial_statements {"symbol": "NASDAQ:AAPL"}
8. Apply median multiples to AAPL's metrics
9. Calculate implied share price
10. get_quote {"symbol": "NASDAQ:AAPL"}
11. Compare implied price to current price
12. Calculate premium/discount to peer group
```

## Output

Your comps analysis should include:

- Peer universe with rationale
- Comps table with all multiples
- Statistical summary (mean, median, range)
- Implied valuation range
- Premium/discount analysis
- Key drivers of valuation differences
- Investment recommendation

## Tips

- Use median instead of mean to avoid outlier distortion
- Exclude companies with negative earnings from P/E calculations
- Consider growth-adjusted multiples (PEG ratio)
- Add a football field chart showing valuation range
