# /model-update

Update a financial model with new TradingView data.

## Usage

```
/model-update [ticker] [trigger]
```

## Examples

```
/model-update AAPL earnings
/model-update MSFT guidance
/model-update TSLA macro
```

## Description

Updates an existing financial model with new data from TradingView MCP. Handles:

- **Earnings updates** - Plug actuals from get_fundamentals
- **Guidance changes** - Update forward estimates
- **Macro updates** - Refresh assumptions
- **Event-driven** - M&A, restructuring, etc.

## Data Sources

**TradingView MCP:**
- `get_fundamentals` - Latest quarterly actuals
- `get_earnings_calendar` - Consensus for beat/miss
- `get_quote` - Current price for valuation
- `get_financial_statements` - Detailed updates

**User Input Required:**
- Updated management guidance
- Earnings call commentary
- Segment performance
- Revised assumptions

## Output

- Updated Excel model with TradingView citations
- Estimate change summary (markdown)
- Updated price target derivation

## See Also

- `model-update` skill for detailed workflow
- `earnings` command for full earnings reports
