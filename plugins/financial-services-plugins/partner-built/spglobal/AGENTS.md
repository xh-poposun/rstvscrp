# S&P Global Partner Plugin

S&P Capital IQ data integration via Kensho LLM-ready API. Requires active S&P Global subscription.

## Structure

```
spglobal/
├── .claude-plugin/plugin.json    # Plugin manifest
├── .mcp.json                     # Kensho API connection
├── skills/
│   ├── tear-sheet/              # Company one-pagers (4 audiences)
│   ├── funding-digest/           # Weekly deal flow slides
│   └── earnings-preview-beta/    # 4-5 page ER reports
└── commands/
    └── (via skills)
```

## Where to Look

| Task | Skill | Command | Output |
|------|-------|---------|--------|
| Company profile | `tear-sheet` | `/tear-sheet [company]` | DOCX |
| Deal flow summary | `funding-digest` | `/funding-digest [sectors]` | PPTX |
| Earnings preview | `earnings-preview-beta` | `/earnings-preview [ticker]` | HTML |

## Conventions

**Data Source:** S&P Capital IQ via Kensho MCP only. No web fallback.

**Audience Targeting (tear-sheet):**
- `equity-research` — Investment thesis, trading comps
- `ib-ma` — Transaction context, strategic fit
- `corp-dev` — Acquisition target evaluation
- `sales-bd` — Meeting prep, conversation starters

**Entity Resolution:**
- Pre-validate identifiers with `get_info_from_identifiers`
- Subsidiaries return zero funding rounds (query parent)
- Use legal entity names for failed brand lookups

**Output Standards:**
- Tear sheets: 1-2 pages, navy banner, S&P footer
- Funding digest: Single-slide PPTX, monochrome + logos
- Earnings preview: 4-5 pages HTML, hyperlinked sources

**Required Disclaimers:**
- "Data: S&P Capital IQ via Kensho | Analysis: AI-generated"
- "For informational purposes only. Not investment advice."
