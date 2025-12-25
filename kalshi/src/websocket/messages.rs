use crate::portfolio::{Action, Side};
use serde::{Deserialize, Serialize};

/// Envelope for all WebSocket messages.
#[derive(Debug, Deserialize)]
pub struct MessageEnvelope {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub sid: Option<i32>,
    pub seq: Option<i64>,
    pub id: Option<i32>,
    pub msg: Option<serde_json::Value>,
}

/// Unified WebSocket message type.
#[derive(Debug)]
pub enum WebSocketMessage {
    // Control messages
    Subscribed(SubscribedMsg),
    Ok(OkMsg),
    Error(ErrorMsg),

    // Orderbook
    OrderbookSnapshot(OrderbookSnapshotMsg),
    OrderbookDelta(OrderbookDeltaMsg),

    // Market data
    Ticker(TickerMsg),
    Trade(TradeMsg),

    // User data (auth required)
    Fill(FillMsg),
    MarketPosition(MarketPositionMsg),

    // Lifecycle
    MarketLifecycle(MarketLifecycleMsg),
    EventLifecycle(EventLifecycleMsg),

    // Multivariate
    MultivariateLookup(MultivariateLookupMsg),

    // Communications
    RfqCreated(RfqCreatedMsg),
    QuoteCreated(QuoteCreatedMsg),
    QuoteAccepted(QuoteAcceptedMsg),

    // Unknown
    Unknown(serde_json::Value),
}

// --- Control Messages ---

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribedMsg {
    pub channel: String,
    pub sid: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OkMsg {
    pub sid: i32,
    pub seq: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorMsg {
    pub code: i32,
    pub msg: String,
}

// --- Orderbook Messages ---

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderbookSnapshotMsg {
    pub market_ticker: String,
    pub yes: Vec<Vec<i32>>,
    pub yes_dollars: Vec<Vec<String>>,
    pub no: Vec<Vec<i32>>,
    pub no_dollars: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderbookDeltaMsg {
    pub market_ticker: String,
    pub price: i32,
    pub price_dollars: String,
    pub delta: i32,
    pub side: String, // "yes" or "no"
}

// --- Market Data Messages ---

#[derive(Debug, Deserialize, Serialize)]
pub struct TickerMsg {
    pub market_ticker: String,
    pub price: Option<i32>,
    pub yes_bid: Option<i32>,
    pub yes_ask: Option<i32>,
    pub price_dollars: Option<String>,
    pub volume: Option<i64>,
    pub open_interest: Option<i64>,
    pub ts: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeMsg {
    pub market_ticker: String,
    pub yes_price: i32,
    pub no_price: i32,
    pub count: i32,
    pub taker_side: String,
    pub ts: i64,
}

// --- User Data Messages (auth required) ---

#[derive(Debug, Deserialize, Serialize)]
pub struct FillMsg {
    pub trade_id: String,
    pub order_id: String,
    pub market_ticker: String,
    pub side: Side,
    pub action: Action,
    pub count: i32,
    pub post_position: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketPositionMsg {
    pub market_ticker: String,
    pub position: i32,
    /// Position cost in centi-cents (divide by 10,000 for USD)
    pub position_cost: i64,
    /// Realized P&L in centi-cents
    pub realized_pnl: i64,
    /// Fees paid in centi-cents
    pub fees_paid: i64,
}

// --- Lifecycle Messages ---

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketLifecycleMsg {
    pub market_ticker: String,
    pub event_type: MarketLifecycleEvent,
    pub open_ts: Option<i64>,
    pub close_ts: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum MarketLifecycleEvent {
    Created,
    Activated,
    Deactivated,
    CloseDateUpdated,
    Determined,
    Settled,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EventLifecycleMsg {
    pub event_ticker: String,
    pub title: String,
}

// --- Multivariate Messages ---

#[derive(Debug, Deserialize, Serialize)]
pub struct MultivariateLookupMsg {
    pub collection_ticker: String,
    pub market_ticker: String,
    pub selected_markets: Vec<SelectedMarket>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SelectedMarket {
    pub market_ticker: String,
    pub side: String,
}

// --- Communications Messages ---

#[derive(Debug, Deserialize, Serialize)]
pub struct RfqCreatedMsg {
    pub id: String,
    pub market_ticker: String,
    pub contracts: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteCreatedMsg {
    pub quote_id: String,
    pub rfq_id: String,
    pub yes_bid: i32,
    pub no_bid: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteAcceptedMsg {
    pub quote_id: String,
    pub accepted_side: String,
}

// --- Message Parsing ---

impl WebSocketMessage {
    /// Parse a raw WebSocket message into a typed message.
    pub fn parse(text: &str) -> Result<Self, serde_json::Error> {
        let envelope: MessageEnvelope = serde_json::from_str(text)?;

        match envelope.msg_type.as_str() {
            "subscribed" => {
                let msg: SubscribedMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::Subscribed(msg))
            }
            "ok" => Ok(WebSocketMessage::Ok(OkMsg {
                sid: envelope.sid.unwrap_or(0),
                seq: envelope.seq.unwrap_or(0),
            })),
            "error" => {
                let msg: ErrorMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::Error(msg))
            }
            "orderbook_snapshot" => {
                let msg: OrderbookSnapshotMsg =
                    serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::OrderbookSnapshot(msg))
            }
            "orderbook_delta" => {
                let msg: OrderbookDeltaMsg =
                    serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::OrderbookDelta(msg))
            }
            "ticker" => {
                let msg: TickerMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::Ticker(msg))
            }
            "trade" => {
                let msg: TradeMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::Trade(msg))
            }
            "fill" => {
                let msg: FillMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::Fill(msg))
            }
            "market_position" => {
                let msg: MarketPositionMsg =
                    serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::MarketPosition(msg))
            }
            "market_lifecycle_v2" => {
                let msg: MarketLifecycleMsg =
                    serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::MarketLifecycle(msg))
            }
            "event_lifecycle" => {
                let msg: EventLifecycleMsg =
                    serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::EventLifecycle(msg))
            }
            "multivariate_lookup" => {
                let msg: MultivariateLookupMsg =
                    serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::MultivariateLookup(msg))
            }
            "rfq_created" => {
                let msg: RfqCreatedMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::RfqCreated(msg))
            }
            "quote_created" => {
                let msg: QuoteCreatedMsg =
                    serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::QuoteCreated(msg))
            }
            "quote_accepted" => {
                let msg: QuoteAcceptedMsg =
                    serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::QuoteAccepted(msg))
            }
            _ => Ok(WebSocketMessage::Unknown(serde_json::json!({
                "type": envelope.msg_type,
                "msg": envelope.msg
            }))),
        }
    }
}
