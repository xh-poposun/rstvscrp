---
name: lbo-model
description: Build LBO (Leveraged Buyout) models using TradingView MCP as the sole data source. Retrieves target company financials, debt structure, and market data to construct professional LBO models with Sources & Uses, Operating Projections, Debt Schedule, and Returns Analysis (IRR/MOIC). Use for private equity transactions, deal materials, or investment committee presentations.
---

---

## TEMPLATE REQUIREMENT

**This skill uses templates for LBO models. Always check for an attached template file first.**

Before starting any LBO model:
1. **If a template file is attached/provided**: Use that template's structure exactly - copy it and populate with the user's data
2. **If no template is attached**: Ask the user: *"Do you have a specific LBO template you'd like me to use? If not, I can use the standard template which includes Sources & Uses, Operating Model, Debt Schedule, and Returns Analysis."*
3. **If using the standard template**: Copy `examples/LBO_Model.xlsx` as your starting point and populate it with the user's assumptions

**IMPORTANT**: When a file like `LBO_Model.xlsx` is attached, you MUST use it as your template - do not build from scratch. Even if the template seems complex or has more features than needed, copy it and adapt it to the user's requirements. Never decide to "build from scratch" when a template is provided.

---

## CRITICAL INSTRUCTIONS FOR CLAUDE - READ FIRST

### Environment: Office JS vs Python

**If running inside Excel (Office Add-in / Office JS environment):**
- Use Office JS (`Excel.run(async (context) => {...})`) directly — do NOT use Python/openpyxl
- Write formulas via `range.formulas = [["=B5*B6"]]` — Office JS formulas recalculate natively in the live workbook
- The same formulas-over-hardcodes rule applies: set `range.formulas`, never `range.values` for anything that should be a calculation
- Use `range.format.font.color` / `range.format.fill.color` for the blue/black/purple/green convention
- No separate recalc step needed — Excel handles calculation natively
- **Merged cell pitfall:** Do NOT call `.merge()` then set `.values` on the merged range (throws `InvalidArgument` — range still reports original dimensions). Instead: write value to top-left cell alone (`ws.getRange("A7").values = [["SOURCES & USES"]]`), then merge + format the full range (`ws.getRange("A7:F7").merge(); ws.getRange("A7:F7").format.fill.color = "#1F4E79";`)

**If generating a standalone .xlsx file (no live Excel session):**
- Use Python/openpyxl as described below
- Write formula strings (`ws["D20"] = "=B5*B6"`), then run `recalc.py` before delivery

The rest of this skill is written with openpyxl examples, but the same principles apply to Office JS — just translate the API calls.

### Core Principles
* **Every calculation must be an Excel formula** - NEVER compute values in Python and hardcode results into cells. When using openpyxl, write `cell.value = "=B5*B6"` (formula string), NOT `cell.value = 1250` (computed result). The model must be dynamic and update when inputs change.
* **Use the template structure** - Follow the organization in `examples/LBO_Model.xlsx` or the user's provided template. Do not invent your own layout.
* **Use proper cell references** - All formulas should reference the appropriate cells. Never type numbers that should come from other cells.
* **Maintain sign convention consistency** - Follow whatever sign convention the template uses (some use negative for outflows, some use positive). Be consistent throughout.
* **Work section by section, verify with user at each step** - Complete one section fully, show the user what was built, run the section's verification checks, and get confirmation BEFORE moving to the next section. Do NOT build the entire model end-to-end and then present it — later sections depend on earlier ones, so catching a mistake in Sources & Uses after the returns are already built means rework everywhere.

### Formula Color Conventions
* **Blue (0000FF)**: Hardcoded inputs - typed numbers that don't reference other cells
* **Black (000000)**: Formulas with calculations - any formula using operators or functions (`=B4*B5`, `=SUM()`, `=-MAX(0,B4)`)
* **Purple (800080)**: Links to cells on the **same tab** - direct references with no calculation (`=B9`, `=B45`)
* **Green (008000)**: Links to cells on **different tabs** - cross-sheet references (`=Assumptions!B5`, `='Operating Model'!C10`)

### Fill Color Palette — Professional Blues & Greys (Default unless user/template specifies otherwise)
* **Keep it minimal** — only use blues and greys for cell fills. Do NOT introduce greens, yellows, reds, or multiple accents. A professional LBO model uses restraint.
* **Default fill palette:**
* **Section headers** (Sources & Uses, Operating Model, etc.): Dark blue `#1F4E79` with white bold text
* **Column headers** (Year 1, Year 2, etc.): Light blue `#D9E1F2` with black bold text
* **Input cells**: Light grey `#F2F2F2` (or just white) — the blue *font* is the signal, fill is secondary
* **Formula/calculated cells**: White, no fill
* **Key outputs** (IRR, MOIC, Exit Equity): Medium blue `#BDD7EE` with black bold text
* **That's the whole palette.** 3 blues + 1 grey + white. If the template uses its own colors, follow the template instead.
* Note: The blue/black/purple/green **font** colors above are for distinguishing inputs vs formulas vs links. Those are separate from the **fill** palette here — both work together.

### Number Formatting Standards
* **Currency**: `$#,##0;($#,##0);"-"` or `$#,##0.0` depending on template
* **Percentages**: `0.0%` (one decimal)
* **Multiples**: `0.0"x"` (one decimal)
* **MOIC/Detailed Ratios**: `0.00"x"` (two decimals for precision)
* **All numeric cells**: Right-aligned

---

## DATA SOURCES - TRADINGVIEW MCP ONLY

This skill uses **TradingView MCP** as the sole data source. No PitchBook, Daloopa, or other paid MCP servers.

### Available TradingView MCP Tools

| Tool | Purpose | Key Fields |
|------|---------|------------|
| `get_fundamentals` | Financial metrics | net_debt_fq, ebitda, ebitda_interst_cover_fy, debt_to_equity_current, free_cash_flow_ttm, market_cap_basic |
| `get_debt_maturity` | Existing debt structure | Debt maturity schedule from SEC EDGAR |
| `get_quote` | Current market data | price, market_cap |

### TradingView Field Mapping for LBO

| Required Field | TradingView Source | Coverage | Gap Handling |
|---------------|-------------------|----------|--------------|
| Net Debt | `get_fundamentals` → `net_debt_fq` | 90% | Prompt if missing |
| EBITDA | `get_fundamentals` → `ebitda` | 100% | Direct use |
| Interest Coverage | `get_fundamentals` → `ebitda_interst_cover_fy` | 85% | Prompt if missing |
| Debt/Equity | `get_fundamentals` → `debt_to_equity_current` | 90% | Prompt if missing |
| Free Cash Flow | `get_fundamentals` → `free_cash_flow_ttm` | 100% | Direct use |
| Market Cap | `get_quote` → `market_cap` | 100% | Direct use |
| Current Price | `get_quote` → `price` | 100% | Direct use |
| Debt Maturity | `get_debt_maturity` | 80% | Prompt if missing |
| Purchase Price | User input | 0% | **PROMPT USER** |
| Debt Financing Terms | User input | 0% | **PROMPT USER** |
| Exit Multiple | User assumption | 0% | **PROMPT USER** |
| Hold Period | User assumption | 0% | **PROMPT USER** |

**Coverage: 75%** - Transaction assumptions require user input

---

## LBO TRANSACTION ASSUMPTIONS - USER PROMPTS

TradingView provides target company financials. Transaction structure requires user input:

### 1. Purchase Price Premium

```markdown
## Purchase Price Premium

TradingView provides the current market cap. For an LBO, you need to specify the purchase price premium:

**Current Market Cap**: $[X] million (from TradingView)
**Current Share Price**: $[X] (from TradingView)

**Control Premium Options:**
1. **Conservative**: 15-20% (distressed/forced sale)
2. **Standard**: 20-30% (typical control premium)
3. **Aggressive**: 30-40% (competitive auction)

**Enter premium percentage**: ___%

**Calculated Equity Purchase Price**: $[Market Cap × (1 + Premium)]
```

### 2. Debt Financing Structure

```markdown
## Debt Financing Structure

Specify the capital structure for the LBO:

**Total Sources Required**: $[Equity Purchase Price + Transaction Fees + Existing Net Debt]

**Debt Financing Mix:**
| Tranche | % of Purchase Price | Interest Rate | Tenor |
|---------|---------------------|---------------|-------|
| Senior Debt (Revolver) | ___% | ___% | ___ years |
| Senior Debt (Term Loan A) | ___% | ___% | ___ years |
| Senior Debt (Term Loan B) | ___% | ___% | ___ years |
| Mezzanine / Subordinated | ___% | ___% | ___ years |
| **Total Debt** | ___% | | |
| **Equity Contribution** | ___% | | |

**Standard Ranges:**
- Total Debt/EBITDA: 4.0x - 6.0x (conservative to aggressive)
- Senior Debt: 3.0x - 4.5x EBITDA
- Mezzanine: 0.5x - 1.5x EBITDA
- Equity: 30% - 50% of total sources
```

### 3. Exit Assumptions

```markdown
## Exit Assumptions

**Hold Period**: ___ years (typically 4-6 years)

**Exit Multiple**: ___x EBITDA
- Current trading multiple: [X]x (from TradingView)
- Conservative exit: [Current - 1x]
- Base case exit: [Current multiple]
- Optimistic exit: [Current + 1x]

**Exit Year EBITDA**: $[Projected] million (from operating model)
**Exit Enterprise Value**: $[EBITDA × Exit Multiple]

**Net Debt at Exit**: $[Projected] million (from debt schedule)
**Exit Equity Value**: $[Exit EV - Net Debt at Exit]
```

### 4. Operating Model Assumptions

```markdown
## Operating Model Assumptions

**Revenue Growth (Years 1-5):**
- Year 1: ___%
- Year 2: ___%
- Year 3: ___%
- Year 4: ___%
- Year 5: ___%

**EBITDA Margin:**
- Current: [X]% (from TradingView)
- Target (Year 5): ___%

**CapEx (% of Revenue)**: ___% (typically 3-5%)
**D&A (% of Revenue)**: ___% (typically 2-4%)
**Working Capital (% of Revenue Change)**: ___% (typically -2% to +2%)

**Cost Synergies (if applicable)**: $___ million annually
**Revenue Synergies (if applicable)**: $___ million annually
```

---

## LBO MODEL STRUCTURE

### Standard LBO Model Sections

1. **Transaction Summary** - Key metrics at a glance
2. **Sources & Uses** - How the deal is financed
3. **Pro Forma Balance Sheet** - Opening balance sheet adjustments
4. **Operating Model** - 5-year projections
5. **Debt Schedule** - Debt paydown and interest
6. **Returns Analysis** - IRR and MOIC calculations
7. **Sensitivity Tables** - Returns under different scenarios

---

## FILLING FORMULAS - LBO SPECIFIC

### Sources & Uses Section

**Sources must equal Uses (the plug is typically the equity contribution).**

**Sources:**
```
Senior Debt (Revolver)     = [Input]
Senior Debt (Term Loan A)  = [Input]
Senior Debt (Term Loan B)  = [Input]
Mezzanine Debt             = [Input]
Equity Contribution        = [Plug: Total Uses - Total Debt]
Total Sources              = SUM(above)
```

**Uses:**
```
Equity Purchase Price      = Market Cap × (1 + Premium%)
Existing Net Debt          = From TradingView get_fundamentals → net_debt_fq
Transaction Fees           = [Input: typically 2-4% of EV]
Financing Fees             = [Input: typically 1-2% of debt raised]
Total Uses                 = SUM(above)
```

**Verification:** Sources = Uses (exactly)

### Operating Model Projections

**Revenue Build:**
```
Revenue Year 1 = LTM Revenue × (1 + Growth Rate Year 1)
Revenue Year 2 = Revenue Year 1 × (1 + Growth Rate Year 2)
...
```

**EBITDA Build:**
```
EBITDA = Revenue × EBITDA Margin
```

**Free Cash Flow:**
```
EBITDA
(-) D&A
= EBIT
(-) Taxes (EBIT × Tax Rate)
= NOPAT
(+) D&A
(-) CapEx
(-) Δ Working Capital
(-) Debt Repayment (from debt schedule)
= Free Cash Flow
```

### Debt Schedule

**Key Principles:**
- Interest calculated on **Beginning Balance** (not average or ending)
- Cash sweep: Excess FCF used to pay down debt
- Respect debt priority: Revolver → Term Loan A → Term Loan B → Mezzanine
- Balances cannot go negative

**Structure:**
```
Beginning Balance
(+) Draws (if any)
(-) Mandatory Repayments
(-) Cash Sweep (optional prepayments)
= Ending Balance

Interest Expense = Beginning Balance × Interest Rate
```

### Returns Analysis (IRR and MOIC)

**Cash Flow Series:**
```
Year 0: -Equity Investment (negative)
Year 1: Dividends/Recap (if any)
Year 2: Dividends/Recap (if any)
...
Year N: Exit Proceeds (positive)
```

**IRR Calculation:**
```
=IRR(Equity CF Year 0:Year N)
```

**MOIC Calculation:**
```
Total Proceeds = Sum of all positive cash flows (dividends + exit)
MOIC = Total Proceeds / |Equity Investment|
```

**Example:**
```
Equity Investment: $(100)
Dividend Year 3: $20
Dividend Year 4: $25
Exit Proceeds Year 5: $180

Total Proceeds: $225
MOIC: 2.25x
IRR: ~18.5%
```

---

## COMMON PROBLEM AREAS

### Balancing Sections
* When two sections must equal (e.g., Sources = Uses), one item is typically the "plug" (balancing figure)
* Identify which item is the plug and calculate it as the difference

### Interest and Circular References
* Interest calculations can create circularity if they reference balances affected by cash flows
* Use **Beginning Balance** (not average or ending) to break circular references
* Pattern: Interest → Cash Flow → Paydown → Ending Balance (if interest uses ending balance, this circles back)

### Debt Paydown / Cash Sweeps
* When multiple debt tranches exist, there's usually a priority order
* Cash sweep should respect the priority waterfall
* Balances cannot go negative - use MAX or MIN functions appropriately

### Returns Calculations (IRR/MOIC)
* Cash flows must have correct signs: Investment = negative, Proceeds = positive
* If using XIRR, need corresponding dates
* If using IRR, cash flows should be in consecutive periods
* MOIC = Total Proceeds / Total Investment

### Sensitivity Tables
* **Use ODD dimensions** (5×5 or 7×7) — never 4×4 or 6×6. Odd dimensions guarantee a true center cell.
* **Center cell = base case.** Build the row and column axis values symmetrically around the model's actual assumptions (e.g., if base entry multiple = 10.0x, axis = `[8.0x, 9.0x, 10.0x, 11.0x, 12.0x]`). The center cell's IRR/MOIC MUST then equal the model's actual IRR/MOIC output — this is the proof the table is wired correctly.
* **Highlight the center cell** — medium-blue fill (`#BDD7EE`) + bold font so the base case is visually anchored.
* Excel's DATA TABLE function may not work with openpyxl — instead write explicit formulas that reference row/column headers
* Each cell should show a DIFFERENT value — if all same, formulas aren't varying correctly
* Use mixed references (e.g., `$A5` for row input, `B$4` for column input)

---

## VERIFICATION CHECKLIST - RUN AFTER COMPLETION

### Run Formula Validation
```bash
python /mnt/skills/public/xlsx/recalc.py model.xlsx
```
Must return success with zero errors.

### Section Balancing
- [ ] Sources = Uses (exactly)
- [ ] Plug items are calculated correctly as the balancing figure
- [ ] Amounts that should match across sections are consistent

### Transaction Summary
- [ ] Purchase price calculated correctly with premium
- [ ] Net debt from TradingView properly included
- [ ] Transaction fees reasonable (2-4% of EV)
- [ ] Total sources = Total uses

### Operating Projections
- [ ] Revenue builds correctly from growth assumptions
- [ ] EBITDA margin assumptions reasonable
- [ ] All cost and expense items calculated appropriately
- [ ] Subtotals and totals sum correctly
- [ ] Free cash flow positive in most years

### Debt Schedule
- [ ] Beginning balances tie to sources or prior period
- [ ] Interest calculated on appropriate balance (beginning)
- [ ] Paydowns respect cash availability and priority
- [ ] Ending balances cannot be negative
- [ ] Total debt/EBITDA within reasonable range (4-6x)

### Returns/Output Analysis
- [ ] Exit enterprise value calculated correctly
- [ ] Net debt at exit from debt schedule
- [ ] Exit equity value = Exit EV - Net Debt at Exit
- [ ] Cash flow signs are correct (negative for investment, positive for proceeds)
- [ ] IRR/MOIC formulas reference complete ranges
- [ ] Results are reasonable for the scenario (IRR typically 15-25%)

### Sensitivity Tables (if applicable)
- [ ] Grid dimensions are ODD (5×5 or 7×7) — there is a true center cell
- [ ] Row and column axis values are symmetric around the base case
- [ ] Center cell output equals the model's actual IRR/MOIC
- [ ] Center cell is highlighted (medium-blue fill `#BDD7EE`, bold font)
- [ ] Row and column headers contain appropriate input values
- [ ] Each data cell contains a formula (not hardcoded)
- [ ] Each data cell shows a DIFFERENT value
- [ ] Values move in expected directions (higher exit multiple → higher IRR, etc.)

### Formatting
- [ ] Hardcoded inputs are blue (0000FF)
- [ ] Calculated formulas are black (000000)
- [ ] Same-tab links are purple (800080)
- [ ] Cross-tab links are green (008000)
- [ ] All numbers are right-aligned
- [ ] Appropriate number formats applied throughout
- [ ] No cells show error values (#REF!, #DIV/0!, #VALUE!, #NAME?)

### Logical Sanity Checks
- [ ] Numbers are reasonable order of magnitude
- [ ] Trends make sense (growth, decline, stabilization as expected)
- [ ] No obviously wrong values (negative where should be positive, impossible percentages, etc.)
- [ ] Key outputs are within reasonable ranges for the type of analysis
- [ ] IRR between 10-30% (typical LBO range)
- [ ] MOIC between 1.5x - 3.0x (typical LBO range)

---

## COMMON ERRORS TO AVOID

| Error | What Goes Wrong | How to Fix |
|-------|-----------------|------------|
| Hardcoding calculated values | Model doesn't update when inputs change | Always use formulas that reference source cells |
| Wrong cell references after copying | Formulas point to wrong cells | Verify all links, use appropriate $ anchoring |
| Circular reference errors | Model can't calculate | Use beginning balances for interest-type calcs, break the circle |
| Sources ≠ Uses | Totals that should match don't | Ensure one item is the plug (calculated as difference) |
| Negative balances where impossible | Paying/using more than available | Use MAX(0, ...) or MIN functions appropriately |
| IRR/return errors | Wrong signs or incomplete ranges | Check cash flow signs and ensure formula covers all periods |
| Sensitivity table shows same value | Formula not varying with inputs | Check cell references - need mixed references ($A5, B$4) |
| Roll-forwards don't tie | Beginning ≠ prior ending | Verify links between periods |
| Inconsistent sign conventions | Additions become subtractions or vice versa | Follow template's convention consistently throughout |

---

## WORKING WITH THE USER — SECTION-BY-SECTION CHECKPOINTS

* **If the template structure is unclear**, ask before proceeding
* **If the user's requirements conflict with the template**, confirm their preference
* **After completing each major section**, STOP and verify with the user before continuing:
- **After Sources & Uses** → show the balanced table, confirm the plug is correct, get sign-off before building the operating model
- **After Operating Model / Projections** → show the projected P&L, confirm growth rates and margins look right, get sign-off before the debt schedule
- **After Debt Schedule** → show beginning/ending balances and interest, confirm the waterfall logic, get sign-off before returns
- **After Returns (IRR/MOIC)** → show the cash flow series and outputs, confirm signs and ranges, get sign-off before sensitivity tables
- **After Sensitivity Tables** → show that each cell varies, confirm the base case lands where expected
* **If errors are found during verification**, fix them before moving to the next section
* **Show your work** - explain key formulas or assumptions when helpful
* **Never present a completed model without having checked in at each section** — it's faster to catch a wrong cell reference at the source than to trace it backwards from a broken IRR

---

## DATA RETRIEVAL WORKFLOW

### Step 1: Fetch Target Company Data from TradingView

```python
# Fetch fundamentals
fundamentals = mcp_call("get_fundamentals", {"symbol": symbol})

# Fetch current quote
quote = mcp_call("get_quote", {"symbol": symbol})

# Fetch debt maturity schedule
debt_maturity = mcp_call("get_debt_maturity", {"symbol": symbol})
```

### Step 2: Extract Key Metrics

```python
key_metrics = {
    "net_debt": fundamentals.get("net_debt_fq"),
    "ebitda": fundamentals.get("ebitda"),
    "interest_coverage": fundamentals.get("ebitda_interst_cover_fy"),
    "debt_to_equity": fundamentals.get("debt_to_equity_current"),
    "free_cash_flow": fundamentals.get("free_cash_flow_ttm"),
    "market_cap": quote.get("market_cap"),
    "current_price": quote.get("price"),
}
```

### Step 3: Prompt User for Transaction Assumptions

After retrieving TradingView data, prompt user for:
1. Purchase price premium
2. Debt financing structure
3. Exit assumptions
4. Operating model assumptions

---

**This skill produces investment banking-quality LBO models using TradingView MCP as the sole data source. It fills templates with correct formulas, proper formatting, and validated calculations while ensuring financial accuracy and professional presentation standards.**
