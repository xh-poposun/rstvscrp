#!/bin/bash
# Comprehensive API verification script for TradingView MCP Server
# Tests all 15 endpoints to verify real data is returned

set -e

BASE_URL="http://localhost:3000"
MCP_ENDPOINT="$BASE_URL/mcp"

echo "=========================================="
echo "TradingView MCP API Verification Test"
echo "=========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

# Helper function to make MCP requests
mcp_call() {
    local method=$1
    local params=$2
    curl -s -X POST "$MCP_ENDPOINT" \
        -H "Content-Type: application/json" \
        -d "{\"jsonrpc\": \"2.0\", \"id\": 1, \"method\": \"$method\", \"params\": $params}"
}

# Helper function to call tools
tool_call() {
    local tool=$1
    local args=$2
    mcp_call "tools/call" "{\"name\": \"$tool\", \"arguments\": $args}"
}

echo -e "${YELLOW}1. Testing Health Endpoint${NC}"
HEALTH=$(curl -s "$BASE_URL/health")
if echo "$HEALTH" | grep -q '"status":"ok"'; then
    echo -e "${GREEN}✓ Health check passed${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Health check failed${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}2. Testing Initialize${NC}"
INIT_RESULT=$(mcp_call "initialize" '{"protocolVersion": "2024-11-05", "clientInfo": {"name": "test", "version": "1.0"}}')
if echo "$INIT_RESULT" | grep -q '"protocolVersion":"2024-11-05"'; then
    echo -e "${GREEN}✓ Initialize passed${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Initialize failed${NC}"
    echo "$INIT_RESULT"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}3. Testing Tools/List${NC}"
TOOLS_RESULT=$(mcp_call "tools/list" '{}')
TOOL_COUNT=$(echo "$TOOLS_RESULT" | grep -o '"name":' | wc -l)
if [ "$TOOL_COUNT" -eq 15 ]; then
    echo -e "${GREEN}✓ All 15 tools registered${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Expected 15 tools, found $TOOL_COUNT${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}4. Testing get_quote for AAPL${NC}"
QUOTE_AAPL=$(tool_call "get_quote" '{"symbol": "NASDAQ:AAPL"}')
if echo "$QUOTE_AAPL" | grep -q '"price":' && echo "$QUOTE_AAPL" | grep -q '"USD"'; then
    PRICE=$(echo "$QUOTE_AAPL" | grep -o '"price":[0-9.]*' | cut -d: -f2)
    echo -e "${GREEN}✓ AAPL quote: \$$PRICE${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Failed to get AAPL quote${NC}"
    echo "$QUOTE_AAPL"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}5. Testing get_quote for MSFT${NC}"
QUOTE_MSFT=$(tool_call "get_quote" '{"symbol": "NASDAQ:MSFT"}')
if echo "$QUOTE_MSFT" | grep -q '"price":'; then
    PRICE=$(echo "$QUOTE_MSFT" | grep -o '"price":[0-9.]*' | cut -d: -f2)
    echo -e "${GREEN}✓ MSFT quote: \$$PRICE${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Failed to get MSFT quote${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}6. Testing search_symbols${NC}"
SEARCH=$(tool_call "search_symbols" '{"query": "Apple"}')
if echo "$SEARCH" | grep -q '"symbol":"NASDAQ:AAPL"'; then
    echo -e "${GREEN}✓ Search returned AAPL${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Search failed${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}7. Testing get_fundamentals${NC}"
FUND=$(tool_call "get_fundamentals" '{"symbol": "NASDAQ:AAPL"}')
if echo "$FUND" | grep -q '"market_cap":' && echo "$FUND" | grep -q '"pe_ratio":'; then
    MARKET_CAP=$(echo "$FUND" | grep -o '"market_cap":[0-9.]*' | cut -d: -f2)
    echo -e "${GREEN}✓ Fundamentals: Market Cap \$$MARKET_CAP${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Failed to get fundamentals${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}8. Testing get_historical${NC}"
HIST=$(tool_call "get_historical" '{"symbol": "NASDAQ:AAPL", "interval": "1d", "count": 5}')
if echo "$HIST" | grep -q '"points":' && echo "$HIST" | grep -q '"open":'; then
    POINTS=$(echo "$HIST" | grep -o '"points":\[' | wc -l)
    echo -e "${GREEN}✓ Historical data returned with OHLCV points${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Failed to get historical data${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}9. Testing get_financial_statements${NC}"
FIN=$(tool_call "get_financial_statements" '{"symbol": "NASDAQ:AAPL", "period": "annual"}')
if echo "$FIN" | grep -q '"statements":' && echo "$FIN" | grep -q '"income_statement"'; then
    echo -e "${GREEN}✓ Financial statements returned${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Failed to get financial statements${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}10. Testing get_credit_ratings${NC}"
RATING=$(tool_call "get_credit_ratings" '{"symbol": "NASDAQ:AAPL"}')
if echo "$RATING" | grep -q '"rating":'; then
    echo -e "${GREEN}✓ Credit ratings returned${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Failed to get credit ratings${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}11. Testing calculate_rsi${NC}"
RSI=$(tool_call "calculate_rsi" '{"symbol": "NASDAQ:AAPL", "period": 14}')
if echo "$RSI" | grep -q '"rsi":'; then
    RSI_VAL=$(echo "$RSI" | grep -o '"rsi":[0-9.]*' | cut -d: -f2)
    echo -e "${GREEN}✓ RSI calculated: $RSI_VAL${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Failed to calculate RSI${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}12. Testing calculate_macd${NC}"
MACD=$(tool_call "calculate_macd" '{"symbol": "NASDAQ:AAPL", "fast": 12, "slow": 26, "signal": 9}')
if echo "$MACD" | grep -q '"macd_line":'; then
    echo -e "${GREEN}✓ MACD calculated${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Failed to calculate MACD${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}13. Testing Error Handling${NC}"
ERROR=$(tool_call "get_quote" '{"symbol": ""}')
if echo "$ERROR" | grep -q '"error":' && echo "$ERROR" | grep -q '32602'; then
    echo -e "${GREEN}✓ Error handling works correctly${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Error handling test failed${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}14. Testing scan_stocks${NC}"
SCAN=$(tool_call "scan_stocks" '{"market": "america", "limit": 10}')
if echo "$SCAN" | grep -q '"results":' && echo "$SCAN" | grep -q '"symbol":'; then
    echo -e "${GREEN}✓ Scan stocks returned results${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Scan stocks failed${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}15. Testing get_company_profile${NC}"
PROFILE=$(tool_call "get_company_profile" '{"symbol": "NASDAQ:AAPL"}')
if echo "$PROFILE" | grep -q '"name":' && echo "$PROFILE" | grep -q '"sector":'; then
    echo -e "${GREEN}✓ Company profile returned${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Failed to get company profile${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo -e "${YELLOW}16. Testing get_market_news${NC}"
NEWS=$(tool_call "get_market_news" '{"symbol": "NASDAQ:AAPL", "limit": 5}')
if echo "$NEWS" | grep -q '"news":'; then
    echo -e "${GREEN}✓ Market news returned${NC}"
    ((TESTS_PASSED++))
else
    echo -e "${RED}✗ Failed to get market news${NC}"
    ((TESTS_FAILED++))
fi
echo ""

echo "=========================================="
echo -e "${YELLOW}Test Results:${NC}"
echo "=========================================="
echo -e "${GREEN}Tests Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Tests Failed: $TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}✗ Some tests failed${NC}"
    exit 1
fi