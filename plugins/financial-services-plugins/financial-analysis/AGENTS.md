# Financial Analysis Plugin

Core financial modeling and valuation plugin. Install this first before other financial services plugins.

## OVERVIEW

Institutional-grade financial modeling: DCF, comparable company analysis, LBO, 3-statement models, competitive analysis, and deck quality control. Powers equity research, investment banking, and private equity workflows.

## STRUCTURE

```
financial-analysis/
├── .claude-plugin/plugin.json    # Plugin manifest
├── .mcp.json                      # 11 MCP data integrations
├── commands/                      # Slash commands
│   ├── dcf.md                     # /dcf [ticker]
│   ├── comps.md                   # /comps [company]
│   ├── lbo.md                     # /lbo [target]
│   ├── 3-statement-model.md       # /3-statement-model [company]
│   ├── competitive-analysis.md    # /competitive-analysis [company]
│   ├── check-deck.md              # /check-deck [file]
│   └── debug-model.md             # /debug-model [file]
├── skills/                        # Domain knowledge
│   ├── dcf-model/SKILL.md         # DCF valuation methodology
│   ├── comps-analysis/SKILL.md    # Trading comps
│   ├── lbo-model/SKILL.md         # LBO modeling
│   ├── 3-statement-model/SKILL.md # Integrated financial statements
│   ├── competitive-analysis/      # Porter framework analysis
│   ├── ib-check-deck/             # Pitch deck QC
│   ├── ppt-template-creator/      # Template generation
│   ├── deck-refresh/              # Format standardization
│   ├── clean-data-xls/            # Data cleaning
│   ├── audit-xls/                 # Model auditing
│   └── skill-creator/             # Skill development helper
├── hooks/hooks.json               # Event automation
└── skills/*/scripts/              # Python validation scripts
```

## WHERE TO LOOK

**Primary Skills:**
- `dcf-model/` — Discounted cash flow valuation with sensitivity analysis
- `comps-analysis/` — Comparable company multiples analysis
- `lbo-model/` — Leveraged buyout modeling
- `3-statement-model/` — Integrated IS/BS/CF projections

**Quality Control:**
- `ib-check-deck/` — Pitch deck review and formatting
- `audit-xls/` — Model error detection and formula auditing
- `clean-data-xls/` — Data standardization

**Commands:**
- `/dcf [ticker]` — Build DCF model with Bear/Base/Bull scenarios
- `/comps [company]` — Generate trading comps
- `/lbo [target]` — Build LBO model
- `/check-deck [file]` — Review pitch deck

**MCP Data Sources:**
- Daloopa, Morningstar, S&P Global, FactSet, Moody's, MTNewswire, Aiera, LSEG, PitchBook, Chronograph, Egnyte

## CONVENTIONS

**Excel Modeling Standards:**

| Element | Format |
|---------|--------|
| Hardcoded inputs | Blue font (RGB: 0,0,255) |
| Formulas/calculations | Black font (RGB: 0,0,0) |
| Links to other sheets | Green font (RGB: 0,128,0) |
| Section headers | Dark blue fill (#1F4E79), white bold text |
| Output/summary rows | Medium blue fill (#BDD7EE), bold |
| Negative values | Parentheses, not minus signs |
| Currency | Units in header ($ millions), no decimals for large figures |
| Percentages | 1 decimal place |

**Formula Rules:**
- Use INDEX/OFFSET for scenario selection, never nested IFs
- Every derived value must be a live formula, never hardcoded
- Consolidation columns centralize scenario logic
- Add cell comments to ALL hardcoded inputs: "Source: [System], [Date], [Reference]"

**Model Structure:**
- Define ALL row positions before writing formulas
- Write headers first, then section dividers, then formulas
- Run `python recalc.py model.xlsx 30` before delivery
- Zero formula errors required (#REF!, #DIV/0!, etc.)

**Sensitivity Tables:**
- 5x5 grids (odd dimensions for center cell)
- Center cell = base case, highlighted in medium blue
- Populate ALL cells with full DCF recalculation formulas
- No Excel Data Table feature, no linear approximations

## ANTI-PATTERNS

**Excel Modeling:**
- NEVER use nested IF for scenario selection (use INDEX/OFFSET)
- NEVER hardcode computed values (use formulas)
- NEVER deliver without running recalc.py
- NEVER skip cell comments on blue inputs
- NEVER use minus signs for negatives (use parentheses)
- NEVER put sensitivity tables on separate sheets (bottom of DCF sheet)

**Data Handling:**
- NEVER use web search as primary data source when MCPs available
- NEVER skip DCF verification steps (WACC, terminal value checks)
- NEVER use HTML tables in pitch decks (use native PowerPoint)

**Model Quality:**
- NEVER deliver models without professional borders
- NEVER leave sensitivity cells empty or with placeholder text
- NEVER use linear approximations for sensitivity tables
- NEVER mix book and market values in capital structure
- NEVER allow terminal growth >= WACC (infinite value)

**Common Errors:**
- Operating expenses based on gross profit (must be revenue)
- Terminal value >80% of EV (over-reliance on terminal assumptions)
- Tax rate outside 21-28% range
- Missing column headers in scenario blocks
- Formula row references off due to late header insertion

## Notes

- 12 skills, 7 commands, 11 MCP integrations
- Python validation scripts in `skills/dcf-model/scripts/` and `skills/skill-creator/scripts/`
- No build step required, markdown changes take effect immediately
- Core plugin required before installing investment-banking, equity-research, private-equity, or wealth-management plugins
