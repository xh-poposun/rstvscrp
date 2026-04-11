---
name: morning-note
description: Draft concise morning meeting notes using TradingView market data. Summarizes overnight developments, earnings results, and key events for coverage stocks using real-time quotes and earnings calendar. Designed for the 7am morning meeting format — tight, opinionated, actionable. Triggers on "morning note", "morning meeting", "what happened overnight", "trade idea", "morning call prep", or "daily note".
allowed-tools: [docx]
---

# Morning Note (TradingView Edition)

Draft concise morning meeting notes using TradingView MCP data for real-time market information. Summarizes overnight developments, earnings results, and key events for coverage stocks.

## TradingView MCP Data Sources

### Primary Tools
| Tool | Purpose | Coverage |
|------|---------|----------|
| `get_quote` | Real-time prices, pre-market moves | 100% |
| `get_earnings_calendar` | Earnings results, consensus vs actual | 90% |
| `get_fundamentals` | Key metrics, valuation | 85% |
| `scan_stocks` | Sector performance, screeners | 100% |

### Key TradingView Fields for Morning Notes

**Market Data (get_quote):**
- Current Price: `price`
- Pre-market/After-hours: `premarket_price`, `postmarket_price`
- Change: `change`, `change_percent`
- Volume: `volume`
- Market Cap: `market_cap_basic`

**Earnings Data (get_earnings_calendar):**
- Reported companies
- Consensus EPS vs Actual
- Consensus Revenue vs Actual
- Report date
- Surprise %

**Fundamentals (get_fundamentals):**
- P/E: `price_earnings_ttm`
- EV/EBITDA: `enterprise_value_ebitda_current`
- Buyback Yield: `buyback_yield`
- Key metrics for quick reference

## Workflow

### Step 1: Fetch Overnight Data

```python
# Get pre-market moves for coverage universe
get_quote(symbols=["NASDAQ:AAPL", "NASDAQ:MSFT", "NASDAQ:GOOGL", ...])

# Check for earnings reports
get_earnings_calendar(
    symbols=["NASDAQ:AAPL", "NASDAQ:MSFT", ...],
    from_date="2024-01-15",
    to_date="2024-01-16"
)

# Get sector ETF performance
get_quote(symbols=["AMEX:XLK", "AMEX:XLF", "AMEX:XLE", ...])
```

### Step 2: Overnight Developments

**Earnings & Guidance (from TradingView):**
- Any coverage companies reporting overnight or pre-market?
- Earnings surprises (beat/miss on revenue, EPS from get_earnings_calendar)
- Guidance changes (user-provided)

**Market Context (from TradingView):**
- Pre-market moves (get_quote change_percent)
- Sector ETF performance
- Volume spikes

**News & Events (user-provided):**
- M&A announcements or rumors
- Management changes
- Product launches or regulatory decisions
- Analyst upgrades/downgrades from competitors
- Macro data or policy changes affecting the sector

### Step 3: Morning Note Format

Keep it tight — a morning note should be readable in 2 minutes:

---

**[Date] Morning Note — [Analyst Name]**
**[Sector Coverage]**

**Top Call: [Headline — the one thing PMs need to hear]**
- 2-3 sentences on the key development and why it matters
- Stock impact: price target, rating reiteration/change

**Overnight/Pre-Market Developments**
- [Company A]: [TradingView: +X% pre-market] One-line summary of earnings/news + our take
- [Company B]: [TradingView: -Y% pre-market] One-line summary + our take
- [Sector/Macro]: [TradingView: ETF performance] Relevant sector-wide development

**Key Events Today**
- [Time]: [Company] earnings call
- [Time]: Economic data release (expectations vs. our view)
- [Time]: Conference or investor day

**Trade Ideas** (if any)
- [Long/Short] [Company]: 1-2 sentence thesis + catalyst
- Risk: What would make this wrong

**Buyback Activity** (if material)
- [Company]: Buyback yield [buyback_yield]% + Dividend yield → Total Shareholder Yield [sum]%
- Share count change QoQ: [total_shares_outstanding] → notable if buyback accelerated or paused

---

### Step 4: Quick Takes on Earnings

If a coverage company reported (from TradingView earnings calendar):

| Metric | Consensus | Actual | Beat/Miss |
|--------|-----------|--------|-----------|
| Revenue | [from earnings calendar] | [from earnings calendar] | +/- X% |
| EPS | [from earnings calendar] | [from earnings calendar] | +/- Y% |
| Stock Reaction | [from get_quote change_percent] | | |

**Our Take**: 2-3 sentences — is this good or bad for the stock? Does it change our thesis?

**Action**: Maintain / Upgrade / Downgrade rating? Adjust price target?

### Step 5: Output

- Markdown text for email/Slack distribution
- Word document if formal distribution is needed
- Keep to 1 page max — PMs and traders won't read more

## TradingView Data Integration

### Pre-Market Movers

```python
# Fetch coverage universe quotes
quotes = get_quote(symbols=coverage_universe)

# Identify movers > 2%
movers = [q for q in quotes if abs(q['change_percent']) > 2]

# Sort by magnitude
movers.sort(key=lambda x: abs(x['change_percent']), reverse=True)
```

### Earnings Summary

```python
# Fetch earnings for coverage universe
earnings = get_earnings_calendar(
    symbols=coverage_universe,
    from_date=yesterday,
    to_date=today
)

# Calculate surprises
for e in earnings:
    eps_surprise = (e['actual_eps'] - e['consensus_eps']) / e['consensus_eps']
    rev_surprise = (e['actual_revenue'] - e['consensus_revenue']) / e['consensus_revenue']
```

### Sector Performance

```python
# Major sector ETFs
sector_etfs = {
    "Technology": "AMEX:XLK",
    "Financials": "AMEX:XLF",
    "Energy": "AMEX:XLE",
    "Healthcare": "AMEX:XLV",
    "Consumer": "AMEX:XLY",
    "Industrials": "AMEX:XLI"
}

etf_quotes = get_quote(symbols=list(sector_etfs.values()))
```

## User Prompts for Data Gaps

TradingView provides market data but NOT qualitative news. Prompt users for:

### Missing Data: News Context

```markdown
## Missing Data: Overnight News

TradingView provides price data but not news context. Please provide:

**Key Overnight Developments:**
- Earnings call highlights (if any)
- Management guidance changes
- M&A announcements
- Analyst upgrades/downgrades
- Macro events

**Options:**
1. Paste key headlines
2. Provide bullet points
3. Focus on price action only
```

### Missing Data: Trade Ideas

```markdown
## Missing Data: Trade Ideas

Do you have any trade ideas for today's morning note?

**Trade Idea Format:**
- Direction: Long/Short
- Ticker: [Symbol]
- Thesis: 1-2 sentences
- Catalyst: What drives the trade
- Risk: What would make this wrong
- Time horizon: Day trade / Swing / Position

**Options:**
1. Provide trade ideas
2. Skip trade ideas section
```

## Important Notes

- Be opinionated — morning notes that just summarize data without a view are useless
- Lead with the most important thing — don't bury the headline
- "No news" is a valid morning note — say "nothing material overnight, maintaining positioning"
- Distinguish between actionable events (earnings, M&A) and noise (minor price moves)
- Time-stamp your takes — if you're writing at 6am, note that pre-market may change by open
- If you're wrong, own it in the next morning note — credibility matters more than being right every time
- **TradingView provides the data, you provide the opinion**

## Anti-Patterns

- ❌ NEVER just list prices without context or opinion
- ❌ NEVER use web search for market data (use TradingView MCP)
- ❌ NEVER write more than 1 page — PMs won't read it
- ❌ NEVER bury the lead — most important item first
- ❌ NEVER skip user prompts for news context

## Dependencies

**Required:**
- TradingView MCP server (tvdata)
- Coverage universe list
- DOCX skill (optional)

**Data Flow:**
```
TradingView MCP → Market Data → Analysis → Opinion
User Input → News Context → Qualitative Commentary
```
