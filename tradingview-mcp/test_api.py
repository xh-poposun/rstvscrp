#!/usr/bin/env python3
"""
API Verification Test for TradingView MCP Server
Tests all endpoints to verify real data is returned
"""

import requests
import json
import sys

BASE_URL = "http://localhost:3000"
MCP_ENDPOINT = f"{BASE_URL}/mcp"

def mcp_call(method, params=None):
    """Make MCP request"""
    payload = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params or {}
    }
    resp = requests.post(MCP_ENDPOINT, json=payload, headers={"Content-Type": "application/json"})
    return resp.json()

def tool_call(name, args):
    """Call MCP tool"""
    return mcp_call("tools/call", {"name": name, "arguments": args})

def test_health():
    """Test health endpoint"""
    resp = requests.get(f"{BASE_URL}/health")
    data = resp.json()
    assert data.get("status") == "ok", "Health check failed"
    print("✓ Health check passed")

def test_initialize():
    """Test initialize endpoint"""
    result = mcp_call("initialize", {
        "protocolVersion": "2024-11-05",
        "clientInfo": {"name": "test", "version": "1.0"}
    })
    assert "result" in result, "Initialize failed"
    assert result["result"].get("protocolVersion") == "2024-11-05", "Protocol version mismatch"
    print("✓ Initialize passed")

def test_tools_list():
    """Test tools/list endpoint"""
    result = mcp_call("tools/list")
    tools = result.get("result", {}).get("tools", [])
    assert len(tools) == 15, f"Expected 15 tools, got {len(tools)}"
    print(f"✓ All 15 tools registered")

def test_get_quote_aapl():
    """Test get_quote for AAPL"""
    result = tool_call("get_quote", {"symbol": "NASDAQ:AAPL"})
    content = json.loads(result["result"]["content"][0]["text"])
    assert "price" in content, "No price in response"
    assert content["currency"] == "USD", "Wrong currency"
    print(f"✓ AAPL quote: ${content['price']}")

def test_get_quote_msft():
    """Test get_quote for MSFT"""
    result = tool_call("get_quote", {"symbol": "NASDAQ:MSFT"})
    content = json.loads(result["result"]["content"][0]["text"])
    assert "price" in content, "No price in response"
    print(f"✓ MSFT quote: ${content['price']}")

def test_search_symbols():
    """Test search_symbols"""
    result = tool_call("search_symbols", {"query": "Apple"})
    content = json.loads(result["result"]["content"][0]["text"])
    assert len(content["results"]) > 0, "No search results"
    print(f"✓ Search returned {len(content['results'])} results")

def test_get_fundamentals():
    """Test get_fundamentals"""
    result = tool_call("get_fundamentals", {"symbol": "NASDAQ:AAPL"})
    content = json.loads(result["result"]["content"][0]["text"])
    assert "market_cap" in content, "No market_cap in response"
    print(f"✓ Fundamentals: Market Cap ${content['market_cap']:,.0f}")

def test_get_historical():
    """Test get_historical"""
    result = tool_call("get_historical", {
        "symbol": "NASDAQ:AAPL",
        "interval": "1d",
        "count": 5
    })
    content = json.loads(result["result"]["content"][0]["text"])
    assert len(content["points"]) == 5, "Expected 5 data points"
    print(f"✓ Historical data: {len(content['points'])} points")

def test_get_financial_statements():
    """Test get_financial_statements"""
    result = tool_call("get_financial_statements", {
        "symbol": "NASDAQ:AAPL",
        "period": "annual"
    })
    content = json.loads(result["result"]["content"][0]["text"])
    assert "statements" in content, "No statements in response"
    print(f"✓ Financial statements: {len(content['statements'])} statements")

def test_get_credit_ratings():
    """Test get_credit_ratings"""
    result = tool_call("get_credit_ratings", {"symbol": "NASDAQ:AAPL"})
    content = json.loads(result["result"]["content"][0]["text"])
    assert "rating" in content, "No rating in response"
    print(f"✓ Credit ratings returned")

def test_calculate_rsi():
    """Test calculate_rsi"""
    result = tool_call("calculate_rsi", {
        "symbol": "NASDAQ:AAPL",
        "period": 14
    })
    content = json.loads(result["result"]["content"][0]["text"])
    assert "rsi" in content, "No RSI in response"
    print(f"✓ RSI calculated: {content['rsi']}")

def test_calculate_macd():
    result = tool_call("calculate_macd", {
        "symbol": "NASDAQ:AAPL",
        "fast": 12,
        "slow": 26,
        "signal": 9
    })
    content = json.loads(result["result"]["content"][0]["text"])
    assert "macd" in content, "No MACD in response"
    print(f"✓ MACD calculated: {content['macd']}")

def test_error_handling():
    """Test error handling"""
    result = tool_call("get_quote", {"symbol": ""})
    assert "error" in result, "Expected error for empty symbol"
    assert result["error"]["code"] == -32602, "Wrong error code"
    print(f"✓ Error handling works correctly")

def test_scan_stocks():
    result = tool_call("scan_stocks", {"market": "america", "limit": 10, "filters": None})
    content = json.loads(result["result"]["content"][0]["text"])
    assert "results" in content, "No results in response"
    print(f"✓ Scan stocks: {len(content['results'])} results")

def test_get_company_profile():
    result = tool_call("get_company_profile", {"symbol": "NASDAQ:AAPL"})
    content = json.loads(result["result"]["content"][0]["text"])
    has_profile = "profile" in content or "name" in content or "description" in content or "overview" in content
    assert has_profile, "No profile data in response"
    name = content.get("name", content.get("overview", {}).get("name", "Unknown"))
    print(f"✓ Company profile: {name}")

def test_get_market_news():
    result = tool_call("get_market_news", {"symbol": "NASDAQ:AAPL", "limit": 5})
    content = json.loads(result["result"]["content"][0]["text"])
    assert "news" in content or "articles" in content or "headlines" in content, "No news data in response"
    news_count = len(content.get("news", content.get("articles", content.get("headlines", []))))
    print(f"✓ Market news: {news_count} articles")

def main():
    print("=" * 50)
    print("TradingView MCP API Verification Test")
    print("=" * 50)
    print()
    
    tests = [
        test_health,
        test_initialize,
        test_tools_list,
        test_get_quote_aapl,
        test_get_quote_msft,
        test_search_symbols,
        test_get_fundamentals,
        test_get_historical,
        test_get_financial_statements,
        test_get_credit_ratings,
        test_calculate_rsi,
        test_calculate_macd,
        test_error_handling,
        test_scan_stocks,
        test_get_company_profile,
        test_get_market_news,
    ]
    
    passed = 0
    failed = 0
    
    for test in tests:
        try:
            test()
            passed += 1
        except Exception as e:
            print(f"✗ {test.__name__}: {e}")
            failed += 1
        print()
    
    print("=" * 50)
    print(f"Results: {passed} passed, {failed} failed")
    print("=" * 50)
    
    if failed == 0:
        print("✓ All tests passed!")
        return 0
    else:
        print("✗ Some tests failed")
        return 1

if __name__ == "__main__":
    sys.exit(main())