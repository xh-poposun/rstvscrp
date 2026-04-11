---
name: cim-builder
description: "Structure and draft a Confidential Information Memorandum for sell-side M&A processes using TradingView data for public company targets. Organizes company information into a professional, investor-ready document with market data integration. Use when preparing sell-side materials for public companies, drafting a CIM with TradingView fundamentals, or organizing company data for a sale process."
allowed-tools: [docx, xlsx]
---

# CIM Builder with TradingView Integration

## Overview

This skill structures and drafts Confidential Information Memorandums (CIMs) for sell-side M&A processes, leveraging TradingView MCP for public company financial data. It combines automated data fetching with professional document formatting.

## TradingView MCP Integration

### Data Sources for CIMs

| CIM Section | TradingView Tool | Fields Used |
|-------------|------------------|-------------|
| Executive Summary - Financials | `get_fundamentals(symbol)` | Revenue, EBITDA, margins |
| Company Overview | `get_quote(symbol)` | Market cap, price, volume |
| Financial Overview | `get_financial_statements(symbol)` | 3-5 year statements |
| Industry Context | `scan_stocks(filters)` | Comparable universe |
| Valuation Context | `get_fundamentals(symbol)` | Trading multiples |

### Key TradingView Fields for CIMs

| CIM Section | TradingView Field | Description |
|-------------|-------------------|-------------|
| **Executive Summary** | | |
| Revenue (TTM) | `total_revenue_ttm` | Trailing twelve months revenue |
| EBITDA (TTM) | `ebitda_ttm` | Trailing twelve months EBITDA |
| EBITDA Margin | `ebitda_margin_ty` | EBITDA margin % |
| Revenue Growth | `total_revenue_fq_h` | Historical for CAGR |
| **Company Overview** | | |
| Market Cap | `market_cap_basic` | Current market capitalization |
| Stock Price | `price` | Current share price |
| Shares Outstanding | `shares_outstanding_fq` | Basic shares outstanding |
| Beta | `beta_3_year` | 3-year beta |
| Buyback Yield | `buyback_yield` | Share buyback yield % |
| Buyback Ratio | `share_buyback_ratio_fy` | Shares buyback ratio |
| **Financial Overview** | | |
| Revenue History | `total_revenue_fq_h` | 5-year quarterly revenue |
| Gross Profit | `gross_profit_fq` | Gross profit |
| Operating Income | `operating_income_fq` | Operating income |
| Net Income | `net_income_fq` | Net income |
| Total Assets | `total_assets_fq` | Balance sheet assets |
| Total Debt | `total_debt_fq` | Short + long-term debt |
| Cash | `cash_n_short_term_invest_fq` | Cash & equivalents |
| CapEx | `capital_expenditures_fq_h` | Capital expenditures |
| **Valuation Context** | | |
| P/E Ratio | `price_earnings_ttm` | Price-to-earnings |
| EV/EBITDA | `enterprise_value_ebitda_current` | Enterprise multiple |
| P/B Ratio | `price_book_fq_h` | Price-to-book |
| Dividend Yield | `dividend_yield_recent` | Dividend yield % |
| Buyback Yield | `buyback_yield` | Share buyback yield % |
| **Total Shareholder Yield** | Combined dividend + buyback yield | Total return to shareholders |

## Workflow

### Step 1: Gather Source Materials

**From TradingView (Public Companies):**
- Company ticker symbol
- Historical financials (3-5 years)
- Current market data
- Comparable company universe

**From User (Required):**
- Management presentations
- Budget/forecast (if available)
- Company website and marketing materials
- Customer data (anonymized)
- Org chart
- Quality of earnings report (if available)

**User Prompt for Data Source:**
```markdown
## CIM Data Source Selection

**Is the target company publicly traded?**

1. **Yes, public company** → I will fetch data from TradingView MCP
   - Provide ticker symbol (e.g., "NASDAQ:AAPL", "NYSE:MSFT")
   - TradingView coverage: ~85% for CIM financials

2. **No, private company** → Manual data entry required
   - Provide historical financials (Excel/CSV)
   - Provide management projections

**Your selection:**
```

### Step 2: Fetch TradingView Data (Public Companies)

If public company, fetch:

1. **Current Market Data** via `get_quote(symbol)`:
   - Market cap, stock price, volume
   
2. **Fundamentals** via `get_fundamentals(symbol)`:
   - Revenue, EBITDA, margins (TTM)
   - Historical revenue (5-year)
   - Balance sheet highlights
   - Valuation multiples

3. **Financial Statements** via `get_financial_statements(symbol, period="annual")`:
   - Income statement (5 years)
   - Balance sheet (5 years)
   - Cash flow statement (5 years)

4. **Comparable Universe** via `scan_stocks(filters)`:
   - Sector peers
   - Similar market cap range

### Step 3: Data Gap Analysis

**TradingView Coverage: 85% for CIMs**

| Data Needed | TradingView | Gap Handling |
|-------------|-------------|--------------|
| Historical financials (IS/BS/CF) | ✓ 100% | Direct use |
| Current market data | ✓ 100% | Direct use |
| Valuation multiples | ✓ 95% | Direct use |
| **Management projections** | ✗ 0% | **USER PROMPT** |
| **Customer concentration** | ✗ 0% | **USER PROMPT** |
| **Management team bios** | ✗ 0% | **USER PROMPT** |
| **Business description** | ✗ 0% | **USER PROMPT** |
| **Growth opportunities** | ✗ 0% | **USER PROMPT** |
| **Risk factors** | ✗ 0% | **USER PROMPT** |
| **Transaction structure** | ✗ 0% | **USER PROMPT** |

### Step 4: User Prompts for Data Gaps

#### Missing Data: Management Projections

```markdown
## Missing Data: Management Projections

TradingView provides historical data, not forward projections.

**Please provide management's financial projections:**
- Revenue forecast (Years 1-3 or 1-5)
- EBITDA margin targets
- CapEx requirements
- Working capital assumptions

**Or select:**
1. I have projections to provide
2. Use historical CAGR as proxy (note in disclaimer)
3. Exclude projections from CIM

**Your selection:**
```

#### Missing Data: Customer Information

```markdown
## Missing Data: Customer Concentration

TradingView does not provide customer-level data.

**Please provide customer information:**
- Number of customers
- Top customer concentration (% of revenue)
- Customer retention rates
- Geographic distribution

**Or select:**
1. I have customer data to provide
2. Include anonymized summary only
3. Exclude customer section

**Your selection:**
```

#### Missing Data: Management Team

```markdown
## Missing Data: Management Team

TradingView does not provide management biographies.

**Please provide management information:**
- Key executives (CEO, CFO, COO)
- Tenure with company
- Prior experience
- Equity ownership

**Or select:**
1. I have management bios to provide
2. Include names only (no bios)
3. Exclude management section

**Your selection:**
```

#### Missing Data: Transaction Structure

```markdown
## Missing Data: Transaction Structure

**Please specify the transaction parameters:**
- Percentage being sold (100%, majority, minority)
- Preferred consideration (cash, stock, mix)
- Indicative timeline
- Key deal terms (earnouts, escrows, etc.)

**Your input:**
```

### Step 5: CIM Structure

Standard CIM table of contents with TradingView data integration:

**I. Executive Summary** (2-3 pages)
- Company overview — TradingView sector + user description
- Investment highlights (5-7 key selling points) — User provided
- **Financial summary** — TradingView: Revenue, EBITDA, margins, growth, Total Shareholder Yield
- Transaction overview — User provided

**II. Company Overview** (3-5 pages)
- History and founding story — User provided
- Mission and value proposition — User provided
- Products and services description — User provided
- Business model — User provided
- **Key metrics** — TradingView: Market cap, employees, locations

**III. Industry Overview** (3-5 pages)
- **Market size** — User provided (TradingView has no TAM data)
- Key industry trends — User provided
- **Competitive landscape** — TradingView: Comparable companies from scan_stocks
- Regulatory environment — User provided
- Barriers to entry — User provided

**IV. Growth Opportunities** (2-3 pages)
- Organic growth levers — User provided
- M&A opportunities — User provided
- **Financial growth** — TradingView: Historical CAGR, trends

**V. Customers & Sales** (3-5 pages)
- Customer overview — User provided
- Top customer analysis — User provided (anonymized)
- **Revenue quality** — TradingView: Revenue consistency, growth

**VI. Operations** (2-3 pages)
- Organizational structure — User provided
- Key personnel — User provided
- **Capital intensity** — TradingView: CapEx trends, D&A

**VII. Financial Overview** (5-8 pages)
- **Historical income statement** — TradingView: 5-year via get_financial_statements
- Revenue analysis — TradingView: By segment if available
- **EBITDA bridge** — TradingView: Calculate from fundamentals
- **Balance sheet** — TradingView: 5-year history
- **Cash flow summary** — TradingView: CFO, CapEx, FCF
- **Capital structure** — TradingView: Debt, equity, cash
- Management forecast — User provided

**VIII. Valuation Context** (2-3 pages)
- **Trading multiples** — TradingView: P/E, EV/EBITDA, P/B
- **Capital allocation signal** — TradingView: `buyback_yield` + `dividend_yield_recent` = Total Shareholder Yield; high buyback yield signals management confidence and supports valuation premium
- **Comparable company analysis** — TradingView: scan_stocks peers
- **Historical trading range** — TradingView: Price history
- **Precedent transactions** — User provided (TradingView has no M&A data)

**IX. Appendix**
- Detailed financial statements — TradingView: Full statements
- Customer list — User provided (anonymized)
- Product catalog — User provided
- Management bios — User provided

### Step 6: Drafting Guidelines

- **Tone**: Professional, factual, compelling but not hyperbolic
- **Data-driven**: Support claims with TradingView data
  - "Strong growth" → "Revenue grew at a 15% CAGR from 2020-2024 (TradingView)"
- **Visuals**: Charts for financial trends from TradingView data
- **Length**: 40-60 pages total
- **Confidentiality**: Include disclaimer page
- **Source citations**: Footnote TradingView data sources

### Step 7: Output

- Word document (.docx) with professional formatting
- Separate Excel appendix with TradingView financials
- Charts and exhibits embedded
- **Footnote format**: "Source: TradingView MCP (get_fundamentals), [Date]"

## Data Fetching Examples

### Example 1: Fetch CIM Financial Summary

```python
# Fetch comprehensive data for CIM
symbol = "NASDAQ:AAPL"

# Current market data
quote = mcp_call("get_quote", {"symbol": symbol})

# Fundamentals
fundamentals = mcp_call("get_fundamentals", {"symbol": symbol})

# Financial statements (5-year annual)
income_stmt = mcp_call("get_financial_statements", {
    "symbol": symbol,
    "statement": "income",
    "period": "annual"
})

balance_sheet = mcp_call("get_financial_statements", {
    "symbol": symbol,
    "statement": "balance",
    "period": "annual"
})

cim_financials = {
    "market_cap": quote.get("market_cap"),
    "current_price": quote.get("price"),
    "revenue_ttm": fundamentals.get("total_revenue_ttm"),
    "ebitda_ttm": fundamentals.get("ebitda_ttm"),
    "ebitda_margin": fundamentals.get("ebitda_margin_ty"),
    "net_income": fundamentals.get("net_income_ttm"),
    "revenue_history": fundamentals.get("total_revenue_fq_h", []),
    "total_assets": fundamentals.get("total_assets_fq"),
    "total_debt": fundamentals.get("total_debt_fq"),
    "cash": fundamentals.get("cash_n_short_term_invest_fq"),
    "pe_ratio": fundamentals.get("price_earnings_ttm"),
    "ev_ebitda": fundamentals.get("enterprise_value_ebitda_current"),
}
```

### Example 2: Calculate Historical CAGR

```python
# Calculate revenue CAGR from TradingView history
revenue_history = fundamentals.get("total_revenue_fq_h", [])

# Get annual figures (last 5 years)
annual_revenue = revenue_history[::4][-5:]  # Every 4th quarter = annual

if len(annual_revenue) >= 2:
    beginning = annual_revenue[0]
    ending = annual_revenue[-1]
    years = len(annual_revenue) - 1
    cagr = (ending / beginning) ** (1/years) - 1
```

### Example 3: Fetch Comparable Universe

```python
# Get comparable companies for valuation context
symbol = "NASDAQ:AAPL"
target = mcp_call("get_fundamentals", {"symbol": symbol})

sector = target.get("sector")
market_cap = target.get("market_cap_basic")

# Find peers
peers = mcp_call("scan_stocks", {
    "filters": {
        "sector": sector,
        "market_cap_min": market_cap * 0.2,
        "market_cap_max": market_cap * 5.0
    }
})

# Fetch data for top 5-10 peers
peer_data = []
for peer in peers[:10]:
    peer_fund = mcp_call("get_fundamentals", {"symbol": peer})
    peer_data.append({
        "company": peer,
        "market_cap": peer_fund.get("market_cap_basic"),
        "revenue": peer_fund.get("total_revenue_ttm"),
        "ebitda": peer_fund.get("ebitda_ttm"),
        "ev_ebitda": peer_fund.get("enterprise_value_ebitda_current"),
        "pe": peer_fund.get("price_earnings_ttm"),
    })
```

## Important Notes

- **TradingView Coverage**: 85% for CIM financials — all historical statements and market data available, but management projections and qualitative data require user input
- **Data Freshness**: TradingView provides real-time market data and latest reported financials
- **Private Companies**: TradingView only covers public companies — private company CIMs require manual data entry
- **M&A Data**: TradingView has no transaction comps or M&A multiples — user must provide precedent transactions
- **Forward Projections**: Always include disclaimer: "Forward-looking statements based on management projections"
- **Legal Review**: Work with legal on confidentiality disclaimer and risk factors

## Anti-Patterns

### ❌ Never Do These

1. **Use web search as primary data source** — TradingView MCP is primary for public companies
2. **Skip user prompts for management data** — TradingView has no projections or customer data
3. **Present TradingView data as projections** — Historical only; label user projections separately
4. **Omit TradingView citations** — All financial data must be sourced
5. **Use stale data** — Fetch fresh data at CIM creation time

### ✅ Correct Patterns

1. **Fetch TradingView data first** — For public companies, use MCP tools
2. **Prompt user for gaps** — Management projections, customer data, transaction terms
3. **Clearly label data sources** — TradingView vs. user-provided
4. **Include disclaimers** — Forward projections, TradingView data timestamp
5. **Validate data** — Check for nulls, verify calculations
