# tvdata-service

A TradingView data monitoring and alerting service built in Rust. Monitor stock prices, technical indicators (RSI, MACD), and corporate events (earnings, dividends, IPOs) with real-time alerts sent to Feishu.

## Features

- **Real-time Price Monitoring**: Track price changes and trigger alerts based on thresholds
- **Technical Indicators**: RSI and MACD indicator monitoring with customizable parameters
- **Calendar Events**: Monitor earnings, dividends, and IPO calendars
- **Feishu Alerts**: Rich interactive card alerts via Feishu webhook
- **REST API**: Full CRUD operations for monitors, rules, and alerts
- **SQLite Storage**: Zero-dependency local database
- **Proxy Support**: Automatic HTTP_PROXY/HTTPS_PROXY support
- **Chinese Stock Support**: Support for SSE (Shanghai), HKEX (Hong Kong) exchanges

## Requirements

| Component | Requirement |
|-----------|-------------|
| Rust | 1.85+ (MSRV) |
| SQLite | 3.x |

## Installation

```bash
# Clone and build
cd tvdata-service
cargo build --release

# Or run in development mode
cargo run
```

## Configuration

Edit `config.yaml` to configure the service:

```yaml
server:
  host: "0.0.0.0"   # Server bind address
  port: 8080        # Server port

database:
  path: "./data/tvmonitor.db"  # SQLite database path

monitor:
  check_interval_secs: 60     # Check interval in seconds
  max_concurrent_checks: 10   # Max concurrent monitoring tasks
  cooldown_secs: 300           # System-wide cooldown (seconds)
  max_monitors: 500           # Maximum monitors allowed

alert:
  rate_limit_per_hour: 10     # Webhook rate limit
  webhook:
    url: "https://open.feishu.cn/open-apis/bot/v2/hook/your-webhook-id"
    msg_type: "interactive"
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `HTTP_PROXY` | HTTP proxy URL |
| `HTTPS_PROXY` | HTTPS proxy URL |

The service automatically respects proxy settings from environment variables.

## API Endpoints

### Health Check

```bash
GET /health
```

Response:
```json
{"status": "ok"}
```

### Historical Data

```bash
GET /api/v1/history/:symbol
POST /api/v1/history/refresh/:symbol
```

### Graceful Shutdown

```bash
POST /shutdown
```

Triggers graceful server shutdown. Waits for in-flight requests to complete before exiting.

### Monitors

#### List Monitors

```bash
GET /api/v1/monitors
```

Response:
```json
{
  "data": [
    {
      "id": "uuid",
      "symbol": "NASDAQ:AAPL",
      "name": "Apple",
      "enabled": true,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    }
  ],
  "page": 1,
  "page_size": 20,
  "total": 1
}
```

#### Create Monitor

```bash
POST /api/v1/monitors
Content-Type: application/json

{
  "symbol": "NASDAQ:AAPL",
  "name": "Apple Inc."
}
```

#### Get Monitor

```bash
GET /api/v1/monitors/:id
```

#### Update Monitor

```bash
PUT /api/v1/monitors/:id
Content-Type: application/json

{
  "name": "Apple Updated",
  "enabled": false
}
```

#### Delete Monitor

```bash
DELETE /api/v1/monitors/:id
```

### Alert Rules

#### List Rules

```bash
GET /api/v1/rules
```

#### Create Rule

```bash
POST /api/v1/rules
Content-Type: application/json

{
  "monitor_id": "uuid-of-monitor",
  "rule_type": "price",
  "name": "Price Alert",
  "condition": {
    "op": ">",
    "threshold": 5.0
  },
  "severity": "warning",
  "cooldown_secs": 300
}
```

**Rule Types:**

| Type | Description | Condition Fields |
|------|-------------|------------------|
| `price` | Price change monitoring | `op`: comparison operator, `threshold`: percentage |
| `indicator` | RSI/MACD monitoring | `indicator`: RSI/MACD, `op`, `threshold`, `period` |
| `calendar` | Calendar event monitoring | `calendar_type`: earnings/dividends/ipo |

**Operators:**
- `>`, `>=`, `<`, `<=`, `==`, `!=`

**Severity Levels:**
- `info` - Informational
- `warning` - Warning
- `critical` - Critical

**Indicator Conditions:**

RSI Example:
```json
{
  "indicator": "RSI",
  "op": ">",
  "threshold": 70,
  "period": 14
}
```

MACD Example:
```json
{
  "indicator": "MACD",
  "op": ">",
  "threshold": 0,
  "fast": 12,
  "slow": 26,
  "signal": 9
}
```

#### Get Rule

```bash
GET /api/v1/rules/:id
```

#### Delete Rule

```bash
DELETE /api/v1/rules/:id
```

### Alerts

#### List Alerts

```bash
GET /api/v1/alerts?page=1&page_size=20&symbol=NASDAQ:AAPL&acknowledged=false
```

Query Parameters:
| Parameter | Type | Description |
|------------|------|-------------|
| `page` | int | Page number (default: 1) |
| `page_size` | int | Items per page (default: 20) |
| `symbol` | string | Filter by symbol |
| `acknowledged` | bool | Filter by acknowledgment status |

Response:
```json
{
  "data": [
    {
      "id": "uuid",
      "rule_id": "uuid",
      "symbol": "NASDAQ:AAPL",
      "message": "NASDAQ:AAPL: Price change 5.50% (threshold: 5.0 >)",
      "severity": "warning",
      "triggered_at": "2024-01-01T00:00:00Z",
      "acknowledged": false,
      "acknowledged_at": null,
      "ack_by": null
    }
  ],
  "page": 1,
  "page_size": 20,
  "total": 1
}
```

#### Acknowledge Alert

```bash
POST /api/v1/alerts/:id/ack
Content-Type: application/json

{
  "ack_by": "admin"
}
```

### Quotes

#### Get Quote

```bash
GET /api/v1/quotes/:symbol
```

Example:
```bash
GET /api/v1/quotes/NASDAQ:AAPL
```

Response:
```json
{
  "symbol": "NASDAQ:AAPL",
  "bid": 150.25,
  "ask": 150.30,
  "last": 150.28,
  "price": 150.28,
  "change": 2.50,
  "change_percent": 1.69,
  "volume": 50000000,
  "high": 151.00,
  "low": 148.50,
  "open": 148.00,
  "prev_close": 147.78,
  " timeframe": "D"
}
```

### Symbol Search

#### Search Symbols

```bash
GET /api/v1/search?q=apple&type=equity
```

Query Parameters:
| Parameter | Type | Description |
|------------|------|-------------|
| `q` | string | Search keyword (required) |
| `type` | string | Asset type: equity/forex/crypto (optional, default: equity) |

Example:
```bash
# Search for stocks
curl "http://localhost:8080/api/v1/search?q=aapl"

# Search for crypto
curl "http://localhost:8080/api/v1/search?q=btc&type=crypto"

# Search for forex
curl "http://localhost:8080/api/v1/search?q=eur&type=forex"
```

Response:
```json
[
  {
    "symbol": "NASDAQ:AAPL",
    "description": "Apple Inc",
    "exchange": "NASDAQ",
    "instrument_type": "stock"
  }
]
```

### Historical Data

```bash
GET /api/v1/history/:symbol?from=YYYY-MM-DD&to=YYYY-MM-DD
```

Query historical price data stored in local database.

**Parameters:**
- `symbol` - TradingView symbol (e.g., NASDAQ:AAPL)
- `from` - Start date (YYYY-MM-DD)
- `to` - End date (YYYY-MM-DD)

**Response:**
```json
[
  {
    "timestamp": 1704067200,
    "open": 185.50,
    "high": 187.20,
    "low": 184.80,
    "close": 186.50,
    "volume": 50000000
  }
]
```

**Manual Refresh:**
```bash
POST /api/v1/history/refresh/:symbol
```

Force refresh historical data for a symbol.

**Example:**
```bash
curl "http://localhost:8080/api/v1/history/NASDAQ:AAPL?from=2024-01-01&to=2024-12-31"
curl -X POST "http://localhost:8080/api/v1/history/refresh/NASDAQ:AAPL"
```

## Usage Examples

### Create a Price Alert

1. First, create a monitor for the symbol:
```bash
curl -X POST http://localhost:8080/api/v1/monitors \
  -H "Content-Type: application/json" \
  -d '{"symbol": "NASDAQ:AAPL", "name": "Apple Inc."}'
```

2. Then create a rule to monitor price changes:
```bash
curl -X POST http://localhost:8080/api/v1/rules \
  -H "Content-Type: application/json" \
  -d '{
    "monitor_id": "<monitor-id>",
    "rule_type": "price",
    "name": "Price Increase Alert",
    "condition": {"op": ">", "threshold": 5.0},
    "severity": "warning",
    "cooldown_secs": 300
  }'
```

### Create an RSI Alert

```bash
curl -X POST http://localhost:8080/api/v1/rules \
  -H "Content-Type: application/json" \
  -d '{
    "monitor_id": "<monitor-id>",
    "rule_type": "indicator",
    "name": "RSI Overbought",
    "condition": {
      "indicator": "RSI",
      "op": ">",
      "threshold": 70,
      "period": 14
    },
    "severity": "warning",
    "cooldown_secs": 300
  }'
```

### Query Alerts

```bash
# List all unacknowledged alerts
curl "http://localhost:8080/api/v1/alerts?acknowledged=false"

# Filter by symbol
curl "http://localhost:8080/api/v1/alerts?symbol=NASDAQ:AAPL"
```

### Acknowledge an Alert

```bash
curl -X POST http://localhost:8080/api/v1/alerts/<alert-id>/ack \
  -H "Content-Type: application/json" \
  -d '{"ack_by": "admin"}'
```

## Architecture

```
tvdata-service/
├── src/
│   ├── main.rs           # Application entry point
│   ├── lib.rs           # Library exports
│   ├── config.rs         # Configuration management
│   ├── db.rs             # Database operations
│   ├── error.rs          # Error types
│   ├── models.rs         # Domain models
│   ├── tvclient.rs       # TradingView client wrapper
│   ├── api/              # REST API handlers
│   │   ├── monitors.rs   # Monitor CRUD
│   │   ├── rules.rs      # Alert rule CRUD
│   │   ├── alerts.rs     # Alert management
│   │   └── quotes.rs     # Quote retrieval
│   ├── monitor/          # Monitoring engine
│   │   ├── price.rs      # Price monitoring
│   │   ├── indicator.rs  # Indicator monitoring
│   │   └── calendar.rs   # Calendar event monitoring
│   ├── alert/            # Alert system
│   │   └── webhook.rs    # Feishu webhook integration
│   └── indicators/       # Technical indicators
│       ├── rsi.rs        # Relative Strength Index
│       └── macd.rs       # Moving Average Convergence Divergence
├── config.yaml           # Service configuration
└── Cargo.toml            # Dependencies
```

## Database Schema

### monitors
| Column | Type | Description |
|--------|------|-------------|
| id | TEXT | UUID primary key |
| symbol | TEXT | TradingView symbol (e.g., NASDAQ:AAPL) |
| name | TEXT | Optional display name |
| enabled | INTEGER | Enable/disable flag |
| created_at | TEXT | Creation timestamp |
| updated_at | TEXT | Last update timestamp |

### alert_rules
| Column | Type | Description |
|--------|------|-------------|
| id | TEXT | UUID primary key |
| monitor_id | TEXT | Foreign key to monitors |
| rule_type | TEXT | price, indicator, or calendar |
| name | TEXT | Rule name |
| condition | TEXT | JSON condition |
| severity | TEXT | info, warning, or critical |
| cooldown_secs | INTEGER | Per-rule cooldown |
| enabled | INTEGER | Enable/disable flag |
| created_at | TEXT | Creation timestamp |

### alerts
| Column | Type | Description |
|--------|------|-------------|
| id | TEXT | UUID primary key |
| rule_id | TEXT | Foreign key to alert_rules |
| symbol | TEXT | Symbol that triggered alert |
| message | TEXT | Alert message |
| severity | TEXT | Alert severity |
| triggered_at | TEXT | Trigger timestamp |
| acknowledged | INTEGER | Acknowledgment status |
| acknowledged_at | TEXT | Acknowledgment timestamp |
| ack_by | TEXT | Acknowledged by |

## Limits

| Resource | Limit |
|----------|-------|
| Max Monitors | 500 |
| Check Interval | 60 seconds (default) |
| Cooldown | 300 seconds (system-wide) |
| Alert Rate Limit | 10 per hour (webhook) |

## Development

```bash
# Run tests
cargo test

# Run with specific test threads (required for some tests)
cargo test -- --test-threads=1

# Run clippy lints
cargo clippy --all-targets --all-features -- -D warnings

# Format code
cargo fmt --all
```

## License

MIT
