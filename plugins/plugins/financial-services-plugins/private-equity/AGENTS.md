# Private Equity Plugin

Private equity deal sourcing, screening, and portfolio management workflows.

## Structure

```
private-equity/
├── .claude-plugin/plugin.json    # Plugin manifest
├── .mcp.json                     # MCP server connections
├── commands/                     # Slash commands
│   ├── source.md                 # /source — deal sourcing
│   ├── screen-deal.md            # /screen-deal — quick screening
│   ├── dd-checklist.md           # /dd-checklist — DD tracker
│   ├── dd-prep.md                # /dd-prep — meeting prep
│   ├── ic-memo.md                # /ic-memo — IC memo
│   ├── unit-economics.md         # /unit-economics — unit analysis
│   ├── value-creation.md         # /value-creation — VCP
│   ├── returns.md                # /returns — returns analysis
│   ├── portfolio.md                # /portfolio — portfolio monitoring
│   └── ai-readiness.md           # /ai-readiness — AI assessment
├── skills/                       # Domain knowledge
│   ├── deal-sourcing/            # Sourcing workflow
│   ├── deal-screening/           # Screening framework
│   ├── dd-checklist/             # DD workstreams
│   ├── dd-meeting-prep/          # DD meeting prep
│   ├── ic-memo/                  # IC memo structure
│   ├── unit-economics/           # Unit economics
│   ├── value-creation-plan/      # Value creation
│   ├── returns-analysis/         # Returns modeling
│   ├── portfolio-monitoring/     # Portfolio tracking
│   └── ai-readiness/             # AI readiness score
└── hooks/hooks.json              # Event automation
```

## Where to Look

| Task | Skill | Command |
|------|-------|---------|
| Find targets, founder outreach | `deal-sourcing` | `/source` |
| Screen CIMs, teasers | `deal-screening` | `/screen-deal` |
| Track DD workstreams | `dd-checklist` | `/dd-checklist` |
| Prep for DD meetings | `dd-meeting-prep` | `/dd-prep` |
| Write IC memo | `ic-memo` | `/ic-memo` |
| Analyze unit economics | `unit-economics` | `/unit-economics` |
| Build value creation plan | `value-creation-plan` | `/value-creation` |
| Model returns (MOIC/IRR) | `returns-analysis` | `/returns` |
| Monitor portfolio | `portfolio-monitoring` | `/portfolio` |
| Assess AI readiness | `ai-readiness` | `/ai-readiness` |

## Conventions

**Deal Flow Pipeline:**
1. Source — discover targets, check CRM, draft outreach
2. Screen — triage CIMs against fund criteria
3. DD — run commercial, financial, legal, operational workstreams
4. IC — write memo, present to committee
5. Close — execute, then portfolio monitoring

**IC Memo Structure:**
- Executive Summary (1 page)
- Company Overview (1-2 pages)
- Industry & Market (1 page)
- Financial Analysis (2-3 pages)
- Investment Thesis (1 page)
- Deal Terms & Structure (1 page)
- Returns Analysis (1 page)
- Risk Factors (1 page)
- Recommendation

**Returns Analysis:**
- Base, upside, downside scenarios
- IRR and MOIC for each
- Sensitivity tables
- Key value creation levers

**CRM Integration:**
- Check Gmail for prior founder contact
- Search Slack for internal discussions
- Flag existing relationships before outreach

## Anti-Patterns

- NEVER skip CRM check before founder outreach
- NEVER bury red flags in IC memos — be direct
- NEVER use generic templates for founder emails
- NEVER hardcode returns assumptions — build sensitivity
- NEVER skip the bear case in investment thesis
- DO NOT send emails without explicit user approval
