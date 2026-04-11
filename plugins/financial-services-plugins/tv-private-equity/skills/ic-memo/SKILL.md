---
name: ic-memo
description: Draft a structured investment committee memo for PE deal approval using TradingView data for public targets. Synthesizes due diligence findings, financial analysis from TradingView MCP, and deal terms into a professional IC-ready document. Use when preparing for investment committee on public or private targets, writing up a deal, or creating a formal recommendation. Triggers on "write IC memo", "investment committee memo", "deal write-up", "prepare IC materials", or "recommendation memo".
---

# Investment Committee Memo with TradingView

## Overview

This skill drafts IC memos using TradingView MCP data for public company targets. For private targets, it prompts users for required financial data while using TradingView for public comparable benchmarking.

**Coverage**: TradingView provides 90% coverage for public target analysis. Private targets require user input for financials.

## Workflow

### Step 1: Gather Inputs

**For Public Targets (TradingView Available):**

Use `get_fundamentals` to retrieve:
- Company overview and business description (supplement with web search if needed)
- Historical financials (3-5 years): `total_revenue_fq_h`, `ebitda_fq`
- Current metrics: `market_cap_basic`, `net_debt_fq`, `free_cash_flow_ttm`
- Market data: `get_quote` for current price, volume

**Key TradingView Fields for IC Memo:**
| Section | TradingView Fields |
|---------|-------------------|
| Company Overview | `get_quote` → description, sector |
| Financial Analysis | `total_revenue_fq_h`, `ebitda_fq`, `net_income_fq` |
| Capital Structure | `net_debt_fq`, `total_debt_fq`, `cash_n_short_term_invest_fq` |
| Market Context | `market_cap_basic`, `enterprise_value_ebitda_current` |
| Credit Profile | `get_credit_ratings` → Fitch, S&P, Moody's |

**For Private Targets (User Input Required):**

```markdown
## Private Target Data Required

TradingView provides public company data. For private targets, please provide:

**Required Financials:**
- Revenue (last 12 months and 3-year history)
- EBITDA and EBITDA margin
- Net income
- Total debt and cash
- Free cash flow

**Business Context:**
- Industry sector
- Geographic footprint
- Customer concentration
- Management team background

**Deal Terms:**
- Enterprise value or equity value
- Implied entry multiple
- Debt financing structure
- Equity contribution
```

### Step 2: Draft Memo Structure

Standard IC memo format with TradingView data integration:

**I. Executive Summary** (1 page)
- Company description
- Deal rationale and key terms
- Recommendation and headline returns
- Top 3 risks and mitigants

**II. Company Overview** (1-2 pages)
- Business description (supplement TradingView data with web research)
- Customer base and go-to-market
- Competitive positioning
- Management team

**III. Industry & Market** (1 page)
- Market size and growth (web search + TradingView sector data)
- Competitive landscape
- Secular trends / tailwinds
- Regulatory environment

**IV. Financial Analysis** (2-3 pages)

*For Public Targets - TradingView Data:*

| Metric | Source | Value |
|--------|--------|-------|
| Revenue (TTM) | `total_revenue_ttm` | $XM |
| Revenue Growth | Calculated from `total_revenue_fq_h` | X% |
| EBITDA (TTM) | `ebitda_ttm` | $XM |
| EBITDA Margin | Calculated | X% |
| Net Income | `net_income_ttm` | $XM |
| Free Cash Flow | `free_cash_flow_ttm` | $XM |
| Net Debt | `net_debt_fq` | $XM |

*Historical Financial Table (5 years):*
Build from `total_revenue_fq_h` and `ebitda_fq` arrays

*For Private Targets - User Data:*
- Present user-provided financials
- Note data source: "Provided by [User/Company]"

**V. Investment Thesis** (1 page)
- 3-5 investment pillars
- Value creation levers
- 100-day priorities

**VI. Deal Terms & Structure** (1 page)

*Prompt User for Deal Terms:*
```markdown
## Deal Terms Required

TradingView provides market data, not transaction terms. Please provide:

**Valuation:**
- Enterprise value: $___M
- Equity value: $___M
- Implied EV/EBITDA: ___x
- Premium to market (if public): ___%

**Capital Structure:**
- Senior debt: $___M
- Mezzanine/subordinated: $___M
- Equity contribution: $___M
- Total leverage: ___x EBITDA

**Key Terms:**
- Hold period: ___ years
- Management equity: ___%
- Board seats: ___
- Key covenants
```

**VII. Returns Analysis** (1 page)

*For Public Targets - Benchmarking via TradingView:*

Use `scan_stocks` to find comparable companies for exit multiple benchmarking:

```json
{
  "filters": {
    "sector": "[Target Sector]",
    "market_cap_min": [Target EV * 0.5],
    "market_cap_max": [Target EV * 2.0]
  }
}
```

Calculate sector median EV/EBITDA from peers for exit assumption.

*Returns Scenarios:*
- Base, upside, downside cases
- IRR and MOIC for each
- Sensitivity analysis

**VIII. Risk Factors** (1 page)
- Key risks ranked by severity
- Mitigants for each
- Deal-breaker risks

**IX. Recommendation**
- Clear recommendation: Proceed / Pass / Conditional proceed
- Key conditions or next steps

### Step 3: Public Comparable Analysis

For valuation context, use TradingView to analyze comparable public companies:

1. **Find Comparables:**
   - Use `scan_stocks` with sector and size filters
   - Retrieve 5-10 comparable companies

2. **Fetch Trading Multiples:**
   - `enterprise_value_ebitda_current`
   - `price_earnings_ttm`
   - `return_on_equity_fy`
   - `normalized_roe_fy`
   - `ebitda_margin` (calculated)

3. **Present Comparison:**

| Metric | Target | Peer Median | Peer Range |
|--------|--------|-------------|------------|
| EV/EBITDA | X.Xx | X.Xx | X.Xx - X.Xx |
| P/E | X.Xx | X.Xx | X.Xx - X.Xx |
| EBITDA Margin | X% | X% | X% - X% |
| ROE | X% | X% | X% - X% |
| Normalized ROE | X% | X% | X% - X% |

> **Normalized ROE Guidance**: Use Normalized ROE (`net_income_fy / (total_equity_fq + market_cap × buyback_yield/100)`) to understand if high ROE is driven by operational efficiency or financial engineering via share repurchases. A large gap between reported ROE and Normalized ROE signals buyback-inflated returns.

### Step 4: Output Format

- Default: Word document (.docx) with professional formatting
- Alternative: Markdown for quick review
- Include tables for financials and returns
- Source attribution: "Source: TradingView MCP, [Date]"

## Data Coverage Summary

| IC Memo Section | TradingView Coverage | Gap Handling |
|-----------------|---------------------|--------------|
| Company Overview | 60% | Supplement with web search |
| Financial Analysis (Public) | 100% | `get_fundamentals` |
| Financial Analysis (Private) | 0% | **USER PROMPT REQUIRED** |
| Market Context | 90% | `scan_stocks` for peers |
| Credit Ratings | 90% | `get_credit_ratings` |
| Deal Terms | 0% | **USER PROMPT REQUIRED** |
| Returns Analysis | 50% | User assumptions + TV benchmarking |

## User Prompts for Data Gaps

### Private Company Financials
```markdown
## Private Company Data

TradingView provides public company data. For private targets:

**Options:**
1. Enter private company financials manually
2. Use public comparable as proxy
3. Combine both approaches

**Required if private target:**
- Revenue (current and historical)
- EBITDA and margin
- Net income
- Total debt and cash
- Free cash flow
- Industry sector
```

### Transaction Multiples
```markdown
## Transaction Multiples

TradingView does not provide M&A transaction data. For deal comps:

**Options:**
1. Provide recent comparable transactions manually
2. Use public trading multiples as proxy
3. Reference sector transaction trends

**Typical PE Entry Multiples by Sector:**
- Software/SaaS: 12-20x EBITDA
- Industrials: 8-12x EBITDA
- Healthcare: 10-16x EBITDA
- Consumer: 6-10x EBITDA
```

### Deal Terms
```markdown
## Deal Structure

Please provide transaction details:

**Purchase Price:**
- Enterprise value: $___M
- Equity value: $___M
- Implied multiple: ___x EBITDA

**Financing:**
- Senior debt: $___M (___x EBITDA)
- Junior capital: $___M
- Equity contribution: $___M

**Key Terms:**
- Hold period: ___ years
- Management rollover: ___%
- Board composition
```

## Important Notes

- **IC memos should be factual and balanced** — present both bull and bear cases honestly
- **Don't minimize risks** — IC members will find them anyway; credibility matters
- **Use the firm's standard memo template** if the user provides one
- **Financial tables should tie** — check that EBITDA bridges, S&U balances, and returns math is consistent
- **Ask for missing inputs** rather than making assumptions on deal terms or returns
- **Source attribution**: Always note "TradingView MCP" for data sourced via MCP tools

## Anti-Patterns

- ❌ Never use PitchBook, Capital IQ, or other paid MCP references
- ❌ Never present TradingView public data as private company data
- ❌ Never skip user prompts for private target financials
- ❌ Never skip user prompts for deal terms and structure
- ❌ Never assume transaction multiples — always prompt user
- ❌ Never bury red flags in IC memos — be direct
- ❌ Never skip the bear case in investment thesis
