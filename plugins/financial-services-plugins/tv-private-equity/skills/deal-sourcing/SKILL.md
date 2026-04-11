---
name: deal-sourcing
description: PE deal sourcing workflow using TradingView data — discover public target companies via stock screening, analyze fundamentals, and prepare investment theses. Use when sourcing public company targets, screening for take-private opportunities, or evaluating public companies as proxies for private deals. Triggers on "find public targets", "screen for take-private", "public company sourcing", or "analyze public comparable".
---

# Deal Sourcing with TradingView

## Overview

This skill uses TradingView MCP tools to identify and analyze public company targets for private equity deal sourcing. It replaces paid data sources (PitchBook, Capital IQ) with TradingView's stock screening and fundamental data.

**Coverage**: TradingView provides 100% coverage for public company screening and 90% for fundamental analysis. Private company data requires user input.

## Workflow

### Step 1: Define Screening Criteria

Collect from the user:
- **Sector/industry focus**: Specific sectors or sub-sectors (e.g., "B2B SaaS", "industrial services")
- **Market cap range**: For take-private targets (e.g., $500M - $5B)
- **Financial criteria**: Revenue, EBITDA, margins, growth rates
- **Geography**: US-only, international, or specific exchanges
- **Ownership structure**: Already public (take-private), or public as proxy for private

### Step 2: Screen Public Companies

Use `scan_stocks` to find matching public targets:

**Available Filters:**
- `market_cap_min` / `market_cap_max`: Market capitalization range
- `sector`: Industry sector (e.g., "Technology", "Healthcare", "Industrials")
- `exchange`: Stock exchange (e.g., "NASDAQ", "NYSE")
- `ebitda_margin_min` / `ebitda_margin_max`: EBITDA margin thresholds
- `revenue_growth_min`: Minimum revenue growth rate
- `pe_min` / `pe_max`: P/E ratio range

**Example Screening Call:**
```json
{
  "filters": {
    "sector": "Technology",
    "market_cap_min": 500000000,
    "market_cap_max": 5000000000,
    "ebitda_margin_min": 0.15,
    "exchange": "NASDAQ"
  }
}
```

### Step 3: Analyze Fundamentals

For each screened company, use `get_fundamentals` to retrieve:

**Key Fields for PE Analysis:**
| Field | Description | Usage |
|-------|-------------|-------|
| `total_revenue_ttm` | Revenue (TTM) | Size assessment |
| `ebitda_ttm` | EBITDA (TTM) | Profitability |
| `ebitda_margin` | EBITDA margin | Efficiency |
| `market_cap_basic` | Market cap | Valuation |
| `enterprise_value_ebitda_current` | EV/EBITDA | Entry multiple |
| `net_debt_fq` | Net debt | Capital structure |
| `free_cash_flow_ttm` | FCF (TTM) | Cash generation |
| `return_on_equity_fy` | ROE | Returns |
| `normalized_roe_fy` | Normalized ROE | Cyclical-adjusted returns |
| `buyback_yield` | Buyback yield | Capital return |
| `total_revenue_fq_h` | Revenue history | Growth trend |

### Step 4: Calculate Take-Private Metrics

**Enterprise Value Calculation:**
```
EV = Market Cap + Net Debt
```

**Implied Entry Multiple:**
```
EV/EBITDA = Enterprise Value / EBITDA
```

**Premium Analysis:**
- Current trading multiple vs. sector median
- Historical trading range (if available)
- Buyback yield as signal of capital return capacity
- **Total Shareholder Yield** (`dividend_yield + buyback_yield`) as screening criterion — targets with high total yield may be under-levered or returning excess capital rather than reinvesting
- High buyback yield may indicate management believes stock is undervalued — potential take-private signal
- Potential take-private premium (typically 20-40%)

### Step 5: Private Company Data Prompt

**TradingView Limitation**: TradingView only provides public company data.

**For Private Targets:**

```markdown
## Private Company Data Required

TradingView provides public company data. For private targets:

**Options:**
1. **Enter private company financials manually**
2. **Use public comparable as proxy**
3. **Combine both approaches**

**Required if private target:**
- Revenue (last 12 months)
- EBITDA and margin
- Growth rate assumptions
- Industry sector
- Ownership structure (founder-owned, PE-backed, etc.)

**Optional but helpful:**
- Customer concentration
- Management team background
- Competitive positioning
- Key risks
```

### Step 6: CRM Check

Before proceeding with any target:
- Search Gmail for prior correspondence with the company
- Search Slack for internal discussions
- Ask user: "Have you or your team evaluated [Company] before?"
- Flag any existing relationships or prior passes

### Step 7: Output Target Shortlist

Present findings in a structured format:

| Company | Ticker | Market Cap | Revenue | EBITDA | EV/EBITDA | FCF | Status |
|---------|--------|------------|---------|--------|-----------|-----|--------|
| [Name] | [SYM] | $XM | $XM | $XM | X.Xx | $XM | New/Existing |

**Include for each:**
- Business description (2-3 sentences)
- Why it fits the thesis
- Key strengths and concerns
- Suggested next steps

## Data Coverage Summary

| Data Element | TradingView Coverage | Gap Handling |
|--------------|---------------------|--------------|
| Public company screening | 100% | Direct use via `scan_stocks` |
| Market cap, price | 100% | `get_quote` |
| Revenue, EBITDA | 100% | `get_fundamentals` |
| Net debt, FCF | 90% | `get_fundamentals` |
| Historical financials | 100% | `get_fundamentals` |
| Private company data | 0% | **USER PROMPT REQUIRED** |
| Transaction multiples | 0% | **USER PROMPT REQUIRED** |
| Deal terms/structure | 0% | **USER PROMPT REQUIRED** |

## Important Notes

- **Public targets only**: TradingView screens public companies. For private targets, use public comparables as proxies or enter data manually.
- **Real-time data**: TradingView provides current market data. For historical analysis, use `total_revenue_fq_h` for revenue history.
- **No transaction data**: TradingView does not provide M&A transaction multiples or deal comps. Prompt user for this data.
- **Sector classification**: TradingView uses standard GICS sectors. Verify sector alignment with user's thesis.
- **Geographic coverage**: TradingView covers major US and international exchanges. Confirm coverage for specific markets.

## Example Interaction

**User**: "Find me take-private targets in industrial services with $500M-$2B market cap"

**Assistant**:
1. Calls `scan_stocks` with filters: sector="Industrials", market_cap_min=500M, market_cap_max=2B
2. For each result, calls `get_fundamentals` to get financials
3. Calculates EV/EBITDA and other key metrics
4. Checks Gmail/Slack for prior contact
5. Presents shortlist with investment thesis for each
6. For private targets, prompts for manual data entry

## Anti-Patterns

- ❌ Never use PitchBook or Capital IQ MCP references
- ❌ Never assume private company data is available via TradingView
- ❌ Never skip the private company data prompt for non-public targets
- ❌ Never present public company data as private company data
- ❌ Never skip CRM check before flagging targets as "New"
