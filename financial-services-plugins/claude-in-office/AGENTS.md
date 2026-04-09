# Claude in Office Plugin

Microsoft Office add-in deployment and cloud access provisioning.

## Structure

```
claude-in-office/
├── .claude-plugin/
│   └── plugin.json          # Plugin manifest
├── commands/
│   ├── setup.md             # /setup - Initial deployment
│   ├── debug.md             # /debug - Troubleshoot add-in
│   ├── consent.md           # /consent - Azure admin consent
│   ├── update-user-attrs.md # /update-user-attrs - Graph extensions
│   └── manifest.md          # /manifest - Generate XML manifest
└── scripts/
    └── build-manifest.mjs   # Node.js manifest generator
```

## Where to Look

| Task | Location |
|------|----------|
| Deploy add-in to tenant | `commands/setup.md` |
| Debug installation issues | `commands/debug.md` |
| Azure admin consent flow | `commands/consent.md` |
| Write user config to Graph | `commands/update-user-attrs.md` |
| Generate Office manifest | `commands/manifest.md` |
| Manifest XML generation | `scripts/build-manifest.mjs` |

## Conventions

**Cloud Providers:** Vertex AI, Bedrock, or LLM gateway (configurable).

**Manifest Generation:** Node.js script outputs customized XML for Office add-in.

**Azure Flow:** Admin consent required before add-in activation.

**Graph Extensions:** Per-user config stored in Microsoft Graph extension attributes.

**No Skills:** Command-only plugin. No `skills/` directory.

## Commands

```bash
# Deploy and configure
/setup
/consent
/update-user-attrs

# Generate and debug
/manifest
/debug
```
