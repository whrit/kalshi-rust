# kalshi-rust

[![Crates.io](https://img.shields.io/crates/v/kalshi.svg)](https://crates.io/crates/kalshi)
[![Documentation](https://docs.rs/kalshi/badge.svg)](https://docs.rs/kalshi)
[![License](https://img.shields.io/crates/l/kalshi.svg)](https://github.com/dpeachpeach/kalshi-rust)

A comprehensive async Rust wrapper for the [Kalshi](https://kalshi.com/) trading API, providing both HTTPS and WebSocket support for building high-performance trading bots on the Kalshi prediction markets platform.

## Features

- **Async/Await**: Built on `tokio` for high-performance concurrent operations
- **Complete API Coverage**: Full support for Kalshi API v2 endpoints
- **WebSocket Support**: Real-time market data and order updates
- **Type Safety**: Strongly typed API with comprehensive error handling
- **RSA-PSS Authentication**: Secure key-based authentication
- **Demo & Production**: Support for both paper trading (demo) and live trading environments
- **Batch Operations**: Efficient batch order creation and cancellation
- **Comprehensive Examples**: Well-documented examples for common use cases

## Installation

Add `kalshi` to your `Cargo.toml`:

```toml
[dependencies]
kalshi = "0.9"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Getting API Credentials

1. Sign up for an account at [demo.kalshi.com](https://demo.kalshi.com) (for testing) or [kalshi.com](https://kalshi.com) (for production)
2. Navigate to your account settings and generate an API key
3. Download the private key file (`.pem` format)
4. Save the key ID (UUID format) shown in the UI

### Basic Example

```rust
use kalshi::{Kalshi, TradingEnvironment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with key-based authentication
    let kalshi = Kalshi::new(
        TradingEnvironment::DemoMode,
        "your-key-id",
        "path/to/private.pem"
    ).await?;

    // Get exchange status
    let status = kalshi.get_exchange_status().await?;
    println!("Exchange is open: {}", status.trading_active);

    // Fetch markets
    let (cursor, markets) = kalshi.get_markets(
        Some(10),  // limit
        None,      // cursor
        None,      // event_ticker
        None,      // series_ticker
        Some("open".to_string()),  // status
        None, None, None, None, None, None, None, None
    ).await?;

    println!("Found {} markets", markets.len());

    Ok(())
}
```

## Authentication

kalshi-rust uses RSA-PSS key-based authentication for secure API access. Authentication is handled automatically when creating a `Kalshi` instance.

### Demo vs Production Environments

```rust
// Demo mode (paper trading - recommended for testing)
let kalshi_demo = Kalshi::new(
    TradingEnvironment::DemoMode,
    key_id,
    pem_path
).await?;

// Production mode (real money trading)
let kalshi_prod = Kalshi::new(
    TradingEnvironment::ProdMode,
    key_id,
    pem_path
).await?;
```

### Credential Management

Store your credentials securely using environment variables:

```rust
use std::env;

let key_id = env::var("KALSHI_API_KEY")?;
let pem_path = env::var("KALSHI_PEM_PATH")?;

let kalshi = Kalshi::new(
    TradingEnvironment::DemoMode,
    &key_id,
    &pem_path
).await?;
```

## Core Features

### Trading Operations

#### Create Orders

```rust
use kalshi::{Action, Side, OrderType};

// Create a limit order to buy 10 "Yes" contracts at 55 cents
let order = kalshi.create_order(
    Action::Buy,
    None,                          // client_order_id (auto-generated if None)
    10,                            // count
    Side::Yes,
    "MARKET-TICKER-2024".to_string(),
    OrderType::Limit,
    None,                          // buy_max_cost
    None,                          // expiration_ts
    Some(55),                      // yes_price (cents)
    None,                          // no_price
    None,                          // sell_position_floor
    None,                          // yes_price_dollars
    None,                          // no_price_dollars
    None,                          // time_in_force
    None,                          // post_only
    None,                          // reduce_only
    None,                          // self_trade_prevention_type
    None,                          // order_group_id
    None,                          // cancel_order_on_pause
).await?;

println!("Order ID: {}", order.order_id);
```

#### Batch Order Creation

```rust
use kalshi::OrderCreationField;

let orders = vec![
    OrderCreationField {
        action: Action::Buy,
        count: 5,
        side: Side::Yes,
        ticker: "MARKET1-2024".to_string(),
        input_type: OrderType::Limit,
        yes_price: Some(50),
        client_order_id: None,
        // ... other fields
    },
    OrderCreationField {
        action: Action::Buy,
        count: 10,
        side: Side::No,
        ticker: "MARKET2-2024".to_string(),
        input_type: OrderType::Limit,
        no_price: Some(45),
        client_order_id: None,
        // ... other fields
    },
];

let results = kalshi.batch_create_order(orders).await?;

for (i, result) in results.iter().enumerate() {
    match result {
        Ok(order) => println!("Order {} created: {}", i, order.order_id),
        Err(e) => println!("Order {} failed: {}", i, e),
    }
}
```

#### Amend Orders

```rust
use kalshi::{Side, Action};

// Change the price of an existing order
let response = kalshi.amend_order(
    "order-uuid",
    "MARKET-TICKER",
    Side::Yes,
    Action::Buy,
    "original-client-id",
    "updated-client-id",
    Some(60),  // new yes_price
    None,      // no_price
    None,      // yes_price_dollars
    None,      // no_price_dollars
    Some(15),  // new count
).await?;

println!("Old order: {:?}", response.old_order);
println!("New order: {:?}", response.order);
```

#### Cancel Orders

```rust
// Cancel a single order
let (order, reduced_by) = kalshi.cancel_order("order-id").await?;
println!("Cancelled order, reduced by {} contracts", reduced_by);

// Batch cancel orders
let order_ids = vec!["order-1".to_string(), "order-2".to_string()];
let results = kalshi.batch_cancel_order(order_ids).await?;
```

#### Get Portfolio Information

```rust
// Get account balance
let balance = kalshi.get_balance().await?;
println!("Balance: {} cents", balance);

// Get positions
let (cursor, event_positions, market_positions) = kalshi.get_positions(
    None, None, None, None, None, None
).await?;

for position in market_positions {
    println!("Market: {}, Position: {}, PnL: {}",
        position.ticker, position.position, position.realized_pnl);
}

// Get fills
let (cursor, fills) = kalshi.get_fills(
    Some("MARKET-TICKER".to_string()),
    None, None, None, Some(100), None
).await?;

// Get settlements
let (cursor, settlements) = kalshi.get_settlements(
    Some(100), None, None, None, None, None
).await?;
```

### Market Data

#### Get Markets

```rust
// Get all open markets
let (cursor, markets) = kalshi.get_markets(
    Some(20),                      // limit
    None,                          // cursor
    None,                          // event_ticker
    None,                          // series_ticker
    Some("open".to_string()),      // status
    None,                          // tickers
    None, None, None, None, None, None, None
).await?;

for market in markets {
    println!("{}: {} - Last price: {}",
        market.ticker, market.title, market.last_price);
}

// Get a specific market
let market = kalshi.get_market("MARKET-TICKER-2024").await?;
println!("Market: {}", market.title);
```

#### Get Orderbook

```rust
// Get orderbook with depth limit
let orderbook = kalshi.get_orderbook("MARKET-TICKER", Some(10)).await?;

// Best bid/ask in cents
if let Some(yes_bids) = orderbook.yes {
    if let Some(best_bid) = yes_bids.first() {
        println!("Best Yes bid: {} cents, size: {}", best_bid[0], best_bid[1]);
    }
}

// Dollar prices (as tuples of (f32, i32))
for (price, size) in &orderbook.yes_dollars {
    println!("Yes: ${:.4} x {}", price, size);
}
```

#### Get Candlestick Data

```rust
use chrono::Utc;

let now = Utc::now().timestamp();
let one_day_ago = now - 86400;

// Single market candlesticks
let candlesticks = kalshi.get_market_candlesticks(
    "MARKET-TICKER",
    "SERIES-TICKER",
    Some(one_day_ago),
    Some(now),
    Some(60)  // 60-minute candles
).await?;

for candle in candlesticks {
    println!("Time: {}, Yes Close: {}, Volume: {}",
        candle.end_ts, candle.yes_close, candle.volume);
}

// Batch candlestick retrieval for multiple markets
let tickers = vec!["MARKET-1".to_string(), "MARKET-2".to_string()];
let batch_data = kalshi.batch_get_market_candlesticks(
    tickers,
    one_day_ago,
    now,
    60,
    None
).await?;

for market_data in batch_data {
    println!("Market: {}, Candles: {}",
        market_data.ticker, market_data.candlesticks.len());
}
```

#### Get Trades

```rust
// Get recent trades for a market
let (cursor, trades) = kalshi.get_trades(
    Some(100),
    None,
    Some("MARKET-TICKER".to_string()),
    None,
    None
).await?;

for trade in trades {
    println!("Trade: {} contracts at {} cents",
        trade.count, trade.yes_price);
}
```

#### Get Series and Events

```rust
// Get series
let (cursor, series_list) = kalshi.get_series_list(
    Some(20),
    None,
    Some("politics".to_string()),
    None
).await?;

// Get specific series
let series = kalshi.get_series("SERIES-TICKER").await?;

// Get events
let (cursor, events) = kalshi.get_multiple_events(
    Some(10),
    None,
    None,
    None,
    None
).await?;
```

### WebSocket Real-Time Data

Connect to the WebSocket API for real-time market data and trading updates.

```rust
use kalshi::Channel;
use futures_util::StreamExt;

// Create WebSocket client from existing Kalshi instance
let mut ws = kalshi.websocket();

// Connect to WebSocket
ws.connect().await?;

// Subscribe to orderbook updates for a market
let sub = ws.subscribe(Channel::Orderbook {
    ticker: "MARKET-TICKER".to_string(),
}).await?;

println!("Subscribed with SID: {}", sub.sid);

// Listen for messages
while let Some(msg_result) = ws.next_message().await {
    match msg_result {
        Ok(msg) => {
            println!("Received: {:?}", msg);
        }
        Err(e) => {
            eprintln!("WebSocket error: {}", e);
            break;
        }
    }
}

// Unsubscribe and disconnect
ws.unsubscribe(sub.sid).await?;
ws.disconnect().await?;
```

#### Available WebSocket Channels

```rust
use kalshi::Channel;

// Market-specific channels
let orderbook_channel = Channel::Orderbook {
    ticker: "MARKET-TICKER".to_string(),
};

let ticker_channel = Channel::Ticker {
    ticker: "MARKET-TICKER".to_string(),
};

let trade_channel = Channel::Trade {
    ticker: "MARKET-TICKER".to_string(),
};

// Portfolio channels
let fill_channel = Channel::Fill;
let order_channel = Channel::OrderUpdate;

// Subscribe to a channel
let subscription = ws.subscribe(orderbook_channel).await?;
```

### Exchange Information

```rust
// Get exchange status
let status = kalshi.get_exchange_status().await?;
println!("Trading active: {}", status.trading_active);
println!("Exchange active: {}", status.exchange_active);

// Get exchange schedule
let schedule = kalshi.get_exchange_schedule().await?;
println!("Standard hours: {:?}", schedule.standard_hours);
```

## Error Handling

kalshi-rust provides comprehensive error handling through the `KalshiError` enum:

```rust
use kalshi::KalshiError;

match kalshi.create_order(/* ... */).await {
    Ok(order) => {
        println!("Order created: {}", order.order_id);
    }
    Err(e) => match e {
        KalshiError::RequestError(req_err) => {
            eprintln!("Request failed: {}", req_err);
        }
        KalshiError::UserInputError(msg) => {
            eprintln!("Invalid input: {}", msg);
        }
        KalshiError::Auth(msg) => {
            eprintln!("Authentication error: {}", msg);
        }
        KalshiError::InternalError(msg) => {
            eprintln!("Internal error: {}", msg);
        }
    }
}
```

### Error Types

- **`RequestError`**: HTTP/network errors, serialization failures, client/server errors
- **`UserInputError`**: Invalid parameters or input validation failures
- **`Auth`**: Authentication and authorization errors
- **`InternalError`**: Unexpected internal errors (please report these!)

## API Reference

### Core Modules

| Module | Description |
|--------|-------------|
| `portfolio` | Orders, positions, fills, settlements, balance management |
| `market` | Markets, orderbooks, trades, series, candlesticks |
| `events` | Event data and multi-event queries |
| `exchange` | Exchange status and trading schedule |
| `websocket` | Real-time WebSocket data streaming |
| `api_keys` | API key management |
| `collection` | Collection-related endpoints |
| `search` | Search functionality |
| `live_data` | Live milestone data feeds |

### Key Enums

```rust
// Trading environment
pub enum TradingEnvironment {
    DemoMode,  // Paper trading
    ProdMode,  // Real money
}

// Order parameters
pub enum Action { Buy, Sell }
pub enum Side { Yes, No }
pub enum OrderType { Market, Limit }
pub enum OrderStatus { Resting, Canceled, Executed, Pending }

// Advanced order parameters
pub enum TimeInForce {
    FillOrKill,
    GoodTillCanceled,
    ImmediateOrCancel,
}

pub enum SelfTradePreventionType {
    TakerAtCross,
    Maker,
}
```

For complete API documentation, run:

```bash
cargo doc --open
```

Or visit the [online documentation](https://docs.rs/kalshi).

## Testing

The library includes comprehensive tests. To run them:

```bash
cd kalshi
cargo test
```

For authenticated tests, set up environment variables:

```bash
export KALSHI_DEMO_API_KEY=your-key-id
export KALSHI_DEMO_PEM_PATH=/path/to/private.pem
export KALSHI_TEST_ENV=demo
cargo test
```

See [TESTING.md](TESTING.md) for detailed testing instructions.

## Examples

Complete examples are available in the [sample_bot](../sample_bot) directory.

### Example: Simple Market Scanner

```rust
use kalshi::{Kalshi, TradingEnvironment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kalshi = Kalshi::new(
        TradingEnvironment::DemoMode,
        &std::env::var("KALSHI_API_KEY")?,
        &std::env::var("KALSHI_PEM_PATH")?
    ).await?;

    let (_, markets) = kalshi.get_markets(
        Some(50),
        None,
        None,
        None,
        Some("open".to_string()),
        None, None, None, None, None, None, None, None
    ).await?;

    println!("Top markets by volume:");
    let mut sorted_markets = markets;
    sorted_markets.sort_by(|a, b| b.volume_24h.cmp(&a.volume_24h));

    for market in sorted_markets.iter().take(10) {
        println!("{}: {} - Volume: {}, Last: {}",
            market.ticker,
            market.title,
            market.volume_24h,
            market.last_price
        );
    }

    Ok(())
}
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Run `cargo fmt` and `cargo clippy`
5. Submit a pull request

### Code Style

This project follows standard Rust conventions:

- Run `cargo fmt` before committing
- Ensure `cargo clippy` passes with no warnings
- Add tests for new functionality
- Document public APIs with doc comments

## Resources

- **GitHub Repository**: [dpeachpeach/kalshi-rust](https://github.com/dpeachpeach/kalshi-rust)
- **Crates.io**: [kalshi](https://crates.io/crates/kalshi)
- **Documentation**: [docs.rs/kalshi](https://docs.rs/kalshi)
- **Kalshi API Documentation**: [Kalshi API Docs](https://trading-api.readme.io/reference/getting-started)
- **Kalshi Platform**: [kalshi.com](https://kalshi.com)
- **Kalshi Demo**: [demo.kalshi.com](https://demo.kalshi.com)

## License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## Disclaimer

This library is not officially affiliated with Kalshi. Use at your own risk. Trading involves financial risk. Always test with demo mode before trading with real money.

## Support

- **Issues**: [GitHub Issues](https://github.com/dpeachpeach/kalshi-rust/issues)
- **Discussions**: [GitHub Discussions](https://github.com/dpeachpeach/kalshi-rust/discussions)

## Acknowledgments

Developed by the Rust trading community. Special thanks to all contributors who have helped improve this library.

If you find this library useful, please consider giving it a star on GitHub!
