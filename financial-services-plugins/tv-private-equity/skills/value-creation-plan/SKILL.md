---
name: value-creation-plan
description: Structure post-acquisition value creation plans using TradingView data for public comparable benchmarking. Includes revenue, cost, and operational levers mapped to an EBITDA bridge with sector benchmarking via TradingView MCP. Use when planning post-close execution, preparing operating partner materials, or building a board-ready value creation roadmap for PE portfolio companies. Triggers on "value creation plan", "100-day plan", "post-close plan", "EBITDA bridge", "operating plan", or "value creation levers".
---

# Value Creation Plan with TradingView

## Overview

This skill structures post-acquisition value creation plans using TradingView MCP data for public comparable benchmarking and sector analysis. It helps PE firms build data-driven EBITDA bridges with market context.

**Coverage**: TradingView provides 90% coverage for public comparable benchmarking. Private company baseline requires user input.

## Workflow

### Step 1: Baseline Assessment

**For Public Targets (TradingView Available):**

Use `get_fundamentals` to establish baseline:

| Metric | TradingView Field | Usage |
|--------|-------------------|-------|
| Current Revenue | `total_revenue_ttm` | Baseline for growth |
| Revenue History | `total_revenue_fq_h` | Growth trend analysis |
| Current EBITDA | `ebitda_ttm` | Baseline margin |
| EBITDA Margin | Calculated | Efficiency benchmark |
| Operating Expenses | `operating_expenses_fq` | Cost structure |
| CapEx | `capital_expenditures_fq_h` | Investment level |
| FCF | `free_cash_flow_ttm` | Cash generation |

**For Private Targets (User Input Required):**

```markdown
## Private Company Baseline Required

TradingView provides public company data. For private portfolio companies:

**Required Baseline Metrics:**
- Current revenue: $___M
- Revenue history (3-5 years)
- Current EBITDA: $___M
- EBITDA margin: ___%
- Operating expenses breakdown
- CapEx as % of revenue
- Working capital requirements
- Current debt and cash

**Organizational Context:**
- Management team strengths/gaps
- Current reporting capabilities
- Quick wins already identified
- Known operational issues
```

### Step 2: Sector Benchmarking via TradingView

Use `scan_stocks` to find public comparable companies for benchmarking:

**Screening Criteria:**
```json
{
  "filters": {
    "sector": "[Portfolio Company Sector]",
    "market_cap_min": [Relevant range],
    "market_cap_max": [Relevant range],
    "ebitda_margin_min": 0.05
  }
}
```

**Fetch Benchmark Metrics:**

For each comparable, retrieve:
- `total_revenue_ttm`: Revenue scale
- `ebitda_ttm`: Profitability
- `ebitda_margin` (calculated): Efficiency
- `return_on_equity_fy`: Returns
- `normalized_roe_fy`: Cyclical-adjusted returns
- `buyback_yield`: Capital return
- `revenue_growth` (calculated from history): Growth rate
- `operating_expenses_fq`: Cost structure

**Calculate Sector Benchmarks:**

| Metric | Portfolio Co. | Sector Median | Best-in-Class | Gap |
|--------|---------------|---------------|---------------|-----|
| EBITDA Margin | X% | X% | X% | +/- X% |
| Revenue Growth | X% | X% | X% | +/- X% |
| ROE | X% | X% | X% | +/- X% |
| Normalized ROE | X% | X% | X% | +/- X% |
| Buyback Yield | X% | X% | X% | +/- X% |
| OpEx % Revenue | X% | X% | X% | +/- X% |

**Use Benchmarks to Set Targets:**
- If below median → target median by Year 3
- If at median → target 75th percentile by Year 5
- Document specific initiatives to close gaps

### Step 3: Value Creation Levers

Map all levers to an EBITDA bridge over the hold period, informed by TradingView benchmarks:

#### Revenue Growth Levers

**Organic Growth:**
- Current: [Portfolio Co. growth rate]
- Target: [Sector median / best-in-class]
- Source: Calculated from `total_revenue_fq_h` of comparables

**Pricing Optimization:**
- Benchmark: Gross margins from comparable `cost_of_goods_fy_h`
- Target: Close gap to sector leader

**Cross-sell/Upsell:**
- Benchmark: Revenue per employee from comparables
- Target: Improve to sector median

**New Market Entry:**
- Benchmark: Geographic revenue mix from comparable analysis
- Target: Replicate successful expansion patterns

**M&A/Add-ons:**
- Benchmark: Sector M&A activity from comparable growth patterns
- Target: [User-defined based on strategy]

#### Margin Expansion Levers

**COGS Reduction:**
- Current: [Portfolio Co. COGS %]
- Benchmark: Median `cost_of_goods_fy_h` / `total_revenue_ttm` from comparables
- Target: Reduce to sector median

**OpEx Optimization:**
- Current: [Portfolio Co. OpEx %]
- Benchmark: Median `operating_expenses_fq` / revenue from comparables
- Target: Reduce to sector median

**Technology Investment:**
- Benchmark: CapEx intensity from `capital_expenditures_fq_h`
- Target: Align with sector leaders

#### Strategic/Multiple Expansion

**Platform Building:**
- Benchmark: EV/EBITDA expansion of acquisitive peers
- Source: `enterprise_value_ebitda_current` trend analysis

**Recurring Revenue Shift:**
- Benchmark: Revenue quality metrics from comparables
- Target: Improve revenue visibility

**Management Upgrades:**
- Benchmark: ROE improvement post-management changes
- Source: `return_on_equity_fy` analysis

**Buyback-Driven Value Creation:**
- Buyback yield as a signal to market of management confidence, potentially supporting multiple expansion
- Benchmark: `buyback_yield` from public comparables — high buyback yield peers often trade at premium multiples
- Target: Implement disciplined buyback program post-close if public, or allocate excess FCF to debt paydown (private equivalent of capital return)

### Step 4: EBITDA Bridge

Build the walk from current to target EBITDA:

| Lever | Year 1 | Year 2 | Year 3 | Year 4 | Year 5 |
|-------|--------|--------|--------|--------|--------|
| Base EBITDA | $XM | $XM | $XM | $XM | $XM |
| Organic revenue growth | +$XM | +$XM | +$XM | +$XM | +$XM |
| Pricing optimization | +$XM | +$XM | +$XM | +$XM | +$XM |
| Add-on M&A | $XM | +$XM | +$XM | +$XM | +$XM |
| COGS savings | +$XM | +$XM | +$XM | +$XM | +$XM |
| OpEx optimization | +$XM | +$XM | +$XM | +$XM | +$XM |
| Technology investment | -$XM | +$XM | +$XM | +$XM | +$XM |
| **Pro Forma EBITDA** | **$XM** | **$XM** | **$XM** | **$XM** | **$XM** |
| **Margin** | **X%** | **X%** | **X%** | **X%** | **X%** |

**Benchmark Context:**
- Note where targets align with / exceed sector benchmarks
- Flag aggressive assumptions requiring specific initiatives

### Step 5: 100-Day Plan

Prioritize the first 100 days post-close:

**Days 1-30: Stabilize & Assess**
- Management alignment and retention
- Quick wins — pricing, obvious cost cuts
- Detailed operational assessment by function
- Customer communication plan
- Set up reporting and KPI dashboards
- **TradingView Action**: Establish baseline metrics for ongoing benchmarking

**Days 31-60: Plan & Initiate**
- Finalize strategic plan using sector benchmarks
- Launch top 3-5 value creation initiatives
- Begin add-on M&A pipeline development
- Hire for critical gaps
- Implement new reporting cadence

**Days 61-100: Execute & Measure**
- First results from quick-win initiatives
- First board meeting with operating metrics
- Progress report on each value creation lever
- Adjust plan based on early learnings
- **TradingView Action**: First quarterly benchmark comparison

### Step 6: KPI Dashboard

Define metrics that will track value creation, with TradingView benchmarks:

| KPI | Current | Sector Median | Year 1 Target | Owner | Frequency |
|-----|---------|---------------|---------------|-------|-----------|
| Revenue | $XM | $XM | $XM | CEO | Monthly |
| Revenue Growth | X% | X% | X% | CEO | Monthly |
| EBITDA | $XM | $XM | $XM | CFO | Monthly |
| EBITDA Margin | X% | X% | X% | CFO | Monthly |
| ROE | X% | X% | X% | CFO | Quarterly |
| FCF | $XM | $XM | $XM | CFO | Monthly |
| Net Retention | X% | X%* | X% | CRO | Monthly |
| Employee Turnover | X% | X%* | X% | CHRO | Monthly |

*Sector benchmarks from TradingView where available; user input otherwise.

### Step 7: Ongoing Benchmarking

**Quarterly TradingView Updates:**

1. **Refresh Comparable Data:**
   - Re-run `scan_stocks` for current peer set
   - Update benchmark metrics

2. **Track Relative Performance:**
   - Compare portfolio company progress vs. sector
   - Identify new gaps or opportunities

3. **Adjust Targets:**
   - If sector median improves, raise targets
   - If falling behind, diagnose and course-correct

## Data Coverage Summary

| VCP Element | TradingView Coverage | Gap Handling |
|-------------|---------------------|--------------|
| Public comparable benchmarking | 90% | `scan_stocks` + `get_fundamentals` |
| Sector median metrics | 90% | Calculated from peer set |
| Best-in-class benchmarks | 80% | Top quartile of peer set |
| Private company baseline | 0% | **USER PROMPT REQUIRED** |
| Management assessment | 0% | **USER PROMPT REQUIRED** |
| Initiative-specific targets | 0% | **USER PROMPT REQUIRED** |
| M&A pipeline | 0% | **USER PROMPT REQUIRED** |

## User Prompts for Data Gaps

### Private Company Baseline
```markdown
## Private Company Baseline

TradingView provides public comparable data. For your portfolio company:

**Current State (Required):**
- Revenue: $___M (TTM)
- EBITDA: $___M (TTM)
- EBITDA margin: ___%
- Revenue growth rate: ___%
- Total employees: ___
- Key customers (% concentration): ___

**Quick Wins Identified:**
- [ ] Pricing optimization
- [ ] Cost reduction
- [ ] Working capital improvement
- [ ] Other: ___
```

### Initiative Targets
```markdown
## Value Creation Initiatives

Based on sector benchmarking, please define initiative targets:

**Revenue Growth:**
- Organic growth target: ___% (Sector median: ___%)
- Pricing improvement: ___% (Sector leader: ___%)
- Cross-sell/upsell target: $___M

**Margin Expansion:**
- COGS reduction target: ___% (Sector median COGS: ___%)
- OpEx reduction target: ___% (Sector median OpEx: ___%)
- Technology investment: $___M

**M&A:**
- Target add-on revenue: $___M
- Target synergies: $___M
```

### Management and Organizational Context
```markdown
## Management Assessment

TradingView provides financial benchmarks. Please provide:

**Management Team:**
- CEO background and tenure
- CFO capabilities
- Key person risks
- Succession planning

**Organizational Gaps:**
- Missing functions/roles
- Reporting capabilities
- Systems and infrastructure
- Cultural considerations
```

## Important Notes

- **Be realistic about timing** — most PE value creation takes 12-24 months to show in financials
- **Quick wins matter** for momentum and credibility, but don't over-rotate on cost cuts at the expense of growth
- **Management buy-in is critical** — co-develop the plan, don't impose it
- **Track initiative-level P&L impact**, not just top-line EBITDA — you need to know what's working
- **Add-on M&A is often the largest value creation lever** — start the pipeline on Day 1
- **Always pressure-test assumptions** with operating partners or industry experts
- **Update benchmarks quarterly** using TradingView to track relative performance

## Anti-Patterns

- ❌ Never use PitchBook or Capital IQ for benchmarking
- ❌ Never set targets without sector context
- ❌ Never skip user prompts for private company baselines
- ❌ Never assume private company can match public peer metrics without specific initiatives
- ❌ Never create VCP without management input
- ❌ Never skip the 100-day plan — first impressions matter
- ❌ Never set EBITDA targets without bridge details
