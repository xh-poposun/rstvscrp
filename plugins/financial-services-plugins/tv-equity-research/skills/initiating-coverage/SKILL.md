---
name: initiating-coverage
description: Create institutional-quality equity research initiation reports through a 5-task workflow using TradingView data. Tasks must be executed individually with verified prerequisites - (1) company research, (2) financial modeling, (3) valuation analysis, (4) chart generation, (5) final report assembly. Each task produces specific deliverables (markdown docs, Excel models, charts, or DOCX reports). Tasks 3-5 have dependencies on earlier tasks.
allowed-tools: [docx, xlsx]
---

# Initiating Coverage (TradingView Edition)

Create institutional-quality equity research initiation reports through a structured 5-task workflow using TradingView MCP data. Each task must be executed separately with verified inputs.

## Overview

This skill produces comprehensive first-time coverage reports following institutional standards (JPMorgan, Goldman Sachs, Morgan Stanley format) using TradingView as the primary data source. Tasks are executed individually, each verifying prerequisites before proceeding.

**Default Font**: Times New Roman throughout all documents (unless user specifies otherwise).

---

## TradingView MCP Data Integration

### Primary Data Tools
| Tool | Purpose | Coverage |
|------|---------|----------|
| `get_fundamentals` | Financial metrics, quarterly data | 85% |
| `get_financial_statements` | Full IS/BS/CF statements | 100% |
| `get_quote` | Real-time price, market cap | 100% |
| `get_debt_maturity` | Debt structure details | 80% |
| `get_credit_ratings` | Credit ratings | 90% |
| `scan_stocks` | Peer identification | 100% |

### Key TradingView Fields for Initiating Coverage

**Income Statement:**
- Revenue: `total_revenue_fq_h` / `total_revenue_fy_h`
- Gross Profit: `gross_profit_fq`
- Operating Income: `operating_income_fq`
- EBITDA: `ebitda_fq` / `ebitda_ttm`
- Net Income: `net_income_fq` / `net_income_ttm`
- EPS: `earnings_per_share_fq` / `earnings_per_share_basic_ttm`

**Balance Sheet:**
- Cash: `cash_n_short_term_invest_fq`
- Total Debt: `total_debt_fq`
- Net Debt: `net_debt_fq`
- Total Assets: `total_assets_fq`
- Total Liabilities: `total_liabilities_fq`
- Shareholders Equity: `total_equity_fq`

**Cash Flow:**
- Operating Cash Flow: `cash_f_operating_activities_fq_h`
- CapEx: `capital_expenditures_fq_h`
- Depreciation: `depreciation_fy_h`
- Free Cash Flow: `free_cash_flow_ttm`

**Market & Valuation:**
- Market Cap: `market_cap_basic`
- Current Price: `price` (from `get_quote`)
- Beta: `beta_3_year`
- P/E: `price_earnings_ttm`
- P/B: `price_book_fq_h`
- EV/EBITDA: `enterprise_value_ebitda_current`
- ROE: `return_on_equity_fy`
- Buyback Yield: `buyback_yield`

**Credit & Debt:**
- Debt Maturity: `get_debt_maturity`
- Credit Ratings: `get_credit_ratings`
- Interest Coverage: `ebitda_interst_cover_fy`
- Debt/Equity: `debt_to_equity_current`

---

## Comps Analysis: Normalized ROE

For comparable company analysis, calculate **Normalized ROE** to account for buyback impact:

```
Normalized ROE = Net Income / Average Equity
Buyback-Adjusted Equity = Reported Equity - (Buyback Amount / Stock Price)
```

TradingView provides:
- `return_on_equity_fy` (reported ROE)
- `buyback_yield` (dividend equivalent yield from buybacks)
- Use these to derive a cleaner peer comparison metric

### Capital Return Policy

Include in company overview section:

**Total Shareholder Yield** = `dividend_yield + buyback_yield`

```
Capital Return Summary:
- Dividend Yield: [dividend_yield]%
- Buyback Yield: [buyback_yield]%
- Total Shareholder Yield: [sum]%
- Share Buyback Ratio (quarterly): [share_buyback_ratio_fq]
- Share Buyback Ratio (annual): [share_buyback_ratio_fy]
```

Flag for investment thesis: companies with Total Shareholder Yield > 5% may signal mature capital allocation; companies with buyback_yield > dividend_yield may be under-returning via dividends.

---

## User Prompts for Data Gaps

TradingView provides comprehensive financial data but has gaps for qualitative research. Prompt users for:

### Missing Data: Management Team & Bios

```markdown
## Missing Data: Management Team

TradingView provides limited executive information. For initiating coverage, please provide:

**CEO & Key Executives:**
- CEO name, tenure, background (300-400 words)
- CFO name, background
- COO/CTO/key division heads
- Board composition highlights

**Options:**
1. Paste executive bios from 10-K or proxy statement
2. Provide LinkedIn/key highlights
3. Skip detailed bios (use brief summaries only)

What would you like to provide?
```

### Missing Data: Business Description & Strategy

```markdown
## Missing Data: Business Strategy

TradingView provides financials but not qualitative business details. Please provide:

**Business Overview:**
- Primary products/services
- Business model (subscription, transactional, etc.)
- Key competitive advantages
- Recent strategic initiatives
- M&A history

**Options:**
1. Paste from 10-K business section
2. Provide bullet points
3. Use TradingView data only (financial focus)
```

### Missing Data: Industry & Competitive Analysis

```markdown
## Missing Data: Industry Context

TradingView provides peer financials but limited industry analysis. Please provide:

**Industry Information:**
- Industry size and growth rate
- Key trends and drivers
- Competitive landscape overview
- Regulatory environment
- 5-10 key competitors (or I can identify from TradingView scan)

**Options:**
1. Provide industry research
2. I can use TradingView scan_stocks to identify peers
3. Focus on financial analysis only
```

### Missing Data: Risk Factors

```markdown
## Missing Data: Risk Assessment

TradingView does not provide risk analysis. Please provide:

**Key Risks (from 10-K):**
- Business risks
- Financial risks
- Operational risks
- Regulatory/legal risks

**Options:**
1. Paste risk factors from 10-K
2. Provide top 5-8 key risks
3. I can infer from financial data (limited)
```

---

## ⚠️ CRITICAL: One Task at a Time

**THIS SKILL OPERATES IN SINGLE-TASK MODE ONLY.**

### If User Requests Full Pipeline

When user requests:
- "Create a coverage initiation report for [Company]"
- "Write an initiation report for [Company]"
- "Do the entire equity research process for [Company]"
- "Complete all 5 tasks for [Company]"
- Any request that implies running multiple tasks or the entire workflow

**REQUIRED RESPONSE:**

1. **Ask which specific task to perform:**
```
I can help you create an equity research initiation report for [Company].
This involves 5 separate tasks that need to be completed individually:

1. Company Research - Research business, management, industry (using TradingView + user inputs)
2. Financial Modeling - Build projection model (using TradingView data)
3. Valuation Analysis - DCF and comparable companies (using TradingView data)
4. Chart Generation - Create 25-35 charts
5. Report Assembly - Compile final report

Which task would you like to start with?
```

2. **When user explicitly requests all tasks together:**
```
I understand you'd like to complete the entire initiation report pipeline.
Currently, this skill supports executing one task at a time, which allows
for better quality control and review at each stage.

We're working on a seamless end-to-end workflow that will make this process
more automated, but for now, we'll need to complete each task separately.

Would you like to start with Task 1 (Company Research)?
```

3. **Never automatically assume which task to start** - always ask user to confirm.

4. **Never execute multiple tasks in sequence** - complete one task, deliver outputs, then wait for next user request.

### Task Execution Rules

- ✅ Execute exactly ONE task per user request
- ✅ Always verify prerequisites before starting a task
- ✅ Deliver task outputs and confirm completion
- ✅ Wait for user to explicitly request the next task
- ❌ Never chain multiple tasks together automatically
- ❌ Never assume user wants to proceed to next task
- ❌ Never execute Tasks 3-5 without verifying required inputs exist

### ⚠️ Deliverables Policy: NO SHORTCUTS

**DELIVER ONLY THE SPECIFIED OUTPUTS. DO NOT CREATE EXTRA DOCUMENTS.**

Each task specifies exact deliverables. Do NOT create:
- ❌ "Completion summaries"
- ❌ "Executive summaries"
- ❌ "Quick reference guides"
- ❌ "Next steps documents"
- ❌ "Task completion reports"
- ❌ Any other "helpful" documentation not explicitly specified

**Why**: These extras waste context and are not part of the professional workflow.

**What TO deliver**:
- ✅ Task 1: Research document (.md) — **NOTHING ELSE**
- ✅ Task 2: Financial model (.xlsx) — **NOTHING ELSE**
- ✅ Task 3: Valuation analysis (.md) + Excel tabs added to Task 2 file — **NOTHING ELSE**
- ✅ Task 4: Charts zip file (.zip) — **NOTHING ELSE**
- ✅ Task 5: Final report (.docx) — **NOTHING ELSE**

**If a deliverable is not listed above, DO NOT CREATE IT.**

---

## Task Selection

Select which task to execute:

| Task | Name | Prerequisites | Output | TradingView Tools Used |
|------|------|--------------|--------|----------------------|
| **1** | Company Research | Company name/ticker | 6-8K word document | get_quote, scan_stocks |
| **2** | Financial Modeling | TradingView data access | Excel model (6 tabs) | get_fundamentals, get_financial_statements |
| **3** | Valuation Analysis | Financial model (Task 2) | Valuation + price target | get_fundamentals, get_debt_maturity, get_credit_ratings |
| **4** | Chart Generation | Tasks 1, 2, 3 + TradingView data | 25-35 PNG/JPG charts | All TradingView tools |
| **5** | Report Assembly | ALL previous tasks (1-4) | 30-50 page DOCX report | N/A (assembly only) |

---

## Task 1: Company Research

**Purpose**: Research company's business, management, competitive position, industry, and risks using TradingView data + user inputs.

**Prerequisites**: ✅ None (fully independent)
- Company name or ticker symbol

**TradingView Data to Fetch:**
```python
# Get basic company info
get_quote(symbols=["NASDAQ:AAPL"])
# Returns: name, description, sector, industry, price, market_cap

# Identify peers for competitive analysis
scan_stocks(
    filters={
        "sector": "Technology",
        "market_cap_basic": { "gte": 100000000000 }
    },
    limit=20
)
```

**User Inputs Required:**
- Management bios (or opt for brief summaries)
- Business description/strategy (or use TradingView sector/industry)
- Industry context (or TradingView peer scan)
- Risk factors (or infer from financial data)

**Process**:
1. Fetch TradingView quote data (sector, industry, market cap)
2. Use scan_stocks to identify peers for competitive analysis
3. Prompt user for qualitative inputs (management, strategy, risks)
4. Load detailed instructions from references/task1-company-research.md
5. Execute qualitative research workflow
6. Deliver research document

**Output**: Company Research Document (6,000-8,000 words)
- Company overview & history (using TradingView description)
- Management bios (user-provided or brief summaries)
- Products & services analysis
- Industry overview (using TradingView sector data)
- Competitive analysis (5-10 peers from TradingView scan)
- TAM sizing
- Risk assessment (user-provided or inferred)

**File name**: `[Company]_Research_Document_[Date].md`

**⚠️ DELIVER ONLY THIS 1 FILE. NO completion summaries, no extra documents.**

---

## Task 2: Financial Modeling

**Purpose**: Extract historical financials from TradingView and build comprehensive Excel financial model with projections and scenarios.

**Prerequisites**: ⚠️ Verify before starting
- **Required**: TradingView MCP access
- Company ticker symbol
- **Optional**: Company research (Task 1) for business context

**TradingView Data to Fetch:**
```python
# Fetch 5 years of fundamentals
get_fundamentals(symbol="NASDAQ:AAPL")
# Key fields for model:
# - total_revenue_fy_h (annual revenue history)
# - ebitda_fy_h (annual EBITDA)
# - net_income_fy_h (annual net income)
# - earnings_per_share_fy_h (annual EPS)
# - total_assets_fq, total_liabilities_fq (balance sheet)
# - cash_f_operating_activities_fq_h (cash flow)

# Fetch detailed financial statements
get_financial_statements(symbol="NASDAQ:AAPL", statement_type="income", period="annual")
get_financial_statements(symbol="NASDAQ:AAPL", statement_type="balance", period="annual")
get_financial_statements(symbol="NASDAQ:AAPL", statement_type="cash", period="annual")
```

**Process**:
1. Verify TradingView MCP access
2. Fetch historical financials (5 years) from TradingView
3. Load detailed instructions from references/task2-financial-modeling.md
4. **Step 1**: Extract historical financials from TradingView
5. **Step 2+**: Build projection model with 6 essential tabs
6. Deliver Excel model

**Output**: Excel Financial Model (.xlsx)
- 6 essential tabs:
  1. **Revenue Model** - Product breakdown (20-30 rows) + Geography breakdown (15-20 rows)
  2. **Income Statement** - Full P&L with 40-50 line items, historical (3-5 years) + projected (5 years)
  3. **Cash Flow Statement** - Operating/Investing/Financing activities, historical + projected
  4. **Balance Sheet** - Assets/Liabilities/Equity, historical + projected
  5. **Scenarios** - Bull/Base/Bear comparison table
  6. **DCF Inputs** - Prepared for Task 3 valuation

**File name**: `[Company]_Financial_Model_[Date].xlsx`

**⚠️ DELIVER ONLY THIS 1 FILE. NO completion summaries, no extra documents.**

**TradingView Data Citations:**
- Every blue input cell must have comment: "Source: TradingView MCP get_fundamentals, [Date], [field]"
- Historical data from `get_financial_statements` must be cited

---

## Task 3: Valuation Analysis

**Purpose**: Perform comprehensive valuation using DCF, comparables, and precedent transactions using TradingView data.

**Prerequisites**: ⚠️ Verify before starting
- **Required**: Financial model from Task 2
- Projected income statements
- Projected cash flows
- Revenue and EBITDA forecasts
- DCF inputs (unlevered FCF)

**⚠️ CRITICAL: DO NOT START THIS TASK UNLESS TASK 2 IS COMPLETE**

**TradingView Data to Fetch:**
```python
# Fetch data for DCF and comps
get_fundamentals(symbol="NASDAQ:AAPL")
# Key fields:
# - beta_3_year (for WACC calculation)
# - net_debt_fq (for enterprise value)
# - market_cap_basic (for equity value)
# - price_earnings_ttm, price_book_fq_h (for comps)
# - enterprise_value_ebitda_current (for comps)
# - return_on_equity_fy (for comps)
# - Normalized ROE = net_income_fy / (total_equity_fq + market_cap × buyback_yield/100) (for buyback-adjusted comps)

# Fetch debt and credit data
get_debt_maturity(symbol="NASDAQ:AAPL")
get_credit_ratings(symbol="NASDAQ:AAPL")

# Fetch peer data for comparables
# Use scan_stocks to identify peers, then get_fundamentals for each
```

**User Inputs Required (DCF Assumptions):**
```markdown
## DCF Assumptions Required

TradingView provides historical data but not forward assumptions. Please provide:

**1. Risk-Free Rate** (10Y Treasury):
- Current rate: ~4.2%
- Options: Use current, custom value, or default 4.0%

**2. Market Risk Premium**:
- Options: 5.0% (conservative), 5.5% (moderate), 6.0% (aggressive)

**3. Terminal Growth Rate**:
- Guidelines: 2.0-2.5% (GDP), 2.5-3.0% (moderate), 3.0-3.5% (aggressive)
- Must be < WACC

**4. Exit Multiple** (for exit multiple method):
- Based on current trading multiple or industry average
```

**Process**:
1. Verify financial model is accessible
2. Fetch TradingView data (beta, debt, credit ratings)
3. Prompt user for DCF assumptions
4. Load detailed instructions from references/task3-valuation.md
5. Execute valuation workflow
6. Deliver valuation analysis

**Output**: Valuation Analysis (4-6 pages + Excel tabs)
- DCF analysis with sensitivity tables
- Comparable companies (5-10 peers with statistical summary)
- Precedent transactions (if applicable)
- Valuation football field
- **Price target**: $XX.XX
- **Recommendation**: BUY/HOLD/SELL
- **Upside**: XX%
- Key catalysts (3-5)

**Files**:
- `[Company]_Valuation_Analysis_[Date].md` (written analysis document)
- Excel tabs added to `[Company]_Financial_Model_[Date].xlsx` (from Task 2)
  - DCF tab with calculations
  - Sensitivity analysis tab
  - Comparable companies tab
  - Valuation summary tab

**⚠️ DELIVER ONLY: 1 markdown file + 4 tabs added to existing Excel. NO completion summaries, no extra documents.**

---

## Task 4: Chart Generation

**Purpose**: Generate 25-35 professional financial charts for the report using TradingView data.

**Prerequisites**: ⚠️ Verify before starting
- **Required**: Company research from Task 1
- **Required**: Financial model from Task 2 (with Task 3 valuation tabs added)
- **Required**: TradingView data access

**⚠️ CRITICAL: DO NOT START THIS TASK UNLESS TASKS 1, 2, AND 3 ARE COMPLETE**

**TradingView Data for Charts:**
```python
# Historical price data for stock charts
get_quote(symbols=["NASDAQ:AAPL"])  # Current data

# Historical fundamentals for trend charts
get_fundamentals(symbol="NASDAQ:AAPL")
# Fields for charts:
# - total_revenue_fq_h (quarterly revenue trend)
# - ebitda_fq (quarterly EBITDA)
# - earnings_per_share_fq (quarterly EPS)
# - gross_margin_fq, operating_margin_fq (margins)

# Peer data for comparison charts
scan_stocks(filters={"sector": "Technology"}, limit=10)
# Then get_fundamentals for each peer
```

**4 MANDATORY Charts** (must be present) ⭐:
- chart_03: Revenue by product (stacked area) - from Task 2 model
- chart_04: Revenue by geography (stacked bar) - from Task 2 model
- chart_28: DCF sensitivity (2-way heatmap) - from Task 3
- chart_32: Valuation football field (horizontal bars) - from Task 3

**25 REQUIRED Charts** (specific list):
- Investment Summary: chart_01 (stock price from TradingView)
- Financial Performance: charts 02, 03⭐, 04⭐, 10, 11, 12, 14 (TradingView data)
- Company 101: charts 05, 06, 07, 08, 09, 15, 16 (Task 1 data)
- Competitive/Market: charts 17, 18 (TradingView peer data)
- Scenario Analysis: chart 13 (Task 2 model)
- Valuation: charts 28⭐, 29, 30, 31, 32⭐, 33, 34 (Task 3 + TradingView)

**Output**: 25-35 Professional Chart Files (PNG/JPG, 300 DPI) packaged in zip

**File naming**: `chart_01_description.png`, `chart_02_description.png`, etc.

**Deliverable**: `[Company]_Charts_[Date].zip` containing all 25-35 chart files + chart_index.txt

---

## Task 5: Report Assembly

**Purpose**: Write and assemble the comprehensive final DOCX report.

**Prerequisites**: ⚠️ Verify before starting
- **Required**: Company research from Task 1
- **Required**: Financial model from Task 2
- **Required**: Valuation analysis from Task 3
- **Required**: Chart files from Task 4

**⚠️ CRITICAL: DO NOT START THIS TASK UNLESS ALL TASKS 1-4 ARE COMPLETE**

**Process**:
1. **CRITICAL**: Verify ALL prerequisites before starting
2. Load detailed instructions from references/task5-report-assembly.md
3. Execute report assembly workflow using Claude's built-in skills:
   - **Use DOCX skill** to create and manipulate the Word document
   - **Use XLSX skill** to read Excel data from Task 2/3
   - **Use Read tool** to read Task 1 and Task 3 markdown files
4. Save and deliver final DOCX report

**Output**: Comprehensive Equity Research Report (.docx)

**Specifications**:
- **Length**: 30-50 pages (MINIMUM 30)
- **Word count**: 10,000-15,000 words (MINIMUM 10,000)
- **Charts**: 25-35 embedded images
- **Tables**: 12-20 comprehensive tables
- **Format**: Professional DOCX with TradingView citations

**Structure**:
- Page 1: Investment Summary (INITIATING COVERAGE format)
- Pages 2-5: Investment thesis & risks
- Pages 6-17: Company 101
- Pages 18-30: Financial analysis & projections (TradingView-sourced)
- Pages 31-40: Valuation analysis (TradingView-sourced)
- Pages 41-50: Appendices

**File name**: `[Company]_Initiation_Report_[Date].docx`

**⚠️ DELIVER ONLY THIS 1 DOCX FILE. NO executive summaries, no "highlights" documents, no extra files.**

---

## TradingView Coverage Summary

### Available via TradingView (85% coverage):
- ✅ Historical financials (5 years)
- ✅ Quarterly and annual metrics
- ✅ Real-time market data
- ✅ Peer identification (scan_stocks)
- ✅ Credit ratings
- ✅ Debt maturity schedule
- ✅ Valuation multiples

### Requires User Input (15% gap):
- ❌ Detailed management bios
- ❌ Business strategy description
- ❌ Industry analysis depth
- ❌ Risk factor details
- ❌ DCF assumptions (risk-free rate, ERP, terminal growth)

---

## Anti-Patterns

- ❌ NEVER use web search as primary data source (use TradingView MCP)
- ❌ NEVER skip user prompts for missing qualitative data
- ❌ NEVER skip DCF verification steps
- ❌ NEVER use nested IF in Excel (use INDEX/OFFSET)
- ❌ NEVER hardcode computed values in Excel (use formulas)
- ❌ NEVER omit TradingView citations on financial data
- ❌ NEVER skip clickable hyperlinks in citations
- ❌ NEVER chain tasks automatically
- ❌ NEVER create extra deliverables beyond specified outputs

---

## Dependencies

**Required:**
- TradingView MCP server (tvdata)
- tv-financial-analysis plugin (for DCF, comps models)
- Python (matplotlib, pandas, seaborn) for chart generation
- DOCX skill for report creation
- XLSX skill for model creation

**Optional:**
- User-provided qualitative data (management bios, strategy, risks)
