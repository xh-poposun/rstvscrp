# Equity Research Plugin

Claude Code plugin for equity research workflows: initiating coverage, earnings analysis, thesis tracking, and catalyst monitoring.

## Structure

```
equity-research/
├── .claude-plugin/plugin.json    # Plugin manifest
├── commands/                      # Slash commands
│   ├── initiate.md               # /initiate - Coverage initiation
│   ├── earnings.md               # /earnings - Earnings update
│   ├── earnings-preview.md       # /earnings-preview - Pre-earnings note
│   ├── morning-note.md           # /morning-note - Daily morning note
│   ├── thesis.md                 # /thesis - Thesis tracker
│   ├── catalysts.md              # /catalysts - Catalyst calendar
│   ├── model-update.md           # /model-update - Model refresh
│   ├── screen.md                 # /screen - Idea generation
│   └── sector.md                 # /sector - Sector overview
├── skills/                        # Domain knowledge
│   ├── initiating-coverage/      # 5-task workflow (Tasks 1-5)
│   ├── earnings-analysis/        # Post-earnings reports
│   ├── earnings-preview/         # Pre-earnings previews
│   ├── morning-note/             # Daily market notes
│   ├── thesis-tracker/           # Investment thesis tracking
│   ├── catalyst-calendar/        # Event monitoring
│   ├── model-update/             # Financial model updates
│   ├── idea-generation/          # Screening workflows
│   └── sector-overview/          # Sector reports
└── hooks/hooks.json              # Event automation
```

## Where to Look

| Task | Skill | Command |
|------|-------|---------|
| Initiation report (30-50 pages) | `initiating-coverage` | `/initiate` |
| Earnings update (8-12 pages) | `earnings-analysis` | `/earnings` |
| Pre-earnings preview | `earnings-preview` | `/earnings-preview` |
| Daily morning note | `morning-note` | `/morning-note` |
| Track thesis vs reality | `thesis-tracker` | `/thesis` |
| Monitor upcoming events | `catalyst-calendar` | `/catalysts` |
| Refresh model post-earnings | `model-update` | `/model-update` |
| Generate new ideas | `idea-generation` | `/screen` |
| Sector deep dive | `sector-overview` | `/sector` |

## Conventions

**Report Formats:**
- Initiation: 30-50 pages, 10-15K words, 25-35 charts, Times New Roman
- Earnings update: 8-12 pages, 3-5K words, 8-12 charts, 1-3 tables
- Morning note: 2-4 pages, bullet format, market color
- Thesis tracker: Scorecard format, red/yellow/green flags

**Initiating Coverage Workflow:**
- 5 sequential tasks, execute ONE at a time
- Task 1: Company research (6-8K words)
- Task 2: Financial modeling (6-tab Excel)
- Task 3: Valuation (DCF + comps, price target)
- Task 4: Chart generation (25-35 charts, 4 mandatory)
- Task 5: Report assembly (DOCX, 30-50 pages)

**Citations:**
- Every figure needs source with clickable hyperlink
- SEC filings link to EDGAR viewer
- Earnings releases link to investor relations
- Consensus data cite Bloomberg/FactSet with date

**File Naming:**
- `[Company]_Research_Document_[Date].md`
- `[Company]_Financial_Model_[Date].xlsx`
- `[Company]_Valuation_Analysis_[Date].md`
- `[Company]_Charts_[Date].zip`
- `[Company]_Initiation_Report_[Date].docx`
- `[Company]_Q[Quarter]_[Year]_Earnings_Update.docx`

## Anti-Patterns

- NEVER skip input verification for Tasks 3-5 (dependencies required)
- NEVER create "completion summaries" or extra documents (deliver ONLY specified outputs)
- NEVER use web search as primary data source (use MCP or SEC filings)
- NEVER skip DCF sensitivity analysis (required for Task 3)
- NEVER omit the 4 mandatory charts (revenue by product/geo, DCF sensitivity, football field)
- NEVER chain tasks automatically (wait for explicit user request)
- NEVER use outdated earnings data (always verify within 3 months)
- NEVER skip clickable hyperlinks in citations (plain URLs not acceptable)

## Dependencies

- Requires `financial-analysis` core plugin (install first)
- No direct MCP integrations (uses core plugin data sources)
- Python helpers: matplotlib, pandas, seaborn for charts
