//! WebSocket client for real-time Kalshi market data and trading events.
//!
//! This module provides a WebSocket connection to the Kalshi exchange
//! for receiving real-time updates on orderbooks, trades, positions, and more.

mod channels;
mod connection;
mod messages;
mod subscription;

pub use channels::Channel;
pub use connection::{CommandResponse, KalshiWebSocket};
pub use messages::*;
pub use subscription::{SubscribeResponse, Subscription, UpdateAction};
