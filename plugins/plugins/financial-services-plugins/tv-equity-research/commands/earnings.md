# /earnings

Create an earnings update report for a company using TradingView data.

## Usage

```
/earnings [ticker] [quarter] [year]
```

## Examples

```
/earnings AAPL Q4 2024
/earnings MSFT Q2 FY25
/earnings TSLA Q3 2024
```

## Description

Generates a professional equity research earnings update report (8-12 pages) using TradingView MCP data for financial metrics. The report includes:

- Beat/miss analysis using TradingView actuals vs consensus
- Key metric performance (revenue, EPS, EBITDA from get_fundamentals)
- Margin trend analysis
- Updated estimates and guidance
- Investment thesis impact

## Data Sources

**TradingView MCP:**
- `get_fundamentals` - Quarterly actuals (revenue, EPS, EBITDA)
- `get_earnings_calendar` - Consensus estimates
- `get_quote` - Current price, market cap
- `get_financial_statements` - Detailed line items

**User Input Required:**
- Earnings call transcript/key quotes
- Management guidance (current and prior)
- Segment performance details

## Output

- `[Company]_Q[Quarter]_[Year]_Earnings_Update.docx` (8-12 pages)
- 8-12 charts using TradingView data
- 1-3 summary tables

## See Also

- `earnings-analysis` skill for detailed workflow
- `model-update` skill for updating Excel models post-earnings
