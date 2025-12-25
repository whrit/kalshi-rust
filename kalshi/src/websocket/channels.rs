use serde::{Deserialize, Serialize};
use std::fmt;

/// Available WebSocket subscription channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    /// Real-time orderbook updates (snapshots + deltas)
    OrderbookDelta,
    /// Market ticker updates (price, volume, open interest)
    Ticker,
    /// Public trade feed
    Trade,
    /// User fills (requires authentication)
    Fill,
    /// User market positions (requires authentication)
    MarketPosition,
    /// Market lifecycle events (created, activated, settled, etc.)
    MarketLifecycleV2,
    /// Event lifecycle events
    EventLifecycle,
    /// Multivariate market lookups
    Multivariate,
    /// RFQ and quote events (requires authentication)
    Communications,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Channel::OrderbookDelta => "orderbook_delta",
            Channel::Ticker => "ticker",
            Channel::Trade => "trade",
            Channel::Fill => "fill",
            Channel::MarketPosition => "market_position",
            Channel::MarketLifecycleV2 => "market_lifecycle_v2",
            Channel::EventLifecycle => "event_lifecycle",
            Channel::Multivariate => "multivariate",
            Channel::Communications => "communications",
        };
        write!(f, "{}", s)
    }
}

impl Channel {
    /// Returns true if this channel requires authentication.
    pub fn requires_auth(&self) -> bool {
        matches!(
            self,
            Channel::Fill | Channel::MarketPosition | Channel::Communications
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_display() {
        assert_eq!(Channel::OrderbookDelta.to_string(), "orderbook_delta");
        assert_eq!(Channel::Ticker.to_string(), "ticker");
        assert_eq!(Channel::Trade.to_string(), "trade");
        assert_eq!(Channel::Fill.to_string(), "fill");
        assert_eq!(Channel::MarketPosition.to_string(), "market_position");
        assert_eq!(
            Channel::MarketLifecycleV2.to_string(),
            "market_lifecycle_v2"
        );
        assert_eq!(Channel::EventLifecycle.to_string(), "event_lifecycle");
        assert_eq!(Channel::Multivariate.to_string(), "multivariate");
        assert_eq!(Channel::Communications.to_string(), "communications");
    }

    #[test]
    fn test_channel_requires_auth() {
        // Public channels
        assert!(!Channel::OrderbookDelta.requires_auth());
        assert!(!Channel::Ticker.requires_auth());
        assert!(!Channel::Trade.requires_auth());
        assert!(!Channel::MarketLifecycleV2.requires_auth());
        assert!(!Channel::EventLifecycle.requires_auth());
        assert!(!Channel::Multivariate.requires_auth());

        // Private channels
        assert!(Channel::Fill.requires_auth());
        assert!(Channel::MarketPosition.requires_auth());
        assert!(Channel::Communications.requires_auth());
    }

    #[test]
    fn test_channel_serialization() {
        let json = serde_json::to_string(&Channel::OrderbookDelta).unwrap();
        assert_eq!(json, "\"orderbook_delta\"");

        let json = serde_json::to_string(&Channel::Fill).unwrap();
        assert_eq!(json, "\"fill\"");

        let json = serde_json::to_string(&Channel::MarketLifecycleV2).unwrap();
        assert_eq!(json, "\"market_lifecycle_v2\"");
    }

    #[test]
    fn test_channel_deserialization() {
        let channel: Channel = serde_json::from_str("\"orderbook_delta\"").unwrap();
        assert_eq!(channel, Channel::OrderbookDelta);

        let channel: Channel = serde_json::from_str("\"fill\"").unwrap();
        assert_eq!(channel, Channel::Fill);

        let channel: Channel = serde_json::from_str("\"market_lifecycle_v2\"").unwrap();
        assert_eq!(channel, Channel::MarketLifecycleV2);
    }
}
