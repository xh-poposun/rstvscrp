# Claude for Financial Services Plugins

Claude Code plugin marketplace for investment banking, equity research, private equity, and wealth management workflows.

## Structure

```
financial-services-plugins/
├── README.md                    # Project overview and installation
├── CLAUDE.md                    # Plugin development guide
├── .claude-plugin/marketplace.json  # 8 plugin registry
├── financial-analysis/          # Core plugin (install first)
├── investment-banking/        # IB workflows
├── equity-research/            # ER workflows
├── private-equity/           # PE workflows
├── wealth-management/        # WM workflows
├── partner-built/            # Partner integrations
│   ├── lseg/                # LSEG data
│   └── spglobal/            # S&P Global data
└── claude-in-office/        # M365 add-in deployment
```

## Key Patterns

**Plugin Layout:**
- `.claude-plugin/plugin.json` — Plugin manifest
- `.mcp.json` — MCP server connections
- `commands/*.md` — Slash commands (`/plugin:command`)
- `skills/*/SKILL.md` — Domain knowledge files
- `hooks/hooks.json` — Event automation

**File Types:**
- Markdown — Skills, commands, templates
- JSON — Plugin configs, MCP servers, hooks
- Python — Helper scripts (5 total)

## Conventions

**Naming:**
- Plugins: hyphen-case
- Skills: hyphen-case, max 64 chars
- Commands: hyphen-case

**SKILL.md Frontmatter:**
```yaml
---
name: skill-name
description: Max 1024 chars
allowed-tools: [xlsx, pptx, docx]
---
```

**Data Hierarchy:**
1. MCP data sources (preferred)
2. Web search (fallback)

## Anti-Patterns

- NEVER use web search as primary data source
- NEVER skip DCF verification steps
- NEVER use nested IF in Excel (use INDEX/OFFSET)
- NEVER use HTML tables in pitch decks
- NEVER hardcode computed values in Excel (use formulas)
- DO NOT generate markdown for deliverables (use DOCX skill)

## Commands

```bash
# Install via Claude Code
claude plugin marketplace add anthropics/financial-services-plugins
claude plugin install financial-analysis@financial-services-plugins
claude plugin install investment-banking@financial-services-plugins

# Usage examples
/comps [company]
/dcf [company]
/earnings [company] [quarter]
/ic-memo [project]
```

## Notes

- No build step — markdown changes take effect immediately
- No tests — validation via Python scripts
- 41 skills, 38 commands, 11 MCP integrations
- Partner plugins in `partner-built/` directory
- Core plugin (`financial-analysis`) required before add-ons
