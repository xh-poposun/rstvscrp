# AGENTS.md - rstvscrp Root

## OVERVIEW

Rust monorepo with 4 crates. No formal Cargo workspace. Uses `patch.crates-io` for local fork dependencies (tungstenite-rs, tokio-tungstenite).

## STRUCTURE

```
tvdata-service/   # TradingView monitoring service (binary, Axum + SQLite)
tvdata-rs/        # TradingView client library (async)
tokio-tungstenite/ # WebSocket bindings for Tokio (fork)
tungstenite-rs/  # WebSocket core (fork)
```

## WHERE TO LOOK

| Crate | Entry Point | Purpose |
|-------|-------------|---------|
| tvdata-service | `src/main.rs` | HTTP server, monitoring engine |
| tvdata-rs | `src/lib.rs` | TradingView API client |
| tokio-tungstenite | `src/lib.rs` | Async WebSocket |
| tungstenite-rs | `src/lib.rs` | Sync WebSocket |

## CONVENTIONS

- **Format**: `cargo fmt --all`
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Test**: `cargo test`
- **MSRV**: 1.85 (Rust 2024 edition)
- **Testing**: wiremock for HTTP mocking in tvdata-rs
- **Features**: additive, feature-gated modules in lib.rs
- **Naming**: PascalCase types, snake_case functions, SCREAMING_SNAKE_CASE constants

## ANTI-PATTERNS

1. **No workspace** - Each crate independent. No root `[workspace]`. Build per-crate.
2. **wss_debug.log** - Check `tvdata-rs/src/` for stray debug files.
3. **Outdated CI actions** - `.github/workflows/ci.yml` in each crate.
4. **Global state** - Avoid singletons/OnceCell in library crates.
5. **Blocking in async** - Never block in async contexts.

## UNIQUE STYLES

1. **Local patches** - `patch.crates-io` points to sibling crates
2. **Feature-gated modules** - tvdata-rs uses `#[cfg(feature = "...")]`
3. **Request budgets** - Backend-oriented config with concurrency limits
4. **Capability-aware validation** - `validate_scan_query`, `scan_supported`
5. **Preset clients** - `for_backend_history()`, `for_research()`, `for_interactive()`

## COMMANDS

```bash
# Build
cd tvdata-service && cargo build
cd tvdata-rs && cargo build
cd tokio-tungstenite && cargo build
cd tungstenite-rs && cargo build

# Lint
cd tvdata-service && cargo clippy --all-targets --all-features -- -D warnings
cd tvdata-rs && cargo clippy --all-targets --all-features -- -D warnings

# Test
cd tvdata-service && cargo test
cd tvdata-rs && cargo test
```

## NOTES

- TradingView API unofficial - expect payload drift
- CI runs per-crate, no unified root pipeline
- Fork crates treated as external
- Use `#[ignore]` for integration tests requiring live TradingView
