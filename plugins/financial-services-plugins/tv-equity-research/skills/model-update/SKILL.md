---
name: model-update
description: Update financial models with new TradingView data — quarterly earnings, management guidance, macro changes, or revised assumptions. Uses get_fundamentals for actuals, get_earnings_calendar for consensus, and recalculates valuation. Use after earnings, guidance updates, or when assumptions need refreshing. Triggers on "update model", "plug earnings", "refresh estimates", "update numbers for [company]", "new guidance", or "revise estimates".
allowed-tools: [xlsx]
---

# Model Update (TradingView Edition)

Update financial models with new data using TradingView MCP — quarterly earnings, management guidance, macro changes, or revised assumptions. Adjusts estimates, recalculates valuation, and flags material changes.

## TradingView MCP Data Sources

### Primary Tools
| Tool | Purpose | When to Use |
|------|---------|-------------|
| `get_fundamentals` | Latest quarterly actuals | After earnings release |
| `get_earnings_calendar` | Consensus estimates | Beat/miss analysis |
| `get_financial_statements` | Detailed line items | Deep dive updates |
| `get_quote` | Current price, market cap | Valuation refresh |
| `get_debt_maturity` | Debt structure changes | Balance sheet updates |
| `get_credit_ratings` | Credit rating changes | Credit analysis |

### Key TradingView Fields for Model Updates

**Actual Quarterly Results:**
- Revenue: `total_revenue_fq` (latest quarter)
- EBITDA: `ebitda_fq`
- Net Income: `net_income_fq`
- EPS: `earnings_per_share_fq`
- Gross Profit: `gross_profit_fq`
- Operating Income: `operating_income_fq`

**Consensus Data (from earnings calendar):**
- Consensus EPS estimate
- Consensus revenue estimate
- Report date
- Period end date

**Balance Sheet Updates:**
- Cash: `cash_n_short_term_invest_fq`
- Total Debt: `total_debt_fq`
- Net Debt: `net_debt_fq`
- Share Count: `shares_outstanding_fq`
- Total Shares Outstanding: `total_shares_outstanding`
- Buyback Yield: `buyback_yield`
- Share Buyback Ratio: `share_buyback_ratio_fq` / `share_buyback_ratio_fy`

**Market Data:**
- Current Price: `price`
- Market Cap: `market_cap_basic`
- P/E: `price_earnings_ttm`
- EV/EBITDA: `enterprise_value_ebitda_current`

## User Prompts for Data Gaps

TradingView provides actual financials but NOT management guidance or qualitative commentary. Prompt users for:

### Missing Data: Management Guidance

```markdown
## Missing Data: Updated Management Guidance

TradingView provides actuals but not forward guidance. Please provide:

**Current Quarter Guidance:**
- Revenue guidance: $X - $Y (or $Z midpoint)
- EPS guidance: $A - $B
- Margin guidance (gross, operating, EBITDA)
- Other metric guidance

**Full Year Guidance Changes:**
- Prior revenue guidance: $X → New: $Y (change: +/- Z%)
- Prior EPS guidance: $A → New: $B (change: +/- C%)
- Margin guidance changes
- Any new metrics added or removed

**Key Assumption Changes:**
- Revenue growth assumptions
- Margin expansion targets
- CapEx guidance
- Tax rate assumptions

**Options:**
1. Paste guidance from earnings release
2. Provide key changes only
3. Skip guidance update (maintain current estimates)
```

### Missing Data: Earnings Call Commentary

```markdown
## Missing Data: Earnings Call Commentary

TradingView provides financial metrics but not qualitative commentary. Please provide:

**Key Management Quotes:**
- CEO/CFO commentary on results
- Explanation of beat/miss drivers
- Segment performance commentary
- Forward-looking statements

**Q&A Highlights:**
- Key analyst questions and management responses
- Clarifications on guidance
- Strategic updates

**Options:**
1. Paste key quotes from transcript
2. Provide bullet points
3. Skip qualitative commentary
```

### Missing Data: Segment Performance

```markdown
## Missing Data: Segment Breakdown

TradingView provides company-level data. Please provide segment details:

**Segment Revenue & Operating Income:**
- Segment 1: Revenue $X, Op Income $Y, Margin Z%
- Segment 2: Revenue $A, Op Income $B, Margin C%
- Geographic split (if relevant)

**Segment Commentary:**
- Drivers of segment performance
- Segment-specific guidance
- New segment disclosures

**Options:**
1. Provide segment data from earnings release
2. Use company-level data only
3. Skip segment analysis
```

## Workflow

### Step 1: Identify What Changed

Determine the update trigger:
- **Earnings release**: New quarterly actuals to plug in
- **Guidance change**: Company updated forward outlook
- **Estimate revision**: Analyst changing assumptions based on new data
- **Macro update**: Interest rates, FX, commodity prices changed
- **Event-driven**: M&A, restructuring, new product, management change

### Step 2: Fetch TradingView Data

```python
# Fetch latest actuals
get_fundamentals(symbol="NASDAQ:AAPL")
# Key fields to extract:
# - total_revenue_fq (latest quarter actual)
# - ebitda_fq
# - net_income_fq
# - earnings_per_share_fq
# - gross_profit_fq, operating_income_fq

# Fetch consensus for beat/miss analysis
get_earnings_calendar(symbols=["NASDAQ:AAPL"], from_date="2024-01-01", to_date="2024-12-31")

# Fetch current market data
get_quote(symbols=["NASDAQ:AAPL"])

# Fetch updated balance sheet
get_financial_statements(symbol="NASDAQ:AAPL", statement_type="balance", period="quarterly")
```

### Step 3: Plug New Data

#### After Earnings

Update the model with TradingView actuals:

| Line Item | Prior Estimate | TradingView Actual | Delta | Notes |
|-----------|---------------|-------------------|-------|-------|
| Revenue | | [total_revenue_fq] | | |
| Gross Margin | | [calculated] | | |
| Operating Expenses | | [operating_expense_fq] | | |
| EBITDA | | [ebitda_fq] | | |
| EPS | | [earnings_per_share_fq] | | |

**Beat/Miss Analysis (using TradingView consensus):**
```
Revenue: $X actual vs $Y consensus ([get_earnings_calendar]) = $Z variance (A%)
EPS: $A actual vs $B consensus = $C variance (D%)
```

**Segment Detail** (if user-provided):
- Update each segment's revenue and margin
- Note any segment mix shifts

**Balance Sheet / Cash Flow Updates (from TradingView):**
- Cash: [cash_n_short_term_invest_fq]
- Debt: [total_debt_fq]
- Share count: [shares_outstanding_fq]
- CapEx: [capital_expenditures_fq_h]

### Step 4: Prompt for Guidance

After plugging TradingView actuals, prompt user for:
- Updated management guidance
- Earnings call commentary
- Segment performance details

### Step 5: Revise Forward Estimates

Based on TradingView actuals + user-provided guidance:

| | Old FY Est | New FY Est | Change | Old Next FY | New Next FY | Change |
|---|-----------|-----------|--------|------------|------------|--------|
| Revenue | | | | | | |
| EBITDA | | | | | | |
| EPS | | | | | | |

**Key Assumption Changes:**
- What assumptions are you changing and why?
- Revenue growth rate: old → new (reason)
- Margin assumption: old → new (reason)
- Any new items (restructuring charges, one-time gains, etc.)

### Step 6: Valuation Impact

Recalculate valuation with updated estimates:

| Valuation Method | Prior | Updated | Change |
|-----------------|-------|---------|--------|
| DCF fair value | | | |
| P/E (NTM EPS × target multiple) | | | |
| EV/EBITDA (NTM EBITDA × target multiple) | | | |
| **Price Target** | | | |

**TradingView Data for Valuation:**
- Current Price: [get_quote price]
- Market Cap: [get_quote market_cap_basic]
- P/E: [get_fundamentals price_earnings_ttm]
- EV/EBITDA: [get_fundamentals enterprise_value_ebitda_current]

### Step 7: Summary & Action

**Estimate Change Summary:**
- One paragraph: what changed, why, and what it means for the stock
- Is this a thesis-changing event or noise?

**Rating / Price Target:**
- Maintain or change rating?
- New price target (if changed) with methodology
- Upside/downside to current price [from get_quote]

### Step 8: Output

- Updated Excel model (with TradingView citations)
- Estimate change summary (markdown)
- Updated price target derivation

## Excel Model Update Process

### Step 1: Open Existing Model

Use XLSX skill to read the existing model file.

### Step 2: Update Historical Tab

Plug TradingView actuals into historical columns:
```
Cell comment: "Source: TradingView MCP get_fundamentals, [Date], total_revenue_fq"
```

### Step 3: Update Projections

Revise forward estimates based on:
- TradingView actuals (new base)
- User-provided guidance
- Revised assumptions

### Step 4: Recalculate Valuation

Update DCF and comps using:
- New projections
- Current market data from TradingView
- Updated assumptions

### Step 5: Citations

Add cell comments to all updated cells:
```
"Source: TradingView MCP [tool], [Date], [field]"
"Source: User-provided guidance, [Date]"
```

## Important Notes

- Always reconcile your estimates to TradingView actuals before projecting forward
- Note any non-recurring items and whether your estimates are GAAP or adjusted
- Track your estimate revision history — it shows your analytical progression
- If the quarter was noisy, separate signal from noise in your estimate changes
- Check consensus after updating — how do your revised estimates compare to the Street?
- Share count matters — dilution from stock comp, converts, or buybacks can materially affect EPS
- Buyback activity affects per-share metrics — track `buyback_yield` and `share_buyback_ratio_fq` changes
- **Normalized ROE** accounts for buyback-distorted equity: `Normalized ROE = net_income_fy / (total_equity_fq + market_cap × buyback_yield/100)`
- **Total Shareholder Yield** = `dividend_yield + buyback_yield` — use for comprehensive capital return analysis
- **TradingView provides actuals, but guidance requires user input**

## Anti-Patterns

- ❌ NEVER use web search for earnings data (use TradingView MCP)
- ❌ NEVER skip user prompts for guidance updates
- ❌ NEVER hardcode actuals without TradingView citation
- ❌ NEVER update models without verifying against TradingView actuals
- ❌ NEVER skip beat/miss analysis using TradingView consensus
- ❌ NEVER forget to update cell comments with TradingView sources

## Dependencies

**Required:**
- TradingView MCP server (tvdata)
- XLSX skill for model updates
- Existing financial model file

**Data Flow:**
```
TradingView MCP → Actuals → Model Update
User Input → Guidance → Forward Estimates
TradingView + Model → Updated Valuation
```
