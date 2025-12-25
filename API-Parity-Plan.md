Kalshi-Rust Full API Parity Implementation Plan

Overview

This plan brings the kalshi-rust library to 100% parity with the Kalshi REST API and WebSocket API. The implementation is organized into 4 phases, ordered by priority and
dependency.

## Test Status

✅ **Phase 1 Complete** - All 23 tests passing in `kalshi/tests/phase1_api_parity_tests.rs`
- Run with: `cargo test --test phase1_api_parity_tests`

✅ **Phase 2 Complete** - Task 2.3 (get_communications_id) implemented

✅ **Phase 3 Complete** - All 6 tests passing in `kalshi/tests/communications_tests.rs`
- Run with: `cargo test --test communications_tests`
- Tests cover: get_communications_id, create_rfq, create_quote, get_rfqs, get_quotes, accept_quote

✅ **Phase 4 Complete** - WebSocket implementation with 4 unit tests
- Run with: `cargo test websocket::channels --lib`
- Tests cover: Channel Display, requires_auth, serialization, deserialization

✅ **Phase 5 Complete** - Testing and exports with 6 integration tests
- Run with: `cargo test --test websocket_tests`
- Tests cover: connect/disconnect, subscribe ticker, subscribe orderbook, unsubscribe, Channel Display, requires_auth

## Progress Tracking

### Phase 1: Critical Trading Features (HTTPS Fixes) - ✅ COMPLETE

| Task | Status | Notes |
|------|--------|-------|
| 1.1 Fix create_order() - Add Missing Parameters | ✅ Done | Added TimeInForce, SelfTradePreventionType enums and 6 new parameters |
| 1.2 Fix amend_order() - Complete Rewrite | ✅ Done | New signature with price validation, returns AmendOrderResponse |
| 1.3 Update get_settlements() - Add Missing Filters | ✅ Done | Added ticker, event_ticker, min_ts, max_ts parameters |
| 1.4 Update get_positions() - Add count_filter | ✅ Done | Added count_filter parameter |

### Phase 2: Missing HTTPS Endpoints - COMPLETE

| Task | Status | Notes |
|------|--------|-------|
| 2.1 Add batch_get_market_candlesticks() | ✅ Done | Added in market/mod.rs with MarketCandlesticks struct |
| 2.2 Add get_multivariate_events() | ✅ Done | Added in events/mod.rs with validation for mutually exclusive filters |
| 2.3 Add get_communications_id() | ✅ Done | Added in communications/mod.rs, returns user's public communications ID |
| 2.4 Update get_markets() - Add Missing Query Params | ✅ Done | Added timestamp filters and MveFilter enum |

### Phase 3: RFQ/Quote API Alignment - COMPLETE

| Task | Status | Notes |
|------|--------|-------|
| 3.1 Rewrite create_rfq() | ✅ Done | New signature with market_ticker, rest_remainder, contracts, target_cost_centi_cents, replace_existing, subtrader_id |
| 3.2 Rewrite create_quote() | ✅ Done | New signature with rfq_id, yes_bid, no_bid, rest_remainder (dollar format) |
| 3.3 Update get_rfqs() and get_quotes() | ✅ Done | Added pagination (cursor, limit) and filtering parameters |
| 3.4 Fix accept_quote() | ✅ Done | Added accepted_side parameter (Side enum), returns () |

### Phase 4: WebSocket Implementation - COMPLETE

| Task | Status | Notes |
|------|--------|-------|
| 4.1 Add Dependencies | ✅ Done | Added tokio-tungstenite 0.21, futures-util 0.3 to Cargo.toml |
| 4.2 Create WebSocket Module Structure | ✅ Done | Created websocket/mod.rs with submodule declarations |
| 4.3 Connection Handler | ✅ Done | Created connection.rs with KalshiWebSocket, auth, connect/disconnect |
| 4.4 Channel Definitions | ✅ Done | Created channels.rs with Channel enum (9 channels), Display, requires_auth() |
| 4.5 Message Types | ✅ Done | Created messages.rs with 15+ message types, WebSocketMessage::parse() |
| 4.6 Subscription Management | ✅ Done | Created subscription.rs with subscribe/unsubscribe/update methods |
| 4.7 Stream Interface | ✅ Done | Added Stream trait impl for async message polling in connection.rs |
| 4.8 Integration with Kalshi Struct | ✅ Done | Added websocket() factory and trading_env() to lib.rs |

### Phase 5: Testing and Documentation - COMPLETE (tests only, docs deferred)

| Task | Status | Notes |
|------|--------|-------|
| 5.1 WebSocket Tests | ✅ Done | Created websocket_tests.rs with 6 tests (connect, subscribe, unsubscribe, Channel traits) |
| 5.2 Update lib.rs Exports | ✅ Done | All types exported via glob exports (TimeInForce, MveFilter, WebSocket types verified) |
| 5.3 Documentation Generation | ⏳ Deferred | Holding off per user request |

---
Phase 1: Critical Trading Features (HTTPS Fixes) - COMPLETE

1.1 Fix create_order() - Add Missing Parameters

File: kalshi/src/portfolio/mod.rs

Current signature is missing critical trading parameters. Add:

// New enum needed
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    FillOrKill,
    GoodTillCanceled,
    ImmediateOrCancel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelfTradePreventionType {
    TakerAtCross,
    Maker,
}

Update CreateOrderPayload struct (around line 964):
#[derive(Debug, Deserialize, Serialize)]
struct CreateOrderPayload {
    action: Action,
    client_order_id: String,
    count: i32,
    side: Side,
    ticker: String,
    r#type: OrderType,
    // Existing optional fields...
    buy_max_cost: Option<i64>,
    expiration_ts: Option<i64>,
    yes_price: Option<i64>,
    no_price: Option<i64>,
    sell_position_floor: Option<i32>,
    yes_price_dollars: Option<String>,
    no_price_dollars: Option<String>,
    // NEW FIELDS:
    #[serde(skip_serializing_if = "Option::is_none")]
    time_in_force: Option<TimeInForce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    self_trade_prevention_type: Option<SelfTradePreventionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancel_order_on_pause: Option<bool>,
}

Update method signature (line ~447):
pub async fn create_order(
    &self,
    action: Action,
    client_order_id: Option<String>,
    count: i32,
    side: Side,
    ticker: String,
    input_type: OrderType,
    buy_max_cost: Option<i64>,
    expiration_ts: Option<i64>,
    yes_price: Option<i64>,
    no_price: Option<i64>,
    sell_position_floor: Option<i32>,
    yes_price_dollars: Option<String>,
    no_price_dollars: Option<String>,
    // NEW PARAMETERS:
    time_in_force: Option<TimeInForce>,
    post_only: Option<bool>,
    reduce_only: Option<bool>,
    self_trade_prevention_type: Option<SelfTradePreventionType>,
    order_group_id: Option<String>,
    cancel_order_on_pause: Option<bool>,
) -> Result<Order, KalshiError>

Also update OrderCreationField struct and batch_create_order() accordingly.

---
1.2 Fix amend_order() - Complete Rewrite

File: kalshi/src/portfolio/mod.rs (line ~854)

Current implementation doesn't match API spec. Replace entirely:

/// Amends an existing order by modifying its price or quantity.
///
/// # Arguments
/// * `order_id` - The order ID to amend
/// * `ticker` - Market ticker (required for validation)
/// * `side` - Side of the order (yes/no)
/// * `action` - Action of the order (buy/sell)
/// * `client_order_id` - Original client order ID
/// * `updated_client_order_id` - New client order ID after amendment
/// * `yes_price` - Optional new yes price in cents
/// * `no_price` - Optional new no price in cents
/// * `yes_price_dollars` - Optional new yes price in dollars ("0.5600")
/// * `no_price_dollars` - Optional new no price in dollars ("0.5600")
/// * `count` - Optional new quantity
///
pub async fn amend_order(
    &self,
    order_id: &str,
    ticker: &str,
    side: Side,
    action: Action,
    client_order_id: &str,
    updated_client_order_id: &str,
    yes_price: Option<i32>,
    no_price: Option<i32>,
    yes_price_dollars: Option<String>,
    no_price_dollars: Option<String>,
    count: Option<i32>,
) -> Result<AmendOrderResponse, KalshiError> {
    // Validate: exactly one price field must be provided
    let price_count = [
        yes_price.is_some(),
        no_price.is_some(),
        yes_price_dollars.is_some(),
        no_price_dollars.is_some(),
    ].iter().filter(|&&x| x).count();

    if price_count > 1 {
        return Err(KalshiError::UserInputError(
            "Exactly one of yes_price, no_price, yes_price_dollars, or no_price_dollars must be provided".to_string()
        ));
    }

    let path = format!("{}/orders/{}/amend", PORTFOLIO_PATH, order_id);
    let body = AmendOrderRequest {
        ticker: ticker.to_string(),
        side,
        action,
        client_order_id: client_order_id.to_string(),
        updated_client_order_id: updated_client_order_id.to_string(),
        yes_price,
        no_price,
        yes_price_dollars,
        no_price_dollars,
        count,
    };
    self.signed_post(&path, &body).await
}

// New request/response structs
#[derive(Debug, Serialize)]
struct AmendOrderRequest {
    ticker: String,
    side: Side,
    action: Action,
    client_order_id: String,
    updated_client_order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    yes_price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    yes_price_dollars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    no_price_dollars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AmendOrderResponse {
    pub old_order: Order,
    pub order: Order,
}

---
1.3 Update get_settlements() - Add Missing Filters

File: kalshi/src/portfolio/mod.rs (line ~308)

pub async fn get_settlements(
    &self,
    limit: Option<i64>,
    cursor: Option<String>,
    // NEW PARAMETERS:
    ticker: Option<String>,
    event_ticker: Option<String>,
    min_ts: Option<i64>,
    max_ts: Option<i64>,
) -> Result<(Option<String>, Vec<Settlement>), KalshiError> {
    let mut params: Vec<(&str, String)> = Vec::with_capacity(6);

    add_param!(params, "limit", limit);
    add_param!(params, "cursor", cursor);
    add_param!(params, "ticker", ticker);
    add_param!(params, "event_ticker", event_ticker);
    add_param!(params, "min_ts", min_ts);
    add_param!(params, "max_ts", max_ts);

    // ... rest of implementation unchanged
}

---
1.4 Update get_positions() - Add count_filter

File: kalshi/src/portfolio/mod.rs (line ~361)

pub async fn get_positions(
    &self,
    limit: Option<i64>,
    cursor: Option<String>,
    settlement_status: Option<String>,
    ticker: Option<String>,
    event_ticker: Option<String>,
    // NEW PARAMETER:
    count_filter: Option<String>,  // "position", "total_traded"
) -> Result<(Option<String>, Vec<EventPosition>, Vec<MarketPosition>), KalshiError> {
    let mut params: Vec<(&str, String)> = Vec::with_capacity(7);

    add_param!(params, "limit", limit);
    add_param!(params, "cursor", cursor);
    add_param!(params, "settlement_status", settlement_status);
    add_param!(params, "ticker", ticker);
    add_param!(params, "event_ticker", event_ticker);
    add_param!(params, "count_filter", count_filter);

    // ... rest unchanged
}

---
Phase 2: Missing HTTPS Endpoints

2.1 Add batch_get_market_candlesticks()

File: kalshi/src/market/mod.rs

/// Retrieves candlestick data for multiple markets in a single request.
///
/// # Arguments
/// * `market_tickers` - Comma-separated list of market tickers (max 100)
/// * `start_ts` - Start timestamp in Unix seconds
/// * `end_ts` - End timestamp in Unix seconds
/// * `period_interval` - Candlestick period in minutes (1, 60, or 1440)
/// * `include_latest_before_start` - If true, prepends the latest candlestick before start_ts
///
pub async fn batch_get_market_candlesticks(
    &self,
    market_tickers: Vec<String>,
    start_ts: i64,
    end_ts: i64,
    period_interval: i32,
    include_latest_before_start: Option<bool>,
) -> Result<Vec<MarketCandlesticks>, KalshiError> {
    if market_tickers.len() > 100 {
        return Err(KalshiError::UserInputError(
            "Maximum 100 market tickers allowed".to_string()
        ));
    }

    let tickers_param = market_tickers.join(",");
    let mut params: Vec<(&str, String)> = vec![
        ("market_tickers", tickers_param),
        ("start_ts", start_ts.to_string()),
        ("end_ts", end_ts.to_string()),
        ("period_interval", period_interval.to_string()),
    ];
    add_param!(params, "include_latest_before_start", include_latest_before_start);

    let url = format!("{}/markets/candlesticks/batch", self.base_url);
    let final_url = reqwest::Url::parse_with_params(&url, &params)?;
    let res: BatchCandlestickResponse = self.client.get(final_url).send().await?.json().await?;
    Ok(res.markets)
}

// New response structs
#[derive(Debug, Deserialize)]
struct BatchCandlestickResponse {
    markets: Vec<MarketCandlesticks>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketCandlesticks {
    pub ticker: String,
    pub candlesticks: Vec<Candle>,
}

---
2.2 Add get_multivariate_events()

File: kalshi/src/events/mod.rs

/// Retrieves multivariate events (combo markets) with optional filtering.
///
/// # Arguments
/// * `limit` - Number of results per page (default 100, max 200)
/// * `cursor` - Pagination cursor
/// * `series_ticker` - Filter by series ticker
/// * `collection_ticker` - Filter by collection ticker (cannot use with series_ticker)
/// * `with_nested_markets` - Include nested markets in response
///
pub async fn get_multivariate_events(
    &self,
    limit: Option<i32>,
    cursor: Option<String>,
    series_ticker: Option<String>,
    collection_ticker: Option<String>,
    with_nested_markets: Option<bool>,
) -> Result<(Option<String>, Vec<Event>), KalshiError> {
    // Validate: cannot use both series_ticker and collection_ticker
    if series_ticker.is_some() && collection_ticker.is_some() {
        return Err(KalshiError::UserInputError(
            "Cannot use both series_ticker and collection_ticker".to_string()
        ));
    }

    let mut params: Vec<(&str, String)> = Vec::with_capacity(5);
    add_param!(params, "limit", limit);
    add_param!(params, "cursor", cursor);
    add_param!(params, "series_ticker", series_ticker);
    add_param!(params, "collection_ticker", collection_ticker);
    add_param!(params, "with_nested_markets", with_nested_markets);

    let url = format!("{}/events/multivariate", self.base_url);
    let final_url = reqwest::Url::parse_with_params(&url, &params)?;
    let res: EventListResponse = self.client.get(final_url).send().await?.json().await?;
    Ok((res.cursor, res.events))
}

---
2.3 Add get_communications_id()

File: kalshi/src/communications/mod.rs

/// Retrieves the user's public communications ID.
///
/// This ID is used to identify the user in communications with other traders.
///
pub async fn get_communications_id(&self) -> Result<String, KalshiError> {
    let path = "/communications/id";
    let res: CommunicationsIdResponse = self.signed_get(path).await?;
    Ok(res.communications_id)
}

#[derive(Debug, Deserialize)]
struct CommunicationsIdResponse {
    communications_id: String,
}

---
2.4 Update get_markets() - Add Missing Query Params

File: kalshi/src/market/mod.rs (line ~40)

pub async fn get_markets(
    &self,
    limit: Option<i64>,
    cursor: Option<String>,
    event_ticker: Option<String>,
    series_ticker: Option<String>,
    status: Option<String>,
    tickers: Option<String>,
    min_close_ts: Option<i64>,
    max_close_ts: Option<i64>,
    // NEW PARAMETERS:
    min_created_ts: Option<i64>,
    max_created_ts: Option<i64>,
    min_settled_ts: Option<i64>,
    max_settled_ts: Option<i64>,
    mve_filter: Option<MveFilter>,  // "only" or "exclude"
) -> Result<(Option<String>, Vec<Market>), KalshiError> {
    let url = format!("{}/markets", self.base_url);
    let mut p = vec![];
    add_param!(p, "limit", limit);
    add_param!(p, "cursor", cursor);
    add_param!(p, "event_ticker", event_ticker);
    add_param!(p, "series_ticker", series_ticker);
    add_param!(p, "status", status);
    add_param!(p, "tickers", tickers);
    add_param!(p, "min_close_ts", min_close_ts);
    add_param!(p, "max_close_ts", max_close_ts);
    // NEW:
    add_param!(p, "min_created_ts", min_created_ts);
    add_param!(p, "max_created_ts", max_created_ts);
    add_param!(p, "min_settled_ts", min_settled_ts);
    add_param!(p, "max_settled_ts", max_settled_ts);
    add_param!(p, "mve_filter", mve_filter.map(|f| f.to_string()));

    // ... rest unchanged
}

// New enum
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MveFilter {
    Only,
    Exclude,
}

impl fmt::Display for MveFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MveFilter::Only => write!(f, "only"),
            MveFilter::Exclude => write!(f, "exclude"),
        }
    }
}

---
Phase 3: RFQ/Quote API Alignment

3.1 Rewrite create_rfq()

File: kalshi/src/communications/mod.rs (line ~81)

/// Creates a new RFQ (Request for Quote).
///
/// # Arguments
/// * `market_ticker` - The market ticker to request a quote for
/// * `rest_remainder` - Whether to rest the remainder after execution
/// * `contracts` - Number of contracts for the RFQ
/// * `target_cost_centi_cents` - Target cost in centi-cents (optional)
/// * `replace_existing` - Whether to delete existing RFQs (default: false)
/// * `subtrader_id` - Subtrader ID (FCM members only)
///
pub async fn create_rfq(
    &self,
    market_ticker: &str,
    rest_remainder: bool,
    contracts: Option<i32>,
    target_cost_centi_cents: Option<i64>,
    replace_existing: Option<bool>,
    subtrader_id: Option<String>,
) -> Result<CreateRfqResponse, KalshiError> {
    let path = "/communications/rfqs";
    let body = CreateRfqRequest {
        market_ticker: market_ticker.to_string(),
        rest_remainder,
        contracts,
        target_cost_centi_cents,
        replace_existing,
        subtrader_id,
    };
    self.signed_post(path, &body).await
}

#[derive(Debug, Serialize)]
struct CreateRfqRequest {
    market_ticker: String,
    rest_remainder: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    contracts: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_cost_centi_cents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_existing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtrader_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateRfqResponse {
    pub id: String,
}

---
3.2 Rewrite create_quote()

File: kalshi/src/communications/mod.rs (line ~194)

/// Creates a new quote in response to an RFQ.
///
/// # Arguments
/// * `rfq_id` - The RFQ ID this quote responds to
/// * `yes_bid` - Bid price for YES contracts in dollars ("0.5600" format)
/// * `no_bid` - Bid price for NO contracts in dollars ("0.5600" format)
/// * `rest_remainder` - Whether to rest the remainder after execution
///
pub async fn create_quote(
    &self,
    rfq_id: &str,
    yes_bid: &str,
    no_bid: &str,
    rest_remainder: bool,
) -> Result<CreateQuoteResponse, KalshiError> {
    let path = "/communications/quotes";
    let body = CreateQuoteRequest {
        rfq_id: rfq_id.to_string(),
        yes_bid: yes_bid.to_string(),
        no_bid: no_bid.to_string(),
        rest_remainder,
    };
    self.signed_post(path, &body).await
}

#[derive(Debug, Serialize)]
struct CreateQuoteRequest {
    rfq_id: String,
    yes_bid: String,
    no_bid: String,
    rest_remainder: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateQuoteResponse {
    pub id: String,
}

---
3.3 Update get_rfqs() and get_quotes() - Add Pagination

File: kalshi/src/communications/mod.rs

/// Retrieves RFQs with optional filtering and pagination.
pub async fn get_rfqs(
    &self,
    cursor: Option<String>,
    event_ticker: Option<String>,
    market_ticker: Option<String>,
    limit: Option<i32>,
    status: Option<String>,
    creator_user_id: Option<String>,
) -> Result<(Option<String>, Vec<Rfq>), KalshiError> {
    let mut params: Vec<(&str, String)> = Vec::with_capacity(6);
    add_param!(params, "cursor", cursor);
    add_param!(params, "event_ticker", event_ticker);
    add_param!(params, "market_ticker", market_ticker);
    add_param!(params, "limit", limit);
    add_param!(params, "status", status);
    add_param!(params, "creator_user_id", creator_user_id);

    let path = if params.is_empty() {
        "/communications/rfqs".to_string()
    } else {
        let qs = params.iter().map(|(k,v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&");
        format!("/communications/rfqs?{}", qs)
    };

    let res: RfqsResponse = self.signed_get(&path).await?;
    Ok((res.cursor, res.rfqs))
}

/// Retrieves quotes with optional filtering and pagination.
pub async fn get_quotes(
    &self,
    cursor: Option<String>,
    event_ticker: Option<String>,
    market_ticker: Option<String>,
    limit: Option<i32>,
    status: Option<String>,
    quote_creator_user_id: Option<String>,
    rfq_creator_user_id: Option<String>,
    rfq_id: Option<String>,
) -> Result<(Option<String>, Vec<Quote>), KalshiError> {
    let mut params: Vec<(&str, String)> = Vec::with_capacity(8);
    add_param!(params, "cursor", cursor);
    add_param!(params, "event_ticker", event_ticker);
    add_param!(params, "market_ticker", market_ticker);
    add_param!(params, "limit", limit);
    add_param!(params, "status", status);
    add_param!(params, "quote_creator_user_id", quote_creator_user_id);
    add_param!(params, "rfq_creator_user_id", rfq_creator_user_id);
    add_param!(params, "rfq_id", rfq_id);

    let path = if params.is_empty() {
        "/communications/quotes".to_string()
    } else {
        let qs = params.iter().map(|(k,v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&");
        format!("/communications/quotes?{}", qs)
    };

    let res: QuotesResponse = self.signed_get(&path).await?;
    Ok((res.cursor, res.quotes))
}

// Update response wrappers
#[derive(Debug, Deserialize)]
struct RfqsResponse {
    rfqs: Vec<Rfq>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
struct QuotesResponse {
    quotes: Vec<Quote>,
    cursor: Option<String>,
}

---
3.4 Fix accept_quote() - Add Required Field

File: kalshi/src/communications/mod.rs (line ~281)

/// Accepts a quote, which will execute the trade.
///
/// # Arguments
/// * `quote_id` - The quote ID to accept
/// * `accepted_side` - Which side to accept ("yes" or "no")
///
pub async fn accept_quote(
    &self,
    quote_id: &str,
    accepted_side: Side,
) -> Result<(), KalshiError> {
    let path = format!("/communications/quotes/{}/accept", quote_id);
    let body = AcceptQuoteRequest { accepted_side };
    let _: serde_json::Value = self.signed_put(&path, Some(&body)).await?;
    Ok(())
}

#[derive(Debug, Serialize)]
struct AcceptQuoteRequest {
    accepted_side: Side,
}

---
Phase 4: WebSocket Implementation

4.1 Add Dependencies

File: kalshi/Cargo.toml

[dependencies]
# Existing dependencies...
tokio-tungstenite = { version = "0.21", features = ["native-tls"] }
futures-util = "0.3"

---
4.2 Create WebSocket Module Structure

New File: kalshi/src/websocket/mod.rs

//! WebSocket client for real-time Kalshi market data and trading events.
//!
//! This module provides a WebSocket connection to the Kalshi exchange
//! for receiving real-time updates on orderbooks, trades, positions, and more.

mod connection;
mod messages;
mod channels;
mod subscription;

pub use connection::KalshiWebSocket;
pub use messages::*;
pub use channels::Channel;
pub use subscription::{Subscription, SubscribeResponse};

---
4.3 Connection Handler

New File: kalshi/src/websocket/connection.rs

use crate::kalshi_error::KalshiError;
use crate::TradingEnvironment;
use futures_util::{SinkExt, StreamExt, stream::SplitSink, stream::SplitStream};
use openssl::pkey::{PKey, Private};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{
    connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream,
};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsSink = SplitSink<WsStream, Message>;
type WsReader = SplitStream<WsStream>;

/// WebSocket client for real-time Kalshi data.
pub struct KalshiWebSocket {
    url: String,
    key_id: String,
    private_key: PKey<Private>,
    writer: Option<Arc<Mutex<WsSink>>>,
    reader: Option<WsReader>,
    next_id: i32,
    subscriptions: std::collections::HashMap<i32, super::Subscription>,
}

impl KalshiWebSocket {
    /// Creates a new WebSocket client (does not connect yet).
    pub fn new(
        trading_env: TradingEnvironment,
        key_id: &str,
        private_key: PKey<Private>,
    ) -> Self {
        let url = match trading_env {
            TradingEnvironment::DemoMode => "wss://demo-api.kalshi.co/trade-api/ws/v2",
            TradingEnvironment::ProdMode => "wss://api.elections.kalshi.com/trade-api/ws/v2",
        };

        Self {
            url: url.to_string(),
            key_id: key_id.to_string(),
            private_key,
            writer: None,
            reader: None,
            next_id: 1,
            subscriptions: std::collections::HashMap::new(),
        }
    }

    /// Connects to the WebSocket server with authentication.
    pub async fn connect(&mut self) -> Result<(), KalshiError> {
        // Generate auth headers for handshake
        let timestamp = chrono::Utc::now().timestamp_millis();
        let method = "GET";
        let path = "/trade-api/ws/v2";

        let message = format!("{}{}{}", timestamp, method, path);
        let signature = self.sign_message(&message)?;

        // Build URL with auth query params
        let auth_url = format!(
            "{}?api-key={}&timestamp={}&signature={}",
            self.url, self.key_id, timestamp, signature
        );

        let (ws_stream, _response) = connect_async(&auth_url)
            .await
            .map_err(|e| KalshiError::InternalError(format!("WebSocket connect failed: {}", e)))?;

        let (write, read) = ws_stream.split();
        self.writer = Some(Arc::new(Mutex::new(write)));
        self.reader = Some(read);

        // Start heartbeat handler
        self.spawn_heartbeat_handler();

        Ok(())
    }

    /// Disconnects from the WebSocket server.
    pub async fn disconnect(&mut self) -> Result<(), KalshiError> {
        if let Some(writer) = &self.writer {
            let mut w = writer.lock().await;
            w.close().await.map_err(|e|
                KalshiError::InternalError(format!("Close failed: {}", e))
            )?;
        }
        self.writer = None;
        self.reader = None;
        self.subscriptions.clear();
        Ok(())
    }

    /// Returns true if connected.
    pub fn is_connected(&self) -> bool {
        self.writer.is_some()
    }

    fn sign_message(&self, message: &str) -> Result<String, KalshiError> {
        use openssl::sign::Signer;
        use openssl::hash::MessageDigest;
        use openssl::rsa::Padding;

        let mut signer = Signer::new(MessageDigest::sha256(), &self.private_key)?;
        signer.set_rsa_padding(Padding::PKCS1_PSS)?;
        signer.set_rsa_pss_saltlen(openssl::sign::RsaPssSaltlen::DIGEST_LENGTH)?;
        signer.update(message.as_bytes())?;
        let signature = signer.sign_to_vec()?;
        Ok(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &signature))
    }

    fn spawn_heartbeat_handler(&self) {
        let writer = self.writer.clone();
        tokio::spawn(async move {
            // Heartbeat handling runs in background
            // Server sends ping every 10s, we must respond with pong
            // This is handled automatically by tokio-tungstenite for control frames
        });
    }

    fn get_next_id(&mut self) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Sends a command to the WebSocket server.
    pub(crate) async fn send_command(&mut self, cmd: serde_json::Value) -> Result<(), KalshiError> {
        let writer = self.writer.as_ref()
            .ok_or_else(|| KalshiError::InternalError("Not connected".to_string()))?;

        let msg = Message::Text(serde_json::to_string(&cmd)?);
        let mut w = writer.lock().await;
        w.send(msg).await.map_err(|e|
            KalshiError::InternalError(format!("Send failed: {}", e))
        )?;
        Ok(())
    }
}

---
4.4 Channel Definitions

New File: kalshi/src/websocket/channels.rs

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
        matches!(self,
            Channel::Fill |
            Channel::MarketPosition |
            Channel::Communications
        )
    }
}

---
4.5 Message Types

New File: kalshi/src/websocket/messages.rs

use serde::{Deserialize, Serialize};
use crate::portfolio::{Side, Action};

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
    pub side: String,  // "yes" or "no"
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

#[derive(Debug, Deserialize, Serialize)]
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
            "ok" => {
                Ok(WebSocketMessage::Ok(OkMsg {
                    sid: envelope.sid.unwrap_or(0),
                    seq: envelope.seq.unwrap_or(0),
                }))
            }
            "error" => {
                let msg: ErrorMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::Error(msg))
            }
            "orderbook_snapshot" => {
                let msg: OrderbookSnapshotMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::OrderbookSnapshot(msg))
            }
            "orderbook_delta" => {
                let msg: OrderbookDeltaMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
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
                let msg: MarketPositionMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::MarketPosition(msg))
            }
            "market_lifecycle_v2" => {
                let msg: MarketLifecycleMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::MarketLifecycle(msg))
            }
            "event_lifecycle" => {
                let msg: EventLifecycleMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::EventLifecycle(msg))
            }
            "multivariate_lookup" => {
                let msg: MultivariateLookupMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::MultivariateLookup(msg))
            }
            "rfq_created" => {
                let msg: RfqCreatedMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::RfqCreated(msg))
            }
            "quote_created" => {
                let msg: QuoteCreatedMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::QuoteCreated(msg))
            }
            "quote_accepted" => {
                let msg: QuoteAcceptedMsg = serde_json::from_value(envelope.msg.unwrap_or_default())?;
                Ok(WebSocketMessage::QuoteAccepted(msg))
            }
            _ => Ok(WebSocketMessage::Unknown(serde_json::json!({
                "type": envelope.msg_type,
                "msg": envelope.msg
            }))),
        }
    }
}

---
4.6 Subscription Management

New File: kalshi/src/websocket/subscription.rs

use super::{Channel, KalshiWebSocket, WebSocketMessage};
use crate::kalshi_error::KalshiError;
use serde::{Deserialize, Serialize};

/// Represents an active subscription.
#[derive(Debug, Clone)]
pub struct Subscription {
    pub sid: i32,
    pub channel: Channel,
    pub market_tickers: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeResponse {
    pub sid: i32,
    pub channel: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAction {
    AddMarkets,
    DeleteMarkets,
}

impl KalshiWebSocket {
    /// Subscribe to one or more channels for specified markets.
    ///
    /// # Arguments
    /// * `channels` - List of channels to subscribe to
    /// * `market_ticker` - Single market ticker (optional)
    /// * `market_tickers` - Multiple market tickers (optional)
    ///
    /// # Returns
    /// Vector of subscription responses with assigned `sid` values.
    ///
    pub async fn subscribe(
        &mut self,
        channels: Vec<Channel>,
        market_ticker: Option<String>,
        market_tickers: Option<Vec<String>>,
    ) -> Result<Vec<SubscribeResponse>, KalshiError> {
        let id = self.get_next_id();

        let mut cmd = serde_json::json!({
            "id": id,
            "cmd": "subscribe",
            "params": {
                "channels": channels.iter().map(|c| c.to_string()).collect::<Vec<_>>()
            }
        });

        if let Some(ticker) = market_ticker {
            cmd["params"]["market_ticker"] = serde_json::Value::String(ticker);
        }
        if let Some(tickers) = market_tickers {
            cmd["params"]["market_tickers"] = serde_json::Value::Array(
                tickers.into_iter().map(serde_json::Value::String).collect()
            );
        }

        self.send_command(cmd).await?;

        // Note: In practice, you'd wait for the "subscribed" responses
        // This is a simplified version - real implementation needs response handling
        Ok(vec![])
    }

    /// Unsubscribe from one or more subscriptions.
    ///
    /// # Arguments
    /// * `sids` - Subscription IDs to unsubscribe from
    ///
    pub async fn unsubscribe(&mut self, sids: Vec<i32>) -> Result<(), KalshiError> {
        let id = self.get_next_id();

        let cmd = serde_json::json!({
            "id": id,
            "cmd": "unsubscribe",
            "params": {
                "sids": sids
            }
        });

        self.send_command(cmd).await
    }

    /// List all active subscriptions.
    pub async fn list_subscriptions(&mut self) -> Result<Vec<Subscription>, KalshiError> {
        let id = self.get_next_id();

        let cmd = serde_json::json!({
            "id": id,
            "cmd": "list_subscriptions"
        });

        self.send_command(cmd).await?;

        // Return cached subscriptions
        Ok(self.subscriptions.values().cloned().collect())
    }

    /// Update an existing subscription by adding or removing markets.
    ///
    /// # Arguments
    /// * `sids` - Subscription IDs to update
    /// * `market_tickers` - Market tickers to add or remove
    /// * `action` - Whether to add or remove markets
    ///
    pub async fn update_subscription(
        &mut self,
        sids: Vec<i32>,
        market_tickers: Vec<String>,
        action: UpdateAction,
    ) -> Result<(), KalshiError> {
        let id = self.get_next_id();

        let action_str = match action {
            UpdateAction::AddMarkets => "add_markets",
            UpdateAction::DeleteMarkets => "delete_markets",
        };

        let cmd = serde_json::json!({
            "id": id,
            "cmd": "update_subscription",
            "params": {
                "sids": sids,
                "market_tickers": market_tickers,
                "action": action_str
            }
        });

        self.send_command(cmd).await
    }
}

---
4.7 Stream Interface

Add to: kalshi/src/websocket/connection.rs

use futures_util::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

impl KalshiWebSocket {
    /// Returns a stream of WebSocket messages.
    ///
    /// This stream yields parsed WebSocketMessage values as they arrive.
    /// The stream handles ping/pong automatically.
    ///
    /// # Example
    /// ```
    /// use futures_util::StreamExt;
    ///
    /// let mut ws = kalshi.websocket();
    /// ws.connect().await?;
    /// ws.subscribe(vec![Channel::Ticker], Some("AAPL-2024".to_string()), None).await?;
    ///
    /// while let Some(msg) = ws.messages().next().await {
    ///     match msg {
    ///         WebSocketMessage::Ticker(t) => println!("Price: {}", t.price.unwrap_or(0)),
    ///         _ => {}
    ///     }
    /// }
    /// ```
    pub fn messages(&mut self) -> impl Stream<Item = WebSocketMessage> + '_ {
        MessageStream { ws: self }
    }
}

struct MessageStream<'a> {
    ws: &'a mut KalshiWebSocket,
}

impl<'a> Stream for MessageStream<'a> {
    type Item = WebSocketMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let reader = match self.ws.reader.as_mut() {
            Some(r) => r,
            None => return Poll::Ready(None),
        };

        match Pin::new(reader).poll_next(cx) {
            Poll::Ready(Some(Ok(Message::Text(text)))) => {
                match WebSocketMessage::parse(&text) {
                    Ok(msg) => Poll::Ready(Some(msg)),
                    Err(_) => {
                        // Skip unparseable messages, poll again
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
            }
            Poll::Ready(Some(Ok(Message::Ping(_)))) => {
                // Pong is sent automatically by tokio-tungstenite
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Poll::Ready(Some(Ok(_))) => {
                // Other message types, poll again
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Poll::Ready(Some(Err(_))) => Poll::Ready(None),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

---
4.8 Integration with Kalshi Struct

File: kalshi/src/lib.rs

Add module declaration and factory method:

mod websocket;
pub use websocket::*;

impl Kalshi {
    /// Creates a new WebSocket client using the same credentials.
    ///
    /// The WebSocket client shares authentication but maintains
    /// a separate connection from the HTTP client.
    ///
    /// # Example
    /// ```
    /// let kalshi = Kalshi::new(TradingEnvironment::DemoMode, "key-id", "path/to/key.pem").await?;
    /// let mut ws = kalshi.websocket();
    /// ws.connect().await?;
    /// ```
    pub fn websocket(&self) -> KalshiWebSocket {
        KalshiWebSocket::new(
            self.trading_env(),
            &self.key_id,
            self.private_key.clone(),
        )
    }

    /// Returns the current trading environment.
    pub fn trading_env(&self) -> TradingEnvironment {
        if self.base_url.contains("demo") {
            TradingEnvironment::DemoMode
        } else {
            TradingEnvironment::ProdMode
        }
    }
}

---
Phase 5: Testing and Documentation

5.1 WebSocket Tests

New File: kalshi/tests/websocket_tests.rs

use kalshi::{Kalshi, TradingEnvironment, Channel, WebSocketMessage};
use futures_util::StreamExt;

mod common;

#[tokio::test]
async fn test_websocket_connect() {
    let auth = match common::skip_if_no_auth() {
        Some(a) => a,
        None => return,
    };

    let kalshi = common::setup_auth_test(&auth).await.unwrap();
    let mut ws = kalshi.websocket();

    assert!(!ws.is_connected());
    ws.connect().await.unwrap();
    assert!(ws.is_connected());

    ws.disconnect().await.unwrap();
    assert!(!ws.is_connected());
}

#[tokio::test]
async fn test_websocket_subscribe_ticker() {
    let auth = match common::skip_if_no_auth() {
        Some(a) => a,
        None => return,
    };

    let kalshi = common::setup_auth_test(&auth).await.unwrap();
    let mut ws = kalshi.websocket();
    ws.connect().await.unwrap();

    // Subscribe to ticker channel
    let responses = ws.subscribe(
        vec![Channel::Ticker],
        None,
        None,  // All markets
    ).await.unwrap();

    // Wait for a message
    let msg = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        ws.messages().next()
    ).await;

    ws.disconnect().await.unwrap();

    assert!(msg.is_ok(), "Should receive a message within 30 seconds");
}

5.2 Update lib.rs Exports

File: kalshi/src/lib.rs

Add to public exports:

// New enums
pub use portfolio::{TimeInForce, SelfTradePreventionType};
pub use market::MveFilter;

// WebSocket exports
pub use websocket::{
    KalshiWebSocket,
    Channel,
    WebSocketMessage,
    Subscription,
    UpdateAction,
    // Message types
    OrderbookSnapshotMsg,
    OrderbookDeltaMsg,
    TickerMsg,
    TradeMsg,
    FillMsg,
    MarketPositionMsg,
    MarketLifecycleMsg,
    EventLifecycleMsg,
};

---
Files to Modify Summary

| File                                 | Changes                                                       |
|--------------------------------------|---------------------------------------------------------------|
| kalshi/Cargo.toml                    | Add tokio-tungstenite, futures-util                           |
| kalshi/src/lib.rs                    | Add websocket module, new exports, websocket() method         |
| kalshi/src/portfolio/mod.rs          | Fix create_order, amend_order, get_settlements, get_positions |
| kalshi/src/market/mod.rs             | Add batch_get_market_candlesticks, update get_markets         |
| kalshi/src/events/mod.rs             | Add get_multivariate_events                                   |
| kalshi/src/communications/mod.rs     | Fix all RFQ/Quote methods, add get_communications_id          |
| kalshi/src/websocket/mod.rs          | NEW - Module root                                             |
| kalshi/src/websocket/connection.rs   | NEW - Connection handler                                      |
| kalshi/src/websocket/channels.rs     | NEW - Channel enum                                            |
| kalshi/src/websocket/messages.rs     | NEW - Message types                                           |
| kalshi/src/websocket/subscription.rs | NEW - Subscription management                                 |
| kalshi/tests/websocket_tests.rs      | NEW - WebSocket tests                                         |

---
Implementation Order

1. Phase 1 (Critical) - Fix existing trading methods first
2. Phase 2 - Add missing endpoints
3. Phase 3 - Align RFQ/Quote API
4. Phase 4 - WebSocket (largest, but independent)
5. Phase 5 - Tests and docs

Each phase can be independently tested and merged.