//! WebSocket client for real-time Kalshi market data and trading events.
//!
//! This module provides a WebSocket connection to the Kalshi exchange for receiving
//! real-time updates on orderbooks, trades, fills, positions, and more. The WebSocket
//! API enables low-latency market data streaming and live trading notifications.
//!
//! # Overview
//!
//! The WebSocket API is built on authenticated persistent connections that stream
//! real-time data updates. This is significantly more efficient than polling REST
//! endpoints when you need continuous market data or order updates.
//!
//! ## Key Features
//!
//! - **Real-time market data**: Live orderbook updates, trades, and ticker changes
//! - **Portfolio updates**: Instant notifications of fills, order updates, and position changes
//! - **Multiple channel subscriptions**: Subscribe to multiple markets and data types simultaneously
//! - **Automatic authentication**: RSA-PSS signing handled automatically during connection
//! - **Asynchronous stream interface**: Integrates with Tokio and futures for efficient async programming
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use kalshi::{Kalshi, TradingEnvironment};
//! use futures_util::StreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create HTTP client first
//! let kalshi = Kalshi::new(
//!     TradingEnvironment::DemoMode,
//!     "your-key-id",
//!     "path/to/private.pem"
//! ).await?;
//!
//! // Create WebSocket client from HTTP client
//! let mut ws = kalshi.websocket();
//!
//! // Connect to WebSocket server
//! ws.connect().await?;
//!
//! // Subscribe to orderbook updates for a market
//! ws.subscribe_to_orderbook_delta("HIGHNY-24JAN15-T50").await?;
//!
//! // Process incoming messages
//! let mut stream = ws.messages();
//! while let Some(msg) = stream.next().await {
//!     match msg {
//!         WebSocketMessage::OrderbookDelta(delta) => {
//!             println!("Orderbook updated: {:?}", delta);
//!         }
//!         WebSocketMessage::Trade(trade) => {
//!             println!("Trade executed: {:?}", trade);
//!         }
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Available Channels
//!
//! The WebSocket API supports several subscription channels:
//!
//! ## Market Data Channels
//!
//! - **Orderbook Delta** - Real-time orderbook updates (incremental changes)
//! - **Orderbook Snapshot** - Full orderbook snapshots at regular intervals
//! - **Ticker** - Market ticker updates (best bid/ask, last price, volume)
//! - **Trade** - Individual trade executions
//! - **Trades** - Trade feed for specific markets
//!
//! ## Portfolio Channels (Authenticated)
//!
//! - **Fill** - Your order fills as they occur
//! - **Order** - Your order status updates (created, canceled, filled)
//!
//! # Message Types
//!
//! All messages received from the WebSocket conform to the [`WebSocketMessage`] enum,
//! which includes:
//!
//! - Control messages: `Subscribed`, `Ok`, `Error`, `Heartbeat`
//! - Market data: `OrderbookDelta`, `OrderbookSnapshot`, `Ticker`, `Trade`, `Trades`
//! - Portfolio updates: `Fill`, `Order`
//!
//! # Connection Lifecycle
//!
//! 1. **Create** - Initialize the client with credentials
//! 2. **Connect** - Establish WebSocket connection with authentication
//! 3. **Subscribe** - Subscribe to desired channels
//! 4. **Stream** - Receive and process messages via the async stream
//! 5. **Unsubscribe** (optional) - Remove subscriptions dynamically
//! 6. **Disconnect** - Close the connection gracefully
//!
//! # Error Handling
//!
//! WebSocket operations return [`KalshiError`](crate::KalshiError) for:
//! - Connection failures
//! - Authentication errors
//! - Subscription errors (invalid ticker, channel not available, etc.)
//! - Network timeouts
//!
//! # Examples
//!
//! ## Subscribe to Multiple Markets
//!
//! ```rust,ignore
//! # use kalshi::Kalshi;
//! # async fn example(ws: &mut kalshi::KalshiWebSocket) -> Result<(), Box<dyn std::error::Error>> {
//! // Subscribe to ticker updates for multiple markets
//! ws.subscribe_to_ticker("HIGHNY-24JAN15-T50").await?;
//! ws.subscribe_to_ticker("INXD-24FEB01").await?;
//! ws.subscribe_to_ticker("NASDAQ-24MAR15").await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Monitor Your Fills
//!
//! ```rust,ignore
//! # use kalshi::{Kalshi, WebSocketMessage};
//! # use futures_util::StreamExt;
//! # async fn example(ws: &mut kalshi::KalshiWebSocket) -> Result<(), Box<dyn std::error::Error>> {
//! // Subscribe to your fill notifications
//! ws.subscribe_to_fills().await?;
//!
//! let mut stream = ws.messages();
//! while let Some(msg) = stream.next().await {
//!     if let WebSocketMessage::Fill(fill) = msg {
//!         println!("Fill received!");
//!         println!("  Ticker: {}", fill.ticker);
//!         println!("  Side: {:?}", fill.side);
//!         println!("  Count: {}", fill.count);
//!         println!("  Price: {}", fill.yes_price);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Track Orderbook Changes
//!
//! ```rust,ignore
//! # use kalshi::{Kalshi, WebSocketMessage};
//! # use futures_util::StreamExt;
//! # async fn example(ws: &mut kalshi::KalshiWebSocket) -> Result<(), Box<dyn std::error::Error>> {
//! ws.subscribe_to_orderbook_delta("HIGHNY-24JAN15-T50").await?;
//!
//! let mut stream = ws.messages();
//! while let Some(msg) = stream.next().await {
//!     if let WebSocketMessage::OrderbookDelta(delta) = msg {
//!         println!("Market: {}", delta.ticker);
//!         if let Some(yes_levels) = delta.yes {
//!             println!("Yes side updates: {} levels", yes_levels.len());
//!         }
//!         if let Some(no_levels) = delta.no {
//!             println!("No side updates: {} levels", no_levels.len());
//!         }
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Performance Considerations
//!
//! - **Selective subscriptions**: Only subscribe to the channels and markets you need
//! - **Message filtering**: Filter messages in your application to process only what's needed
//! - **Orderbook deltas vs snapshots**: Use deltas for efficiency, snapshots for periodic synchronization
//! - **Connection management**: Reuse a single connection for multiple subscriptions
//!
//! # See Also
//!
//! - [`KalshiWebSocket`](connection::KalshiWebSocket) - The main WebSocket client
//! - [`WebSocketMessage`](messages::WebSocketMessage) - All message types
//! - [`Channel`](channels::Channel) - Available subscription channels
//! - [`Subscription`](subscription::Subscription) - Subscription management

mod channels;
mod connection;
mod messages;
mod subscription;

pub use channels::Channel;
pub use connection::{CommandResponse, KalshiWebSocket};
pub use messages::*;
pub use subscription::{SubscribeResponse, Subscription, UpdateAction};
