# /morning-note

Create a morning meeting note using TradingView market data.

## Usage

```
/morning-note [date]
```

## Examples

```
/morning-note today
/morning-note 2024-01-15
```

## Description

Drafts a concise morning meeting note (1 page) summarizing overnight developments using TradingView real-time data.

**Includes:**
- Pre-market movers (from get_quote)
- Earnings results (from get_earnings_calendar)
- Sector performance (ETF quotes)
- Trade ideas
- Key events

## Data Sources

**TradingView MCP:**
- `get_quote` - Real-time prices, pre-market moves
- `get_earnings_calendar` - Earnings results, surprises
- `get_fundamentals` - Quick metrics
- `scan_stocks` - Sector screens

**User Input Required:**
- News context
- Earnings call highlights
- Trade ideas
- Analyst actions

## Output

- Morning note (markdown, 1 page)
- Optional: Word document for formal distribution

## See Also

- `morning-note` skill for detailed workflow
