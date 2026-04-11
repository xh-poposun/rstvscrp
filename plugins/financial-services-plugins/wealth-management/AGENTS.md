# Wealth Management Plugin

Claude Code plugin for financial advisors: client reviews, financial planning, portfolio rebalancing, tax-loss harvesting, and client reporting.

## Structure

```
wealth-management/
├── .claude-plugin/
│   └── plugin.json          # Plugin manifest
├── commands/
│   ├── client-review.md     # /client-review [client]
│   ├── financial-plan.md    # /financial-plan [client]
│   ├── proposal.md          # /investment-proposal [prospect]
│   ├── rebalance.md         # /rebalance [client]
│   ├── client-report.md     # /client-report [client]
│   └── tlh.md               # /tlh [client]
├── skills/
│   ├── client-review/       # Meeting prep workflows
│   ├── financial-plan/      # Retirement and goal planning
│   ├── investment-proposal/ # New client pitches
│   ├── portfolio-rebalance/ # Allocation and drift analysis
│   ├── client-report/       # Performance reporting
│   └── tax-loss-harvesting/ # TLH opportunity identification
└── hooks/
    └── hooks.json           # Event automation
```

## Where to Look

| Task | Skill | Command |
|------|-------|---------|
| Client meeting prep | `client-review` | `/client-review [name]` |
| Financial planning | `financial-plan` | `/financial-plan [name]` |
| New client pitch | `investment-proposal` | `/investment-proposal [prospect]` |
| Portfolio rebalancing | `portfolio-rebalance` | `/rebalance [client]` |
| Performance reports | `client-report` | `/client-report [client]` |
| Tax-loss harvesting | `tax-loss-harvesting` | `/tlh [client]` |

## Conventions

**Client-First Workflow:**
- Always start with client context (IPS, goals, risk tolerance)
- Lead with what the client cares about, not what you want to discuss
- Document action items with clear owners and dates

**Data Sources:**
1. Portfolio management system (via MCP)
2. CRM for client notes and history
3. Market data for performance and benchmarks

**Output Standards:**
- Client-facing materials: PDF or Word (professional formatting)
- Internal workpapers: Excel (formulas, not hardcoded values)
- Presentations: PowerPoint with firm branding

**Compliance:**
- All materials need compliance review before distribution
- Include required disclaimers on all projections and performance
- Document rationale for recommendations
- Suitability/fiduciary standard applies to all advice

**Tax Awareness:**
- Consider tax implications in all taxable account recommendations
- Watch wash sale rules across all household accounts
- Model tax impact of trades before executing

## Anti-Patterns

- NEVER present performance without appropriate benchmarks
- NEVER skip the IPS review before making recommendations
- NEVER ignore tax consequences in taxable accounts
- NEVER use web search as primary data source for client portfolios
- DO NOT generate markdown for client deliverables (use DOCX/PPTX skills)

## Notes

- Requires `financial-analysis` core plugin (install first)
- 6 skills, 6 commands
- All workflows assume access to client portfolio data via MCP
- Client reports typically quarterly; financial plans annually
