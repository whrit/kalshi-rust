//! Market data retrieval and analysis.
//!
//! This module provides comprehensive access to Kalshi market data, enabling you to
//! query markets, retrieve orderbooks, track trades, analyze price history, and explore
//! series and events. Market data is mostly public and does not require authentication
//! (with some exceptions for authenticated user-specific queries).
//!
//! # Overview
//!
//! The market module encompasses several data categories:
//!
//! - **Markets**: Individual binary prediction markets (Yes/No outcomes)
//! - **Series**: Collections of related events (e.g., "HIGHNY" for NYC temperature)
//! - **Events**: Specific prediction events containing one or more markets
//! - **Orderbooks**: Current bid/ask levels and market depth
//! - **Trades**: Historical trade executions
//! - **Candlesticks**: OHLC price data for technical analysis
//!
//! # Quick Start - Getting Market Data
//!
//! ```rust,ignore
//! use kalshi::{Kalshi, TradingEnvironment};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let kalshi = Kalshi::new(
//!     TradingEnvironment::DemoMode,
//!     "your-key-id",
//!     "path/to/private.pem"
//! ).await?;
//!
//! // Get a specific market
//! let market = kalshi.get_market("HIGHNY-24JAN15-T50").await?;
//! println!("Market: {}", market.title);
//! println!("Yes bid: {} | Yes ask: {}", market.yes_bid, market.yes_ask);
//! println!("Volume: {} | Open interest: {}", market.volume, market.open_interest);
//!
//! // Get the current orderbook
//! let orderbook = kalshi.get_orderbook("HIGHNY-24JAN15-T50", Some(5)).await?;
//! println!("Orderbook depth: {:?}", orderbook.yes);
//! # Ok(())
//! # }
//! ```
//!
//! # Key Concepts
//!
//! ## Market Structure
//!
//! Each market represents a binary outcome (Yes/No) on a specific question:
//! - **Ticker**: Unique identifier (e.g., "HIGHNY-24JAN15-T50")
//! - **Event Ticker**: Parent event (e.g., "HIGHNY-24JAN15")
//! - **Series Ticker**: Series family (e.g., "HIGHNY")
//! - **Title**: Human-readable question
//! - **Status**: open, closed, or settled
//!
//! ## Pricing
//!
//! Market prices are displayed in cents (0-100):
//! - **yes_bid**: Highest price someone is willing to pay for YES
//! - **yes_ask**: Lowest price someone is willing to sell YES for
//! - **no_bid/no_ask**: Equivalent for NO contracts
//! - **last_price**: Most recent trade execution price
//!
//! ## Orderbook
//!
//! The orderbook shows all active bids and asks at various price levels:
//! - Each level shows [price, quantity]
//! - **yes** side: Bids and asks for YES contracts
//! - **no** side: Bids and asks for NO contracts
//!
//! # Common Workflows
//!
//! ## Finding Markets
//!
//! ```rust,ignore
//! # use kalshi::Kalshi;
//! # async fn example(kalshi: &Kalshi) -> Result<(), Box<dyn std::error::Error>> {
//! // Search for open markets in a specific event
//! let (cursor, markets) = kalshi.get_markets(
//!     Some(20),                          // limit
//!     None,                              // cursor
//!     Some("HIGHNY-24JAN15".to_string()), // event_ticker
//!     None,                              // series_ticker
//!     Some("open".to_string()),          // status
//!     None,                              // tickers
//!     None, None, None, None, None, None, None, // timestamps & filters
//! ).await?;
//!
//! for market in markets {
//!     println!("{}: {} (volume: {})", market.ticker, market.title, market.volume);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Analyzing the Orderbook
//!
//! ```rust,ignore
//! # use kalshi::Kalshi;
//! # async fn example(kalshi: &Kalshi) -> Result<(), Box<dyn std::error::Error>> {
//! let orderbook = kalshi.get_orderbook("HIGHNY-24JAN15-T50", Some(10)).await?;
//!
//! // Check YES side liquidity
//! if let Some(yes_levels) = &orderbook.yes {
//!     println!("YES orderbook:");
//!     for level in yes_levels {
//!         if level.len() >= 2 {
//!             println!("  Price: {} | Quantity: {}", level[0], level[1]);
//!         }
//!     }
//! }
//!
//! // Dollar prices are also available
//! for (price, qty) in &orderbook.yes_dollars {
//!     println!("  ${:.4} | {} contracts", price, qty);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Viewing Recent Trades
//!
//! ```rust,ignore
//! # use kalshi::Kalshi;
//! # async fn example(kalshi: &Kalshi) -> Result<(), Box<dyn std::error::Error>> {
//! let (cursor, trades) = kalshi.get_trades(
//!     Some(50),                          // limit
//!     None,                              // cursor
//!     Some("HIGHNY-24JAN15-T50".to_string()), // ticker
//!     None,                              // min_ts
//!     None,                              // max_ts
//! ).await?;
//!
//! for trade in trades {
//!     println!("Trade: {} contracts @ {} ({})",
//!         trade.count,
//!         trade.yes_price,
//!         trade.created_time
//!     );
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Getting Historical Price Data
//!
//! ```rust,ignore
//! # use kalshi::Kalshi;
//! # async fn example(kalshi: &Kalshi) -> Result<(), Box<dyn std::error::Error>> {
//! // Get 1-hour candlesticks for the past 24 hours
//! let now = chrono::Utc::now().timestamp();
//! let day_ago = now - 86400;
//!
//! let candlesticks = kalshi.get_market_candlesticks(
//!     "HIGHNY-24JAN15-T50",
//!     "HIGHNY",
//!     Some(day_ago),
//!     Some(now),
//!     Some(60),  // 60-minute intervals
//! ).await?;
//!
//! for candle in candlesticks {
//!     println!("{}: O:{} H:{} L:{} C:{} V:{}",
//!         candle.start_ts,
//!         candle.yes_open,
//!         candle.yes_high,
//!         candle.yes_low,
//!         candle.yes_close,
//!         candle.volume
//!     );
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Exploring Series and Events
//!
//! ```rust,ignore
//! # use kalshi::Kalshi;
//! # async fn example(kalshi: &Kalshi) -> Result<(), Box<dyn std::error::Error>> {
//! // Get a series and its events
//! let series = kalshi.get_series("HIGHNY").await?;
//! println!("Series: {}", series.title.as_ref().unwrap_or(&"Unknown".to_string()));
//!
//! // Get events in the series
//! let (cursor, events) = kalshi.get_events(
//!     Some(10),                   // limit
//!     None,                       // cursor
//!     Some("open".to_string()),   // status
//!     Some("HIGHNY".to_string()), // series_ticker
//!     Some(true),                 // with_nested_markets
//!     None, None,                 // with_milestones, min_close_ts
//! ).await?;
//!
//! for event in events {
//!     println!("Event: {} - {}", event.event_ticker, event.title);
//!     if let Some(markets) = &event.markets {
//!         println!("  Contains {} markets", markets.len());
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Advanced Features
//!
//! ## Batch Candlestick Retrieval
//!
//! Fetch candlestick data for multiple markets simultaneously for better performance:
//!
//! ```rust,ignore
//! # use kalshi::Kalshi;
//! # async fn example(kalshi: &Kalshi) -> Result<(), Box<dyn std::error::Error>> {
//! let tickers = vec![
//!     "MARKET-1".to_string(),
//!     "MARKET-2".to_string(),
//!     "MARKET-3".to_string(),
//! ];
//!
//! let now = chrono::Utc::now().timestamp();
//! let hour_ago = now - 3600;
//!
//! let results = kalshi.batch_get_market_candlesticks(
//!     tickers,
//!     hour_ago,
//!     now,
//!     60,        // 60-minute interval
//!     Some(true), // include_latest_before_start
//! ).await?;
//!
//! for market_candles in results {
//!     println!("Market {}: {} candlesticks",
//!         market_candles.ticker,
//!         market_candles.candlesticks.len()
//!     );
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Filtering Markets with MVE
//!
//! Filter multivariate event (MVE) markets:
//!
//! ```rust,ignore
//! # use kalshi::{Kalshi, MveFilter};
//! # async fn example(kalshi: &Kalshi) -> Result<(), Box<dyn std::error::Error>> {
//! // Only get MVE markets
//! let (cursor, markets) = kalshi.get_markets(
//!     Some(50),
//!     None, None, None, None, None,
//!     None, None, None, None, None, None,
//!     Some(MveFilter::Only),  // Only MVE markets
//! ).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Data Structures
//!
//! ## Market
//!
//! The [`Market`] struct contains comprehensive information about a prediction market:
//! - Market identification (ticker, event_ticker, series_ticker)
//! - Status and lifecycle (status, open_time, close_time, expiration_time)
//! - Current prices (yes_bid, yes_ask, no_bid, no_ask, last_price)
//! - Market activity (volume, volume_24h, open_interest, liquidity)
//! - Settlement (result, settlement_value)
//!
//! ## Orderbook
//!
//! The [`Orderbook`] struct represents the current market depth:
//! - `yes` / `no`: Price levels in cents as `Vec<Vec<i32>>`
//! - `yes_dollars` / `no_dollars`: Price levels in dollars as `Vec<(f32, i32)>`
//!
//! ## Candle
//!
//! The [`Candle`] struct provides OHLC data for price analysis:
//! - Time range (start_ts, end_ts)
//! - YES prices (yes_open, yes_high, yes_low, yes_close)
//! - NO prices (no_open, no_high, no_low, no_close)
//! - Volume and open_interest for the period
//!
//! # Best Practices
//!
//! 1. **Use filters**: Narrow down market queries with status, event_ticker, or series_ticker
//! 2. **Limit results**: Always specify a reasonable limit to avoid overwhelming responses
//! 3. **Batch operations**: Use batch endpoints when fetching data for multiple markets
//! 4. **WebSocket for real-time**: For live data, use WebSocket subscriptions instead of polling
//! 5. **Candlestick intervals**: Choose appropriate intervals (1, 60, or 1440 minutes)
//!
//! # See Also
//!
//! - [`get_market`](crate::Kalshi::get_market) - Retrieve a specific market
//! - [`get_markets`](crate::Kalshi::get_markets) - Query multiple markets
//! - [`get_orderbook`](crate::Kalshi::get_orderbook) - Get current orderbook
//! - [`get_trades`](crate::Kalshi::get_trades) - Retrieve trade history
//! - [`get_market_candlesticks`](crate::Kalshi::get_market_candlesticks) - Historical price data

use super::Kalshi;
use crate::kalshi_error::*;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

impl Kalshi {
    /// Retrieves a list of markets from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple markets, allowing for filtering by event ticker, series ticker,
    /// status, tickers, time range, and pagination. Markets represent the individual trading
    /// instruments within events.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of markets returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `event_ticker` - An optional string to filter markets by event ticker.
    /// * `series_ticker` - An optional string to filter markets by series ticker.
    /// * `status` - An optional string to filter markets by their status.
    /// * `tickers` - An optional string to filter markets by specific tickers.
    /// * `min_close_ts` - An optional minimum timestamp for market close time.
    /// * `max_close_ts` - An optional maximum timestamp for market close time.
    /// * `min_created_ts` - An optional minimum timestamp for market creation time.
    /// * `max_created_ts` - An optional maximum timestamp for market creation time.
    /// * `min_settled_ts` - An optional minimum timestamp for market settlement time.
    /// * `max_settled_ts` - An optional maximum timestamp for market settlement time.
    /// * `mve_filter` - An optional filter for multivariate events (Only or Exclude).
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Market>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Market` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let (cursor, markets) = kalshi_instance.get_markets(
    ///     Some(10), None, Some("SOME-EVENT".to_string()), None,
    ///     Some("open".to_string()), None, None, None, None, None, None, None, None
    /// ).await.unwrap();
    /// ```
    ///
    #[allow(clippy::too_many_arguments)]
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
        min_created_ts: Option<i64>,
        max_created_ts: Option<i64>,
        min_settled_ts: Option<i64>,
        max_settled_ts: Option<i64>,
        mve_filter: Option<MveFilter>,
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
        add_param!(p, "min_created_ts", min_created_ts);
        add_param!(p, "max_created_ts", max_created_ts);
        add_param!(p, "min_settled_ts", min_settled_ts);
        add_param!(p, "max_settled_ts", max_settled_ts);
        add_param!(p, "mve_filter", mve_filter);

        let res: MarketListResponse = self
            .client
            .get(reqwest::Url::parse_with_params(&url, &p)?)
            .send()
            .await?
            .json()
            .await?;
        Ok((res.cursor, res.markets))
    }

    /// Retrieves detailed information about a specific market from the Kalshi exchange.
    ///
    /// This method fetches data for a single market identified by its ticker.
    /// The market represents a specific trading instrument within an event.
    ///
    /// # Arguments
    ///
    /// * `ticker` - A string slice referencing the market's unique ticker identifier.
    ///
    /// # Returns
    ///
    /// - `Ok(Market)`: Detailed information about the specified market on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let ticker = "SOME-MARKET-2024";
    /// let market = kalshi_instance.get_market(ticker).await.unwrap();
    /// ```
    ///
    pub async fn get_market(&self, ticker: &str) -> Result<Market, KalshiError> {
        let url = format!("{}/markets/{}", self.base_url, ticker);
        let res: SingleMarketResponse = self.client.get(url).send().await?.json().await?;
        Ok(res.market)
    }

    /// Retrieves the orderbook for a specific market from the Kalshi exchange.
    ///
    /// This method fetches the current orderbook data for a market, showing the current
    /// bid and ask orders for both Yes and No sides of the market.
    ///
    /// # Arguments
    ///
    /// * `ticker` - A string slice referencing the market's unique ticker identifier.
    /// * `depth` - Optional depth parameter to limit the number of price levels returned.
    ///
    /// # Returns
    ///
    /// - `Ok(Orderbook)`: The current orderbook data for the specified market on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let ticker = "SOME-MARKET-2024";
    /// let orderbook = kalshi_instance.get_orderbook(ticker, Some(10)).await.unwrap();
    /// ```
    ///
    pub async fn get_orderbook(
        &self,
        ticker: &str,
        depth: Option<i32>,
    ) -> Result<Orderbook, KalshiError> {
        let mut url = format!("{}/markets/{}/orderbook", self.base_url, ticker);

        if let Some(d) = depth {
            url.push_str(&format!("?depth={}", d));
        }

        let response = self.client.get(&url).send().await?;
        let response_text = response.text().await?;

        // Try to parse as JSON first to see what we're getting
        let json_value: serde_json::Value = serde_json::from_str(&response_text).map_err(|e| {
            eprintln!(
                "ERROR: Failed to parse response as JSON for ticker {}: {}",
                ticker, e
            );
            eprintln!("ERROR: Raw response: {}", response_text);
            KalshiError::UserInputError(format!("Failed to parse JSON: {}", e))
        })?;

        // Check if the response has an "orderbook" field
        if !json_value.is_object() || !json_value.as_object().unwrap().contains_key("orderbook") {
            eprintln!(
                "ERROR: Response does not contain 'orderbook' field for ticker: {}",
                ticker
            );
            eprintln!(
                "ERROR: Available keys: {:?}",
                json_value
                    .as_object()
                    .map(|obj| obj.keys().collect::<Vec<_>>())
            );
            eprintln!(
                "ERROR: Full response: {}",
                serde_json::to_string_pretty(&json_value).unwrap()
            );
            return Err(KalshiError::UserInputError(
                "missing field `orderbook`".to_string(),
            ));
        }

        let res: OrderbookResponse = serde_json::from_value(json_value).map_err(|e| {
            eprintln!(
                "ERROR: Failed to deserialize OrderbookResponse for ticker {}: {}",
                ticker, e
            );
            KalshiError::UserInputError(format!("Failed to deserialize: {}", e))
        })?;

        Ok(res.orderbook)
    }

    /// Retrieves the orderbook for a specific market from the Kalshi exchange (without depth limit).
    ///
    /// This is a convenience method that calls `get_orderbook(ticker, None)`.
    ///
    /// # Arguments
    ///
    /// * `ticker` - A string slice referencing the market's unique ticker identifier.
    ///
    /// # Returns
    ///
    /// - `Ok(Orderbook)`: The current orderbook data for the specified market on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let ticker = "SOME-MARKET-2024";
    /// let orderbook = kalshi_instance.get_orderbook_full(ticker).await.unwrap();
    /// ```
    ///
    pub async fn get_orderbook_full(&self, ticker: &str) -> Result<Orderbook, KalshiError> {
        self.get_orderbook(ticker, None).await
    }

    /// Retrieves candlestick data for a specific market from the Kalshi exchange.
    ///
    /// This method fetches historical price data in candlestick format for a market,
    /// allowing for analysis of price movements over time with various time intervals.
    ///
    /// # Arguments
    ///
    /// * `ticker` - A string slice referencing the market's unique ticker identifier.
    /// * `series_ticker` - A string slice referencing the series ticker.
    /// * `start_ts` - Optional timestamp for the start of the data range (restricts candlesticks to those ending on or after this timestamp).
    /// * `end_ts` - Optional timestamp for the end of the data range (restricts candlesticks to those ending on or before this timestamp).
    /// * `period_interval` - Optional integer specifying the length of each candlestick period in minutes (must be 1, 60, or 1440).
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Candle>)`: A vector of `Candle` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let candlesticks = kalshi_instance.get_market_candlesticks(
    ///     "SOME-MARKET-2024", "SOME-SERIES", 1640995200, 1641081600, 60
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_market_candlesticks(
        &self,
        ticker: &str,
        series_ticker: &str,
        start_ts: Option<i64>,
        end_ts: Option<i64>,
        period_interval: Option<i32>,
    ) -> Result<Vec<Candle>, KalshiError> {
        let url = format!(
            "{}/series/{}/markets/{}/candlesticks",
            self.base_url, series_ticker, ticker
        );
        let mut p = vec![];
        add_param!(p, "start_ts", start_ts);
        add_param!(p, "end_ts", end_ts);
        add_param!(p, "period_interval", period_interval);

        let res: CandlestickListResponse = self
            .client
            .get(reqwest::Url::parse_with_params(&url, &p)?)
            .send()
            .await?
            .json()
            .await?;
        Ok(res.candlesticks)
    }

    /// Retrieves candlestick data for multiple markets in a single request.
    ///
    /// This method fetches historical price data in candlestick format for multiple markets
    /// simultaneously, which is more efficient than making individual requests for each market.
    ///
    /// # Arguments
    ///
    /// * `market_tickers` - A vector of market ticker identifiers (max 100)
    /// * `start_ts` - Start timestamp in Unix seconds
    /// * `end_ts` - End timestamp in Unix seconds
    /// * `period_interval` - Candlestick period in minutes (must be 1, 60, or 1440)
    /// * `include_latest_before_start` - If true, prepends the latest candlestick before start_ts
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<MarketCandlesticks>)`: A vector of `MarketCandlesticks` objects, one per market
    /// - `Err(KalshiError)`: An error if there is an issue with the request
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let tickers = vec!["MARKET-1".to_string(), "MARKET-2".to_string()];
    /// let now = chrono::Utc::now().timestamp();
    /// let start = now - 86400; // 1 day ago
    /// let candlesticks = kalshi_instance.batch_get_market_candlesticks(
    ///     tickers, start, now, 60, None
    /// ).await.unwrap();
    /// ```
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
                "Maximum 100 market tickers allowed per batch request".to_string(),
            ));
        }

        let url = format!("{}/markets/candlesticks/batch", self.base_url);

        // Join tickers with commas as required by the API
        let tickers_param = market_tickers.join(",");

        let mut p = vec![
            ("market_tickers", tickers_param),
            ("start_ts", start_ts.to_string()),
            ("end_ts", end_ts.to_string()),
            ("period_interval", period_interval.to_string()),
        ];

        if let Some(include_latest) = include_latest_before_start {
            p.push(("include_latest_before_start", include_latest.to_string()));
        }

        let res: BatchCandlestickResponse = self
            .client
            .get(reqwest::Url::parse_with_params(&url, &p)?)
            .send()
            .await?
            .json()
            .await?;
        Ok(res.markets)
    }

    /// Retrieves a list of trades from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple trades, allowing for filtering by ticker, time range,
    /// and pagination. Trades represent executed orders between buyers and sellers.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of trades returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `ticker` - An optional string to filter trades by market ticker.
    /// * `min_ts` - An optional minimum timestamp for trade creation time.
    /// * `max_ts` - An optional maximum timestamp for trade creation time.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Trade>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Trade` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let (cursor, trades) = kalshi_instance.get_trades(
    ///     Some(100), None, Some("SOME-MARKET-2024".to_string()),
    ///     Some(1640995200), Some(1641081600)
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_trades(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        ticker: Option<String>,
        min_ts: Option<i64>,
        max_ts: Option<i64>,
    ) -> Result<(Option<String>, Vec<Trade>), KalshiError> {
        let url = format!("{}/markets/trades", self.base_url);
        let mut p = vec![];
        add_param!(p, "limit", limit);
        add_param!(p, "cursor", cursor);
        add_param!(p, "ticker", ticker);
        add_param!(p, "min_ts", min_ts);
        add_param!(p, "max_ts", max_ts);

        let res: TradeListResponse = self
            .client
            .get(reqwest::Url::parse_with_params(&url, &p)?)
            .send()
            .await?
            .json()
            .await?;
        Ok((res.cursor, res.trades))
    }

    /// Retrieves a list of series from the Kalshi exchange based on specified criteria.
    ///
    /// This method fetches multiple series, allowing for filtering by category, tags,
    /// and pagination. Series represent collections of related events and markets.
    ///
    /// # Arguments
    ///
    /// * `limit` - An optional integer to limit the number of series returned.
    /// * `cursor` - An optional string for pagination cursor.
    /// * `category` - An optional string to filter series by category.
    /// * `tags` - An optional string to filter series by tags.
    ///
    /// # Returns
    ///
    /// - `Ok((Option<String>, Vec<Series>))`: A tuple containing an optional pagination cursor
    ///   and a vector of `Series` objects on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let (cursor, series) = kalshi_instance.get_series_list(
    ///     Some(20), None, Some("politics".to_string()), Some("election".to_string())
    /// ).await.unwrap();
    /// ```
    ///
    pub async fn get_series_list(
        &self,
        limit: Option<i64>,
        cursor: Option<String>,
        category: Option<String>,
        tags: Option<String>,
    ) -> Result<(Option<String>, Vec<Series>), KalshiError> {
        // --- build query string ------------------------------------------------
        let mut p = Vec::new();
        add_param!(p, "limit", limit);
        add_param!(p, "cursor", cursor);
        add_param!(p, "category", category);
        add_param!(p, "tags", tags);

        let path = if p.is_empty() {
            "/series".to_string()
        } else {
            format!("/series?{}", serde_urlencoded::to_string(&p)?)
        };

        // --- signed GET --------------------------------------------------------
        #[derive(Debug, serde::Deserialize)]
        struct SeriesListResponse {
            cursor: Option<String>,
            series: Option<Vec<Series>>, // ← tolerate `null`
        }

        let res: SeriesListResponse = self.signed_get(&path).await?;
        Ok((res.cursor, res.series.unwrap_or_default()))
    }

    /// Retrieves detailed information about a specific series from the Kalshi exchange.
    ///
    /// This method fetches data for a single series identified by its series ticker.
    /// The series represents a collection of related events and markets.
    ///
    /// # Arguments
    ///
    /// * `series_ticker` - A string slice referencing the series' unique ticker identifier.
    ///
    /// # Returns
    ///
    /// - `Ok(Series)`: Detailed information about the specified series on successful retrieval.
    /// - `Err(KalshiError)`: An error if there is an issue with the request.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming `kalshi_instance` is an instance of `Kalshi`
    /// let series_ticker = "SOME-SERIES";
    /// let series = kalshi_instance.get_series(series_ticker).await.unwrap();
    /// ```
    ///
    pub async fn get_series(&self, series_ticker: &str) -> Result<Series, KalshiError> {
        let url = format!("{}/series/{}", self.base_url, series_ticker);
        let res: SingleSeriesResponse = self.client.get(url).send().await?.json().await?;
        Ok(res.series)
    }
}

/// When the API gives `"field": null` treat it as an empty Vec.
fn null_to_empty_vec<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<Vec<T>>::deserialize(d)?;
    Ok(opt.unwrap_or_default())
}

/// Deserializes dollar price levels from the API format [[string, number], ...]
/// to Vec<(f32, i32)> where the string is converted to f32.
fn deserialize_dollar_levels<'de, D>(d: D) -> Result<Vec<(f32, i32)>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    // First, deserialize as a Vec of generic JSON values
    let opt = Option::<Vec<serde_json::Value>>::deserialize(d)?;

    // If null or missing, return empty vec
    let Some(arr) = opt else {
        return Ok(Vec::new());
    };

    // Convert each [price_string, count] to (f32, i32)
    let mut result = Vec::new();
    for item in arr {
        let level = item
            .as_array()
            .ok_or_else(|| Error::custom("Expected array for price level"))?;

        if level.len() != 2 {
            return Err(Error::custom("Expected array of length 2 for price level"));
        }

        // Parse price (can be string or number)
        let price: f32 = match &level[0] {
            serde_json::Value::String(s) => s
                .parse()
                .map_err(|_| Error::custom(format!("Failed to parse price string: {}", s)))?,
            serde_json::Value::Number(n) => n
                .as_f64()
                .ok_or_else(|| Error::custom("Failed to convert price number to f64"))?
                as f32,
            _ => return Err(Error::custom("Price must be string or number")),
        };

        // Parse count (should be a number)
        let count: i32 = level[1]
            .as_i64()
            .ok_or_else(|| Error::custom("Count must be a number"))?
            as i32;

        result.push((price, count));
    }

    Ok(result)
}

// -------- public models --------

/// Represents an event on the Kalshi exchange.
///
/// An event is a prediction market that contains multiple markets for trading.
/// Events can have various statuses and may include nested markets.
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub event_ticker: String,
    pub series_ticker: String,
    pub title: String,
    pub sub_title: String,
    pub mutually_exclusive: bool,
    pub category: String,
    pub strike_date: Option<String>,
    pub strike_period: Option<String>,
    pub markets: Option<Vec<Market>>,
}

/// Represents a market on the Kalshi exchange.
///
/// A market is a specific trading instrument within an event, representing
/// a binary outcome that users can trade on (Yes/No).
#[derive(Debug, Deserialize, Serialize)]
pub struct Market {
    pub ticker: String,
    pub event_ticker: String,
    pub market_type: String,
    pub title: String,
    pub subtitle: String,
    pub yes_sub_title: String,
    pub no_sub_title: String,
    pub open_time: String,
    pub close_time: String,
    pub expected_expiration_time: Option<String>,
    pub expiration_time: Option<String>,
    pub latest_expiration_time: String,
    pub settlement_timer_seconds: i64,
    pub status: String,
    pub response_price_units: String,
    pub notional_value: i64,
    pub tick_size: i64,
    pub yes_bid: i64,
    pub yes_ask: i64,
    pub no_bid: i64,
    pub no_ask: i64,
    pub last_price: i64,
    pub previous_yes_bid: i64,
    pub previous_yes_ask: i64,
    pub previous_price: i64,
    pub volume: i64,
    pub volume_24h: i64,
    pub liquidity: i64,
    pub open_interest: i64,
    pub result: SettlementResult,
    pub cap_strike: Option<f64>,
    pub can_close_early: bool,
    pub expiration_value: String,
    pub category: String,
    pub risk_limit_cents: i64,
    pub strike_type: Option<String>,
    pub floor_strike: Option<f64>,
    pub rules_primary: String,
    pub rules_secondary: String,
    pub settlement_value: Option<String>,
    pub functional_strike: Option<String>,
}

/// Represents a series on the Kalshi exchange.
///
/// A series is a collection of related events and markets, typically
/// organized around a common theme or category.
#[derive(Debug, Deserialize, Serialize)]
pub struct Series {
    #[serde(default)]
    pub ticker: Option<String>,
    #[serde(default)]
    pub frequency: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default, deserialize_with = "null_to_empty_vec")]
    pub tags: Vec<String>,
    #[serde(default, deserialize_with = "null_to_empty_vec")]
    pub settlement_sources: Vec<SettlementSource>,
    #[serde(default)]
    pub contract_url: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Represents a multivariate event collection on the Kalshi exchange.
///
/// A multivariate event collection contains multiple related markets
/// that are analyzed together as a group.
#[derive(Debug, Deserialize, Serialize)]
pub struct MultivariateEventCollection {
    pub collection_ticker: String,
    pub title: String,
    pub description: String,
    pub category: String,
    #[serde(default, deserialize_with = "null_to_empty_vec")]
    pub tags: Vec<String>,
    #[serde(default, deserialize_with = "null_to_empty_vec")]
    pub markets: Vec<Market>,
    pub created_time: String,
    pub updated_time: String,
}

/// Represents a candlestick data point for market analysis.
///
/// Candlesticks provide historical price data including open, high, low, and close
/// prices for both Yes and No sides of a market over a specific time period.
#[derive(Debug, Deserialize, Serialize)]
pub struct Candle {
    pub start_ts: i64,
    pub end_ts: i64,
    pub yes_open: i32,
    pub yes_high: i32,
    pub yes_low: i32,
    pub yes_close: i32,
    pub no_open: i32,
    pub no_high: i32,
    pub no_low: i32,
    pub no_close: i32,
    pub volume: i64,
    pub open_interest: i64,
}

/// Represents the orderbook for a market on the Kalshi exchange.
///
/// The orderbook contains current bid and ask orders for both Yes and No sides
/// of a market, showing the current market depth and liquidity.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Orderbook {
    /// Price levels in cents: [[price_cents, count], ...]
    pub yes: Option<Vec<Vec<i32>>>,
    /// Price levels in cents: [[price_cents, count], ...]
    pub no: Option<Vec<Vec<i32>>>,
    /// Price levels in dollars: [[price_dollars, count], ...]
    /// The price_dollars string from API is converted to f32 (4 dp, range 0-1)
    #[serde(default, deserialize_with = "deserialize_dollar_levels")]
    pub yes_dollars: Vec<(f32, i32)>,
    /// Price levels in dollars: [[price_dollars, count], ...]
    /// The price_dollars string from API is converted to f32 (4 dp, range 0-1)
    #[serde(default, deserialize_with = "deserialize_dollar_levels")]
    pub no_dollars: Vec<(f32, i32)>,
}

/// Represents a market snapshot at a specific point in time.
///
/// A snapshot provides a summary of market activity including current prices,
/// volume, and open interest at a specific timestamp.
#[derive(Debug, Deserialize, Serialize)]
pub struct Snapshot {
    pub yes_price: i32,
    pub yes_bid: i32,
    pub yes_ask: i32,
    pub no_bid: i32,
    pub no_ask: i32,
    pub volume: i32,
    pub open_interest: i32,
    pub ts: i64,
}

/// Represents a trade executed on the Kalshi exchange.
///
/// A trade represents a completed transaction between a buyer and seller,
/// including the price, quantity, and timing of the execution.
#[derive(Debug, Deserialize, Serialize)]
pub struct Trade {
    pub trade_id: String,
    pub taker_side: String,
    pub ticker: String,
    pub count: i32,
    pub yes_price: i32,
    pub no_price: i32,
    pub created_time: String,
}

/// Represents the possible settlement results for a market.
///
/// Markets can settle in various ways depending on the outcome of the event
/// and the specific market rules.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettlementResult {
    Yes,
    No,
    #[serde(rename = "")]
    Void,
    #[serde(rename = "all_no")]
    AllNo,
    #[serde(rename = "all_yes")]
    AllYes,
}

/// Represents the possible statuses of a market.
///
/// Markets can be in various states throughout their lifecycle from creation to settlement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarketStatus {
    Open,
    Closed,
    Settled,
}

/// Filter for multivariate events (MVE) in market queries.
///
/// This enum allows filtering markets based on whether they belong to
/// multivariate event collections.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MveFilter {
    /// Only include markets that are part of multivariate events
    Only,
    /// Exclude markets that are part of multivariate events
    Exclude,
}

impl std::fmt::Display for MveFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MveFilter::Only => write!(f, "only"),
            MveFilter::Exclude => write!(f, "exclude"),
        }
    }
}

/// Represents candlestick data for a specific market in batch responses.
///
/// Contains the market ticker and its associated candlestick data.
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketCandlesticks {
    /// The market ticker identifier
    pub ticker: String,
    /// The candlestick data for this market
    pub candlesticks: Vec<Candle>,
}

/// Represents a settlement source for a series.
///
/// Settlement sources provide the data or methodology used to determine
/// the final outcome of markets in a series.
#[derive(Debug, Deserialize, Serialize)]
pub struct SettlementSource {
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

// -------- response wrappers --------

#[derive(Debug, Deserialize)]
struct MarketListResponse {
    cursor: Option<String>,
    markets: Vec<Market>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Used by serde for deserialization
struct SeriesListResponse {
    cursor: Option<String>,
    #[serde(default)]
    series: Vec<Series>,
}

#[derive(Debug, Deserialize)]
struct TradeListResponse {
    cursor: Option<String>,
    trades: Vec<Trade>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)] // cursor field reserved for future pagination support
struct CandlestickListResponse {
    cursor: Option<String>,
    candlesticks: Vec<Candle>,
}

#[derive(Debug, Deserialize)]
struct SingleMarketResponse {
    market: Market,
}

#[derive(Debug, Deserialize)]
struct SingleSeriesResponse {
    series: Series,
}

#[derive(Debug, Deserialize)]
struct OrderbookResponse {
    orderbook: Orderbook,
}

#[derive(Debug, Deserialize)]
struct BatchCandlestickResponse {
    markets: Vec<MarketCandlesticks>,
}
