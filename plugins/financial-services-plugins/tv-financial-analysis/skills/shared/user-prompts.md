---
name: user-prompts
description: Standardized user prompt templates for TradingView data gaps. Reusable prompt patterns for missing financial data, assumptions input, data source alternatives, and sector-specific metrics across all tv-financial-analysis skills.
---

# User Prompting System for TradingView Data Gaps

## Overview

This document provides standardized prompt templates for handling data gaps when using TradingView MCP as the primary data source. TradingView provides excellent coverage for public company financials (85-95%), but certain specialized data requires user input.

### Coverage Summary by Workflow

| Workflow | TradingView Coverage | Primary Gaps |
|----------|---------------------|--------------|
| DCF Model | 92% | Risk-free rate, MRP, terminal growth |
| Comps Analysis | 95% | Sector-specific metrics (ARR, NDR, etc.) |
| LBO Model | 75% | Transaction assumptions, deal terms |
| 3-Statement Model | 85% | Detailed line items, management guidance |
| Investment Banking | 70% | M&A transaction comps, deal rumors |
| Equity Research | 85% | Earnings transcripts, management commentary |
| Private Equity | 60% | Private company data, transaction terms |

### Prompt Philosophy

1. **Transparent**: Clearly explain why data is needed and what TradingView cannot provide
2. **Actionable**: Provide clear options with standard values
3. **Flexible**: Allow users to skip if not critical
4. **Documented**: Record assumptions in model outputs

---

## Prompt Format Standards

### Standard Template Structure

```markdown
## ⚠️ Data Gap: [Field Name]

**Why this is needed:** [1-2 sentence explanation of how this data is used in the analysis]

**TradingView Coverage:** [X%] - [Brief explanation of why TV doesn't have this]

**Options:**
1. **[Standard Option]** - [Description with typical value/range]
2. **[Conservative Option]** - [Description with typical value/range]
3. **[Custom Value]** - Enter your own: ___
4. **[Skip]** - Use default assumption or proceed without

**Enter your choice (1-4) or provide custom value:**

---
*Source: This prompt triggered because TradingView does not provide [field] data.*
*Assumption recorded in: [Model/Document location]*
```

### Prompt Design Principles

| Principle | Implementation |
|-----------|----------------|
| Context First | Always explain why the data is needed |
| Standard Values | Provide industry-standard options |
| Range Guidance | Include typical ranges for numerical inputs |
| Skip Option | Always allow skipping non-critical data |
| Documentation | Note where assumption will be recorded |

### Response Handling

```python
# Standard pattern for handling user prompt responses
def handle_user_prompt(field_name, options, default=None):
    """
    Process user response for data gap prompt.
    
    Returns: (value, source_note)
    """
    # Option 1-3: Predefined values
    if response in ["1", "2", "3"]:
        value = options[int(response) - 1]["value"]
        source_note = f"User selected: {options[int(response) - 1]['label']}"
    # Option 4: Custom value
    elif response == "4" and custom_input:
        value = parse_input(custom_input)
        source_note = f"User provided custom: {custom_input}"
    # Skip
    else:
        value = default
        source_note = f"Default assumption used: {default}"
    
    return value, source_note
```

---

## DCF Model Data Gap Prompts

### Prompt 1: Risk-Free Rate

```markdown
## ⚠️ Data Gap: Risk-Free Rate

**Why this is needed:** The risk-free rate is a critical input for calculating the cost of equity via CAPM (Capital Asset Pricing Model). It represents the theoretical return of an investment with zero risk, typically proxied by the 10-Year Treasury yield.

**TradingView Coverage:** 0% - TradingView provides market data but not macroeconomic rates suitable for valuation.

**Current Market Context (as of 2026):**
- 10-Year Treasury Yield: ~4.2%
- Recent range: 3.8% - 4.5%

**Options:**
1. **Current Market Rate (~4.2%)** - Use current 10Y Treasury yield
2. **Conservative (4.0%)** - Slightly below current for margin of safety
3. **Custom Rate** - Enter your own: ___
4. **Skip** - Use default 4.2%

**Enter your choice (1-4) or provide custom value:**

---
*Source: This prompt triggered because TradingView does not provide risk-free rate data.*
*Assumption recorded in: WACC calculation section, cell comment on risk-free rate input*
```

### Prompt 2: Market Risk Premium (MRP)

```markdown
## ⚠️ Data Gap: Market Risk Premium

**Why this is needed:** MRP represents the excess return investors expect from the stock market over the risk-free rate. Combined with beta, it determines the equity risk premium in the CAPM formula: Cost of Equity = Risk-Free Rate + (Beta × MRP).

**TradingView Coverage:** 0% - TradingView does not provide implied or historical market risk premium estimates.

**Industry Standards:**
- Historical long-term average: 4.5% - 5.5%
- Current consensus (Damodaran): ~5.0%
- Conservative practitioners: 5.0% - 6.0%

**Options:**
1. **Consensus (5.0%)** - Based on current market implied premium
2. **Conservative (5.5%)** - Higher premium for margin of safety
3. **Aggressive (4.5%)** - Lower premium for bullish scenarios
4. **Custom Premium** - Enter your own: ___

**Enter your choice (1-4) or provide custom value:**

---
*Source: This prompt triggered because TradingView does not provide market risk premium data.*
*Assumption recorded in: WACC calculation section, cell comment on MRP input*
```

### Prompt 3: Terminal Growth Rate

```markdown
## ⚠️ Data Gap: Terminal Growth Rate

**Why this is needed:** The terminal growth rate represents the perpetual growth rate of free cash flows beyond the explicit forecast period. It significantly impacts the terminal value calculation (often 60-80% of total enterprise value).

**TradingView Coverage:** 0% - This is a valuation assumption, not market data.

**Industry Guidelines:**
- **Conservative:** 2.0% - 2.5% (near long-term GDP growth)
- **Moderate:** 2.5% - 3.0% (typical for mature companies)
- **Aggressive:** 3.0% - 3.5% (for companies with strong competitive advantages)

**⚠️ Critical Constraint:** Terminal growth must be LESS than WACC (typically g < 4% when WACC = 8-10%)

**Options:**
1. **Conservative (2.0%)** - Near GDP growth, minimal competitive advantage
2. **Moderate (2.5%)** - Standard assumption for mature companies
3. **Growth (3.0%)** - For companies with strong moats
4. **Custom Rate** - Enter your own: ___

**Enter your choice (1-4) or provide custom value:**

---
*Source: This prompt triggered because terminal growth is a valuation assumption.*
*Assumption recorded in: Terminal value section, sensitivity table axis, cell comment*
```

### Prompt 4: Cost of Debt (if credit rating unavailable)

```markdown
## ⚠️ Data Gap: Cost of Debt

**Why this is needed:** Cost of debt is required for the WACC calculation. TradingView provides credit ratings for 90% of rated companies, but unrated companies or recent issuances may lack data.

**TradingView Coverage:** 90% via `get_credit_ratings` - May be unavailable for unrated companies.

**Estimation Guidelines by Credit Profile:**
- Investment Grade (A- to AAA): Risk-free rate + 1.0% - 2.0%
- Crossover (BBB- to BB+): Risk-free rate + 2.0% - 4.0%
- High Yield (B+ to BB-): Risk-free rate + 4.0% - 8.0%
- Distressed (CCC+ and below): Risk-free rate + 8.0%+

**Options:**
1. **Investment Grade (+1.5%)** - For strong balance sheets
2. **Crossover (+3.0%)** - For moderate leverage
3. **High Yield (+6.0%)** - For leveraged companies
4. **Custom Spread** - Enter spread over risk-free: ___

**Enter your choice (1-4) or provide custom value:**

---
*Source: This prompt triggered because credit rating data is unavailable.*
*Assumption recorded in: WACC calculation, debt schedule, cell comment*
```

---

## Comps Analysis Data Gap Prompts

### Prompt 5: Sector-Specific Metrics (SaaS/Tech)

```markdown
## ⚠️ Data Gap: SaaS-Specific Metrics

**Why this is needed:** SaaS companies are often valued on metrics beyond standard financials. These metrics provide insight into revenue quality, customer retention, and growth sustainability.

**TradingView Coverage:** 0% - TradingView provides GAAP financials, not SaaS operating metrics.

**Common SaaS Metrics:**
- **ARR (Annual Recurring Revenue)** - Subscription revenue run rate
- **Net Dollar Retention (NDR)** - Revenue retention including expansions
- **Gross Dollar Retention (GDR)** - Revenue retention excluding expansions
- **Customer Acquisition Cost (CAC)** - Cost to acquire new customers
- **CAC Payback Period** - Months to recover CAC
- **Net Revenue Retention** - NDR including churn

**Options:**
1. **Provide Manually** - Enter metrics from company filings/investor presentations
2. **Use Public Comparable** - Use median from similar public SaaS companies
3. **Skip SaaS Metrics** - Proceed with standard financial multiples only
4. **Request Alternative** - Use different metric set

**If providing manually, enter values:**
- ARR: $___ million
- NDR: ___%
- CAC Payback: ___ months

**Enter your choice (1-4):**

---
*Source: This prompt triggered because TradingView does not provide SaaS operating metrics.*
*Assumption recorded in: Comps summary page, footnotes*
```

### Prompt 6: Sector-Specific Metrics (Retail/E-Commerce)

```markdown
## ⚠️ Data Gap: Retail/E-Commerce Metrics

**Why this is needed:** Retail and e-commerce companies have unique operating metrics that drive valuation. Same-store sales indicate organic growth, while GMV shows platform scale.

**TradingView Coverage:** 0% - TradingView does not provide retail-specific operating metrics.

**Common Retail Metrics:**
- **Same-Store Sales Growth (SSS)** - Growth from existing locations
- **Gross Merchandise Value (GMV)** - Total transaction volume
- **Active Buyers/Users** - Customer base size
- **Average Order Value (AOV)** - Revenue per transaction
- **Customer Lifetime Value (LTV)** - Long-term customer value
- **Take Rate** - Platform revenue / GMV

**Options:**
1. **Provide Manually** - Enter from company filings/earnings calls
2. **Use Industry Average** - Use sector median from external sources
3. **Skip Retail Metrics** - Proceed with standard multiples
4. **Focus on GMV Only** - Just provide GMV for platform companies

**If providing manually, enter values:**
- Same-Store Sales Growth: ___%
- GMV (if applicable): $___ million
- Active Buyers: ___ million

**Enter your choice (1-4):**

---
*Source: This prompt triggered because TradingView does not provide retail operating metrics.*
*Assumption recorded in: Comps summary page, footnotes*
```

### Prompt 7: Sector-Specific Metrics (Healthcare/Biotech)

```markdown
## ⚠️ Data Gap: Healthcare/Biotech Metrics

**Why this is needed:** Healthcare companies, especially biotech and pharma, require specialized metrics for valuation. Pipeline value and R&D efficiency are critical for pre-revenue or early-stage companies.

**TradingView Coverage:** 0% - TradingView does not provide pipeline or clinical trial data.

**Common Healthcare Metrics:**
- **Pipeline Value** - Risk-adjusted NPV of drug candidates
- **R&D Spend by Program** - Allocation across pipeline
- **Clinical Trial Milestones** - Phase transitions, readouts
- **Patient Population (TAM)** - Addressable market size
- **Peak Sales Estimates** - Revenue potential per drug
- **Probability of Success** - Phase-specific success rates

**Options:**
1. **Provide Pipeline Data** - Enter key pipeline metrics
2. **Use External Research** - Reference sell-side research estimates
3. **Skip Pipeline Metrics** - Use standard financials only
4. **Focus on Commercial** - For commercial-stage companies only

**If providing pipeline data:**
- Lead candidate phase: ___
- Peak sales estimate: $___ million
- Probability of approval: ___%

**Enter your choice (1-4):**

---
*Source: This prompt triggered because TradingView does not provide healthcare pipeline data.*
*Assumption recorded in: Comps summary page, risk factors*
```

---

## LBO Model Data Gap Prompts

### Prompt 8: Purchase Price Premium

```markdown
## ⚠️ Data Gap: Purchase Price Premium

**Why this is needed:** LBO analysis requires determining the entry price. The purchase price premium over the current market price reflects the control premium and strategic value.

**TradingView Coverage:** 0% - TradingView provides current price, not transaction assumptions.

**Typical Premium Ranges:**
- **Conservative (15-20%)** - For stable, mature companies
- **Standard (20-30%)** - Typical control premium
- **Aggressive (30-40%)** - For strategic acquisitions, competitive bidding
- **Auction (40%+)** - Hot sectors, multiple bidders

**Current Market Cap: $___ million (from TradingView)**

**Options:**
1. **Conservative (+20%)** - Total equity value: $___ million
2. **Standard (+30%)** - Total equity value: $___ million
3. **Aggressive (+40%)** - Total equity value: $___ million
4. **Custom Premium** - Enter percentage: ___%

**Enter your choice (1-4) or provide custom premium:**

---
*Source: This prompt triggered because LBO transaction assumptions require user input.*
*Assumption recorded in: Sources & Uses, returns analysis, sensitivity tables*
```

### Prompt 9: Debt Financing Structure

```markdown
## ⚠️ Data Gap: Debt Financing Structure

**Why this is needed:** LBO returns are highly sensitive to leverage. The debt financing structure determines interest expense, debt repayment capacity, and equity contribution.

**TradingView Coverage:** 0% - TradingView provides existing debt, not transaction financing assumptions.

**Standard LBO Capital Structures:**
- **Conservative (3.0x-4.0x EBITDA)** - Lower risk, higher equity contribution
- **Standard (4.0x-5.0x EBITDA)** - Typical buyout leverage
- **Aggressive (5.0x-6.0x EBITDA)** - Higher returns, higher risk
- **Covenant-Lite (6.0x+ EBITDA)** - Hot credit markets only

**Typical Debt Mix:**
- Revolver: 0-10% of total debt
- Term Loan A: 20-30%
- Term Loan B: 40-50%
- Mezzanine/HY Bonds: 10-20%

**Options:**
1. **Conservative Structure** - 3.5x EBITDA, 35% equity
2. **Standard Structure** - 4.5x EBITDA, 30% equity
3. **Aggressive Structure** - 5.5x EBITDA, 25% equity
4. **Custom Structure** - Define your own:
   - Total Debt/EBITDA: ___x
   - Equity Contribution: ___%

**Enter your choice (1-4):**

---
*Source: This prompt triggered because LBO financing structure requires user assumptions.*
*Assumption recorded in: Sources & Uses, debt schedule, returns analysis*
```

### Prompt 10: Exit Multiple Assumptions

```markdown
## ⚠️ Data Gap: Exit Multiple Assumptions

**Why this is needed:** LBO returns depend on the exit valuation. The exit multiple (typically EV/EBITDA) determines the sale proceeds and ultimately the IRR and MOIC.

**TradingView Coverage:** 0% - Exit assumptions are transaction-specific.

**Exit Multiple Guidelines:**
- **Conservative:** Entry multiple - 0.5x to - 1.0x (multiple contraction)
- **Standard:** Entry multiple (flat)
- **Aggressive:** Entry multiple + 0.5x to + 1.0x (multiple expansion)
- **IPO Exit:** Use forward multiples for public comparables

**Hold Period Considerations:**
- 3 years: Market conditions likely similar
- 5 years: Moderate multiple expansion possible
- 7 years: Significant business transformation expected

**Options:**
1. **Conservative Exit** - Exit at 8.0x EBITDA (entry - 1.0x)
2. **Standard Exit** - Exit at 9.0x EBITDA (flat to entry)
3. **Aggressive Exit** - Exit at 10.0x EBITDA (entry + 1.0x)
4. **Custom Exit** - Enter exit multiple: ___x

**Hold Period:** ___ years

**Enter your choice (1-4) and hold period:**

---
*Source: This prompt triggered because exit assumptions require user input.*
*Assumption recorded in: Returns analysis, exit assumptions, sensitivity tables*
```

### Prompt 11: Operating Model Assumptions

```markdown
## ⚠️ Data Gap: Operating Model Assumptions

**Why this is needed:** LBO value creation comes from operational improvements. Revenue growth, margin expansion, and working capital efficiency drive cash flow available for debt repayment.

**TradingView Coverage:** 50% - TradingView provides historicals, not projections.

**Typical LBO Operating Assumptions:**

**Revenue Growth (Years 1-5):**
- Conservative: 3-5% annually
- Standard: 5-8% annually
- Aggressive: 8-12% annually

**EBITDA Margin Expansion:**
- Conservative: 50-100 bps improvement
- Standard: 100-200 bps improvement
- Aggressive: 200-300 bps improvement

**Working Capital:**
- As % of revenue change: 10-15%

**Options:**
1. **Conservative Case** - 4% growth, 100bps margin expansion
2. **Standard Case** - 6% growth, 150bps margin expansion
3. **Aggressive Case** - 10% growth, 250bps margin expansion
4. **Custom Assumptions** - Enter your own:
   - Revenue CAGR: ___%
   - EBITDA margin improvement: ___ bps

**Enter your choice (1-4):**

---
*Source: This prompt triggered because operating projections require user assumptions.*
*Assumption recorded in: Operating model, value creation bridge*
```

---

## 3-Statement Model Data Gap Prompts

### Prompt 12: Detailed Line Items

```markdown
## ⚠️ Data Gap: Detailed Line Items

**Why this is needed:** TradingView provides comprehensive financial statement data, but some detailed breakdowns (goodwill, deferred taxes, pension items) may require additional granularity for accurate modeling.

**TradingView Coverage:** 85% - Core IS/BS/CF lines covered; detailed breakdowns may be aggregated.

**Common Detailed Items:**
- **Goodwill:** Impairment history, allocation by segment
- **Deferred Taxes:** Breakdown of DTA/DTL components
- **Pension Obligations:** Funded status, assumptions
- **Intangibles:** Amortization schedules, useful lives
- **Stock-Based Compensation:** Equity vs cash breakdown
- **Restructuring:** One-time charges, provisions

**Options:**
1. **Use Aggregates** - Use TradingView totals (recommended for most cases)
2. **Provide from 10-K** - Enter detailed breakdowns from filings
3. **Skip Details** - Model at summary level only
4. **Flag for Review** - Note assumption and continue

**If providing details, enter key items:**
- Goodwill impairment (if any): $___ million
- SBC breakdown: $___ million

**Enter your choice (1-4):**

---
*Source: This prompt triggered because detailed line items may require manual input.*
*Assumption recorded in: Model assumptions page, cell comments*
```

### Prompt 13: Management Guidance

```markdown
## ⚠️ Data Gap: Management Guidance

**Why this is needed:** Forward-looking projections benefit from management guidance on revenue, margins, and capital allocation. TradingView provides historicals, not forward guidance.

**TradingView Coverage:** 0% - TradingView does not provide management guidance.

**Typical Guidance Categories:**
- **Revenue Guidance** - Full year or quarterly outlook
- **EBITDA/Operating Income** - Profitability targets
- **CapEx** - Capital investment plans
- **Tax Rate** - Effective tax rate guidance
- **Share Count** - Dilution expectations

**Options:**
1. **Provide Guidance** - Enter from recent earnings call/8-K
2. **Use Consensus** - Use analyst estimates as proxy
3. **Build Bottom-Up** - Derive from historical trends
4. **Skip Guidance** - Use historical averages

**If providing guidance:**
- Revenue guidance: $___ million (range: ___ to ___)
- EBITDA margin target: ___%
- CapEx guidance: $___ million

**Enter your choice (1-4):**

---
*Source: This prompt triggered because TradingView does not provide management guidance.*
*Assumption recorded in: Projection assumptions, scenario analysis*
```

---

## Investment Banking Data Gap Prompts

### Prompt 14: M&A Transaction Comps

```markdown
## ⚠️ Data Gap: M&A Transaction Comps

**Why this is needed:** M&A valuation requires precedent transaction multiples, not just trading multiples. TradingView provides current market data, not historical transaction data.

**TradingView Coverage:** 0% - TradingView does not provide M&A transaction databases.

**Required Transaction Data:**
- **Target Company** - Name, ticker (if public)
- **Transaction Date** - When deal closed/announced
- **Transaction Value** - Enterprise value paid
- **Financial Metrics** - Revenue, EBITDA at time of deal
- **Implied Multiples** - EV/Revenue, EV/EBITDA paid
- **Deal Terms** - Cash vs stock, premium paid
- **Buyer Type** - Strategic vs financial buyer

**Options:**
1. **Provide Transactions** - Enter 3-5 comparable deals
2. **Use Trading Multiples** - Acknowledge limitation, use trading comps
3. **Skip Precedent Analysis** - Focus on other valuation methods
4. **Request Research** - Reference sell-side M&A research

**If providing transactions, enter summary:**
- Deal 1: Target ___, Date ___, EV/EBITDA ___x
- Deal 2: Target ___, Date ___, EV/EBITDA ___x

**Enter your choice (1-4):**

---
*Source: This prompt triggered because TradingView does not provide M&A transaction data.*
*Assumption recorded in: Valuation summary, footnotes, disclaimers*
```

### Prompt 15: Deal Context and Rumors

```markdown
## ⚠️ Data Gap: Deal Context

**Why this is needed:** Pitch materials and CIMs require context on market activity, strategic rationale, and competitive dynamics. TradingView provides financials, not deal intelligence.

**TradingView Coverage:** 0% - TradingView does not provide M&A rumors or deal context.

**Deal Context Elements:**
- **Strategic Rationale** - Why this deal makes sense
- **Competitive Dynamics** - Other potential bidders
- **Market Timing** - Why now, market conditions
- **Synergy Opportunities** - Cost and revenue synergies
- **Rumored Terms** - Any leaked deal terms
- **Process Status** - Early stage, LOI, definitive agreement

**Options:**
1. **Provide Context** - Enter deal-specific information
2. **Use Public Information** - Reference announced deals only
3. **Skip Context** - Focus on financial analysis
4. **Generic Template** - Use standard strategic rationale

**Enter your choice (1-4):**

---
*Source: This prompt triggered because TradingView does not provide deal intelligence.*
*Assumption recorded in: Pitch deck/CIM narrative sections*
```

---

## Equity Research Data Gap Prompts

### Prompt 16: Earnings Call Transcripts

```markdown
## ⚠️ Data Gap: Earnings Call Transcripts

**Why this is needed:** Earnings analysis requires management commentary, guidance updates, and Q&A insights. TradingView provides financial data, not transcript content.

**TradingView Coverage:** 0% - TradingView does not provide earnings call transcripts.

**Transcript Key Elements:**
- **Prepared Remarks** - Management highlights
- **Guidance Updates** - Changes to outlook
- **Q&A Highlights** - Key analyst questions and answers
- **Forward Statements** - Strategic initiatives, outlook
- **Tone/Body Language** - Management confidence level

**Options:**
1. **Provide Summary** - Enter key quotes and highlights
2. **Reference External** - Link to external transcript service
3. **Skip Transcript** - Focus on financial results only
4. **Use FactSet/Bloomberg** - If available via other sources

**If providing summary, enter key points:**
- Revenue guidance: [raised/lowered/maintained] to ___
- Key quote: "___"
- Strategic update: ___

**Enter your choice (1-4):**

---
*Source: This prompt triggered because TradingView does not provide earnings transcripts.*
*Assumption recorded in: Earnings report, quotes attributed to user input*
```

### Prompt 17: Management Commentary

```markdown
## ⚠️ Data Gap: Management Commentary

**Why this is needed:** Research reports benefit from management perspectives on business trends, competitive positioning, and strategic priorities. TradingView provides quantitative data only.

**TradingView Coverage:** 0% - TradingView does not provide management commentary.

**Commentary Categories:**
- **Business Trends** - Demand trends, pricing, competition
- **Strategic Priorities** - Investment focus, M&A appetite
- **Capital Allocation** - Buybacks, dividends, debt paydown
- **Risk Factors** - Supply chain, regulation, competition
- **ESG Initiatives** - Sustainability, governance updates

**Options:**
1. **Provide Commentary** - Enter from recent investor meetings
2. **Reference Prior Research** - Use existing sell-side notes
3. **Skip Commentary** - Focus on quantitative analysis
4. **Use Public Filings** - Extract from 10-K/10-Q risk factors

**Enter your choice (1-4):**

---
*Source: This prompt triggered because TradingView does not provide management commentary.*
*Assumption recorded in: Research report, investment thesis section*
```

---

## Private Equity Data Gap Prompts

### Prompt 18: Private Company Financials

```markdown
## ⚠️ Data Gap: Private Company Financials

**Why this is needed:** Private equity workflows often involve private targets. TradingView only covers public companies, so private company data requires manual input.

**TradingView Coverage:** 0% - TradingView does not provide private company data.

**Required Financial Data:**
- **Revenue** - LTM or most recent fiscal year
- **EBITDA** - Operating profit before D&A
- **Growth Rate** - Historical and projected revenue growth
- **Margins** - Gross margin, EBITDA margin trends
- **Capital Intensity** - CapEx, working capital needs
- **Debt** - Existing debt, credit facilities

**Options:**
1. **Enter Private Data** - Provide target company financials
2. **Use Public Proxy** - Use comparable public company as proxy
3. **Hybrid Approach** - Private revenue + public margin benchmarks
4. **Request CIM** - Wait for CIM to proceed

**If entering private data:**
- Revenue: $___ million
- EBITDA: $___ million
- Growth rate: ___%
- Industry sector: ___

**Enter your choice (1-4):**

---
*Source: This prompt triggered because TradingView does not provide private company data.*
*Assumption recorded in: IC memo, target profile, valuation assumptions*
```

### Prompt 19: Transaction Terms (Private Deals)

```markdown
## ⚠️ Data Gap: Transaction Terms

**Why this is needed:** Private equity deal analysis requires transaction-specific terms including entry valuation, financing structure, and equity arrangements.

**TradingView Coverage:** 0% - TradingView does not provide private transaction terms.

**Transaction Terms:**
- **Enterprise Value** - Total purchase price
- **Equity Value** - EV minus net debt
- **Entry Multiple** - EV/EBITDA paid
- **Financing** - Debt/equity split
- **Management Equity** - Management rollover, new equity
- **Key Covenants** - Financial maintenance covenants

**Options:**
1. **Enter Terms** - Provide deal-specific terms
2. **Use Standard Assumptions** - Apply typical PE structure
3. **Build from Public** - Derive from public comparable analysis
4. **Flag for Discussion** - Note for IC discussion

**If entering terms:**
- Enterprise value: $___ million
- Entry multiple: ___x EBITDA
- Debt financing: $___ million
- Management rollover: ___%

**Enter your choice (1-4):**

---
*Source: This prompt triggered because private transaction terms require user input.*
*Assumption recorded in: IC memo, sources & uses, returns analysis*
```

### Prompt 20: Value Creation Initiatives

```markdown
## ⚠️ Data Gap: Value Creation Initiatives

**Why this is needed:** Value creation plans require specific initiatives with targets and timelines. TradingView provides benchmarking data but not deal-specific initiatives.

**TradingView Coverage:** 70% - TradingView provides peer benchmarking for targets.

**Common Value Creation Levers:**
- **Revenue Growth** - New products, geographies, channels
- **Margin Expansion** - Procurement, pricing, efficiency
- **Working Capital** - Inventory, receivables, payables
- **CapEx Efficiency** - Maintenance vs growth capex
- **Cost Reduction** - Headcount, facilities, overhead
- **M&A** - Bolt-on acquisitions

**Options:**
1. **Define Initiatives** - Enter specific value creation plan
2. **Use Benchmarks** - Apply peer median improvements
3. **Standard Playbook** - Use typical PE value creation
4. **TBD** - Note for post-close planning

**If defining initiatives:**
- Initiative 1: ___, Target: $___ million, Timeline: ___ years
- Initiative 2: ___, Target: $___ million, Timeline: ___ years

**Enter your choice (1-4):**

---
*Source: This prompt triggered because value creation initiatives require deal-specific input.*
*Assumption recorded in: Value creation plan, IC memo, LP reporting*
```

---

## Integration Guide

### How to Use These Prompts in Skills

#### 1. Detect Data Gap

```python
def check_data_gap(field_name, tradingview_data):
    """Check if data is available from TradingView."""
    value = tradingview_data.get(field_name)
    if value is None or value == 0:
        return True  # Gap detected
    return False
```

#### 2. Trigger Prompt

```python
def prompt_for_data_gap(gap_type, context):
    """Trigger appropriate user prompt based on gap type."""
    prompts = {
        "risk_free_rate": RISK_FREE_RATE_PROMPT,
        "mrp": MRP_PROMPT,
        "terminal_growth": TERMINAL_GROWTH_PROMPT,
        # ... etc
    }
    
    prompt_template = prompts.get(gap_type)
    if prompt_template:
        return render_prompt(prompt_template, context)
    return None
```

#### 3. Record Assumption

```python
def record_assumption(field_name, value, source_note, model_location):
    """Record user assumption in model and documentation."""
    assumption = {
        "field": field_name,
        "value": value,
        "source": source_note,
        "location": model_location,
        "timestamp": datetime.now()
    }
    
    # Add to model cell comment
    add_cell_comment(model_location, f"Source: {source_note}")
    
    # Add to assumptions log
    assumptions_log.append(assumption)
    
    return assumption
```

#### 4. Integration in Skill Workflow

```markdown
### Step X: Handle Data Gaps

Before proceeding, check for TradingView data gaps:

1. **Check Risk-Free Rate** - TradingView does not provide
   - If gap detected → Show Prompt 1
   - Record user input in WACC section

2. **Check Market Risk Premium** - TradingView does not provide
   - If gap detected → Show Prompt 2
   - Record user input in WACC section

3. **Check Terminal Growth** - TradingView does not provide
   - If gap detected → Show Prompt 3
   - Record user input in Terminal Value section

4. **Document All Assumptions**
   - Add cell comments to all user-provided inputs
   - Include assumptions summary in model output
```

### Best Practices

1. **Prompt Early**: Ask for missing data at the beginning of workflows
2. **Validate Inputs**: Check that user inputs are reasonable
3. **Document Everything**: Every assumption must have a source note
4. **Allow Overrides**: Let users change assumptions later
5. **Be Transparent**: Clearly explain why TradingView doesn't have the data

### Common Patterns

| Scenario | Pattern |
|----------|---------|
| Required input, no default | Prompt with no skip option |
| Required input, standard default | Prompt with default value pre-filled |
| Optional enhancement | Prompt with clear skip option |
| Multiple related inputs | Group into single prompt |
| Conditional on prior input | Chain prompts based on responses |

---

## Prompt Quick Reference

| # | Prompt | Workflow | Critical? |
|---|--------|----------|-----------|
| 1 | Risk-Free Rate | DCF | Yes |
| 2 | Market Risk Premium | DCF | Yes |
| 3 | Terminal Growth | DCF | Yes |
| 4 | Cost of Debt | DCF | If unrated |
| 5 | SaaS Metrics | Comps | No |
| 6 | Retail Metrics | Comps | No |
| 7 | Healthcare Metrics | Comps | No |
| 8 | Purchase Premium | LBO | Yes |
| 9 | Debt Structure | LBO | Yes |
| 10 | Exit Multiple | LBO | Yes |
| 11 | Operating Assumptions | LBO | Yes |
| 12 | Detailed Line Items | 3-Statement | No |
| 13 | Management Guidance | 3-Statement | No |
| 14 | M&A Transaction Comps | IB | No |
| 15 | Deal Context | IB | No |
| 16 | Earnings Transcripts | ER | No |
| 17 | Management Commentary | ER | No |
| 18 | Private Company Data | PE | Yes |
| 19 | Transaction Terms | PE | Yes |
| 20 | Value Creation Plan | PE | No |

---

## Example: Complete DCF Workflow with Prompts

```markdown
## DCF Model Workflow

### Step 1: Fetch TradingView Data
- Call `get_fundamentals(symbol)`
- Call `get_credit_ratings(symbol)`
- Call `get_quote(symbol)`

### Step 2: Prompt for Missing Data

**Prompt 1: Risk-Free Rate**
→ User selects: 4.2% (Option 1)
→ Record in WACC section

**Prompt 2: Market Risk Premium**
→ User selects: 5.0% (Option 1)
→ Record in WACC section

**Prompt 3: Terminal Growth**
→ User selects: 2.5% (Option 2)
→ Record in Terminal Value section

### Step 3: Build Model
- Use TradingView data for financials
- Use user inputs for WACC assumptions
- Add cell comments documenting all sources

### Step 4: Output
- Excel model with all assumptions documented
- Assumptions summary page
- Sensitivity tables
```

---

*Last Updated: 2026-04-01*
*Version: 1.0*
*Coverage: 20 standardized prompts across 7 workflows*
