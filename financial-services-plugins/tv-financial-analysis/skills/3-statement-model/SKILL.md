---
name: 3-statement-model
description: Complete, populate and fill out 3-statement financial model templates (Income Statement, Balance Sheet, Cash Flow Statement) using TradingView MCP as the sole data source. Retrieves financial data from TradingView MCP server, populates integrated financial statements with proper linkages, and ensures balance sheet tie-outs and cash flow integrity checks. Use when asked to fill out model templates, complete existing model frameworks, populate financial models with data, complete a partially filled IS/BS/CF framework, or link integrated financial statements within an existing template structure.
---

# 3-Statement Financial Model Template Completion (TradingView Edition)

Complete and populate integrated financial model templates with proper linkages between Income Statement, Balance Sheet, and Cash Flow Statement using TradingView MCP as the sole data source.

## Data Source

**TradingView MCP Tools:**
- `get_financial_statements` — Full income statement, balance sheet, cash flow data
- `get_fundamentals` — Detailed financial metrics (200+ fields)
- `get_quote` — Current market data (price, market cap)

## Key TradingView Fields for 3-Statement Models

| Financial Statement | TradingView Field | Description |
|---------------------|-------------------|-------------|
| **Income Statement** | `total_revenue_fq_h` | Total revenue (quarterly, historical) |
| | `cost_of_goods_fy_h` | Cost of goods sold (historical) |
| | `operating_expenses_fq` | Operating expenses |
| | `ebitda_fq` | EBITDA |
| | `net_income_fq` | Net income |
| | `income_tax_fq` | Income tax expense |
| | `pretax_income_fq` | Pre-tax income |
| **Balance Sheet** | `total_current_assets` | Total current assets |
| | `total_current_liabilities` | Total current liabilities |
| | `cash_n_short_term_invest_fq` | Cash & short-term investments |
| | `total_debt_fq` | Total debt |
| | `net_debt_fq` | Net debt |
| | `total_assets_fq` | Total assets |
| | `total_liabilities_fq` | Total liabilities |
| **Cash Flow** | `cash_f_operating_activities_fq_h` | Cash from operations |
| | `capital_expenditures_fq_h` | Capital expenditures |
| | `depreciation_fy_h` | Depreciation |
| **Shares & Buybacks** | `total_shares_outstanding` | Shares outstanding |
| | `buyback_yield` | Annualized buyback yield as % of market cap |

## ⚠️ CRITICAL PRINCIPLES — Read Before Populating Any Template

**Environment — Office JS vs Python:**
- **If running inside Excel (Office Add-in / Office JS):** Use Office JS directly. Write formulas via `range.formulas = [["=D14*(1+Assumptions!$B$5)"]]` — never `range.values` for derived cells. No separate recalc; Excel computes natively. Use `context.workbook.worksheets.getItem(...)` to navigate tabs.
- **If generating a standalone .xlsx file:** Use Python/openpyxl. Write `ws["D15"] = "=D14*(1+Assumptions!$B$5)"`, then run `recalc.py` before delivery.
- **Office JS merged cell pitfall:** Do NOT call `.merge()` then set `.values` on the merged range — throws `InvalidArgument`. Instead write value to top-left cell alone, then merge + format the full range: `ws.getRange("A1").values = [["INCOME STATEMENT"]]; const h = ws.getRange("A1:G1"); h.merge(); h.format.fill.color = "#1F4E79";`
- All principles below apply identically in either environment.

**Formulas over hardcodes (non-negotiable):**
- Every projection cell, roll-forward, linkage, and subtotal MUST be an Excel formula — never a pre-computed value
- When using Python/openpyxl: write formula strings (`ws["D15"] = "=D14*(1+Assumptions!$B$5)"`), NOT computed results (`ws["D15"] = 12500`)
- The ONLY cells that should contain hardcoded numbers are: (1) historical actuals from TradingView, (2) assumption drivers in the Assumptions tab
- If you find yourself computing a value in Python and writing the result to a cell — STOP. Write the formula instead.
- Why: the model must flex when scenarios toggle or assumptions change. Hardcodes break every downstream integrity check silently.

**Verify step-by-step with the user:**
1. **After mapping the template** → show the user which tabs/sections you've identified and confirm before touching any cells
2. **After populating historicals** → show the user the historical block and confirm values/periods match TradingView data
3. **After building IS projections** → run the subtotal checks, show the user the projected IS, confirm before moving to BS
4. **After building BS** → show the user the balance check (Assets = L+E) for every period, confirm before moving to CF
5. **After building CF** → show the user the cash tie-out (CF ending cash = BS cash), confirm before finalizing
6. **Do NOT populate the entire model end-to-end and present it complete** — break at each statement, show the work, catch errors early

## Data Retrieval from TradingView

### Step 1: Fetch Financial Statements

```python
# Fetch complete financial statements
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

cash_flow = mcp_call("get_financial_statements", {
    "symbol": symbol,
    "statement": "cash",
    "period": "annual"
})
```

### Step 2: Fetch Detailed Fundamentals

```python
# Fetch detailed metrics for line items
fundamentals = mcp_call("get_fundamentals", {"symbol": symbol})

# Key fields for 3-statement model:
key_fields = {
    # Income Statement
    "revenue": fundamentals.get("total_revenue_fq_h"),
    "cogs": fundamentals.get("cost_of_goods_fy_h"),
    "opex": fundamentals.get("operating_expenses_fq"),
    "ebitda": fundamentals.get("ebitda_fq"),
    "net_income": fundamentals.get("net_income_fq"),
    "income_tax": fundamentals.get("income_tax_fq"),
    "pretax_income": fundamentals.get("pretax_income_fq"),
    
    # Balance Sheet
    "current_assets": fundamentals.get("total_current_assets"),
    "current_liabilities": fundamentals.get("total_current_liabilities"),
    "cash": fundamentals.get("cash_n_short_term_invest_fq"),
    "total_debt": fundamentals.get("total_debt_fq"),
    "net_debt": fundamentals.get("net_debt_fq"),
    "total_assets": fundamentals.get("total_assets_fq"),
    "total_liabilities": fundamentals.get("total_liabilities_fq"),
    
    # Cash Flow
    "cfo": fundamentals.get("cash_f_operating_activities_fq_h"),
    "capex": fundamentals.get("capital_expenditures_fq_h"),
    "depreciation": fundamentals.get("depreciation_fy_h"),
}
```

### Step 3: Calculate Working Capital

```python
# Working capital = Current Assets - Current Liabilities
working_capital = (
    fundamentals.get("total_current_assets", 0) - 
    fundamentals.get("total_current_liabilities", 0)
)

# Changes in working capital for cash flow statement
delta_wc = working_capital_current - working_capital_prior
```

## Missing Data: Detailed Line Items

TradingView provides comprehensive financials with **85% coverage** for 3-statement models. Some detailed items may need user input:

### Example — Balance Sheet Detail Items:
- Goodwill impairment details
- Deferred tax assets breakdown
- Pension obligations
- Intangible assets breakdown
- Minority interest details

### Example — Income Statement Detail Items:
- Stock-based compensation breakdown
- Restructuring charges
- Asset write-downs
- Other income/expense details

### If Detailed Breakdown Needed:

```markdown
## ⚠️ Missing Data: Detailed Line Items

TradingView provides comprehensive financials. Some detailed items may need input:

**Balance Sheet Items:**
- Goodwill impairment: [Not available from TradingView]
- Deferred tax breakdown: [Not available from TradingView]

**Options:**
1. **Provide manually** from 10-K filings
2. **Use aggregate values** (recommended for most cases)
3. **Skip and note assumption** in model comments

**Enter your choice:**
```

## Formatting — Professional Blue/Grey Palette

**Keep colors minimal.** Use only blues and greys for cell fills. Do NOT introduce greens, yellows, oranges, or multiple accent colors.

| Element | Fill | Font |
|---|---|---|
| Section headers (IS / BS / CF titles) | Dark blue `#1F4E79` | White bold |
| Column headers (FY2024A, FY2025E, etc.) | Light blue `#D9E1F2` | Black bold |
| Input cells (historicals from TradingView) | Light grey `#F2F2F2` or white | Blue `#0000FF` |
| Formula cells | White | Black |
| Cross-tab links | White | Green `#008000` |
| Check rows / key totals | Medium blue `#BDD7EE` | Black bold |

**That's 3 blues + 1 grey + white.** If the template has its own color scheme, follow the template instead.

Font color signals *what* a cell is (input/formula/link). Fill color signals *where* you are (header/data/check).

## Model Structure

### Identifying Template Tab Organization

Templates vary in their tab naming conventions and organization. Before populating, review all tabs to understand the template's structure.

| Common Tab Names | Contents to Look For |
|------------------|----------------------|
| IS, P&L, Income Statement | Income Statement |
| BS, Balance Sheet | Balance Sheet |
| CF, CFS, Cash Flow | Cash Flow Statement |
| WC, Working Capital | Working Capital Schedule |
| DA, D&A, Depreciation, PP&E | Depreciation & Amortization Schedule |
| Debt, Debt Schedule | Debt Schedule |
| NOL, Tax, DTA | Net Operating Loss Schedule |
| Assumptions, Inputs, Drivers | Driver assumptions and inputs |
| Checks, Audit, Validation | Error-checking dashboard |

**Template Review Checklist:**
- Identify which tabs exist in the template (not all templates include every schedule)
- Note any template-specific tabs not listed above
- Understand tab dependencies (e.g., which schedules feed into the main statements)
- Locate input cells vs. formula cells on each tab

### Understanding Template Structure

Before populating a template, familiarize yourself with its existing layout to ensure data is entered in the correct locations and formulas remain intact.

**Identifying Row Structure:**
- Locate the model title at top of each tab
- Identify section headers and their visual separation
- Find the units row indicating $ millions, %, x, etc.
- Note column headers distinguishing Actuals vs. Estimates periods
- Confirm period labels (e.g., FY2024A, FY2025E)
- Identify input cells vs. formula cells (typically distinguished by font color)

**Identifying Column Structure:**
- Confirm line item labels in leftmost column
- Verify historical years precede projection years
- Note the visual border separating historical from projected periods
- Check for consistent column order across all tabs

**Working with Named Ranges:**
Templates often use named ranges for key inputs and outputs. Before entering data:
- Review existing named ranges in the template (Formulas → Name Manager in Excel)
- Common named ranges include: Revenue growth rates, cost percentages, key outputs (Net Income, EBITDA, Total Debt, Cash), scenario selector cell
- Ensure inputs are entered in cells that feed into these named ranges

### Projection Period
- Templates typically project 5 years forward from last historical year
- Verify historical (A) vs. projected (E) columns are clearly separated
- Confirm columns use fiscal year notation (e.g., FY2024A, FY2025E)

## Margin Analysis

**Note: The following margin analysis should only be performed if prompted by the user or if the template explicitly requires it. If no prompt is given, skip this section.**

Calculate and display profitability margins on the Income Statement (IS) tab to track operational efficiency and enable peer comparison.

### Core Margins to Include

| Margin | Formula | What It Measures |
|--------|---------|------------------|
| Gross Margin | Gross Profit / Revenue | Pricing power, production efficiency |
| EBITDA Margin | EBITDA / Revenue | Core operating profitability |
| EBIT Margin | EBIT / Revenue | Operating profitability after D&A |
| Net Income Margin | Net Income / Revenue | Bottom-line profitability |

### Income Statement Layout with Margins

Display margin percentages directly below each profit line item:
- Gross Margin % below Gross Profit
- EBIT Margin % below EBIT
- EBITDA Margin % below EBITDA
- Net Income Margin % below Net Income

## Credit Metrics

**Note: The following Credit analysis should only be performed if prompted by the user or if the template explicitly requires it. If no prompt is given, skip this section.**

Calculate and display credit/leverage metrics on the Balance Sheet (BS) tab to assess financial health, debt capacity, and covenant compliance.

### Core Credit Metrics to Include

| Metric | Formula | What It Measures |
|--------|---------|------------------|
| Total Debt / EBITDA | Total Debt / LTM EBITDA | Leverage multiple |
| Net Debt / EBITDA | (Total Debt - Cash) / LTM EBITDA | Leverage net of cash |
| Interest Coverage | EBITDA / Interest Expense | Ability to service debt |
| Debt / Total Cap | Total Debt / (Total Debt + Equity) | Capital structure |
| Debt / Equity | Total Debt / Total Equity | Financial leverage |
| Current Ratio | Current Assets / Current Liabilities | Short-term liquidity |
| Quick Ratio | (Current Assets - Inventory) / Current Liabilities | Immediate liquidity |

### Credit Metric Hierarchy Checks

Validate that Upside shows strongest credit profile:
- Leverage: Upside < Base < Downside (lower is better)
- Coverage: Upside > Base > Downside (higher is better)
- Liquidity: Upside > Base > Downside (higher is better)

### Covenant Compliance Tracking

If debt covenants are known, add explicit compliance checks comparing actual metrics to covenant thresholds.

### Treasury Stock and Buybacks

When modeling stock buybacks in the Balance Sheet:

**Key Fields**: Use `total_shares_outstanding` and `buyback_yield` from TradingView

**Balance Sheet Treatment**:
- Treasury stock appears as a contra-equity account (negative equity)
- Record at cost: Treasury Stock = Shares Repurchased × Purchase Price
- Reduces total equity on the balance sheet

**Cash Flow Statement Treatment**:
- Stock repurchases appear in Financing Activities (CFF)
- Use: (Treasury Stock Purchases) or Dividends Paid section
- Amount = Shares Repurchased × Average Stock Price

**Projection Approach**:
1. Apply buyback_yield to projected shares: Shares_N = Shares_(N-1) × (1 - buyback_yield/100)
2. Calculate cash used: Cash for Buybacks = Market Cap × buyback_yield
3. Flow to CFF: Reduce cash, increase treasury stock (contra-equity)

## Scenario Analysis (Base / Upside / Downside)

Use a scenario toggle (dropdown) in the Assumptions tab with CHOOSE or INDEX/MATCH formulas.

| Scenario | Description |
|----------|-------------|
| Base Case | Management guidance or consensus estimates |
| Upside Case | Above-guidance growth, margin expansion |
| Downside Case | Below-trend growth, margin compression |

**Key Drivers to Sensitize:** Revenue growth, Gross margin, SG&A %, DSO/DIO/DPO, CapEx %, Interest rate, Tax rate.

**Scenario Audit Checks:** Toggle switches all statements, BS balances in all scenarios, Cash ties out, Hierarchy holds (Upside > Base > Downside for NI, EBITDA, FCF, margins).

## Completing Model Templates

This section provides general guidance for completing any 3-statement financial model template while preserving existing formulas and ensuring data integrity.

### Step 1: Analyze the Template Structure

Before entering any data, thoroughly review the template to understand its architecture:

**Identify Input vs. Formula Cells:**
- Look for visual cues (font color, cell shading) that distinguish input cells from formula cells
- Common conventions: Blue font = inputs, Black font = formulas, Green font = links to other sheets
- Use Excel's Trace Precedents/Dependents (Formulas → Trace Precedents) to understand cell relationships
- Check for named ranges that may control key inputs (Formulas → Name Manager)

**Map the Template's Flow:**
- Identify which tabs feed into others (e.g., Assumptions → IS → BS → CF)
- Note any supporting schedules and their linkages to main statements
- Document the template's specific line items and structure before populating

### Step 2: Filling in Data Without Breaking Formulas

**Golden Rules for Data Entry:**

| Rule | Description |
|------|-------------|
| Only edit input cells | Never overwrite cells containing formulas unless intentionally replacing the formula |
| Preserve cell references | When copying data, use Paste Values (Ctrl+Shift+V) to avoid overwriting formulas with source formatting |
| Match the template's units | Verify if template uses thousands, millions, or actual values before entering data |
| Respect sign conventions | Follow the template's existing sign convention (e.g., expenses as positive or negative) |
| Check for circular references | If the template uses iterative calculations, ensure Enable Iterative Calculation is turned on |

**Safe Data Entry Process:**
1. Identify the exact cells designated for input (usually highlighted or labeled)
2. Enter historical data from TradingView first, then verify formulas are calculating correctly for those periods
3. Enter assumption drivers that feed forecast calculations
4. Review calculated outputs to confirm formulas are working as intended
5. If a formula cell must be modified, document the original formula before making changes

**Handling Pre-Built Formulas:**
- If formulas reference cells you haven't populated yet, expect temporary errors (#REF!, #DIV/0!) until all inputs are complete
- When formulas produce unexpected results, trace precedents to identify missing or incorrect inputs
- Never delete rows/columns without checking for formula dependencies across all tabs

### Step 3: Validating Formulas

**Formula Integrity Checks:**

Before relying on template outputs, validate that formulas are functioning correctly:

| Check Type | Method |
|------------|--------|
| Trace precedents | Select a formula cell → Formulas → Trace Precedents to verify it references correct inputs |
| Trace dependents | Verify key inputs flow to expected output cells |
| Evaluate formula | Use Formulas → Evaluate Formula to step through complex calculations |
| Check for hardcodes | Projection formulas should reference assumptions, not contain hardcoded values |
| Test with known values | Input simple test values to verify formulas produce expected results |
| Cross-tab consistency | Ensure the same formula logic applies across all projection periods |

**Common Formula Issues to Watch For:**
- Mixed absolute/relative references causing incorrect results when copied across periods
- Broken links to external files or deleted ranges (#REF! errors)
- Division by zero in early periods before revenue ramps (#DIV/0! errors)
- Circular reference warnings (may be intentional for interest calculations)
- Inconsistent formulas across projection columns (use Ctrl+\ to find differences)

**Validating Cross-Tab Linkages:**
- Confirm values that appear on multiple tabs are linked (not duplicated)
- Verify schedule totals tie to corresponding line items on main statements
- Check that period labels align across all tabs

### Step 4: Quality Checks by Sheet

Perform these validation checks on each sheet after populating the template:

**Income Statement (IS) Quality Checks:**
- Revenue figures match TradingView data for historical periods
- All expense line items sum to reported totals
- Subtotals (Gross Profit, EBIT, EBT, Net Income) calculate correctly
- Tax calculation logic is appropriate (handles losses correctly)
- Forecast drivers reference assumptions tab (no hardcodes)
- Period-over-period changes are directionally reasonable

**Balance Sheet (BS) Quality Checks:**
- **Assets = Liabilities + Equity for every period (PRIMARY CHECK)**
- Cash balance matches Cash Flow Statement ending cash
- Working capital accounts tie to supporting schedules (if applicable)
- Retained Earnings rolls forward correctly: Prior RE + Net Income - Dividends +/- Adjustments = Ending RE
- Debt balances tie to debt schedule (if applicable)
- All balance sheet items have appropriate signs (assets positive, most liabilities positive)

**Cash Flow Statement (CF) Quality Checks:**
- Net Income at top of CFO matches Income Statement Net Income
- Non-cash add-backs (D&A, SBC, etc.) tie to their source schedules/statements
- Working capital changes have correct signs (increase in asset = use of cash = negative)
- CapEx ties to PP&E schedule or fixed asset roll-forward
- Financing activities tie to changes in debt and equity accounts on BS
- **Ending Cash matches Balance Sheet Cash (CRITICAL)**
- Beginning Cash equals prior period Ending Cash

**Supporting Schedule Quality Checks:**
- Opening balances equal prior period closing balances
- Roll-forward logic is complete (Beginning + Additions - Deductions = Ending)
- Schedule totals tie to main statement line items
- Assumptions used in calculations match Assumptions tab

### Step 5: Cross-Statement Integrity Checks

After validating individual sheets, confirm the three statements are properly integrated:

| Check | Formula | Expected Result |
|-------|---------|-----------------|
| **Balance Sheet Balance** | Assets - Liabilities - Equity | = 0 |
| **Cash Tie-Out** | CF Ending Cash - BS Cash | = 0 |
| **Net Income Link** | IS Net Income - CF Starting Net Income | = 0 |
| **Retained Earnings** | Prior RE + NI - Dividends - BS Ending RE | = 0 (adjust for SBC/other items as needed) |

### Step 6: Final Review

Before considering the model complete:
- Toggle through all scenarios (if applicable) to verify checks pass in each case
- Review all #REF!, #DIV/0!, #VALUE!, and #NAME? errors and resolve or document
- Confirm all input cells have been populated (search for placeholder values)
- Verify units are consistent across all tabs
- Save a clean version before making any additional modifications

## Model Validation and Audit

This section consolidates all validation checks and audit procedures for completed templates.

### Core Linkages (Must Always Hold)

| Check | Formula | Expected Result |
|-------|---------|-----------------|
| Balance Sheet Balance | Assets - Liabilities - Equity | = 0 |
| Cash Tie-Out | CF Ending Cash - BS Cash | = 0 |
| Cash Monthly vs Annual | Closing Cash (Monthly) - Closing Cash (Annual) | = 0 |
| Net Income Link | IS Net Income - CF Starting Net Income | = 0 |
| Retained Earnings | Prior RE + NI + SBC - Dividends - BS Ending RE | = 0 |
| Equity Financing | ΔCommon Stock/APIC (BS) - Equity Issuance (CFF) | = 0 |
| Year 0 Equity | Equity Raised (Year 0) - Beginning Equity Capital (Year 1) | = 0 |

### Sign Convention Reference

| Statement | Item | Sign Convention |
|-----------|------|-----------------|
| CFO | D&A, SBC | Positive (add-back) |
| CFO | ΔAR (increase) | Negative (use of cash) |
| CFO | ΔAP (increase) | Positive (source of cash) |
| CFI | CapEx | Negative |
| CFF | Debt issuance | Positive |
| CFF | Debt repayments | Negative |
| CFF | Dividends | Negative |
| CFF | Stock Repurchases (Buybacks) | Negative |

### Circular Reference Handling

Interest expense creates circularity: Interest → Net Income → Cash → Debt Balance → Interest

Enable iterative calculation in Excel: File → Options → Formulas → Enable iterative calculation. Set maximum iterations to 100, maximum change to 0.001. Add a circuit breaker toggle in Assumptions tab.

### Check Categories

**Section 1: Currency Consistency**
- Currency identified and documented in Assumptions
- All tabs use consistent currency symbol and scale
- Units row matches model currency

**Section 2: Balance Sheet Integrity**
- Assets = Liabilities + Equity (for each period)
- Formula: Assets - Liabilities - Equity (must = 0)

**Section 3: Cash Flow Integrity**
- Cash ties to BS (CF Ending Cash = BS Cash)
- Cash Monthly vs Annual: Closing Cash (Monthly) = Closing Cash (Annual)
- NI ties to IS (CF Net Income = IS Net Income)
- D&A ties to schedule
- SBC ties to IS
- ΔAR, ΔInventory, ΔAP tie to WC schedule
- CapEx ties to DA schedule

**Section 4: Retained Earnings**
- RE roll-forward check: Prior RE + NI + SBC - Dividends = Ending RE
- Show component breakdown for debugging

**Section 5: Working Capital**
- AR, Inventory, AP tie to BS
- DSO, DIO, DPO reasonability checks (flag if outside normal ranges)

**Section 6: Debt Schedule**
- Total Debt ties to BS (Current + LT Debt)
- Interest calculation ties to IS

**Section 6b: Equity Financing**
- Equity issuance proceeds tie to BS Common Stock/APIC increase
- Cash increase from equity = Equity account increase (must balance)
- Equity Raise Tie-Out: ΔCommon Stock/APIC (BS) = Equity Issuance (CFF) (must = 0)
- Year 0 Equity Tie-Out: Equity Raised (Year 0) = Beginning Equity Capital (Year 1)

**Section 6c: NOL Schedule**
- Beginning NOL (Year 1 / Formation) = 0 (new business starts with zero NOL)
- NOL increases only when EBT < 0 (losses must be realized to generate NOL)
- DTA ties to BS (NOL Schedule DTA = BS Deferred Tax Asset)
- NOL utilization ≤ 80% of EBT (post-2017 federal limitation)
- NOL balance is non-negative (cannot utilize more than available)
- NOL generated only when EBT < 0
- Tax expense = 0 when taxable income ≤ 0

**Section 7: Scenario Hierarchy**
- Absolute metrics: Upside > Base > Downside (NI, EBITDA, FCF)
- Margins: Upside > Base > Downside (GM%, EBITDA%, NI%)
- Credit metrics: Upside < Base < Downside for leverage (inverted)

**Section 8: Formula Integrity**
- COGS, S&M, G&A, R&D, SBC driven by % of Revenue (no hardcodes)
- Consistent formulas across projection years
- No #REF!, #DIV/0!, #VALUE! errors

**Section 9: Credit Metric Thresholds**
- Flag metrics as Green/Yellow/Red based on covenant thresholds
- Summary of any red flags

### Master Check Formula

Aggregate all section statuses into a single master check:
- If all sections pass → "✓ ALL CHECKS PASS"
- If any section fails → "✗ ERRORS DETECTED - REVIEW BELOW"

### Quick Debug Workflow

When Master Status shows errors:
1. Scroll to find red-highlighted sections
2. Identify which check category has failures
3. Navigate to source tab to investigate
4. Fix the underlying issue
5. Return to Checks tab to verify resolution

## Cell Comments for TradingView Data

**Every hardcoded value from TradingView needs this format:**

"Source: TradingView MCP [tool_name], [Date], [Field Name]"

**Examples:**

| Item | Source Comment |
|------|----------------|
| Revenue | Source: TradingView MCP get_financial_statements, 2025-10-12, total_revenue |
| COGS | Source: TradingView MCP get_fundamentals, 2025-10-12, cost_of_goods_fy_h |
| Cash | Source: TradingView MCP get_fundamentals, 2025-10-12, cash_n_short_term_invest_fq |
| CapEx | Source: TradingView MCP get_fundamentals, 2025-10-12, capital_expenditures_fq_h |
| Working Capital | Source: TradingView MCP get_fundamentals, 2025-10-12, calculated from current assets/liabilities |

## Formula Recalculation (MANDATORY)

After creating or modifying the Excel model, **recalculate all formulas** using the recalc.py script:

```bash
python recalc.py [path_to_excel_file] [timeout_seconds]
```

Example:
```bash
python recalc.py AAPL_3Statement_Model_2025-10-12.xlsx 30
```

The script will:
- Recalculate all formulas in all sheets using LibreOffice
- Scan ALL cells for Excel errors (#REF!, #DIV/0!, #VALUE!, #NAME?, #NULL!, #NUM!, #N/A)
- Return detailed JSON with error locations and counts

**Expected output format:**
```json
{
  "status": "success",
  "total_errors": 0,
  "total_formulas": 42
}
```

**Fix all errors** and re-run recalc.py until status is "success" before delivering the model.

## Summary

TradingView MCP provides **85% coverage** for 3-statement models. The remaining gaps may require user input for:

1. **Detailed line items**: Goodwill impairment, deferred tax breakdowns, pension obligations
2. **Transaction details**: Dividend policy changes, equity issuance timing
3. **Management guidance**: Forward-looking assumptions beyond historical data

Always prompt users transparently when data gaps exist, providing context and standard values as options.
