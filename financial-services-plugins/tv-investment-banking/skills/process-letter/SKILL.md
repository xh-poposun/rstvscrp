---
name: process-letter
description: "Draft process letters and bid instructions for sell-side M&A processes with optional TradingView market context for public company targets. Covers initial indication of interest (IOI) instructions, final bid procedures, and management meeting logistics. Use when drafting process letters where market context from TradingView may be relevant."
allowed-tools: [docx]
---

# Process Letter with TradingView Integration

## Overview

This skill drafts process letters and bid instructions for sell-side M&A processes. For public company targets, it can optionally fetch market context from TradingView MCP to inform process timing and valuation discussions.

## TradingView MCP Integration (Optional)

### When to Use TradingView

TradingView data is **optional** for process letters and most useful for:

| Use Case | TradingView Tool | Purpose |
|----------|------------------|---------|
| Public company target | `get_quote(symbol)` | Current market cap for valuation context |
| Market timing | `get_quote(symbol)` | Recent price volatility for process timing |
| Sector context | `scan_stocks(filters)` | Comparable trading multiples |
| Earnings timing | `get_earnings_calendar(symbol)` | Avoid process overlap with earnings |

### Key TradingView Fields for Process Context

| Context | TradingView Field | Use in Process Letter |
|---------|-------------------|----------------------|
| Market cap | `market_cap_basic` | Reference for valuation discussions |
| Recent volatility | `change_percent` (30d) | Timing considerations |
| Trading volume | `volume` | Liquidity context |
| Sector peers | `scan_stocks` | Comparable universe |
| Earnings date | `get_earnings_calendar` | Process timeline planning |

## Workflow

### Step 1: Determine Letter Type

- **Initial process letter**: Sent with teaser/CIM to outline the process and IOI requirements
- **IOI instructions**: Specific requirements for first-round indications of interest
- **Second round / final bid letter**: Instructions for binding offers after diligence
- **Management meeting invitation**: Logistics for in-person management presentations

### Step 2: Check for TradingView Context (Optional)

**User Prompt:**
```markdown
## TradingView Market Context (Optional)

**Is the target a public company where market context would be helpful?**

1. **Yes, fetch TradingView data** → Provide ticker symbol
   - Current market cap for valuation context
   - Recent trading activity
   - Earnings calendar for timing

2. **No, skip market data** → Proceed with standard process letter

**Your selection:**
```

### Step 3: Fetch TradingView Data (If Selected)

If public company and user opts in:

1. **Current Market Data** via `get_quote(symbol)`:
   - Market cap (for valuation context)
   - Recent price changes (volatility)
   - Trading volume

2. **Earnings Calendar** via `get_earnings_calendar(symbol)`:
   - Upcoming earnings dates
   - Avoid process conflicts

3. **Comparable Context** via `scan_stocks(filters)` (optional):
   - Sector trading multiples
   - Market context for valuation

### Step 4: Initial Process Letter / IOI Instructions

**Header:**
- Date, deal code name
- "Confidential"
- Addressed to prospective buyer

**Sections:**

1. **Introduction**: Brief overview of the opportunity and the seller's objectives

2. **Process Overview**: Timeline, key dates, expected number of rounds
   - *Optional TradingView context*: "Given recent market conditions [TradingView data], the process timeline is structured as..."

3. **IOI Requirements**: What to include in the initial indication:
   - Proposed valuation range (enterprise value)
   - Consideration form (cash, stock, earnout, rollover)
   - Financing sources and certainty
   - Key due diligence requirements
   - Indicative timeline to close
   - Any conditions or contingencies
   - Brief description of the buyer and strategic rationale

4. **Submission Details**: Where to send, deadline (date and time), format

5. **Confidentiality Reminder**: Reference to NDA, data room access

6. **Contact Information**: Banker contacts for questions

### Step 5: Final Bid / Second Round Letter

Additional requirements beyond IOI:

1. **Markup of purchase agreement**: Provide the draft SPA/APA and request markup
2. **Detailed financing commitments**: Committed financing letters required
3. **Remaining diligence items**: Specify what confirmatory diligence is expected
4. **Exclusivity terms**: Duration and conditions of any exclusivity period
5. **Regulatory analysis**: Antitrust filing requirements and timeline
6. **Key personnel terms**: Employment agreements, compensation, rollover equity
7. **Binding vs. non-binding**: Clarify what is binding at this stage
8. **Evaluation criteria**: How bids will be evaluated (price, certainty, speed, fit)

*Optional TradingView context for public targets:*
- Reference current trading price vs. proposed offer
- Note any market developments affecting timing

### Step 6: Management Meeting Invitation

1. **Logistics**: Date, time, location (or video link), duration
2. **Attendees**: Who from the company will present, who from the buyer should attend
3. **Agenda**: Typical management presentation agenda (overview, financials, operations, growth, Q&A)
4. **Ground rules**: No recording, confidentiality, questions format
5. **Materials**: What will be distributed (presentation deck, data room access)
6. **Follow-up**: Process for submitting additional questions after the meeting

### Step 7: Output

- Word document (.docx) with professional letter formatting
- Firm letterhead placeholder
- Track changes version for client review
- *Optional footnote*: "Market data sourced from TradingView MCP" (if used)

## Data Fetching Examples

### Example 1: Fetch Market Context for Public Target

```python
# Optional: Fetch market context for public company
symbol = "NASDAQ:AAPL"

# Current market data
quote = mcp_call("get_quote", {"symbol": symbol})

# Check for earnings conflicts
earnings = mcp_call("get_earnings_calendar", {
    "symbol": symbol,
    "start_date": "2026-04-01",
    "end_date": "2026-06-30"
})

market_context = {
    "market_cap": quote.get("market_cap"),
    "current_price": quote.get("price"),
    "recent_change": quote.get("change_percent"),
    "upcoming_earnings": earnings.get("dates", []),
}

# Use in process letter:
# "As of [date], [Company] trades at a market capitalization of 
# approximately $[X] billion. Given [recent market conditions / 
# upcoming earnings on [date]], the process timeline is structured as..."
```

### Example 2: Fetch Sector Context

```python
# Optional: Get sector trading multiples for context
symbol = "NASDAQ:AAPL"
target = mcp_call("get_fundamentals", {"symbol": symbol})

sector = target.get("sector")

# Get sector peers for context
peers = mcp_call("scan_stocks", {
    "filters": {
        "sector": sector,
        "market_cap_min": 1_000_000_000,
        "market_cap_max": 3_000_000_000_000
    }
})

# Calculate sector average multiples (optional context)
sector_multiples = []
for peer in peers[:20]:
    fund = mcp_call("get_fundamentals", {"symbol": peer})
    if fund.get("enterprise_value_ebitda_current"):
        sector_multiples.append(fund.get("enterprise_value_ebitda_current"))

avg_ev_ebitda = sum(sector_multiples) / len(sector_multiples) if sector_multiples else None
```

## Important Notes

- **TradingView is Optional**: Process letters primarily rely on user-provided deal terms and timelines
- **Market Context Only**: TradingView provides context, not requirements — all deal terms come from the user
- **Private Companies**: TradingView only covers public companies — skip for private targets
- **Timing Considerations**: Use earnings calendar to avoid process conflicts
- **Legal Review**: Always coordinate with legal on any representations or commitments
- **Client Approval**: Client should review and approve before sending

## Anti-Patterns

### ❌ Never Do These

1. **Use TradingView as primary data source** — Process terms come from deal team, not market data
2. **Suggest offer prices based on trading price** — Valuation is user's domain
3. **Skip legal review** — Process letters have contractual implications
4. **Include exact trading data without context** — Only use if relevant to process
5. **Omit confidentiality warnings** — Always remind of NDA obligations

### ✅ Correct Patterns

1. **Ask before fetching TradingView data** — Optional enhancement, not required
2. **Use market data for context only** — "Given market conditions..." not "Based on trading price..."
3. **Focus on process mechanics** — Timeline, requirements, submission details
4. **Get client approval** — Review before sending
5. **Keep log of recipients** — Track for process management

## Process Letter Checklist

### Content
- [ ] Letter type identified (IOI, final bid, management meeting)
- [ ] All required sections included
- [ ] Clear deadlines and submission instructions
- [ ] Evaluation criteria specified
- [ ] Contact information provided

### TradingView Context (If Used)
- [ ] User opted in to market data
- [ ] Data fetched and reviewed
- [ ] Context appropriately framed
- [ ] Source cited in footnote

### Legal/Compliance
- [ ] Confidentiality reminder included
- [ ] No unauthorized representations
- [ ] Exclusivity terms clear (if applicable)
- [ ] Legal review completed

### Final
- [ ] Client approval obtained
- [ ] Track changes version saved
- [ ] Distribution log prepared
