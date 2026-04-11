---
name: earnings-analysis
description: Create professional equity research earnings update reports (8-12 pages, 3,000-5,000 words) analyzing quarterly results using TradingView data. Fast-turnaround format focusing on beat/miss analysis, key metrics, updated estimates, and revised thesis. Includes 1-3 summary tables and 8-12 charts. Use when user requests "earnings update", "quarterly update", "earnings analysis", "Q1/Q2/Q3/Q4 results", or post-earnings report.
allowed-tools: [docx, xlsx]
---

# Equity Research Earnings Update (TradingView Edition)

Create professional **EARNINGS UPDATE REPORTS** analyzing quarterly results using TradingView MCP data, following institutional standards (JPMorgan, Goldman Sachs, Morgan Stanley format).

**Key Characteristics:**
- **Length**: 8-12 pages
- **Word Count**: 3,000-5,000 words
- **Tables**: 1-3 summary tables (NOT comprehensive)
- **Figures**: 8-12 charts
- **Turnaround**: 1-2 days (within 24-48 hours of earnings)
- **Audience**: Clients already familiar with the company
- **Focus**: What's NEW - beat/miss, updated estimates, thesis impact
- **Font**: Times New Roman throughout (unless user specifies otherwise)

## TradingView MCP Data Sources

This skill uses TradingView MCP for financial data. Key tools and fields:

### Primary Data Tools
| Tool | Purpose | Coverage |
|------|---------|----------|
| `get_fundamentals` | Financial metrics, quarterly data | 85% |
| `get_earnings_calendar` | Earnings dates, EPS estimates | 90% |
| `get_financial_statements` | Full IS/BS/CF statements | 100% |
| `get_quote` | Real-time price, market cap | 100% |

### Key TradingView Fields for Earnings Analysis

**Income Statement Metrics:**
- Revenue: `total_revenue_fq_h` (quarterly history)
- EBITDA: `ebitda_fq` / `ebitda_ttm`
- Net Income: `net_income_fq` / `net_income_ttm`
- EPS: `earnings_per_share_fq` / `earnings_per_share_basic_ttm`
- Gross Profit: `gross_profit_fq`
- Operating Income: `operating_income_fq`

**Balance Sheet Metrics:**
- Cash: `cash_n_short_term_invest_fq`
- Total Debt: `total_debt_fq`
- Net Debt: `net_debt_fq`
- Total Assets: `total_assets_fq`
- Shareholders Equity: `total_equity_fq`
- Total Shares Outstanding: `total_shares_outstanding`
- Buyback Yield: `buyback_yield`

**Cash Flow Metrics:**
- Operating Cash Flow: `cash_f_operating_activities_fq_h`
- CapEx: `capital_expenditures_fq_h`
- Free Cash Flow: `free_cash_flow_ttm`

**Market Data:**
- Current Price: `price` (from `get_quote`)
- Market Cap: `market_cap_basic`
- P/E Ratio: `price_earnings_ttm`
- EV/EBITDA: `enterprise_value_ebitda_current`

## User Prompts for Data Gaps

TradingView provides financial data but **NOT** earnings call transcripts or management commentary. You MUST prompt the user for:

### Missing Data: Earnings Call Transcript

```markdown
## Missing Data: Earnings Call Transcript

TradingView provides financial metrics but not earnings call transcripts or management commentary.

**For earnings analysis, please provide:**
- Key management quotes from earnings call
- Guidance provided (revenue, EPS, margins, other metrics)
- Q&A highlights and analyst questions
- Forward-looking statements and strategic updates
- Segment commentary and outlook

**Options:**
1. Paste transcript excerpt or key quotes
2. Provide bullet points of key takeaways
3. Skip qualitative analysis (focus on quantitative only)

What would you like to provide?
```

### Missing Data: Management Guidance

```markdown
## Missing Data: Management Guidance

TradingView does not provide management guidance. Please provide:

**Current Quarter Guidance:**
- Revenue guidance (range or point estimate)
- EPS guidance
- Margin guidance (gross, operating, EBITDA)
- Other key metric guidance

**Full Year Guidance:**
- Revenue guidance
- EPS guidance
- Margin targets
- CapEx guidance
- Any changes from prior guidance

**Options:**
1. Provide guidance details
2. Use consensus estimates as proxy
3. Skip guidance analysis
```

### Missing Data: Segment Breakdown

```markdown
## Missing Data: Segment Performance

TradingView provides company-level data but limited segment detail. Please provide:

**Segment Revenue & Operating Income:**
- Revenue by business segment
- Operating income by segment
- Segment margin trends
- Geographic revenue split

**Options:**
1. Provide segment data from earnings release
2. Use available TradingView data (company-level only)
3. Skip segment analysis
```

## When to Use

Use when the user requests:
- "Create an earnings update for [Company] Q3 2024"
- "Analyze [Company]'s quarterly results"
- "Post-earnings report for [Company]"
- "Q1/Q2/Q3/Q4 update for [Company]"

**Do NOT use if:**
- User requests "initiation report" → Use initiating-coverage skill
- User requests "flash note" or "quick take" → Different format
- Company is not already covered → Need initiation first

## Critical Requirements

### 1. Speed & Timeliness
- Publish within 24-48 hours of earnings release
- Focus on NEW information only
- Don't rehash company background extensively

### 2. Beat/Miss Analysis
- Lead with whether company beat or missed estimates
- Quantify variances (e.g., "Revenue beat by $120M or 3%")
- Explain WHY results differed from expectations
- Use `get_earnings_calendar` for consensus estimates

### 3. Summary Format
- Keep tables to 1-3 (summary only, not comprehensive)
- No full P&L/Cash Flow/Balance Sheet (just key metrics)
- Assume reader has seen initiation report

### 4. Citations & Source Attribution ⭐⭐⭐ MANDATORY

**CRITICAL**: Properly cite all data with SPECIFIC sources.

**Include specific citations in every figure and table:**

```
Source: TradingView MCP get_fundamentals, [Date], [field]
Source: TradingView MCP get_earnings_calendar, [Date]
Source: Company earnings release [Date] (user-provided)
Source: Earnings call transcript [Date] (user-provided)
```

**REQUIRED SOURCES LIST:**

Cite in every earnings update:
- ✅ TradingView MCP data (with tool name and date)
- ✅ Earnings release (with date - user provided)
- ✅ 10-Q filing (with filing date and EDGAR link)
- ✅ Earnings call transcript (with date - user provided)
- ✅ Consensus estimates (TradingView get_earnings_calendar)
- ✅ Prior guidance (from previous quarter's materials - user provided)

**REFERENCE SECTION:**

Include "Sources" section at end of report:

```
SOURCES & REFERENCES

Financial Data (Q3 2024):
• TradingView MCP get_fundamentals, [Date]
  Fields: total_revenue_fq_h, ebitda_fq, net_income_fq, earnings_per_share_fq

• TradingView MCP get_earnings_calendar, [Date]
  Consensus EPS: $X.XX | Actual EPS: $Y.YY

Earnings Materials (Q3 2024):
• Earnings Release (November 7, 2024) [User-provided]
• Form 10-Q (Filed November 8, 2024)
  [Hyperlink to: https://www.sec.gov/cgi-bin/viewer?accession=...]
• Earnings Call Transcript (November 7, 2024) [User-provided]
```

**VERIFICATION CHECKLIST:**
- [ ] Every figure has source with specific TradingView tool and field
- [ ] Every table has source with document reference
- [ ] Beat/miss analysis cites TradingView consensus data
- [ ] Guidance changes cite current and prior guidance sources
- [ ] Key statistics have footnotes
- [ ] Sources section lists all materials

### 5. Updated Estimates
- Update forward estimates based on results
- Show old vs. new estimates clearly
- Explain what changed and why
- Use TradingView historical data for trend analysis

## High-Level Workflow

### Phase 1: Data Collection (30-60 minutes)

**Step 1: Fetch TradingView Data**

Use TradingView MCP tools:

```python
# Fetch fundamentals for quarterly data
get_fundamentals(symbol="NASDAQ:AAPL")
# Fields: total_revenue_fq_h, ebitda_fq, net_income_fq, earnings_per_share_fq

# Fetch earnings calendar for consensus
get_earnings_calendar(symbols=["NASDAQ:AAPL"], from_date="2024-01-01", to_date="2024-12-31")

# Fetch current quote for market data
get_quote(symbols=["NASDAQ:AAPL"])

# Fetch financial statements for detailed line items
get_financial_statements(symbol="NASDAQ:AAPL", statement_type="income", period="quarterly")
```

**Step 2: Prompt User for Missing Data**

After fetching TradingView data, prompt user for:
- Earnings call transcript/key quotes
- Management guidance (current and prior)
- Segment breakdown details
- Any qualitative commentary

**Step 3: Verify Data Completeness**

```
BEFORE PROCEEDING - Check:
- [ ] TradingView fundamentals retrieved (revenue, EPS, EBITDA)
- [ ] Consensus estimates from earnings calendar
- [ ] Current stock price and market cap
- [ ] User provided transcript/guidance (or opted to skip)
- [ ] Historical quarterly data for trend analysis
```

### Phase 2: Analysis (2-3 hours)

**Beat/Miss Analysis:**
- Compare actuals (TradingView) vs consensus (TradingView earnings calendar)
- Calculate variances: $ amount and %
- Analyze key metric performance

**Segment Analysis:**
- Use user-provided segment data
- Compare to TradingView company-level totals

**Margin Analysis:**
- Calculate from TradingView data:
  - Gross Margin = Gross Profit / Revenue
  - Operating Margin = Operating Income / Revenue
  - EBITDA Margin = EBITDA / Revenue
  - Net Margin = Net Income / Revenue

**Guidance Analysis:**
- Compare new guidance (user-provided) vs prior
- Assess guidance raise/cut/maintain

**Buyback Impact on EPS:**
- Use `total_shares_outstanding` to track quarter-over-quarter share count reduction
- Calculate EPS accretion from buybacks: if shares outstanding declined X%, EPS benefited by ~X% all else equal
- **Total Shareholder Yield** = `dividend_yield + buyback_yield` — report alongside earnings summary for full capital return picture

### Phase 3: Chart Generation (1-2 hours)

Create 8-12 charts using TradingView data:

**Required Charts:**
1. Quarterly revenue progression (TradingView `total_revenue_fq_h`)
2. Quarterly EPS progression (TradingView `earnings_per_share_fq`)
3. Quarterly margin trends (calculated from TradingView data)
4. Revenue vs consensus (TradingView actuals vs estimates)
5. EPS beat/miss history (TradingView data)
6. Key operating metrics (TradingView fundamentals)
7. Estimate revisions (user-provided guidance changes)
8. Valuation chart (TradingView P/E, EV/EBITDA)

### Phase 4: Report Creation (2-3 hours)

Create 8-12 page DOCX report with specific structure.

**High-level structure:**
- Page 1: Earnings summary with rating and price target
- Pages 2-3: Detailed results analysis (using TradingView data)
- Pages 4-5: Key metrics & guidance (TradingView + user-provided)
- Pages 6-7: Updated investment thesis
- Pages 8-10: Valuation & estimates
- Pages 11-12: Appendix (optional)

### Phase 5: Quality Check & Delivery (30 minutes)

Verify content, formatting, accuracy, and timeliness before delivery.

**TradingView Data Verification:**
- [ ] All TradingView data properly cited with tool name and field
- [ ] Numbers match TradingView exactly
- [ ] Dates align with fiscal quarter
- [ ] Currency and units correct

## Output Specification

**Primary Deliverable**: DOCX report (8-12 pages)
**File Name**: `[Company]_Q[Quarter]_[Year]_Earnings_Update.docx`
**Example**: `AAPL_Q4_FY24_Earnings_Update.docx`

**Contents:**
- Page 1: Summary with rating, price target, key takeaways
- Pages 2-3: Detailed results analysis (TradingView-sourced)
- Pages 4-5: Key metrics and guidance
- Pages 6-7: Updated thesis assessment
- Pages 8-10: Valuation and estimates
- Pages 11-12: Appendix (optional)
- 8-12 embedded charts
- 1-3 summary tables
- Complete sources section with TradingView citations

**Optional Deliverable**: XLS model update (optional for earnings updates)

## Key Differences from Initiation Report

| Aspect | Earnings Update | Initiation Report |
|--------|----------------|-------------------|
| **Length** | 8-12 pages | 30-50 pages |
| **Words** | 3,000-5,000 | 10,000-15,000 |
| **Tables** | 1-3 summary | 12-20 comprehensive |
| **Figures** | 8-12 | 25-35 |
| **Turnaround** | 1-2 days | 3-6 weeks |
| **Scope** | Quarterly results | Complete company |
| **Focus** | What's NEW | Everything |
| **Company Background** | Brief mention | 6-10 pages |
| **XLS Model** | Optional | Required |
| **Data Source** | TradingView MCP | Multiple sources |

## TradingView Coverage Notes

### What's Available (85% coverage):
- ✅ Quarterly financial metrics (revenue, EPS, EBITDA)
- ✅ Historical quarterly trends
- ✅ Consensus estimates (earnings calendar)
- ✅ Real-time market data (price, market cap)
- ✅ Valuation multiples (P/E, EV/EBITDA)
- ✅ Full financial statements

### What's NOT Available (requires user input):
- ❌ Earnings call transcripts
- ❌ Management guidance
- ❌ Management commentary
- ❌ Segment-level detail (limited)
- ❌ Geographic breakdown
- ❌ Forward-looking statements

## Anti-Patterns

- ❌ NEVER use web search as primary data source (use TradingView MCP)
- ❌ NEVER skip user prompts for transcript/guidance
- ❌ NEVER skip DCF verification steps
- ❌ NEVER use nested IF in Excel (use INDEX/OFFSET)
- ❌ NEVER hardcode computed values in Excel (use formulas)
- ❌ NEVER omit TradingView citations on financial data
- ❌ NEVER use outdated earnings data (always verify within 3 months)
- ❌ NEVER skip clickable hyperlinks in citations (plain URLs not acceptable)

## Dependencies

**Required:**
- TradingView MCP server (tvdata)
- Python (matplotlib, pandas, seaborn) for chart generation
- DOCX skill for report creation

**Optional:**
- XLS skill for model updates (not required for earnings updates)

## Resources

### TradingView MCP Tools
- `get_fundamentals` - Primary financial data source
- `get_earnings_calendar` - Consensus estimates
- `get_financial_statements` - Detailed line items
- `get_quote` - Market data

### Related Skills
- `initiating-coverage` - For first-time coverage reports
- `model-update` - For updating financial models post-earnings
- `morning-note` - For quick post-earnings reactions
