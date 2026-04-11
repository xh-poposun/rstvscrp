---
name: pitch-deck
description: "Populates investment banking pitch deck templates with data from TradingView MCP. Use when: user provides a PowerPoint template to fill in, user has a public company ticker to populate into slides, user mentions populating or filling a pitch deck template with market data, or user needs to transfer TradingView data into existing slide layouts. Not for creating presentations from scratch."
allowed-tools: [pptx]
---

# Populating Investment Banking Pitch Deck Templates with TradingView Data

## Overview

This skill populates pitch deck templates using real-time market data from TradingView MCP. It replaces manual data entry with automated fetching of company fundamentals, market data, and comparable company information.

## TradingView MCP Integration

### Data Sources

| Data Type | TradingView Tool | Fields Used |
|-----------|------------------|---------------|
| Current Market Data | `get_quote(symbol)` | price, change, volume, market_cap |
| Company Fundamentals | `get_fundamentals(symbol)` | revenue, EBITDA, margins, growth |
| Comparable Companies | `scan_stocks(filters)` | peer universe by sector/market cap |
| Financial History | `get_financial_statements(symbol)` | 3-5 year trends |

### Key TradingView Fields for Pitch Decks

| Pitch Deck Section | TradingView Field | Description |
|-------------------|-------------------|-------------|
| Company Overview | `market_cap_basic` | Market capitalization |
| | `price` | Current stock price |
| | `total_revenue_ttm` | Revenue (TTM) |
| Financial Summary | `ebitda_ttm` | EBITDA (TTM) |
| | `net_income_ttm` | Net income |
| | `gross_margin_ttm` | Gross margin % |
| | `ebitda_margin_ty` | EBITDA margin % |
| Growth Metrics | `total_revenue_fq_h` | Revenue history (CAGR calc) |
| | `eps_growth_ttm` | EPS growth |
| Valuation | `price_earnings_ttm` | P/E ratio |
| | `enterprise_value_ebitda_current` | EV/EBITDA |
| | `price_book_fq_h` | P/B ratio |
| Trading Stats | `beta_3_year` | Beta |
| | `dividend_yield_recent` | Dividend yield |
| | `buyback_yield` | Buyback yield |
| | **Total Shareholder Yield** | Combined dividend + buyback yield |

## Workflow Decision Tree

**What type of task is this?**

```
├─ Populating template with TradingView data for public company?
│  └─→ Follow "TradingView Data Population Workflow" below
│
├─ Populating empty template with source data (Excel/CSV)?
│  └─→ Use original pitch-deck skill from investment-banking plugin
│
├─ Editing existing populated slides?
│  └─→ Extract current content, modify, revalidate
│
└─ Fixing formatting issues on existing slides?
   └─→ See "Common Failures" table, apply targeted fixes
```

## TradingView Data Population Workflow

### Phase 1: Fetch TradingView Data

1. **Get company ticker** from user (e.g., "AAPL", "NASDAQ:MSFT")
2. **Fetch current market data** via `get_quote(symbol)`:
   - Current price, price change, volume
   - Market capitalization
3. **Fetch fundamentals** via `get_fundamentals(symbol)`:
   - Revenue, EBITDA, net income (TTM)
   - Margins and growth rates
   - Valuation multiples
4. **Fetch historical data** for trends:
   - 3-5 year revenue history
   - Margin trends
5. **Validate data quality**:
   - Check for null/undefined values
   - Verify non-zero critical fields
   - Confirm currency consistency

### Phase 2: Data Gap Analysis

**TradingView Coverage: 70% for IB pitch decks**

| Data Needed | TradingView Coverage | Gap Handling |
|-------------|---------------------|--------------|
| Market cap, price, volume | ✓ 100% | Direct use |
| Revenue, EBITDA, margins | ✓ 100% | Direct use |
| Valuation multiples | ✓ 95% | Direct use |
| Historical financials | ✓ 90% | Direct use |
| **M&A transaction comps** | ✗ 0% | **USER PROMPT** |
| **Precedent transactions** | ✗ 0% | **USER PROMPT** |
| **Deal rumors/news** | ✗ 0% | **USER PROMPT** |
| **Strategic rationale** | ✗ 0% | **USER PROMPT** |
| **Synergy estimates** | ✗ 0% | **USER PROMPT** |

### Phase 3: User Prompts for Data Gaps

#### Missing Data: M&A Transaction Comps

```markdown
## Missing Data: M&A Transaction Comps

TradingView provides trading multiples, not transaction multiples.

**For M&A analysis, please provide:**
- Recent comparable transactions (target, acquirer, date)
- Transaction multiples paid (EV/Revenue, EV/EBITDA)
- Deal terms (cash/stock mix, earnouts)
- Strategic vs financial buyer distinction

**Options:**
1. Enter transaction data manually
2. Use trading multiples as proxy (note in footnotes)
3. Skip M&A comps section

**Your selection:**
```

#### Missing Data: Precedent Transactions

```markdown
## Missing Data: Precedent Transactions

TradingView does not have M&A transaction history.

**Please provide precedent transaction data:**
- Target company name
- Transaction date
- Transaction value / EV
- Revenue/EBITDA at time of deal
- Strategic rationale

**Or select:**
1. I have precedent data to enter
2. Skip precedent transactions section
3. Use public comps only
```

#### Missing Data: Deal Rumors / Market Intelligence

```markdown
## Missing Data: Deal Rumors & Market Intelligence

TradingView does not provide M&A rumors or deal intelligence.

**For market context, please provide:**
- Known strategic interest in sector
- Recent deal activity rumors
- Competitive dynamics
- Regulatory considerations

**Or select:**
1. I have market intelligence to share
2. Skip market context section
3. Focus on company-specific data only
```

### Phase 4: Content Mapping

Map TradingView data to standard pitch deck sections:

| Slide Section | TradingView Data | Format |
|---------------|------------------|--------|
| **Cover** | Company name, ticker, date | Text |
| **Company Overview** | Market cap, sector, description | Bullets |
| **Financial Summary** | Revenue, EBITDA, margins | Table |
| **Historical Performance** | 3-5 year trends | Chart |
| **Valuation** | P/E, EV/EBITDA, P/B | Table |
| **Trading Statistics** | Beta, dividend yield, volume | Table |
| **Comparable Companies** | scan_stocks peers | Table |
| **M&A Analysis** | User-provided transactions | Table |
| **Investment Thesis** | User-provided rationale | Bullets |

### Phase 5: Template Population

Follow the same population workflow as the original pitch-deck skill:

1. **Remove placeholder boxes** — colored instruction boxes are guidance, not output format
2. **Populate with TradingView data** — use fetched fundamentals and market data
3. **Add user-provided data** — M&A comps, strategic rationale, synergies
4. **Apply formatting** — match template style, create actual table objects
5. **Add footnotes** — cite TradingView as data source

### Phase 6: Validate → Fix → Repeat

**Validation checklist:**
- [ ] All TradingView data populated correctly
- [ ] User-provided M&A data included
- [ ] Tables are actual objects (not pipe-separated text)
- [ ] Charts fill designated areas
- [ ] Text readable against backgrounds
- [ ] No placeholder formatting retained
- [ ] Footnotes cite TradingView sources

## Data Fetching Examples

### Example 1: Fetch Company Overview Data

```python
# Fetch data for pitch deck company overview
symbol = "NASDAQ:AAPL"

# Current market data
quote = mcp_call("get_quote", {"symbol": symbol})
# Returns: price, change, volume, market_cap

# Fundamentals
fundamentals = mcp_call("get_fundamentals", {"symbol": symbol})
# Returns: revenue, EBITDA, margins, multiples

overview_data = {
    "company_name": "Apple Inc.",
    "ticker": "AAPL",
    "market_cap": quote.get("market_cap"),
    "current_price": quote.get("price"),
    "price_change": quote.get("change_percent"),
    "revenue_ttm": fundamentals.get("total_revenue_ttm"),
    "ebitda_ttm": fundamentals.get("ebitda_ttm"),
    "ebitda_margin": fundamentals.get("ebitda_margin_ty"),
    "pe_ratio": fundamentals.get("price_earnings_ttm"),
    "ev_ebitda": fundamentals.get("enterprise_value_ebitda_current"),
}
```

### Example 2: Fetch Comparable Companies

```python
# Find comparable companies using scan_stocks
symbol = "NASDAQ:AAPL"
target = mcp_call("get_fundamentals", {"symbol": symbol})

# Get sector and market cap
sector = target.get("sector")
market_cap = target.get("market_cap_basic")

# Screen for peers (0.3x to 3x market cap)
peers = mcp_call("scan_stocks", {
    "filters": {
        "sector": sector,
        "market_cap_min": market_cap * 0.3,
        "market_cap_max": market_cap * 3.0
    }
})

# Fetch fundamentals for each peer
comps_data = []
for peer_symbol in peers[:10]:  # Top 10 peers
    peer_fund = mcp_call("get_fundamentals", {"symbol": peer_symbol})
    comps_data.append({
        "symbol": peer_symbol,
        "market_cap": peer_fund.get("market_cap_basic"),
        "revenue": peer_fund.get("total_revenue_ttm"),
        "ebitda": peer_fund.get("ebitda_ttm"),
        "ev_ebitda": peer_fund.get("enterprise_value_ebitda_current"),
        "pe": peer_fund.get("price_earnings_ttm"),
    })
```

### Example 3: Fetch Historical Trends

```python
# Fetch 5-year revenue history for trend chart
symbol = "NASDAQ:AAPL"
fundamentals = mcp_call("get_fundamentals", {"symbol": symbol})

revenue_history = fundamentals.get("total_revenue_fq_h", [])
# Returns array of quarterly revenue figures

# Calculate CAGR
def calculate_cagr(values, years):
    if len(values) < 2:
        return None
    beginning = values[0]
    ending = values[-1]
    return (ending / beginning) ** (1/years) - 1

revenue_cagr = calculate_cagr(revenue_history, 5)
```

## Footnote Format

**TradingView Data Citation:**
```
Sources: TradingView MCP (get_quote, get_fundamentals), [Date].
Notes: (1) TTM figures as of [date]; (2) M&A transaction data provided by user.
```

## Anti-Patterns

### ❌ Never Do These

1. **Use web search as primary data source** — TradingView MCP is the primary source
2. **Skip user prompts for M&A data** — TradingView has no transaction data
3. **Hardcode computed values** — Use formulas for all calculations
4. **Use trading multiples as transaction multiples without disclosure** — Always note when using proxies
5. **Omit TradingView citations** — All data must be sourced

### ✅ Correct Patterns

1. **Always fetch from TradingView first** — For public companies, use MCP tools
2. **Prompt user for M&A gaps** — Transaction comps require user input
3. **Use formulas for calculations** — CAGR, growth rates, margins
4. **Disclose data limitations** — Note when using trading multiples as proxies
5. **Cite TradingView sources** — Include in footnotes

## Common Failures

| Failure | Solution |
|---------|----------|
| TradingView returns null for field | Prompt user for manual entry |
| No comparable companies found | Expand filter criteria or prompt user |
| Historical data incomplete | Use available data, note date range |
| M&A section empty | Remind user to provide transaction data |
| Currency mismatch | Convert all to single currency |

## Final Quality Checklist

### Data Accuracy
- [ ] All TradingView figures match fetched data
- [ ] User-provided M&A data included
- [ ] Calculated values (CAGR, margins) verified
- [ ] Same figures identical across all slides

### Content Mapping
- [ ] Every template section populated
- [ ] No `[bracket]` placeholder text remaining
- [ ] TradingView citations in footnotes
- [ ] M&A data gaps disclosed

### Formatting
- [ ] Text readable against backgrounds
- [ ] Tables are actual table objects
- [ ] Charts fill designated areas
- [ ] No placeholder boxes retained

### Data Source Compliance
- [ ] TradingView MCP used for all public company data
- [ ] User prompts for M&A transaction data
- [ ] Footnotes cite TradingView sources
- [ ] No paid MCP server references

## Important Notes

- **TradingView Coverage**: 70% for IB workflows — market data and fundamentals are complete, but M&A transaction data requires user input
- **Data Freshness**: TradingView provides real-time data — note the fetch date in footnotes
- **M&A Limitations**: TradingView has no transaction multiples, deal terms, or rumor intelligence — always prompt user
- **Public Companies Only**: TradingView covers public equities — private company pitches require manual data entry
- **LibreOffice Validation**: Same validation caveats as original pitch-deck skill apply
- **Buyback Presentation**: When presenting to buyers, highlight `buyback_yield` alongside `dividend_yield` to show total capital return to shareholders — Total Shareholder Yield = dividend + buyback provides a more complete picture of shareholder value creation
