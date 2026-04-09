# TradingView MCP Server

Free financial data via TradingView API for DCF, Comps, and LBO analysis.

## Installation

```bash
cargo build --release
```

## Usage

```bash
./target/release/tradingview-mcp
```

## Tools

### Market Data

- **get_quote** - Current price, volume, and daily change
- **get_historical** - Historical OHLCV data with configurable intervals
- **search_symbols** - Find symbols by name or ticker

### Fundamental Data

- **get_fundamentals** - Key metrics (P/E, market cap, EPS, beta, etc.)
- **get_financial_statements** - Full income statement, balance sheet, and cash flow data
- **get_company_profile** - Sector, industry, and company description

### Credit & Debt

- **get_credit_ratings** - Fitch and S&P credit ratings
- **get_debt_maturity** - SEC EDGAR debt maturity schedule

### Screening & Discovery

- **scan_stocks** - Screen by market cap, P/E, sector, and more
- **get_earnings_calendar** - Upcoming earnings announcements
- **get_dividend_calendar** - Dividend payment schedule
- **get_market_news** - Market headlines by topic

### Technical Analysis

- **calculate_rsi** - Relative Strength Index
- **calculate_macd** - MACD indicator values
- **compute_macd_signal** - MACD signal line computation

## Examples

See the `examples/` directory for detailed workflow examples:

- [DCF Analysis](examples/dcf_analysis.md) - Discounted Cash Flow modeling
- [Comps Analysis](examples/comps_analysis.md) - Comparable company valuation

## Configuration

The server runs as an MCP (Model Context Protocol) server and communicates via stdin/stdout. Connect it to Claude Code or any MCP-compatible client.

## Data Sources

- TradingView API for market data and fundamentals
- SEC EDGAR for debt maturity information
- Real-time quotes and historical data

## License

MIT
