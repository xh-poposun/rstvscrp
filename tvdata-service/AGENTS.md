# AGENTS.md - tvdata-service

## Build / Lint / Test Commands

```bash
# From the tvdata-service directory

# Format code
cargo fmt --all

# Check formatting (CI style)
cargo fmt --all --check

# Run clippy lints
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test

# Run a single test by name
cargo test test_name_here

# Run tests matching a pattern
cargo test api

# Build the crate
cargo build

# Build release
cargo build --release

# Run the service
cargo run
```

## Project Overview

- **Name**: tvdata-service
- **Type**: TradingView data monitoring and alerting service
- **Edition**: 2024, MSRV 1.85
- **Dependencies**: tvdata-rs, axum, sqlx, tokio, serde, reqwest

## Code Style Guidelines

### General Principles

- NO unsafe code
- Debug implementations required
- Use global DB pool via OnceCell for state management in API handlers

### Modules & Organization

```
src/
├── main.rs           # Entry point, Axum server
├── lib.rs            # Public exports
├── config.rs         # YAML configuration
├── db.rs             # SQLite connection + migrations
├── error.rs          # Error types (thiserror)
├── models.rs         # Domain models (Monitor, Alert, Rule)
├── tvclient.rs       # tvdata-rs client wrapper
├── api/              # REST API handlers
│   ├── monitors.rs   # Monitor CRUD
│   ├── rules.rs     # Alert rule CRUD
│   ├── alerts.rs   # Alert list + acknowledge
│   └── quotes.rs   # Real-time quotes
├── monitor/          # Monitoring engine
│   ├── price.rs
│   ├── indicator.rs
│   └── calendar.rs
├── alert/            # Alert engine
│   ├── mod.rs
│   └── webhook.rs   # Feishu webhook
└── indicators/        # Technical indicators
    ├── rsi.rs
    └── macd.rs
```

### Naming Conventions

- **Types**: PascalCase (e.g., `Monitor`, `AlertRule`)
- **Functions**: snake_case (e.g., `list_monitors`, `create_rule`)
- **Modules**: snake_case
- **Fields/Properties**: snake_case

### Error Handling

- Use `thiserror` for error enum derivation
- Define `ErrorKind` enum for classification
- Use `pub type Result<T> = std::result::Result<T, Error>`
- Implement `axum::response::IntoResponse` for Error

### Testing

- Tests may use global DB pool (OnceCell)
- Use `#[ignore]` for integration tests that require external services
- Clean up test data using DELETE statements in `setup_global_pool`

### Configuration (config.yaml)

```yaml
server:
  host: "0.0.0.0"
  port: 8080

database:
  path: "./data/tvmonitor.db"

monitor:
  check_interval_secs: 60
  max_concurrent_checks: 10
  cooldown_secs: 300
  max_monitors: 500

alert:
  rate_limit_per_hour: 10
  webhook:
    url: "https://open.feishu.cn/open-apis/bot/v2/hook/xxx"
    msg_type: "interactive"
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/api/v1/monitors` | GET | List monitors |
| `/api/v1/monitors` | POST | Create monitor |
| `/api/v1/monitors/:id` | GET | Get monitor |
| `/api/v1/monitors/:id` | PUT | Update monitor |
| `/api/v1/monitors/:id` | DELETE | Delete monitor |
| `/api/v1/rules` | GET | List rules |
| `/api/v1/rules` | POST | Create rule |
| `/api/v1/rules/:id` | GET | Get rule |
| `/api/v1/rules/:id` | DELETE | Delete rule |
| `/api/v1/alerts` | GET | List alerts (paginated) |
| `/api/v1/alerts/:id/ack` | POST | Acknowledge alert |
| `/api/v1/quotes/:symbol` | GET | Get quote |

## CI Validation

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Development Tools

- **rust-analyzer**: 自动启用，无需额外配置
- 使用 `cargo run` 启动服务
- SQLite 数据库文件自动创建于配置路径
