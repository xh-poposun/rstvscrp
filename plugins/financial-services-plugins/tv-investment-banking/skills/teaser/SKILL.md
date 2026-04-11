---
name: teaser
description: "Draft anonymous one-page company teasers for sell-side M&A processes using TradingView data for public company targets. Creates a compelling summary without revealing the company's identity, designed to gauge buyer interest before NDA execution. Use when creating teasers for public companies with TradingView data integration."
allowed-tools: [docx, pptx]
---

# Teaser with TradingView Integration

## Overview

This skill drafts anonymous one-page company teasers for sell-side M&A processes, leveraging TradingView MCP for public company financial data. It creates compelling, anonymized summaries that generate buyer interest without revealing the company's identity.

## TradingView MCP Integration

### Data Sources for Teasers

| Teaser Section | TradingView Tool | Fields Used |
|----------------|------------------|-------------|
| Financial Summary | `get_fundamentals(symbol)` | Revenue, EBITDA, margins |
| Market Context | `get_quote(symbol)` | Market cap, sector |
| Growth Metrics | `get_fundamentals(symbol)` | Revenue CAGR, trends |
| Comparable Context | `scan_stocks(filters)` | Sector peers |

### Key TradingView Fields for Teasers

| Teaser Element | TradingView Field | Description |
|----------------|-------------------|-------------|
| **Financial Summary** | | |
| Revenue | `total_revenue_ttm` | Revenue (TTM) |
| Revenue Range | `total_revenue_ttm` | Round to range |
| EBITDA | `ebitda_ttm` | EBITDA (TTM) |
| EBITDA Margin | `ebitda_margin_ty` | EBITDA margin % |
| Employees | `number_of_employees` | Employee count |
| **Growth Metrics** | | |
| Revenue CAGR | `total_revenue_fq_h` | Calculate 3-5 year CAGR |
| Growth Trend | `total_revenue_fq_h` | Historical trajectory |
| **Market Context** | | |
| Sector | `sector` | Industry classification |
| Market Cap | `market_cap_basic` | For size context |
| **Anonymization** | | |
| Exact figures | N/A | Convert to ranges |
| Specific locations | N/A | Use regions only |

## Workflow

### Step 1: Gather Inputs

**From TradingView (Public Companies):**
- Company ticker symbol
- Current financial metrics
- Historical growth trends
- Sector classification

**From User (Required):**
- Company description (what they do, how they make money)
- Geographic footprint (region-level)
- Key selling points (3-5 highlights)
- Target buyer audience (strategic, financial, or both)
- Anonymization requirements

**User Prompt for Data Source:**
```markdown
## Teaser Data Source Selection

**Is the target company publicly traded?**

1. **Yes, public company** → I will fetch anonymized financial data from TradingView
   - Provide ticker symbol (e.g., "NASDAQ:AAPL", "NYSE:MSFT")
   - I will round figures to ranges for anonymity
   - TradingView coverage: ~80% for teaser financials

2. **No, private company** → Manual data entry required
   - Provide revenue/EBITDA ranges
   - Provide growth metrics

**Your selection:**
```

### Step 2: Fetch TradingView Data (Public Companies)

If public company, fetch:

1. **Fundamentals** via `get_fundamentals(symbol)`:
   - Revenue (TTM)
   - EBITDA (TTM)
   - EBITDA margin
   - Employee count
   - Historical revenue (3-5 years for CAGR)
   - Sector classification

2. **Market Data** via `get_quote(symbol)`:
   - Market cap (for size context)

### Step 3: Data Gap Analysis

**TradingView Coverage: 80% for Teasers**

| Data Needed | TradingView | Gap Handling |
|-------------|-------------|--------------|
| Revenue, EBITDA | ✓ 100% | Round to ranges |
| Employee count | ✓ 90% | Direct use or estimate |
| Historical growth | ✓ 100% | Calculate CAGR |
| **Business description** | ✗ 0% | **USER PROMPT** |
| **Geographic footprint** | ✗ 0% | **USER PROMPT** |
| **Investment highlights** | ✗ 0% | **USER PROMPT** |
| **Transaction structure** | ✗ 0% | **USER PROMPT** |
| **Target buyers** | ✗ 0% | **USER PROMPT** |

### Step 4: User Prompts for Data Gaps

#### Missing Data: Business Description

```markdown
## Missing Data: Business Description

TradingView provides financial data, not business descriptions.

**Please provide a 2-3 sentence description:**
- What the company does (without naming it)
- How they make money
- Key products/services
- Market position

**Example:**
"A leading provider of cloud-based enterprise software solutions serving Fortune 500 companies. Generates revenue through subscription fees and professional services."

**Your description:**
```

#### Missing Data: Geographic Footprint

```markdown
## Missing Data: Geographic Footprint

**Please specify geographic presence (region-level only for anonymity):**

- Primary region: [e.g., North America, Europe, Asia-Pacific]
- Secondary regions: [e.g., "Limited presence in Latin America"]
- Manufacturing/Distribution: [e.g., "Operations in 12 countries"]

**Note:** Do not specify cities or specific locations to maintain anonymity.

**Your input:**
```

#### Missing Data: Investment Highlights

```markdown
## Missing Data: Investment Highlights

**Please provide 4-6 key selling points:**

Common highlights include:
- Market leadership / positioning
- Revenue quality (recurring %, retention)
- Growth profile and trajectory
- Margin profile and expansion opportunity
- Management team strength
- Strategic value / synergy potential
- Technology/IP advantages
- Customer relationships

**Your highlights:**
1.
2.
3.
4.
5.
6.
```

#### Missing Data: Transaction Structure

```markdown
## Missing Data: Transaction Structure

**Please specify the transaction:**

- What's being offered: [100% sale / Majority stake / Minority growth equity]
- Indicative timeline: [e.g., "Process launching Q2 2026"]
- Contact for expressions of interest: [Banker contact]

**Your input:**
```

### Step 5: Anonymize TradingView Data

**Convert exact figures to ranges:**

| TradingView Data | Anonymized Range |
|------------------|------------------|
| Revenue: $52.3M | Revenue: $50-55M |
| EBITDA: $8.7M | EBITDA: $8-10M |
| EBITDA Margin: 16.6% | EBITDA Margin: 15-18% |
| Employees: 247 | Employees: 200-300 |
| Revenue CAGR: 23.4% | Revenue Growth: 20-25% CAGR |

**Anonymization Rules:**
- Revenue: Round to nearest $5M or $10M range
- EBITDA: Round to nearest $1M or $2M range
- Margins: Round to nearest 2-3% range
- Employees: Round to nearest 50 or 100
- Growth: Round to nearest 5% range

### Step 6: Teaser Structure

One page, professionally formatted:

**Header**
- Deal code name (e.g., "Project [Name]")
- Sector descriptor (e.g., "Leading Specialty Industrial Services Platform")
- "Confidential — For Discussion Purposes Only"

**Company Description** (2-3 sentences)
- What the company does, without naming it — User provided
- Market position (e.g., "a leading provider of...", "a top-3 player in...") — User provided
- Geography (region-level) — User provided

**Investment Highlights** (4-6 bullet points)
- Market leadership / positioning — User provided
- Revenue quality — User provided
- **Growth profile** — TradingView: CAGR rounded to range
- **Margin profile** — TradingView: EBITDA margin rounded
- Management team strength — User provided
- Strategic value — User provided

**Financial Summary** (table or key metrics)

| Metric | Value |
|--------|-------|
| Revenue | $XX-XXM (TradingView, rounded) |
| Revenue Growth | XX-XX% CAGR (TradingView, calculated) |
| EBITDA | $XX-XXM (TradingView, rounded) |
| EBITDA Margin | XX-XX% (TradingView, rounded) |
| Employees | XXX-XXX (TradingView, rounded) |

**Transaction Overview** (2-3 sentences)
- What's being offered — User provided
- Indicative timeline — User provided
- Contact information — User provided

### Step 7: Anonymization Check

Ensure the teaser doesn't inadvertently identify the company:

- ✗ No company name, brand names, or product names
- ✗ No specific city (use region: "Southeast US", "Midwest")
- ✗ No named customers or partners
- ✗ No employee count if it's too distinctive (use range)
- ✓ Revenue ranges instead of exact figures
- ✗ No logos, screenshots, or identifiable imagery
- ✗ No ticker symbol or exchange references

**Additional checks for TradingView data:**
- Round market cap to broad range (e.g., "$500M-1B") if mentioned
- Remove any TradingView-specific identifiers
- Ensure rounded figures don't precisely match public filings

### Step 8: Output

- Word document (.docx) — one page, clean formatting
- PDF version for distribution
- Optional PowerPoint version (single slide)
- **Footnote**: "Financial data sourced from TradingView MCP, rounded for anonymity"

## Data Fetching Examples

### Example 1: Fetch and Anonymize Teaser Data

```python
# Fetch data for teaser
symbol = "NASDAQ:AAPL"

# Fundamentals
fundamentals = mcp_call("get_fundamentals", {"symbol": symbol})

# Get key metrics
revenue = fundamentals.get("total_revenue_ttm")
ebitda = fundamentals.get("ebitda_ttm")
margin = fundamentals.get("ebitda_margin_ty")
employees = fundamentals.get("number_of_employees")

# Historical revenue for CAGR
revenue_history = fundamentals.get("total_revenue_fq_h", [])

# Calculate CAGR (3-year)
def calculate_cagr(values, years=3):
    if len(values) < years * 4:  # Quarterly data
        return None
    annual = [sum(values[i:i+4]) for i in range(0, len(values), 4)]
    if len(annual) < years + 1:
        return None
    beginning = annual[-years-1]
    ending = annual[-1]
    return (ending / beginning) ** (1/years) - 1

cagr = calculate_cagr(revenue_history, 3)

# Anonymize to ranges
def anonymize_revenue(value):
    if value >= 1_000_000_000:
        return f"${value/1_000_000_000:.0f}B+"
    elif value >= 100_000_000:
        lower = (value // 50_000_000) * 50_000_000
        upper = lower + 50_000_000
        return f"${lower/1_000_000:.0f}-{upper/1_000_000:.0f}M"
    else:
        lower = (value // 10_000_000) * 10_000_000
        upper = lower + 10_000_000
        return f"${lower/1_000_000:.0f}-{upper/1_000_000:.0f}M"

def anonymize_ebitda(value):
    lower = (value // 5_000_000) * 5_000_000
    upper = lower + 5_000_000
    return f"${lower/1_000_000:.0f}-{upper/1_000_000:.0f}M"

def anonymize_margin(value):
    lower = int((value // 2) * 2)
    upper = lower + 3
    return f"{lower}-{upper}%"

def anonymize_employees(value):
    if value >= 1000:
        lower = (value // 500) * 500
        upper = lower + 500
        return f"{lower:,}-{upper:,}"
    else:
        lower = (value // 50) * 50
        upper = lower + 50
        return f"{lower}-{upper}"

def anonymize_growth(value):
    lower = int((value // 5) * 5)
    upper = lower + 5
    return f"{lower}-{upper}%"

teaser_data = {
    "revenue_range": anonymize_revenue(revenue),
    "ebitda_range": anonymize_ebitda(ebitda),
    "margin_range": anonymize_margin(margin),
    "employees_range": anonymize_employees(employees),
    "growth_range": anonymize_growth(cagr * 100) if cagr else "Strong growth",
    "sector": fundamentals.get("sector"),
}
```

### Example 2: Generate Teaser Content

```python
# After fetching TradingView data and user inputs
teaser_content = {
    "project_name": "Project Alpha",  # User provided
    "sector_descriptor": f"Leading {teaser_data['sector']} Platform",  # TradingView + user
    "description": user_provided_description,
    "geography": user_provided_geography,
    "highlights": user_provided_highlights,
    "financials": {
        "revenue": teaser_data["revenue_range"],
        "growth": teaser_data["growth_range"],
        "ebitda": teaser_data["ebitda_range"],
        "margin": teaser_data["margin_range"],
        "employees": teaser_data["employees_range"],
    },
    "transaction": user_provided_transaction,
    "contact": user_provided_contact,
}
```

## Important Notes

- **TradingView Coverage**: 80% for teasers — financial metrics are available, but all qualitative data requires user input
- **Anonymization Critical**: TradingView provides exact figures — must round to ranges to maintain anonymity
- **Public Companies Only**: TradingView covers public equities — private company teasers require manual data entry
- **Less is More**: A good teaser makes buyers want to sign the NDA to learn more
- **Legal Review**: Always have client and legal review before distribution
- **Tracking**: Keep a log of who receives the teaser — becomes the outreach log

## Anti-Patterns

### ❌ Never Do These

1. **Use exact TradingView figures** — Always round to ranges for anonymity
2. **Include ticker symbols** — Never reference the public ticker
3. **Use web search for company info** — Risks revealing identity
4. **Skip anonymization check** — Verify no identifying details
5. **Omit TradingView citation** — Note data source in footnote

### ✅ Correct Patterns

1. **Round all financial figures** — Use ranges, not exact numbers
2. **Use region-level geography** — Never specific cities
3. **Fetch TradingView data first** — Then anonymize
4. **Prompt user for all qualitative data** — Description, highlights, transaction terms
5. **Verify anonymity** — Check that teaser doesn't reveal company identity

## Teaser Template

```
CONFIDENTIAL — FOR DISCUSSION PURPOSES ONLY

PROJECT [CODE NAME]
Leading [Sector] Platform

COMPANY DESCRIPTION
[2-3 sentences describing business without naming it]

INVESTMENT HIGHLIGHTS
• [Highlight 1]
• [Highlight 2]
• [Highlight 3]
• [Highlight 4]
• [Highlight 5]

FINANCIAL SUMMARY
Revenue:              $XX-XXM
Revenue Growth:       XX-XX% CAGR
EBITDA:               $XX-XXM
EBITDA Margin:        XX-XX%
Employees:            XXX-XXX

TRANSACTION OVERVIEW
[Brief description of transaction and timeline]

For more information, please contact:
[Banker name and contact]

---
Financial data sourced from TradingView MCP, rounded for anonymity.
```
