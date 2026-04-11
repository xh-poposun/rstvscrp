---
name: comps-analysis
description: |
  Build institutional-grade comparable company analyses with operating metrics, valuation multiples, and statistical benchmarking in Excel/spreadsheet format using TradingView MCP data.

  **Perfect for:**
  - Public company valuation (M&A, investment analysis)
  - Benchmarking performance vs. industry peers
  - Pricing IPOs or funding rounds
  - Identifying valuation outliers (over/under-valued)
  - Supporting investment committee presentations
  - Creating sector overview reports

  **Not ideal for:**
  - Private companies without comparable public peers
  - Highly diversified conglomerates
  - Distressed/bankrupt companies
  - Pre-revenue startups
  - Companies with unique business models
---

# Comparable Company Analysis (TradingView MCP)

## ⚠️ CRITICAL: Data Source Priority (READ FIRST)

**ALWAYS follow this data source hierarchy:**

1. **FIRST: TradingView MCP** - Use `scan_stocks`, `get_fundamentals`, and `get_quote` for all financial data
2. **DO NOT use web search** if TradingView MCP is available
3. **ONLY if TradingView MCP is unavailable:** Then use Bloomberg Terminal, SEC EDGAR filings, or other institutional sources
4. **NEVER use web search as a primary data source** - it lacks the accuracy, audit trails, and reliability required for institutional-grade analysis

**Why this matters:** TradingView MCP provides verified, institutional-grade data with proper citations. Web search results can be outdated, inaccurate, or unreliable for financial analysis.

**TradingView Coverage:** 95% for standard comps metrics. Sector-specific metrics (ARR, Net Dollar Retention, etc.) require user input.

---

## Overview

This skill teaches Claude to build institutional-grade comparable company analyses using TradingView MCP as the sole data source. The output is a structured Excel/spreadsheet that enables informed investment decisions through peer comparison.

**Reference Material & Contextualization:**

An example comparable company analysis is provided in `examples/comps_example.xlsx`. When using this or other example files in this skill directory, use them intelligently:

**DO use examples for:**
- Understanding structural hierarchy (how sections flow)
- Grasping the level of rigor expected (statistical depth, documentation standards)
- Learning principles (clear headers, transparent formulas, audit trails)

**DO NOT use examples for:**
- Exact reproduction of format or metrics
- Copying layout without considering context
- Applying the same visual style regardless of audience

**ALWAYS ask yourself first:**
1. **"Do you have a preferred format or should I adapt the template style?"**
2. **"Who is the audience?"** (Investment committee, board presentation, quick reference, detailed memo)
3. **"What's the key question?"** (Valuation, growth analysis, competitive positioning, efficiency)
4. **"What's the context?"** (M&A evaluation, investment decision, sector benchmarking, performance review)

**Adapt based on specifics:**
- **Industry context**: Big tech mega-caps need different metrics than emerging SaaS startups
- **Sector-specific needs**: Add relevant metrics early (e.g., cloud ARR, enterprise customers, developer ecosystem for tech)
- **Company familiarity**: Well-known companies may need less background, more focus on delta analysis
- **Decision type**: M&A requires different emphasis than ongoing portfolio monitoring

**Core principle:** Use template principles (clear structure, statistical rigor, transparent formulas) but vary execution based on context. The goal is institutional-quality analysis, not institutional-looking templates.

User-provided examples and explicit preferences always take precedence over defaults.

## Core Philosophy

**"Build the right structure first, then let the data tell the story."**

Start with headers that force strategic thinking about what matters, input clean data from TradingView MCP, build transparent formulas, and let statistics emerge automatically. A good comp should be immediately readable by someone who didn't build it.

---

## ⚠️ CRITICAL: Formulas Over Hardcodes + Step-by-Step Verification

**Environment — Office JS vs Python:**
- **If running inside Excel (Office Add-in / Office JS):** Use Office JS directly (`Excel.run(async (context) => {...})`). Write formulas via `range.formulas = [["=E7/C7"]]`, not `range.values`. No separate recalc step — Excel handles it natively. Use `range.format.*` for colors/fonts.
- **If generating a standalone .xlsx file:** Use Python/openpyxl. Write `cell.value = "=E7/C7"` (formula string).
- Same principles either way — just translate the API calls.
- **Office JS merged cell pitfall:** Do NOT call `.merge()` then set `.values` on the merged range (throws `InvalidArgument` — range still reports its pre-merge dimensions). Instead write the value to the top-left cell alone, then merge + format the full range:
```js
ws.getRange("A1").values = [["TECHNOLOGY — COMPARABLE COMPANY ANALYSIS"]];
const hdr = ws.getRange("A1:H1");
hdr.merge();
hdr.format.fill.color = "#1F4E79";
hdr.format.font.color = "#FFFFFF";
hdr.format.font.bold = true;
```

**Formulas, not hardcodes:**
- Every derived value (margin, multiple, statistic) MUST be an Excel formula referencing input cells — never a pre-computed number pasted in
- When using Python/openpyxl to build the sheet: write `cell.value = "=E7/C7"` (formula string), NOT `cell.value = 0.687` (computed result)
- The only hardcoded values should be raw input data (revenue, EBITDA, share price, etc.) — and every one of those gets a cell comment with its source
- Why: the model must update automatically when an input changes. A hardcoded margin is a silent bug waiting to happen.

**Verify step-by-step with the user:**
- After setting up the structure → show the user the header layout before filling data
- After entering raw inputs → show the user the input block and confirm sources/periods before building formulas
- After building operating metrics formulas → show the calculated margins and sanity-check with the user before moving to valuation
- After building valuation multiples → show the multiples and confirm they look reasonable before adding statistics
- Do NOT build the entire sheet end-to-end and then present it — catch errors early by confirming each section

---

## TradingView MCP Data Fetching

### Step 1: Find Comparable Companies

Use `scan_stocks` to identify peer companies:

```python
# Example: Find tech peers for AAPL
peers = mcp_call("scan_stocks", {
    "filters": {
        "sector": "Technology",
        "market_cap_min": 100000000000,  # $100B min
        "market_cap_max": 3000000000000,  # $3T max
        "exchange": "NASDAQ"
    }
})
```

**Key Filters:**
- `sector` — Industry classification
- `market_cap_min/max` — Size range (typically 0.3x to 3x target)
- `exchange` — Exchange filter (NASDAQ, NYSE, etc.)
- `pe_min/max` — P/E ratio range
- `ebitda_margin_min/max` — EBITDA margin filters

### Step 2: Fetch Fundamentals for Each Peer

Use `get_fundamentals` to retrieve financial data:

```python
# Key fields for comps analysis
fundamentals = mcp_call("get_fundamentals", {
    "symbol": "NASDAQ:AAPL",
    "fields": [
        "market_cap_basic",           # Market capitalization
        "total_revenue_ttm",          # Revenue (TTM)
        "ebitda_ttm",                 # EBITDA (TTM)
        "net_income_ttm",             # Net income (TTM)
        "eps_ttm",                    # EPS (TTM)
        "price_earnings_ttm",         # P/E ratio
        "price_book_fq_h",            # P/B ratio
        "enterprise_value_ebitda_current",  # EV/EBITDA
        "return_on_equity_fy",        # ROE
        "total_debt_fq",              # Total debt
        "cash_n_short_term_invest_fq", # Cash & equivalents
        "net_debt_fq"                 # Net debt
    ]
})
```

### Step 3: Calculate Derived Metrics

**Enterprise Value Calculation:**
```python
enterprise_value = market_cap + net_debt
# or if net_debt not available:
enterprise_value = market_cap + total_debt - cash
```

**Revenue Growth Calculation:**
```python
# Fetch historical revenue
revenue_history = mcp_call("get_fundamentals", {
    "symbol": symbol,
    "fields": ["total_revenue_fq_h"]  # Historical quarterly revenue
})

# Calculate YoY growth
current_revenue = revenue_history[-1]  # Most recent
prior_year_revenue = revenue_history[-5]  # 4 quarters ago
growth_rate = (current_revenue - prior_year_revenue) / prior_year_revenue
```

**EV/EBITDA Calculation (if not provided):**
```python
ev_ebitda = enterprise_value / ebitda_ttm
```

### TradingView Field Reference

| Metric | TradingView Field | Coverage |
|--------|------------------|----------|
| Market Cap | `market_cap_basic` | 100% |
| Revenue TTM | `total_revenue_ttm` | 100% |
| EBITDA TTM | `ebitda_ttm` | 100% |
| Net Income TTM | `net_income_ttm` | 100% |
| P/E Ratio | `price_earnings_ttm` | 95% |
| P/B Ratio | `price_book_fq_h` | 95% |
| EV/EBITDA | `enterprise_value_ebitda_current` | 95% |
| ROE | `return_on_equity_fy` | 90% |
| Net Debt | `net_debt_fq` | 90% |
| Total Debt | `total_debt_fq` | 100% |
| Cash | `cash_n_short_term_invest_fq` | 100% |
| Beta | `beta_3_year` | 100% |
| Buyback Yield | `buyback_yield` | 85% |

---

## Section 1: Document Structure & Setup

### Header Block (Rows 1-3)
```
Row 1: [ANALYSIS TITLE] - COMPARABLE COMPANY ANALYSIS
Row 2: [List of Companies with Tickers] • [Company 1 (TICK1)] • [Company 2 (TICK2)] • [Company 3 (TICK3)]
Row 3: As of [Period] | All figures in [USD Millions/Billions] except per-share amounts and ratios
```

**Why this matters:** Establishes context immediately. Anyone opening this file knows what they're looking at, when it was created, and how to interpret the numbers.

### Visual Convention Standards (OPTIONAL - User preferences and uploaded templates always override)

**IMPORTANT: These are suggested defaults only. Always prioritize:**
1. User's explicit formatting preferences
2. Formatting from any uploaded template files
3. Company/team style guides
4. These defaults (only if no other guidance provided)

**Suggested Font & Typography:**
- **Font family**: Times New Roman (professional, readable, industry standard)
- **Font size**: 11pt for data cells, 12pt for headers
- **Bold text**: Section headers, company names, statistic labels

**Default Color & Shading — Professional Blue/Grey Palette (minimal is better):**
- **Keep it restrained** — only blues and greys. Do NOT introduce greens, oranges, reds, or multiple accent colors. A clean comps sheet uses 3-4 colors total.
- **Section headers** (e.g., "OPERATING STATISTICS & FINANCIAL METRICS"):
  - Dark blue background (`#1F4E79` or `#17365D` navy)
  - White bold text
  - Full row shading across all columns
- **Column headers** (e.g., "Company", "Revenue", "Margin"):
  - Light blue background (`#D9E1F2` or similar pale blue)
  - Black bold text
  - Centered alignment
- **Data rows**:
  - White background for company data
  - Black text for formulas; blue text for hardcoded inputs
- **Statistics rows** (Maximum, 75th Percentile, etc.):
  - Light grey background (`#F2F2F2`)
  - Black text, left-aligned labels
- **That's the whole palette**: dark blue + light blue + light grey + white. Nothing else unless the user's template says otherwise.

**Suggested Formatting Conventions:**
- **Decimal precision**:
  - Percentages: 1 decimal (12.3%)
  - Multiples: 1 decimal (13.5x)
  - Dollar amounts: No decimals, thousands separator (69,632)
  - Margins shown as percentages: 1 decimal (68.7%)
- **Borders**: No borders (clean, minimal appearance)
- **Alignment**: All metrics center-aligned for clean, uniform appearance
- **Cell dimensions**: All column widths should be uniform/even, all row heights should be consistent (creates clean, professional grid)

**Note:** If the user provides a template file or specifies different formatting, use that instead.

---

## Section 2: Operating Statistics & Financial Metrics

### Core Columns (Start with these)
1. **Company** - Names with consistent formatting
2. **Revenue** - Size metric (LTM from `total_revenue_ttm`)
3. **Revenue Growth** - Year-over-year percentage change (calculated from `total_revenue_fq_h`)
4. **Gross Profit** - Revenue minus cost of goods sold (if available)
5. **Gross Margin** - GP/Revenue (fundamental profitability)
6. **EBITDA** - Earnings before interest, tax, depreciation, amortization (from `ebitda_ttm`)
7. **EBITDA Margin** - EBITDA/Revenue (operating efficiency)

### Optional Additions (Choose based on industry/purpose)
- **Quarterly vs LTM** - Include both if seasonality matters
- **Free Cash Flow** - For capital-intensive or SaaS businesses (from `free_cash_flow_ttm`)
- **FCF Margin** - FCF/Revenue (cash generation efficiency)
- **Net Income** - For mature, profitable companies (from `net_income_ttm`)
- **Operating Income** - For businesses with varying D&A
- **CapEx metrics** - For asset-heavy industries (from `capital_expenditures_fq_h`)
- **Rule of 40** - Specifically for SaaS (Growth % + Margin %)
- **FCF Conversion** - For quality of earnings analysis (advanced)

### Formula Examples (Using Row 7 as example)
```excel
// Core ratios - these are always calculated
Gross Margin (F7): =E7/C7
EBITDA Margin (H7): =G7/C7

// Optional ratios - include if relevant
FCF Margin: =[FCF]/[Revenue]
Net Margin: =[Net Income]/[Revenue]
Rule of 40: =[Growth %]+[FCF Margin %]
```

**Golden Rule:** Every ratio should be [Something] / [Revenue] or [Something] / [Something from this sheet]. Keep it simple.

### Statistics Block (After company data)

**CRITICAL: Add statistics formulas for all comparable metrics (ratios, margins, growth rates, multiples).**

```
[Leave one blank row for visual separation]
- Maximum: =MAX(B7:B9)
- 75th Percentile: =QUARTILE(B7:B9,3)
- Median: =MEDIAN(B7:B9)
- 25th Percentile: =QUARTILE(B7:B9,1)
- Minimum: =MIN(B7:B9)
```

**Columns that NEED statistics (comparable metrics):**
- Revenue Growth %, Gross Margin %, EBITDA Margin %, EPS
- EV/Revenue, EV/EBITDA, P/E, Dividend Yield %, Beta

**Columns that DON'T need statistics (size metrics):**
- Revenue, EBITDA, Net Income (absolute size varies by company scale)
- Market Cap, Enterprise Value (not comparable across different-sized companies)

**Note:** Add one blank row between company data and statistics rows for visual separation. Do NOT add a "SECTOR STATISTICS" or "VALUATION STATISTICS" header row.

**Why quartiles matter:** They show distribution, not just average. A 75th percentile multiple tells you what "premium" companies trade at.

---

## Section 3: Valuation Multiples & Investment Metrics

### Core Valuation Columns (Start with these)
1. **Company** - Same order as operating section
2. **Market Cap** - Current market valuation (from `market_cap_basic`)
3. **Enterprise Value** - Market Cap ± Net Debt/Cash (calculated)
4. **EV/Revenue** - How much market pays per dollar of sales (calculated)
5. **EV/EBITDA** - How much market pays per dollar of earnings (from `enterprise_value_ebitda_current` or calculated)
6. **P/E Ratio** - Price relative to net earnings (from `price_earnings_ttm`)

### Optional Valuation Metrics (Choose based on context)
- **FCF Yield** - FCF/Market Cap (for cash-focused analysis)
- **PEG Ratio** - P/E/Growth Rate (for growth companies)
- **Price/Book** - Market value vs. book value (from `price_book_fq_h`)
- **ROE/ROA** - Return metrics (from `return_on_equity_fy`)
- **Revenue/EBITDA CAGR** - Historical growth rates (for trend analysis)
- **Asset Turnover** - Revenue/Assets (for operational efficiency)
- **Debt/Equity** - Leverage (from `debt_to_equity_current`)
- **Buyback Yield %** - Annualized buyback as % of market cap (from `buyback_yield`)
- **Normalized ROE** - ROE adjusted for buyback impact: net_income / (total_equity + market_cap × buyback_yield/100)

**Key Principle:** Include 3-5 core multiples that matter for your industry. Don't include every possible metric just because you can.

### Formula Examples
```excel
// Core multiples - always include these
EV/Revenue: =[Enterprise Value]/[LTM Revenue]
EV/EBITDA: =[Enterprise Value]/[LTM EBITDA]
P/E Ratio: =[Market Cap]/[Net Income]

// Optional multiples - include if data available
FCF Yield: =[LTM FCF]/[Market Cap]
PEG Ratio: =[P/E]/[Growth Rate %]
```

### Cross-Reference Rule
**CRITICAL:** Valuation multiples MUST reference the operating metrics section. Never input the same raw data twice. If revenue is in C7, then EV/Revenue formula should reference C7.

### Statistics Block
Same structure as operating section: Max, 75th, Median, 25th, Min for every metric. Add one blank row for visual separation between company data and statistics. Do NOT add a "VALUATION STATISTICS" header row.

---

## Section 4: Missing Data: Industry-Specific Metrics

TradingView provides standard financials. For sector-specific metrics, prompt the user:

```markdown
## Missing Data: Industry-Specific Metrics

TradingView provides standard financials. For sector-specific metrics:

**Example - Tech Sector:**
- ARR (Annual Recurring Revenue) - Not available via TradingView
- Net Dollar Retention - Not available via TradingView
- Customer Count - Not available via TradingView
- CAC Payback Period - Not available via TradingView

**Example - Retail Sector:**
- Same-Store Sales Growth - Not available via TradingView
- GMV (Gross Merchandise Value) - Not available via TradingView
- Active Buyers - Not available via TradingView

**Example - Healthcare Sector:**
- Pipeline Value - Not available via TradingView
- R&D/Revenue (detailed breakdown) - Limited via TradingView

**Would you like to:**
1. Provide these metrics manually
2. Skip and use standard metrics only
3. Use alternative data source (you provide)
```

**When to prompt:**
- User mentions SaaS/tech metrics (ARR, NDR, etc.)
- User asks for retail-specific metrics (SSS, GMV)
- User requests healthcare metrics (pipeline, regulatory)
- Any metric not in the TradingView field reference table

---

## Section 5: Notes & Methodology Documentation

### Required Components

**Data Sources & Quality:**
- Where did the data come from? (TradingView MCP — specify tools: `scan_stocks`, `get_fundamentals`, `get_quote`)
- What period does it cover? (Q4 2024, TTM figures)
- How was it verified? (Cross-checked against company filings)
- Note: TradingView MCP provides real-time and historical data with 95% coverage for standard comps metrics

**Key Definitions:**
- EBITDA calculation method (from TradingView: Operating Income + D&A)
- Free Cash Flow formula (Operating CF - CapEx)
- Special metrics explained (Rule of 40, FCF Conversion)
- Time period definitions (LTM, CAGR calculation periods)
- Enterprise Value calculation: Market Cap + Net Debt (from `market_cap_basic` + `net_debt_fq`)

**Valuation Methodology:**
- How was Enterprise Value calculated? (Market Cap + Net Debt from TradingView)
- What growth rates were used? (Historical CAGR from `total_revenue_fq_h`)
- Any adjustments made? (One-time items excluded, normalized margins)

**Analysis Framework:**
- What's the investment thesis? (Cloud/SaaS efficiency)
- What metrics matter most? (Cash generation, capital efficiency)
- How should readers interpret the statistics? (Quartiles provide context)

---

## Section 6: Choosing the Right Metrics (Decision Framework)

### Start with "What question am I answering?"

**"Which company is undervalued?"**
→ Focus on: EV/Revenue, EV/EBITDA, P/E, Market Cap
→ Skip: Operational details, growth metrics

**"Which company is most efficient?"**
→ Focus on: Gross Margin, EBITDA Margin, FCF Margin, Asset Turnover
→ Skip: Size metrics, absolute dollar amounts

**"Which company is growing fastest?"**
→ Focus on: Revenue Growth %, EBITDA CAGR, User/Customer Growth
→ Skip: Margin metrics, leverage ratios

**"Which is the best cash generator?"**
→ Focus on: FCF, FCF Margin, FCF Conversion, CapEx intensity
→ Skip: EBITDA, P/E ratios

### Industry-Specific Metric Selection

**Software/SaaS:**
Must have: Revenue Growth, Gross Margin, Rule of 40
Optional: ARR, Net Dollar Retention, CAC Payback (user-provided)
Skip: Asset Turnover, Inventory metrics

**Manufacturing/Industrials:**
Must have: EBITDA Margin, Asset Turnover, CapEx/Revenue
Optional: ROA, Inventory Turns, Backlog
Skip: Rule of 40, SaaS metrics

**Financial Services:**
Must have: ROE, ROA, Efficiency Ratio, P/E
Optional: Net Interest Margin, Loan Loss Reserves
Skip: Gross Margin, EBITDA (not meaningful for banks)

**Retail/E-commerce:**
Must have: Revenue Growth, Gross Margin, Inventory Turnover
Optional: Same-Store Sales, Customer Acquisition Cost (user-provided)
Skip: Heavy R&D or CapEx metrics

### The "5-10 Rule"

**5 operating metrics** - Revenue, Growth, 2-3 margins/efficiency metrics
**5 valuation metrics** - Market Cap, EV, 3 multiples
**= 10 total columns** - Enough to tell the story, not so many you lose the thread

If you have more than 15 metrics, you're probably including noise. Edit ruthlessly.

---

## Section 7: Best Practices & Quality Checks

### Before You Start
1. **Define the peer group** - Use `scan_stocks` to find truly comparable companies (similar business model, scale, geography)
2. **Choose the right period** - LTM smooths seasonality; quarterly shows trends
3. **Standardize units upfront** - Millions vs. billions decision affects everything
4. **Map data sources** - Know which TradingView fields provide each metric

### As You Build
1. **Input all raw data first** - Complete the blue text before writing formulas
2. **Add cell comments to ALL hard-coded inputs** - Right-click cell → Insert Comment → Document source OR assumption

**For TradingView-sourced data, cite exactly where it came from:**
- Example: "TradingView MCP - get_fundamentals, field: market_cap_basic, accessed 2024-10-02"
- Example: "TradingView MCP - get_quote, accessed 2024-10-02"
- Example: "TradingView MCP - scan_stocks filter: sector=Technology, market_cap range, accessed 2024-10-02"
- **Include hyperlinks when possible**: Right-click cell → Link → paste URL to TradingView chart or company page

**For assumptions, explain the reasoning:**
- Example: "Assumed 15% EBITDA margin based on peer median, company does not disclose"
- Example: "Estimated Enterprise Value as Market Cap + $50M net debt (from TradingView net_debt_fq)"
- Example: "Forward P/E based on street consensus EPS of $3.45 (average of 12 analyst estimates)"

**Why this matters**: Enables audit trails, data verification, assumption transparency, and future updates
3. **Build formulas row by row** - Test each calculation before moving on
4. **Use absolute references for headers** - $C$6 locks the header row
5. **Format consistently** - Percentages as percentages, not decimals
6. **Add conditional formatting** - Highlight outliers automatically

### Sanity Checks
- **Margin test**: Gross margin > EBITDA margin > Net margin (always true by definition)
- **Multiple reasonableness**:
  - EV/Revenue: typically 0.5-20x (varies widely by industry)
  - EV/EBITDA: typically 8-25x (fairly consistent across industries)
  - P/E: typically 10-50x (depends on growth rate)
- **Growth-multiple correlation**: Higher growth usually means higher multiples
- **Size-efficiency trade-off**: Larger companies often have better margins (scale benefits)

### Common Mistakes to Avoid
❌ Mixing market cap and enterprise value in formulas
❌ Using different time periods for numerator and denominator (LTM vs quarterly)
❌ Hardcoding numbers into formulas instead of cell references
❌ **Hard-coded inputs without cell comments citing the source OR explaining the assumption**
❌ Missing hyperlinks to data sources when available
❌ Including too many metrics without clear purpose
❌ Including non-comparable companies (different business models)
❌ Using outdated data without disclosure
❌ Calculating averages of percentages incorrectly (should be median)
❌ **Using web search when TradingView MCP is available**

---

## Section 8: Advanced Features

### Dynamic Headers
For columns showing calculations, use clear unit labels:
```
Revenue Growth (YoY) % | EBITDA Margin | FCF Margin | Rule of 40
```

### Quartile Analysis Benefits
Instead of just mean/median, quartiles show:
- **75th percentile** = "Premium" companies trade here
- **Median** = Typical market valuation
- **25th percentile** = "Discount" territory

This helps answer: "Is our target company trading rich or cheap vs. peers?"

### Industry-Specific Modifications

**Software/SaaS:**
- Add: ARR, Net Dollar Retention, CAC Payback Period (user-provided)
- Emphasize: Rule of 40, FCF margins, gross margins >70%

**Healthcare:**
- Add: R&D/Revenue, Pipeline value, Regulatory status (user-provided)
- Emphasize: EBITDA margins, growth rates, reimbursement risk

**Industrials:**
- Add: Backlog, Order book trends, Geographic mix (user-provided)
- Emphasize: ROIC, asset turnover, cyclical adjustments

**Consumer:**
- Add: Same-store sales, Customer acquisition cost, Brand value (user-provided)
- Emphasize: Revenue growth, gross margins, inventory turns

---

## Section 9: Workflow & Practical Tips

### Step-by-Step Process

1. **Set up structure** (30 minutes)
   - Create all headers
   - Format cells (blue for inputs, black for formulas)
   - Lock in units and date references

2. **Gather data via TradingView MCP** (30-45 minutes)
   - Use `scan_stocks` to find comparable companies
   - Use `get_fundamentals` for each peer to retrieve financial data
   - Use `get_quote` for current market data
   - Input all raw numbers in blue
   - Document sources in notes section

3. **Build formulas** (30 minutes)
   - Start with simple ratios (margins)
   - Progress to multiples (EV/Revenue)
   - Add cross-checks (do margins make sense?)

4. **Add statistics** (15 minutes)
   - Copy formula structure for all columns
   - Verify ranges are correct (B7:B9, not B7:B10)
   - Check quartile logic

5. **Quality control** (30 minutes)
   - Run sanity checks
   - Verify formula references
   - Check for #DIV/0! or #REF! errors
   - Compare against known benchmarks

6. **Documentation** (15 minutes)
   - Complete notes section
   - Add TradingView MCP data sources
   - Define methodologies
   - Date-stamp the analysis

### Pro Tips
- **Save templates**: Build once, reuse forever
- **Color-code outliers**: Conditional formatting for values >2 standard deviations
- **Link to source files**: Hyperlink to TradingView pages or SEC filings
- **Version control**: Save as "Comps_v1_2024-12-15" with clear dating
- **Collaborative reviews**: Have someone else check your formulas

### Excel Formatting Checklist (Optional - adapt to user preferences)
- [ ] Font set to user's preferred style (default: Times New Roman, 11pt data, 12pt headers)
- [ ] Section headers formatted per user's template (default: dark blue #17365D with white bold text)
- [ ] Column headers formatted per user's template (default: light blue/gray #D9E2F3 with black bold text)
- [ ] Statistics rows formatted per user's template (default: light gray #F2F2F2)
- [ ] No borders applied (clean, minimal appearance)
- [ ] **Column widths set to uniform/even width** (creates clean, professional appearance)
- [ ] **Row heights set to consistent height** (typically 20-25pt for data rows)
- [ ] Numbers formatted with proper decimal precision and thousands separators
- [ ] **All metrics center-aligned** for clean, uniform appearance
- [ ] **One blank row for separation between company data and statistics rows**
- [ ] **No separate "SECTOR STATISTICS" or "VALUATION STATISTICS" header rows**
- [ ] **Every hard-coded input cell has a comment with either: (1) exact data source, OR (2) assumption explanation**
- [ ] **Hyperlinks added to cells where applicable** (TradingView pages, SEC filings, data provider pages)

---

## Section 10: Example Template Layout

**Simple Version (Start here):**
```
┌─────────────────────────────────────────────────────────────┐
│ TECHNOLOGY - COMPARABLE COMPANY ANALYSIS                    │
│ Microsoft • Alphabet • Amazon                               │
│ As of Q4 2024 | All figures in USD Millions                 │
├─────────────────────────────────────────────────────────────┤
│ OPERATING METRICS                                           │
├──────────┬─────────┬─────────┬──────────┬──────────────────┤
│ Company  │ Revenue │ Growth  │ Gross    │ EBITDA │ EBITDA │
│          │ (LTM)   │ (YoY)   │ Margin   │ (LTM)  │ Margin │
├──────────┼─────────┼─────────┼──────────┼─────────┼────────┤
│ MSFT     │ 261,400 │ 12.3%   │ 68.7%    │ 205,100│ 78.4%  │
│ GOOGL    │ 349,800 │ 11.8%   │ 57.9%    │ 239,300│ 68.4%  │
│ AMZN     │ 638,100 │ 10.5%   │ 47.3%    │ 152,600│ 23.9%  │
│          │         │         │          │        │        │ [blank row]
│ Median   │ =MEDIAN │ =MEDIAN │ =MEDIAN  │ =MEDIAN│=MEDIAN │
│ 75th %   │ =QUART  │ =QUART  │ =QUART   │ =QUART │=QUART  │
│ 25th %   │ =QUART  │ =QUART  │ =QUART   │ =QUART │=QUART  │
├─────────────────────────────────────────────────────────────┤
│ VALUATION MULTIPLES                                         │
├──────────┬──────────┬──────────┬──────────┬────────────────┤
│ Company  │ Mkt Cap  │ EV       │ EV/Rev   │ EV/EBITDA │ P/E│
├──────────┼──────────┼──────────┼──────────┼───────────┼────┤
│ MSFT     │3,550,000 │3,530,000 │ 13.5x    │ 17.2x     │36.0│
│ GOOGL    │2,030,000 │1,960,000 │ 5.6x     │ 8.2x      │24.5│
│ AMZN     │2,226,000 │2,320,000 │ 3.6x     │ 15.2x     │58.3│
│          │          │          │          │           │    │ [blank row]
│ Median   │ =MEDIAN  │ =MEDIAN  │ =MEDIAN  │ =MEDIAN   │=MED│
│ 75th %   │ =QUART   │ =QUART   │ =QUART   │ =QUART    │=QRT│
│ 25th %   │ =QUART   │ =QUART   │ =QUART   │ =QUART    │=QRT│
└──────────┴──────────┴──────────┴──────────┴───────────┴────┘
```

**Add complexity only when needed:**
- Include quarterly AND LTM if seasonality matters
- Add FCF metrics if cash generation is key story
- Include industry-specific metrics (Rule of 40 for SaaS, etc.)
- Add more statistics rows if you have >5 companies

---

## Section 11: Industry-Specific Additions (Optional)

Only add these if they're critical to your analysis. Most comps work fine with just core metrics.

**Software/SaaS:**
Add if relevant: ARR, Net Dollar Retention, Rule of 40 (ARR/NDR require user input)

**Financial Services:**
Add if relevant: ROE, Net Interest Margin, Efficiency Ratio, Buyback Yield as capital return metric alongside dividend yield (from TradingView: `return_on_equity_fy`, `buyback_yield`)

**E-commerce:**
Add if relevant: GMV, Take Rate, Active Buyers (require user input)

**Healthcare:**
Add if relevant: R&D/Revenue, Pipeline Value, Patent Timeline (require user input)

**Manufacturing:**
Add if relevant: Asset Turnover, Inventory Turns, Backlog (require user input)

---

## Section 12: Red Flags & Warning Signs

### Data Quality Issues
🚩 Inconsistent time periods (mixing quarterly and annual)
🚩 Missing data without explanation
🚩 Significant differences between data sources (>10% variance)
🚩 TradingView MCP unavailable and falling back to web search

### Valuation Red Flags
🚩 Negative EBITDA companies being valued on EBITDA multiples (use revenue multiples instead)
🚩 P/E ratios >100x without hypergrowth story
🚩 Margins that don't make sense for the industry

### Comparability Issues
🚩 Different fiscal year ends (causes timing problems)
🚩 Mixing pure-play and conglomerates
🚩 Materially different business models labeled as "comps"

**When in doubt, exclude the company.** Better to have 3 perfect comps than 6 questionable ones.

---

## Section 13: Formulas Reference Guide

### Essential Excel Formulas
```excel
// Statistical Functions
=AVERAGE(range) // Simple mean
=MEDIAN(range) // Middle value
=QUARTILE(range, 1) // 25th percentile
=QUARTILE(range, 3) // 75th percentile
=MAX(range) // Maximum value
=MIN(range) // Minimum value
=STDEV.P(range) // Standard deviation

// Financial Calculations
=B7/C7 // Simple ratio (Margin)
=SUM(B7:B9)/3 // Average of multiple companies
=IF(B7>0, C7/B7, "N/A") // Conditional calculation
=IFERROR(C7/D7, 0) // Handle divide by zero

// Cross-Sheet References
='Sheet1'!B7 // Reference another sheet
=VLOOKUP(A7, Table1, 2) // Lookup from data table
=INDEX(MATCH()) // Advanced lookup

// Formatting
=TEXT(B7, "0.0%") // Format as percentage
=TEXT(C7, "#,##0") // Thousands separator
```

### Common Ratio Formulas
```excel
Gross Margin = Gross Profit / Revenue
EBITDA Margin = EBITDA / Revenue
FCF Margin = Free Cash Flow / Revenue
FCF Conversion = FCF / Operating Cash Flow
ROE = Net Income / Shareholders' Equity
ROA = Net Income / Total Assets
Asset Turnover = Revenue / Total Assets
Debt/Equity = Total Debt / Shareholders' Equity
EV/Revenue = Enterprise Value / Revenue
EV/EBITDA = Enterprise Value / EBITDA
P/E = Market Cap / Net Income
```

---

## Key Principles Summary

1. **Structure drives insight** - Right headers force right thinking
2. **Less is more** - 5-10 metrics that matter beat 20 that don't
3. **Choose metrics for your question** - Valuation analysis ≠ efficiency analysis
4. **Statistics show patterns** - Median/quartiles reveal more than average
5. **Transparency beats complexity** - Simple formulas everyone understands
6. **Comparability is king** - Better to exclude than force a bad comp
7. **Document your choices** - Explain which metrics and why in notes section
8. **TradingView MCP first** - Never use web search when MCP is available

---

## Output Checklist

Before delivering a comp analysis, verify:
- [ ] All companies are truly comparable
- [ ] Data is from consistent time periods
- [ ] Units are clearly labeled (millions/billions)
- [ ] Formulas reference cells, not hardcoded values
- [ ] **All hard-coded input cells have comments with either: (1) exact data source with citation, OR (2) clear assumption with explanation**
- [ ] **Hyperlinks added where relevant** (TradingView pages, SEC EDGAR filings)
- [ ] Statistics include at least 5 metrics (Max, 75th, Med, 25th, Min)
- [ ] Notes section documents TradingView MCP sources and methodology
- [ ] Visual formatting follows conventions (blue = input, black = formula)
- [ ] Sanity checks pass (margins logical, multiples reasonable)
- [ ] Date stamp is current ("As of [Date]")
- [ ] Formula auditing shows no errors (#DIV/0!, #REF!, #N/A)
- [ ] **TradingView MCP was used as primary data source** (not web search)

---

## Continuous Improvement

After completing a comp analysis, ask:
1. Did the statistics reveal unexpected insights?
2. Were there any data gaps that limited analysis?
3. Did stakeholders ask for metrics you didn't include?
4. How long did it take vs. how long should it take?
5. What would make this more useful next time?

The best comp analyses evolve with each iteration. Save templates, learn from feedback, and refine the structure based on what decision-makers actually use.
