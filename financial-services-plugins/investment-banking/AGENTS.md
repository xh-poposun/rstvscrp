# Investment Banking Plugin

Investment banking deal execution workflows: pitch materials, CIMs, teasers, buyer lists, and M&A models.

## Structure

```
investment-banking/
├── AGENTS.md              # This file
├── .claude-plugin/
│   └── plugin.json        # Plugin manifest
├── commands/              # Slash commands
│   ├── cim.md
│   ├── teaser.md
│   ├── process-letter.md
│   ├── one-pager.md
│   ├── pitch-deck.md
│   ├── strip-profile.md
│   ├── merger-model.md
│   ├── deal-tracker.md
│   └── buyer-list.md
├── skills/                # Domain knowledge
│   ├── pitch-deck/
│   ├── cim-builder/
│   ├── teaser/
│   ├── strip-profile/
│   ├── process-letter/
│   ├── datapack-builder/
│   ├── merger-model/
│   ├── deal-tracker/
│   └── buyer-list/
└── templates/             # Reusable templates
    ├── pitch/
    ├── cim/
    └── process/
```

## Where to Look

**Pitch Materials:**
- `skills/pitch-deck/SKILL.md` — Presentation architecture, narrative flow
- `skills/cim-builder/SKILL.md` — Confidential Information Memorandum structure
- `skills/teaser/SKILL.md` — Anonymous deal summaries
- `skills/strip-profile/SKILL.md` — Public company tear sheets

**Deal Execution:**
- `skills/merger-model/SKILL.md` — M&A accretion/dilution modeling
- `skills/deal-tracker/SKILL.md` — Pipeline and process management
- `skills/buyer-list/SKILL.md` — Strategic and financial buyer targeting
- `skills/process-letter/SKILL.md` — NDAs, indications, management presentations

**Commands:**
- `/pitch-deck [company] [type]` — Generate pitch presentation
- `/cim [company]` — Build CIM from datapack
- `/teaser [company]` — Create anonymous teaser
- `/strip-profile [ticker]` — Generate company tear sheet
- `/merger-model [acquirer] [target]` — Build M&A model
- `/deal-tracker [action]` — Manage deal pipeline
- `/buyer-list [sector]` — Generate buyer universe
- `/process-letter [type]` — Process correspondence
- `/one-pager [company]` — Executive summary

## Conventions

**Pitch Decks:**
- Use PptxGenJS native tables and charts (no images of tables)
- Master slide layouts defined in templates
- Color scheme: acquirer blue, target green, synergy purple
- Font: Calibri body, Calibri Light headers

**CIMs:**
- 20-40 pages standard length
- Sections: Overview, Market, Business, Financials, Management, Appendix
- Financial data pulled from financial-analysis core
- No forward projections without disclaimer

**Teasers:**
- 2-4 pages maximum
- No identifying information (revenue ranges only)
- Blind maps and industry classifications
- Contact via intermediary only

**Merger Models:**
- Three-statement consolidation
- Sources & Uses, PF balance sheet, accretion/dilution
- Sensitize: offer price, synergies, financing mix
- Foot all assumptions

## Anti-Patterns

**Pitch Decks:**
- NEVER paste Excel tables as images
- NEVER use default PowerPoint chart colors
- NEVER include "work in progress" slides in final
- NEVER hardcode numbers that should link to model
- NEVER use animations or transitions
- NEVER exceed 30 slides for first pitch

**CIMs:**
- NEVER include unaudited financials without disclosure
- NEVER name potential buyers in process
- NEVER forward projections without basis
- NEVER omit risk factors section

**Teasers:**
- NEVER include exact revenue or EBITDA figures
- NEVER name the company or key executives
- NEVER show specific locations or facilities
- NEVER include dated financial information

**Process Letters:**
- NEVER send NDAs without legal review
- NEVER include exclusivity language without approval
- NEVER set deadlines without banker consensus

## Dependencies

Requires `financial-analysis` core plugin. No direct MCP integrations. All data flows through financial-analysis skills.
