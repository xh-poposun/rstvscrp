# LSEG Partner Plugin

LSEG (London Stock Exchange Group) data integration for fixed income, FX, equities, and macro analytics. Requires active LSEG subscription.

## Structure

```
partner-built/lseg/
├── AGENTS.md              # This file
├── .claude-plugin/
│   └── plugin.json        # Plugin manifest
├── .mcp.json              # LSEG Analytics API connection
├── commands/
│   ├── analyze-bond-rv.md
│   ├── analyze-fx-carry.md
│   ├── research-equity.md
│   ├── macro-rates.md
│   ├── analyze-swap-curve.md
│   ├── analyze-option-vol.md
│   ├── review-fi-portfolio.md
│   └── analyze-bond-basis.md
└── skills/
    ├── bond-relative-value/SKILL.md
    ├── fx-carry-trade/SKILL.md
    ├── equity-research/SKILL.md
    ├── macro-rates-monitor/SKILL.md
    ├── swap-curve-strategy/SKILL.md
    ├── option-vol-analysis/SKILL.md
    ├── fixed-income-portfolio/SKILL.md
    └── bond-futures-basis/SKILL.md
```

## Where to Look

| Skill | Command | Purpose |
|-------|---------|---------|
| bond-relative-value | /analyze-bond-rv | Rich/cheap analysis vs swap curve |
| fx-carry-trade | /analyze-fx-carry | Carry, roll, funding cost analysis |
| equity-research | /research-equity | LSEG fundamental data, estimates |
| macro-rates-monitor | /macro-rates | Central bank policy, economic releases |
| swap-curve-strategy | /analyze-swap-curve | Steepeners, flatteners, butterfly |
| option-vol-analysis | /analyze-option-vol | Vol surface, skew, term structure |
| fixed-income-portfolio | /review-fi-portfolio | Duration, convexity, spread attribution |
| bond-futures-basis | /analyze-bond-basis | CTD, implied repo, basis trading |

## Conventions

**Data Sources:**
- Primary: LSEG Analytics API via MCP
- Real-time: Workspace/RFT feed
- Reference: Datastream historical

**Tickers:**
- Bonds: `XS1234567890` (ISIN) or `T 3.5 02/15/30` (Cusip/Name)
- Swaps: `USD3M5Y` (CCY-Tenor)
- FX: `EURUSD` (6-letter ISO pair)
- Equities: `AAPL.O` (LSEG exchange suffix)

**Rate Conventions:**
- Bonds: Act/Act semi-annual yield
- Swaps: 30/360 annual vs 3M LIBOR/SOFR
- FX: CCY1 per CCY2, T+2 spot

**API Limits:**
- 10K requests/hour
- 5Y historical default
- Batch max 100 instruments

**Error Handling:**
- `SUBSCRIPTION_REQUIRED` - Check LSEG entitlements
- `RATE_LIMIT` - Backoff 60s, retry
- `INVALID_RIC` - Verify ticker suffix
