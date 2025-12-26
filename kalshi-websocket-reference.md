# Kalshi WebSocket API — Developer Reference

## Overview

Kalshi provides **a single WebSocket endpoint** for all real-time market data, trading events, and user-specific notifications.

* One connection per client
* All streams multiplexed via **subscriptions**
* All messages are JSON except **Ping/Pong control frames**
* Authentication via **API key at handshake**

**Endpoint**

```
wss://api.elections.kalshi.com
```

---

## Authentication

* Required for **all WebSocket connections**
* API key must be provided **during the WebSocket handshake**
* Certain channels require authentication (user-specific data)

---

## Core Concepts

### Subscription Model

* You **subscribe to channels**
* Each subscription is assigned a **sid** (subscription ID)
* A single subscription can cover **multiple markets**
* You can **add/remove markets dynamically** without re-subscribing

---

### Message Envelope (Common Fields)

Most messages include:

```json
{
  "type": "<message_type>",
  "sid": <subscription_id>,
  "seq": <monotonic_sequence_number>,
  "msg": { ...payload... }
}
```

**Notes**

* `seq` is strictly increasing per subscription
* Use `sid + seq` to ensure ordering correctness
* Some control messages omit `seq`

---

## Connection Keep-Alive (Critical)

Kalshi enforces heartbeat pings.

### Server Behavior

* Sends **Ping (opcode 0x9)** every **10 seconds**
* Ping payload: `"heartbeat"`

### Client Requirements

* MUST respond with **Pong (opcode 0xA)**
* Failure to respond may close the connection

### Client-Initiated Ping

* Allowed
* Kalshi responds with Pong

---

## Subscription Commands

### Subscribe

```json
{
  "id": 1,
  "cmd": "subscribe",
  "params": {
    "channels": ["orderbook_delta"],
    "market_ticker": "CPI-22DEC-TN0.1"
  }
}
```

* `channels`: array of channel names
* `market_ticker`: single market
* OR use `market_tickers`: array of markets

---

### Unsubscribe

```json
{
  "id": 124,
  "cmd": "unsubscribe",
  "params": {
    "sids": [1, 2]
  }
}
```

---

### List Subscriptions

```json
{
  "id": 3,
  "cmd": "list_subscriptions"
}
```

---

### Update Subscription (Add Markets)

```json
{
  "id": 124,
  "cmd": "update_subscription",
  "params": {
    "sids": [456],
    "market_tickers": ["NEW-MARKET-1"],
    "action": "add_markets"
  }
}
```

### Update Subscription (Remove Markets)

```json
{
  "id": 125,
  "cmd": "update_subscription",
  "params": {
    "sid": 456,
    "market_tickers": ["OLD-MARKET"],
    "action": "delete_markets"
  }
}
```

---

## Control Responses

### Subscribed

```json
{
  "id": 1,
  "type": "subscribed",
  "msg": {
    "channel": "orderbook_delta",
    "sid": 1
  }
}
```

### OK

```json
{
  "id": 123,
  "type": "ok",
  "sid": 456,
  "seq": 222
}
```

### Error

```json
{
  "id": 123,
  "type": "error",
  "msg": {
    "code": 6,
    "msg": "Already subscribed"
  }
}
```

---

## Data Channels

---

## 1. Orderbook Updates (`orderbook_delta`)

**Purpose:** Maintain a real-time limit order book.

### Key Invariants

* You ALWAYS receive a **snapshot first**
* Followed by **incremental deltas**
* You must apply deltas in `seq` order

---

### Snapshot

```json
{
  "type": "orderbook_snapshot",
  "sid": 2,
  "seq": 2,
  "msg": {
    "market_ticker": "FED-23DEC-T3.00",
    "yes": [[8, 300]],
    "yes_dollars": [["0.080", 300]],
    "no": [[54, 20]],
    "no_dollars": [["0.540", 20]]
  }
}
```

* Prices are integers (0–100)
* `_dollars` are string decimals

---

### Delta

```json
{
  "type": "orderbook_delta",
  "sid": 2,
  "seq": 3,
  "msg": {
    "market_ticker": "FED-23DEC-T3.00",
    "price": 96,
    "price_dollars": "0.960",
    "delta": -54,
    "side": "yes"
  }
}
```

---

## 2. Market Ticker (`ticker`)

**Purpose:** Real-time market price + stats.

* Market specification optional
* Updates sent whenever **any field changes**

```json
{
  "type": "ticker",
  "sid": 11,
  "msg": {
    "market_ticker": "FED-23DEC-T3.00",
    "price": 48,
    "yes_bid": 45,
    "yes_ask": 53,
    "price_dollars": "0.480",
    "volume": 33896,
    "open_interest": 20422,
    "ts": 1669149841
  }
}
```

---

## 3. Public Trades (`trade`)

**Purpose:** Live tape of executed trades.

* Market filter optional
* Sent immediately after execution

```json
{
  "type": "trade",
  "sid": 11,
  "msg": {
    "market_ticker": "HIGHNY-22DEC23-B53.5",
    "yes_price": 36,
    "no_price": 64,
    "count": 136,
    "taker_side": "no",
    "ts": 1669149841
  }
}
```

---

## 4. User Fills (`fill`) — AUTH REQUIRED

**Purpose:** Your executed orders.

* Market filter ignored
* Always returns **your fills only**

```json
{
  "type": "fill",
  "sid": 13,
  "msg": {
    "trade_id": "...",
    "order_id": "...",
    "market_ticker": "...",
    "side": "yes",
    "action": "buy",
    "count": 278,
    "post_position": 500
  }
}
```

---

## 5. Market Positions (`market_position`) — AUTH REQUIRED

**Purpose:** Live position + P&L tracking.

### Monetary Units

* All monetary fields are **centi-cents**
* Divide by **10,000** to get USD

```json
{
  "type": "market_position",
  "sid": 14,
  "msg": {
    "market_ticker": "FED-23DEC-T3.00",
    "position": 100,
    "position_cost": 500000,
    "realized_pnl": 100000,
    "fees_paid": 10000
  }
}
```

---

## 6. Market & Event Lifecycle

### Market Lifecycle V2

```json
{
  "type": "market_lifecycle_v2",
  "sid": 13,
  "msg": {
    "market_ticker": "INXD-23SEP14-B4487",
    "event_type": "created",
    "open_ts": 1694635200,
    "close_ts": 1694721600
  }
}
```

**Event Types**

* `created`
* `activated`
* `deactivated`
* `close_date_updated`
* `determined`
* `settled`

---

### Event Lifecycle

```json
{
  "type": "event_lifecycle",
  "sid": 5,
  "msg": {
    "event_ticker": "INXD-23SEP14",
    "title": "INX title"
  }
}
```

---

## 7. Multivariate Lookups (`multivariate`)

**Purpose:** Track linked/multivariate markets.

```json
{
  "type": "multivariate_lookup",
  "sid": 13,
  "msg": {
    "collection_ticker": "KXOSCARWINNERS-25",
    "market_ticker": "KXOSCARWINNERS-25C0CE5-36353",
    "selected_markets": [
      { "market_ticker": "...", "side": "yes" }
    ]
  }
}
```

---

## 8. Communications (RFQs & Quotes) — AUTH REQUIRED

### RFQ Created

```json
{
  "type": "rfq_created",
  "sid": 15,
  "msg": {
    "id": "rfq_123",
    "market_ticker": "...",
    "contracts": 100
  }
}
```

### Quote Created

```json
{
  "type": "quote_created",
  "sid": 15,
  "msg": {
    "quote_id": "quote_456",
    "rfq_id": "rfq_123",
    "yes_bid": 35,
    "no_bid": 65
  }
}
```

### Quote Accepted

```json
{
  "type": "quote_accepted",
  "sid": 15,
  "msg": {
    "quote_id": "quote_456",
    "accepted_side": "yes"
  }
}
```

---

## Mental Model for Agents

> **Kalshi WebSockets = ordered event streams over a single multiplexed connection**

### If the agent needs:

* **Live prices** → `ticker`
* **Depth / microstructure** → `orderbook_delta`
* **Tape** → `trade`
* **Own trading state** → `fill`, `market_position`
* **Market discovery** → `market_lifecycle_v2`
* **Complex markets** → `multivariate`
* **RFQ trading** → `communications`

---

## Common Pitfalls

| Issue              | Fix                          |
| ------------------ | ---------------------------- |
| Dropped connection | Respond to Ping every 10s    |
| Out-of-order book  | Enforce `seq` ordering       |
| Wrong P&L          | Divide centi-cents by 10,000 |
| Missed snapshot    | Ignore deltas until snapshot |
| Auth errors        | API key must be in handshake |

---