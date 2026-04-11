# Partner-Built Plugins

Third-party data provider integrations maintained by external partners.

## Structure

```
partner-built/
├── lseg/              # London Stock Exchange Group
│   ├── .mcp.json      # LFA MCP server config
│   ├── commands/      # 8 slash commands
│   └── skills/        # 8 domain skills
└── spglobal/          # S&P Global
    ├── .mcp.json      # Kensho MCP server config
    └── skills/        # 3 domain skills
```

## Where to Look

| Data Need | Partner | Skills |
|-----------|---------|--------|
| Bond pricing, yield curves | LSEG | bond-relative-value, swap-curve-strategy |
| FX carry trades, vol surfaces | LSEG | fx-carry-trade, option-vol-analysis |
| Fixed income portfolio analytics | LSEG | fixed-income-portfolio, macro-rates-monitor |
| Company tearsheets | S&P Global | tear-sheet |
| Earnings previews | S&P Global | earnings-preview-beta |
| M&A transaction summaries | S&P Global | funding-digest |

## Conventions

**Partner Plugin Layout:**
- `.mcp.json` — Partner MCP server endpoint
- `commands/*.md` — Slash commands (LSEG only)
- `skills/*/SKILL.md` — Domain knowledge files
- `README.md` — Partner-specific setup instructions

**Maintenance:**
- Partner-built, not Anthropic-maintained
- Updates released by respective partners
- Support via partner channels

**Authentication:**
- Requires valid partner credentials
- Data entitlements checked at runtime
- MCP servers handle token management

**Naming:**
- Commands: hyphen-case, verb-noun pattern
- Skills: hyphen-case, domain-specific
