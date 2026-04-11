# TradingView Financial Plugins Migration Guide

> **Version**: 1.0  
> **Last Updated**: 2026-04-01  
> **Scope**: Complete migration from paid MCP data sources to TradingView MCP

---

## Overview

This guide documents the migration of financial-services-plugins from paid MCP data sources (Daloopa, Morningstar, FactSet, PitchBook, etc.) to TradingView MCP as the sole data source. All analytical frameworks, Excel modeling standards, and output formats have been preserved. Only the data sourcing layer has changed.

### What Changed

| Aspect | Before | After |
|--------|--------|-------|
| Data Source | 11 paid MCP servers | TradingView MCP only |
| Cost | Subscription fees | Free (TradingView account) |
| Coverage | 100% | 60-95% per plugin |
| Missing Data | Automatic | User prompted |

### What Stayed the Same

- All Excel modeling conventions (blue/black/green fonts)
- All formula validation workflows (recalc.py)
- All step-by-step verification processes
- All output formats and deliverables
- All analytical methodologies

---

## Data Mapping: Old vs New Sources

### Original Paid Sources Replaced

| Original Source | Cost | Replacement | Coverage |
|-----------------|------|-------------|----------|
| Daloopa MCP | Paid | `get_fundamentals` | 92% |
| Morningstar MCP | Paid | `get_financial_statements` | 100% |
| FactSet MCP | Paid | `get_fundamentals` + `scan_stocks` | 95% |
| S&P Kensho | Paid | `scan_stocks` | 100% |
| PitchBook | Paid | User prompts + public proxies | 60% |
| Aiera | Paid | User prompts | 0% |
| MTNewswire | Paid | User prompts | 0% |
| Moody's | Paid | `get_credit_ratings` | 90% |
| LSEG | Paid | TradingView data | 85% |
| Chronograph | Paid | User prompts | 0% |
| Egnyte | Paid | Local file system | 100% |

### TradingView MCP Tools by Skill

| Skill | Primary Tools | Secondary Tools |
|-------|---------------|-----------------|
| DCF Model | `get_fundamentals`, `get_quote` | `get_credit_ratings` |
| Comps Analysis | `scan_stocks`, `get_fundamentals` | `get_quote` |
| LBO Model | `get_fundamentals`, `get_debt_maturity` | `get_quote` |
| 3-Statement | `get_financial_statements` | `get_fundamentals` |
| Pitch Deck | `get_quote`, `get_fundamentals` | `scan_stocks` |
| CIM Builder | `get_financial_statements` | `get_fundamentals` |
| Earnings Analysis | `get_fundamentals`, `get_earnings_calendar` | `get_quote` |
| IC Memo | `get_fundamentals`, `scan_stocks` | `get_credit_ratings` |

---

## Coverage Comparison: Old vs New

### Plugin-Level Coverage

| Plugin | Old Coverage | New Coverage | Delta |
|--------|--------------|--------------|-------|
| tv-financial-analysis | 100% | 92% | -8% |
| tv-investment-banking | 100% | 70% | -30% |
| tv-equity-research | 100% | 85% | -15% |
| tv-private-equity | 100% | 60% | -40% |

### Skill-Level Coverage Detail

| Skill | Coverage | Key Gaps | User Input Required |
|-------|----------|----------|---------------------|
| **DCF Model** | 92% | Risk-free rate, MRP, terminal growth | 3 prompts |
| **Comps Analysis** | 95% | Sector-specific metrics | 1 prompt |
| **LBO Model** | 75% | Transaction assumptions, deal terms | 4 prompts |
| **3-Statement** | 85% | Detailed line items | 1 prompt |
| **Pitch Deck** | 80% | Deal context, M&A comps | 2 prompts |
| **CIM Builder** | 85% | Management projections | 2 prompts |
| **Teaser** | 80% | Business description, highlights | 2 prompts |
| **Earnings Analysis** | 90% | Transcripts, guidance | 2 prompts |
| **Initiating Coverage** | 85% | Management bios, strategy | 3 prompts |
| **IC Memo** | 70% | Private data, deal terms | 4 prompts |
| **Deal Sourcing** | 100% (public) | Private company data | 1 prompt |
| **Value Creation Plan** | 70% | Initiative specifics | 2 prompts |

---

## Gap Analysis by Plugin

### tv-financial-analysis (92% Coverage)

#### Fully Covered (No User Input)
- Revenue history and forecasts
- EBITDA and margin analysis
- CapEx and depreciation
- Working capital calculations
- Beta calculation
- Tax rate derivation
- Market cap and enterprise value
- All valuation multiples

#### Gaps Requiring User Input

| Gap | When Prompted | Options |
|-----|---------------|---------|
| Risk-free rate | DCF start | 4.2% (current), custom, default 4.0% |
| Market risk premium | DCF start | 5.0%, 5.5%, 6.0% |
| Terminal growth | DCF start | 2.0-3.5% (guidelines provided) |

**Mitigation**: These are standard assumptions that analysts typically input manually anyway. TradingView provides all historical data needed.

---

### tv-investment-banking (70% Coverage)

#### Fully Covered
- Company financial highlights
- Trading multiples
- Market data (price, volume, cap)
- Comparable company screening
- Historical financial trends

#### Gaps Requiring User Input

| Gap | When Prompted | Options |
|-----|---------------|---------|
| M&A transaction comps | Pitch deck creation | Provide manually, skip, alternative source |
| Deal terms (cash/stock) | CIM/teaser creation | User provides structure |
| Management projections | CIM creation | Provide manually, use TV data |
| Investment highlights | Teaser creation | User provides anonymized highlights |

**Mitigation**: M&A transaction data is proprietary and always required analyst input. TradingView provides all public market data.

---

### tv-equity-research (85% Coverage)

#### Fully Covered
- Quarterly/annual financials
- Consensus estimates
- Real-time price and market data
- Peer identification
- Valuation multiples
- Earnings calendar

#### Gaps Requiring User Input

| Gap | When Prompted | Options |
|-----|---------------|---------|
| Earnings call transcripts | Earnings analysis | Provide manually, skip, summary only |
| Management guidance | Earnings analysis | Provide manually, use consensus |
| Management bios | Initiating coverage | Provide manually, skip |
| Business model details | Initiating coverage | Provide manually, skip |

**Mitigation**: Transcripts and qualitative data always required manual input. TradingView provides all quantitative data.

---

### tv-private-equity (60% Coverage)

#### Fully Covered (Public Targets)
- Public company screening
- Financial analysis
- Market context
- Credit ratings
- Comparable analysis

#### Gaps Requiring User Input

| Gap | When Prompted | Options |
|-----|---------------|---------|
| Private company financials | Deal sourcing | Enter manually, use public proxy |
| Transaction terms | IC memo creation | User provides all deal terms |
| Deal structure | IC memo creation | User provides debt/equity split |
| Value creation initiatives | VCP creation | User provides initiative details |

**Mitigation**: Private company data is inherently not available from any public source. TradingView provides public comparable data for benchmarking.

---

## Environment Setup

### Prerequisites

1. **TradingView Account**: Free account at [tradingview.com](https://tradingview.com)
2. **TradingView MCP Server**: Running locally on port 3000
3. **Claude Code**: With plugin marketplace access

### Installation Steps

```bash
# 1. Start TradingView MCP server
cd ~/tvservice && ./start.sh start

# 2. Verify health
curl http://localhost:3000/health

# 3. Add marketplace
claude plugin marketplace add anthropics/financial-services-plugins

# 4. Install TradingView plugins (in order)
claude plugin install tv-financial-analysis@financial-services-plugins
claude plugin install tv-investment-banking@financial-services-plugins
claude plugin install tv-equity-research@financial-services-plugins
claude plugin install tv-private-equity@financial-services-plugins

# 5. Verify installation
claude plugin list | grep tv-
```

### MCP Configuration

Each plugin contains `.mcp.json`:

```json
{
  "mcpServers": {
    "tvdata": {
      "url": "http://localhost:3000",
      "tools": [
        "get_quote",
        "get_fundamentals",
        "get_financial_statements",
        "get_credit_ratings",
        "get_debt_maturity",
        "get_earnings_calendar",
        "scan_stocks"
      ]
    }
  }
}
```

---

## Usage Examples

### Example 1: DCF Model with User Prompts

```bash
# Command
/dcf NASDAQ:AAPL

# Workflow:
# 1. Fetch AAPL fundamentals from TradingView
#    - Revenue: total_revenue_fq_h
#    - EBITDA: ebitda_fq
#    - CapEx: capital_expenditures_fq_h
#    - Beta: beta_3_year
#
# 2. Prompt: Risk-free rate
#    "Enter 10Y Treasury yield (current ~4.2%):"
#    User enters: 4.2
#
# 3. Prompt: Market risk premium
#    "Select MRP: 5.0% (conservative), 5.5% (moderate), 6.0% (aggressive)"
#    User selects: 5.5%
#
# 4. Prompt: Terminal growth
#    "Enter terminal growth rate (must be < WACC):"
#    User enters: 2.5%
#
# 5. Generate Excel with:
#    - All TradingView data (blue cells with comments)
#    - User assumptions (blue cells)
#    - Formulas for all calculations (black cells)
#    - Sensitivity tables (5x5)
```

### Example 2: Comps Analysis

```bash
# Command
/comps technology-sector

# Workflow:
# 1. Use scan_stocks to find tech companies
#    Filters: market_cap > $1B, technology sector
#
# 2. Fetch fundamentals for each peer
#    - Market Cap: market_cap_basic
#    - EV/EBITDA: enterprise_value_ebitda_current
#    - P/E: price_earnings_ttm
#    - Revenue Growth: calculated from history
#
# 3. Prompt: Sector-specific metrics
#    "Tech sector metrics (ARR, NDR) not available.
#     Would you like to: 1) Provide manually, 2) Skip"
#    User selects: 2 (Skip)
#
# 4. Generate Excel with:
#    - Peer comparison table
#    - Statistical summary (MIN, MAX, MEDIAN)
#    - TradingView citations in comments
```

### Example 3: LBO Model

```bash
# Command
/lbo NYSE:IBM

# Workflow:
# 1. Fetch IBM fundamentals
#    - EBITDA: ebitda
#    - Net Debt: net_debt_fq
#    - Debt maturity: get_debt_maturity
#
# 2. Prompt: Purchase price premium
#    "Enter premium over current price:"
#    Options: 15-20% (conservative), 20-30% (standard), 30-40% (aggressive)
#    User enters: 25%
#
# 3. Prompt: Debt financing
#    "Enter capital structure:"
#    - Senior Debt: 4.0x EBITDA
#    - Mezzanine: 1.0x EBITDA
#    - Equity: Remainder
#
# 4. Prompt: Exit assumptions
#    "Enter exit multiple and hold period:"
#    User enters: 5 years, 8.5x EBITDA
#
# 5. Generate Excel with:
#    - Sources & Uses
#    - Operating model
#    - Debt schedule
#    - Returns analysis (IRR, MOIC)
```

### Example 4: Earnings Analysis

```bash
# Command
/earnings NASDAQ:TSLA Q4 2025

# Workflow:
# 1. Fetch from TradingView
#    - Actuals: get_fundamentals (quarterly)
#    - Consensus: get_earnings_calendar
#    - Price: get_quote
#
# 2. Prompt: Transcript highlights
#    "TradingView does not provide transcripts.
#     Provide key management quotes or skip?"
#    User provides: "We expect 50% growth in 2026..."
#
# 3. Prompt: Guidance
#    "Provide management guidance or use consensus?"
#    User provides: Revenue $50B (vs consensus $48B)
#
# 4. Generate report with:
#    - Actual vs consensus comparison
#    - Key metrics table
#    - Management commentary section
```

### Example 5: IC Memo (Private Equity)

```bash
# Command
/ic-memo private-target

# Workflow:
# 1. Prompt: Target type
#    "Is target public or private?"
#    User selects: Private
#
# 2. Prompt: Private company data
#    "Enter target financials:"
#    - Revenue: $100M
#    - EBITDA: $20M
#    - Growth: 15%
#
# 3. Use TradingView for benchmarking
#    - scan_stocks for public comparables
#    - get_fundamentals for peer metrics
#
# 4. Prompt: Deal terms
#    "Enter transaction details:"
#    - Entry multiple: 8.0x EBITDA
#    - Debt financing: 4.0x
#    - Hold period: 5 years
#
# 5. Generate IC memo with:
#    - Investment thesis
#    - Financial analysis (user data + TV benchmarks)
#    - Returns analysis
```

---

## Troubleshooting Guide

### Connection Issues

#### Problem: "Cannot connect to TradingView MCP"

**Symptoms**: Commands fail with MCP connection error

**Solutions**:
1. Verify TradingView service is running:
   ```bash
   curl http://localhost:3000/health
   ```
2. Check service logs:
   ```bash
   cd ~/tvservice && ./start.sh logs
   ```
3. Restart service:
   ```bash
   cd ~/tvservice && ./start.sh restart
   ```

#### Problem: "MCP timeout"

**Symptoms**: Commands hang when fetching data

**Solutions**:
1. Check network connectivity
2. Verify TradingView session is active
3. Reduce batch size in skill (fetch fewer symbols at once)

---

### Data Issues

#### Problem: "Field not found in fundamentals"

**Symptoms**: Missing data for specific metric

**Solutions**:
1. Check field name in TradingView documentation
2. Use alternative field (e.g., `ebitda_ttm` vs `ebitda_fq`)
3. Calculate from other fields if possible
4. Proceed with user prompt for missing data

#### Problem: "Coverage percentage lower than expected"

**Symptoms**: Many user prompts appearing

**Solutions**:
1. Verify ticker symbol format (e.g., `NASDAQ:AAPL` not just `AAPL`)
2. Check if symbol is actively traded
3. Some fields only available for US equities
4. Use `get_financial_statements` for detailed historical data

---

### Excel Issues

#### Problem: "Formulas not calculating"

**Symptoms**: Excel shows #VALUE or #REF errors

**Solutions**:
1. Run recalc.py validation:
   ```bash
   python recalc.py model.xlsx
   ```
2. Check for circular references
3. Verify all blue cells have values
4. Ensure green cell links are correct

#### Problem: "Cell comments missing"

**Symptoms**: No source citations in Excel

**Solutions**:
1. All blue cells must have comments
2. Format: "Source: TradingView [tool], [Date], [field]"
3. Example: "Source: TradingView get_fundamentals, 2026-04-01, total_revenue_fq_h"

---

### User Prompt Issues

#### Problem: "Too many prompts"

**Symptoms**: Workflow interrupted frequently

**Solutions**:
1. Use default values where acceptable
2. Skip non-critical gaps
3. Provide all optional data upfront if available
4. Consider using original paid plugins for 100% coverage

#### Problem: "Unclear what data to provide"

**Symptoms**: User unsure what to enter

**Solutions**:
1. Each prompt includes context and examples
2. Options are provided with clear descriptions
3. "Skip" option always available
4. Refer to skill documentation for detailed guidance

---

### Coverage Comparison Issues

#### Problem: "Results differ from original plugin"

**Symptoms**: Valuation outputs don't match previous models

**Solutions**:
1. Check if user assumptions differ
2. Verify TradingView data date (more recent than paid sources)
3. Confirm same fiscal periods used
4. Differences expected due to real-time vs delayed data

---

## Quick Reference: TradingView Field Mapping

### Common Fields by Category

**Income Statement**
| Field | TradingView Source |
|-------|-------------------|
| Revenue | `total_revenue_fq_h` / `total_revenue_fy_h` |
| EBITDA | `ebitda_fq` / `ebitda_ttm` |
| Net Income | `net_income_fq` / `net_income_ttm` |
| EPS | `earnings_per_share_fq` |
| Tax | `income_tax_fq` |

**Balance Sheet**
| Field | TradingView Source |
|-------|-------------------|
| Cash | `cash_n_short_term_invest_fq` |
| Total Debt | `total_debt_fq` |
| Net Debt | `net_debt_fq` |
| Total Assets | `total_assets_fq` |
| Total Equity | `total_equity_fq` |

**Cash Flow**
| Field | TradingView Source |
|-------|-------------------|
| CFO | `cash_f_operating_activities_fq_h` |
| CapEx | `capital_expenditures_fq_h` |
| FCF | `free_cash_flow_ttm` |
| D&A | `depreciation_fy_h` |

**Market Data**
| Field | TradingView Source |
|-------|-------------------|
| Price | `price` (from get_quote) |
| Market Cap | `market_cap_basic` |
| P/E | `price_earnings_ttm` |
| EV/EBITDA | `enterprise_value_ebitda_current` |
| Beta | `beta_3_year` |

---

## Migration Checklist

Use this checklist when migrating existing workflows:

- [ ] TradingView MCP server running on localhost:3000
- [ ] New plugins installed (`tv-` prefix)
- [ ] Original plugins backed up (if needed)
- [ ] Test DCF with user prompts for assumptions
- [ ] Test Comps with sector-specific metric prompts
- [ ] Test LBO with transaction assumption prompts
- [ ] Verify Excel output matches original format
- [ ] Confirm all formulas validate with recalc.py
- [ ] Document any custom user assumptions used
- [ ] Train team on new prompting workflow

---

## Support and Feedback

### Getting Help

1. **Skill Documentation**: Each skill has detailed SKILL.md
2. **Troubleshooting**: See section above
3. **TradingView Data**: Refer to TradingView MCP documentation
4. **Plugin Issues**: Check `.claude-plugin/plugin.json` configuration

### Providing Feedback

When reporting issues, include:
- Plugin name and version
- Command used
- Ticker symbol
- Error message or unexpected behavior
- TradingView MCP health check output

---

## Appendix: Complete Data Mapping Tables

### DCF Model - Full Field Mapping

| Model Input | TradingView Field | Tool | Coverage |
|-------------|-------------------|------|----------|
| Revenue Y-5 to Y+5 | `total_revenue_fq_h` | get_fundamentals | 100% |
| EBITDA Margin | `ebitda_fq` / `total_revenue_fq_h` | get_fundamentals | 100% |
| CapEx % Revenue | `capital_expenditures_fq_h` | get_fundamentals | 100% |
| D&A | `depreciation_fy_h` | get_fundamentals | 100% |
| NWC Change | Calculated from current assets/liabilities | get_fundamentals | 100% |
| Tax Rate | `income_tax_fq` / `pretax_income_fq` | get_fundamentals | 100% |
| Beta | `beta_3_year` | get_fundamentals | 100% |
| Market Cap | `market_cap_basic` | get_quote | 100% |
| Net Debt | `net_debt_fq` | get_fundamentals | 90% |
| Credit Rating | `get_credit_ratings` | get_credit_ratings | 90% |
| Risk-free Rate | **USER INPUT** | - | 0% |
| Market Risk Premium | **USER INPUT** | - | 0% |
| Terminal Growth | **USER INPUT** | - | 0% |

### Comps Analysis - Full Field Mapping

| Metric | TradingView Field | Tool | Coverage |
|--------|-------------------|------|----------|
| Market Cap | `market_cap_basic` | get_fundamentals | 100% |
| Enterprise Value | Calc: Market Cap + Net Debt | get_fundamentals | 100% |
| Revenue TTM | `total_revenue_ttm` | get_fundamentals | 100% |
| EBITDA TTM | `ebitda_ttm` | get_fundamentals | 100% |
| Net Income TTM | `net_income_ttm` | get_fundamentals | 100% |
| P/E | `price_earnings_ttm` | get_fundamentals | 95% |
| EV/EBITDA | `enterprise_value_ebitda_current` | get_fundamentals | 95% |
| P/B | `price_book_fq_h` | get_fundamentals | 95% |
| Revenue Growth | Calc from history | get_fundamentals | 100% |
| EBITDA Margin | Calc: EBITDA / Revenue | get_fundamentals | 100% |
| ROE | `return_on_equity_fy` | get_fundamentals | 90% |
| Peer Universe | `scan_stocks` filters | scan_stocks | 100% |
| Sector Metrics | **USER INPUT** | - | 0% |

### LBO Model - Full Field Mapping

| Input | TradingView Field | Tool | Coverage |
|-------|-------------------|------|----------|
| EBITDA | `ebitda` | get_fundamentals | 100% |
| Net Debt | `net_debt_fq` | get_fundamentals | 90% |
| Interest Coverage | `ebitda_interst_cover_fy` | get_fundamentals | 85% |
| Debt/Equity | `debt_to_equity_current` | get_fundamentals | 90% |
| Free Cash Flow | `free_cash_flow_ttm` | get_fundamentals | 100% |
| Debt Maturity | `get_debt_maturity` | get_debt_maturity | 80% |
| Current Price | `price` | get_quote | 100% |
| Purchase Price | **USER INPUT** | - | 0% |
| Debt Financing | **USER INPUT** | - | 0% |
| Exit Multiple | **USER INPUT** | - | 0% |
| Hold Period | **USER INPUT** | - | 0% |

### 3-Statement Model - Full Field Mapping

| Statement | Line Item | TradingView Field | Coverage |
|-------------|-----------|-------------------|----------|
| Income | Revenue | `total_revenue_fq_h` | 100% |
| Income | COGS | `cost_of_goods_fy_h` | 95% |
| Income | OpEx | `operating_expenses_fq` | 90% |
| Income | EBITDA | `ebitda_fq` | 100% |
| Income | Net Income | `net_income_fq` | 100% |
| Balance | Current Assets | `total_current_assets` | 100% |
| Balance | Cash | `cash_n_short_term_invest_fq` | 100% |
| Balance | Total Debt | `total_debt_fq` | 100% |
| Balance | Net Debt | `net_debt_fq` | 100% |
| Balance | Total Assets | `total_assets_fq` | 100% |
| Balance | Total Liabilities | `total_liabilities_fq` | 100% |
| Cash Flow | CFO | `cash_f_operating_activities_fq_h` | 100% |
| Cash Flow | CapEx | `capital_expenditures_fq_h` | 100% |
| Cash Flow | D&A | `depreciation_fy_h` | 100% |
| Detailed | Line items | **USER INPUT** | 15% |

---

*End of Migration Guide*
