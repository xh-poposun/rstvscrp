# AGENTS.md - tvdata-rs

## Build / Lint / Test Commands

```bash
# From the tvdata-rs crate root (cd tvdata-rs first)

# Format code
cargo fmt --all

# Check formatting (CI style)
cargo fmt --all --check

# Run clippy lints (all targets, all features)
cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test -q

# Run a single test by name
cargo test test_name_here

# Run tests matching a pattern
cargo test search

# Run tests in a specific module
cargo test scanner

# Check examples compile
cargo check --examples -q

# Build the crate
cargo build

# Build with all features
cargo build --all-features

# Release build
cargo build --release
```

## Project Overview

- **Name**: tvdata-rs
- **Type**: Async Rust client for TradingView APIs
- **Edition**: 2024, MSRV 1.85
- **Features**: search, equity, forex, crypto, calendar, economics, tracing

## Code Style Guidelines

### General Principles

- Library-first design: typed APIs, predictable behavior, clean module boundaries
- Prefer high-level product models over leaking raw TradingView payloads
- Keep low-level TradingView field ownership under `src/scanner/fields/`
- NO unsafe code (`#![forbid(unsafe_code)]`)
- Debug implementations required (`#![deny(missing_debug_implementations)]`)

### Modules & Organization

- Use thin facade modules
- Move heavy tests into sibling `tests.rs` files when modules grow
- Group related types in submodules (e.g., `scanner/fields/`)
- Feature-gated modules use `#[cfg(feature = "...")]`

### Naming Conventions

- **Types**: PascalCase (e.g., `TradingViewClient`, `ScanQuery`)
- **Functions**: snake_case (e.g., `to_query_pairs`, `is_retryable`)
- **Constants**: SCREAMING_SNAKE_CASE
- **Enums**: PascalCase variants, also prefix with type name in specific contexts
- **Modules**: snake_case
- **Fields/Properties**: snake_case

### Error Handling

- Use `thiserror` for error enum derivation
- Define `ErrorKind` enum for classification (RateLimited, AuthRequired, SymbolNotFound, Transport, Protocol, Unsupported, Api)
- Provide helper methods: `is_retryable()`, `is_auth_error()`, `is_rate_limited()`, etc.
- Use `pub type Result<T> = std::result::Result<T, Error>`
- Wrap errors with context using `#[source]` attribute in thiserror
- Prefer specific error variants over generic ones

### Imports

- Use absolute imports for crate-public items (`crate::module::Item`)
- Group std, external, then crate imports
- Prefer `super` for sibling module access
- Feature-gated exports in `lib.rs` with clear `#[cfg(...)]` blocks

### Types & Generics

- Use explicit type annotations in public APIs
- Prefer newtype wrappers for semantic clarity
- Use `Box<Error>` for dynamic error sources
- Feature flags must be additive (no breaking existing combinations)

### Formatting

- Follow standard Rust formatting (run `cargo fmt`)
- 100 character line width target
- Use Rust 2024 edition idioms

### Documentation

- Document public APIs with rustdoc
- Update `README.md` and `README.snippet.md` for user-facing changes
- Add examples in doc comments where helpful
- Keep feature docs in sync with feature flags

### Testing

- Unit tests should be deterministic and not depend on live TradingView
- Live checks go in `examples/`, not the test suite
- Use `#[cfg(test)]` module blocks within source files
- Use `wiremock` for HTTP mocking in tests
- Test error paths thoroughly

### Validation

- Never silently weaken validation to make requests pass
- Validate inputs at public API boundaries
- Use the scanner validation system for field compatibility checks

### What NOT to Add

- Built-in storage or database layers in crate core
- Global state or singletons
- Blocking operations in async contexts
- Undocumented feature flags

## File Organization

```
tvdata-rs/
├── src/
│   ├── lib.rs          # Main entry, public exports
│   ├── error.rs        # Error types and Result
│   ├── client/         # HTTP/WebSocket client
│   ├── scanner/        # Screening API
│   │   └── fields/    # TradingView field definitions
│   ├── history/        # OHLCV history
│   ├── search/        # Symbol search
│   ├── equity/        # Equity data
│   ├── forex/         # Forex data
│   ├── crypto/        # Crypto data
│   ├── calendar/     # Earnings/dividend calendars
│   └── economics/    # Economic events
├── tests/            # Integration tests
└── examples/         # Usage examples
```

## CI Validation (runs on PRs)

1. `cargo fmt --all --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test -q`
4. `cargo check --examples -q`

Run these before submitting changes.

## Development Tools

- **rust-analyzer**: 自动启用，提供实时类型检查、代码补全、跳转到定义等功能
- 在 VS Code 中打开项目目录即可，无需额外配置
- rust-analyzer 会自动检测 Cargo.toml 并加载项目