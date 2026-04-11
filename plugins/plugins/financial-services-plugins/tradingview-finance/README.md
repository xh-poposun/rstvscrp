TradingView Finance Plugin

Overview
- This plugin provides TradingView data integration for financial analysis workflows.
- It exposes MCP-backed data sources to power investment research dashboards, screening, and alerting.

Installation
- Ensure the marketplace.json entry for tradingview-finance exists (see marketplace.json at financial-services-plugins/.claude-plugin).
- Create the plugin directory at financial-services-plugins/tradingview-finance with the following contents:
- See repository structure for the required files in this folder.

Configuration
- MCP server URL should be configurable via the MCP_SERVER_URL environment variable.
- Default fallback (for local development): http://localhost:3000

Usage
- After installation, use the tradingview-finance commands (when defined) to query TradingView data within workflows.

Validation
- A simple validation script is provided in validate.sh to verify that the manifest and MCP config are in place.

Note
- This plugin follows the same structural conventions as the financial-analysis plugin to ensure compatibility with the marketplace and MCP routing.
