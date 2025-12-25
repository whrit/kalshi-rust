# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an async Rust wrapper for the Kalshi trading API, providing both HTTPS and WebSocket support for building trading bots on the Kalshi prediction markets platform.

## Build Commands

```bash
# Build the library
cd kalshi && cargo build

# Build in release mode
cd kalshi && cargo build --release

# Run all tests
cd kalshi && cargo test

# Run specific test category
cd kalshi && cargo test auth_tests
cd kalshi && cargo test market_tests
cd kalshi && cargo test portfolio_tests
cd kalshi && cargo test exchange_tests

# Run a single test
cd kalshi && cargo test test_name -- --exact

# Run tests with output visible
cd kalshi && cargo test -- --nocapture

# Check code without building
cd kalshi && cargo check

# Build the sample bot
cd sample_bot && cargo build
```

## Repository Structure

This is a Cargo workspace with two members:
- `kalshi/` - The main library crate
- `sample_bot/` - Example trading bot demonstrating library usage

## Architecture

### Core Type: `Kalshi` Struct (`kalshi/src/lib.rs`)

The `Kalshi` struct is the central interface for all API interactions. It:
- Holds authentication credentials (key_id, private_key)
- Maintains an HTTP client for requests
- Determines base URL based on `TradingEnvironment` (DemoMode vs ProdMode)
- Authenticates on construction via `Kalshi::new()`

### Module Organization (`kalshi/src/`)

Each API domain is a separate module with its own `mod.rs`:
- `auth.rs` - RSA-PSS signing for authenticated requests (`signed_get`, `signed_post`, etc.)
- `portfolio/` - Orders, positions, fills, settlements, balance
- `market/` - Markets, orderbooks, trades, series, candlesticks
- `events/` - Event data and multi-event queries
- `exchange/` - Exchange status and schedule
- `api_keys/` - API key management
- `collection/` - Collection-related endpoints
- `search/` - Search functionality
- `live_data/` - WebSocket/streaming data (in development)

### Authentication Pattern

All authenticated endpoints use RSA-PSS signing (SHA-256). The signing flow in `auth.rs`:
1. Build message: `{timestamp_ms}{METHOD}/trade-api/v2{path}`
2. Sign with RSA-PSS padding
3. Add headers: `KALSHI-ACCESS-KEY`, `KALSHI-ACCESS-TIMESTAMP`, `KALSHI-ACCESS-SIGNATURE`

### Error Handling

`KalshiError` enum in `kalshi_error.rs` covers:
- `UserInputError` - Client-side validation failures
- `HttpError` - Network/HTTP failures
- `InternalError` - Unexpected conditions
- IO, OpenSSL, URL parsing errors

### Query Parameter Macro

The `add_param!` macro in `utils.rs` simplifies optional parameter handling for API queries.

## Testing

Tests require Kalshi API credentials. Set up via environment variables:
```bash
KALSHI_DEMO_API_KEY=your-key
KALSHI_DEMO_PEM_PATH=/path/to/private.pem
KALSHI_TEST_ENV=demo
```

See `kalshi/TESTING.md` for detailed test setup instructions.

## Key Enums

- `TradingEnvironment` - DemoMode (paper trading) vs ProdMode (real money)
- `Action` - Buy/Sell
- `Side` - Yes/No
- `OrderType` - Market/Limit
- `OrderStatus` - Resting/Canceled/Executed/Pending

## API Version

The library targets Kalshi API v2 (`/trade-api/v2`). Prices are in cents; dollar string fields (e.g., `yes_price_dollars`) are also available.
