#!/usr/bin/env bash
set -euo pipefail

# Simple validation for tradingview-finance plugin
ROOT_DIR="$(cd "$(dirname "$0")"/.. && pwd)"
PLUGIN_DIR="${ROOT_DIR}"

REQUIRED_FILES=(
  "$PLUGIN_DIR/.claude-plugin/plugin.json"
  "$PLUGIN_DIR/.mcp.json"
  "$PLUGIN_DIR/README.md"
)

echo "Validating tradingview-finance plugin..."
MISSING=0
for f in "${REQUIRED_FILES[@]}"; do
  if [[ ! -e "$f" ]]; then
    echo "Missing required file: $f" >&2
    MISSING=1
  fi
done

if [[ $MISSING -eq 0 ]]; then
  echo "Validation passed: required files present."
  exit 0
else
  echo "Validation failed: some required files are missing." >&2
  exit 1
fi
