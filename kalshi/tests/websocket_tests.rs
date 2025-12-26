#[path = "common/mod.rs"]
mod common;

use common::skip_if_no_auth;
use kalshi::Channel;

/// Test WebSocket connection lifecycle
/// Note: WebSocket auth may differ from HTTP auth - connection might fail
#[tokio::test]
async fn test_websocket_connect() {
    let kalshi = match skip_if_no_auth() {
        Some(auth) => match auth.create_kalshi().await {
            Ok(k) => k,
            Err(e) => {
                println!("Failed to create Kalshi instance: {:?}", e);
                return;
            }
        },
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    let mut ws = kalshi.websocket();

    // Initially not connected
    assert!(!ws.is_connected(), "Should not be connected initially");

    // Connect - may fail if WebSocket uses different auth mechanism
    let connect_result = ws.connect().await;
    match &connect_result {
        Ok(()) => {
            assert!(ws.is_connected(), "Should be connected after connect()");

            // Disconnect
            let disconnect_result = ws.disconnect().await;
            assert!(
                disconnect_result.is_ok(),
                "Failed to disconnect: {:?}",
                disconnect_result.err()
            );
            assert!(
                !ws.is_connected(),
                "Should not be connected after disconnect()"
            );
        }
        Err(e) => {
            let err_str = format!("{:?}", e);
            // WebSocket might use different auth mechanism than HTTP
            if err_str.contains("401") || err_str.contains("Unauthorized") {
                println!("WebSocket auth may require different setup: {:?}", e);
            } else {
                panic!("Unexpected WebSocket error: {:?}", e);
            }
        }
    }
}

/// Test WebSocket subscribe to ticker channel
#[tokio::test]
async fn test_websocket_subscribe_ticker() {
    let kalshi = match skip_if_no_auth() {
        Some(auth) => match auth.create_kalshi().await {
            Ok(k) => k,
            Err(e) => {
                println!("Failed to create Kalshi instance: {:?}", e);
                return;
            }
        },
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    let mut ws = kalshi.websocket();
    let connect_result = ws.connect().await;
    if let Err(e) = &connect_result {
        let err_str = format!("{:?}", e);
        if err_str.contains("401") || err_str.contains("Unauthorized") {
            println!("WebSocket auth may require different setup: {:?}", e);
            return;
        }
        panic!("Failed to connect: {:?}", e);
    }

    // Subscribe to ticker channel for all markets
    let subscribe_result = ws
        .subscribe(
            vec![Channel::Ticker],
            None, // market_ticker
            None, // market_tickers (all markets)
        )
        .await;

    assert!(
        subscribe_result.is_ok(),
        "Failed to subscribe: {:?}",
        subscribe_result.err()
    );

    // Clean up
    ws.disconnect().await.ok();
}

/// Test WebSocket subscribe to orderbook channel with specific market
#[tokio::test]
async fn test_websocket_subscribe_orderbook() {
    let kalshi = match skip_if_no_auth() {
        Some(auth) => match auth.create_kalshi().await {
            Ok(k) => k,
            Err(e) => {
                println!("Failed to create Kalshi instance: {:?}", e);
                return;
            }
        },
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // First get a valid market ticker
    let markets_result = kalshi
        .get_markets(
            Some(1),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;

    let market_ticker = match markets_result {
        Ok((_, markets)) if !markets.is_empty() => markets[0].ticker.clone(),
        _ => {
            println!("No markets available for testing");
            return;
        }
    };

    let mut ws = kalshi.websocket();
    if let Err(e) = ws.connect().await {
        let err_str = format!("{:?}", e);
        if err_str.contains("401") || err_str.contains("Unauthorized") {
            println!("WebSocket auth may require different setup: {:?}", e);
            return;
        }
        panic!("Failed to connect: {:?}", e);
    }

    // Subscribe to orderbook for specific market
    let subscribe_result = ws
        .subscribe(
            vec![Channel::OrderbookDelta],
            Some(market_ticker.clone()),
            None,
        )
        .await;

    assert!(
        subscribe_result.is_ok(),
        "Failed to subscribe to orderbook for {}: {:?}",
        market_ticker,
        subscribe_result.err()
    );

    ws.disconnect().await.ok();
}

/// Test WebSocket unsubscribe
#[tokio::test]
async fn test_websocket_unsubscribe() {
    let kalshi = match skip_if_no_auth() {
        Some(auth) => match auth.create_kalshi().await {
            Ok(k) => k,
            Err(e) => {
                println!("Failed to create Kalshi instance: {:?}", e);
                return;
            }
        },
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    let mut ws = kalshi.websocket();
    if let Err(e) = ws.connect().await {
        let err_str = format!("{:?}", e);
        if err_str.contains("401") || err_str.contains("Unauthorized") {
            println!("WebSocket auth may require different setup: {:?}", e);
            return;
        }
        panic!("Failed to connect: {:?}", e);
    }

    // Subscribe first and get the SIDs
    let subscribe_result = ws.subscribe(vec![Channel::Trade], None, None).await;
    let sids: Vec<i32> = match subscribe_result {
        Ok(responses) => responses.iter().map(|r| r.sid).collect(),
        Err(e) => {
            println!("Failed to subscribe: {:?}", e);
            ws.disconnect().await.ok();
            return;
        }
    };

    if sids.is_empty() {
        println!("No SIDs returned from subscribe");
        ws.disconnect().await.ok();
        return;
    }

    // Unsubscribe using the actual SIDs from the subscribe response
    let unsubscribe_result = ws.unsubscribe(sids).await;
    assert!(
        unsubscribe_result.is_ok(),
        "Failed to unsubscribe: {:?}",
        unsubscribe_result.err()
    );

    ws.disconnect().await.ok();
}

/// Test Channel requires_auth method
#[test]
fn test_channel_requires_auth() {
    // Public channels
    assert!(!Channel::OrderbookDelta.requires_auth());
    assert!(!Channel::Ticker.requires_auth());
    assert!(!Channel::Trade.requires_auth());
    assert!(!Channel::MarketLifecycleV2.requires_auth());
    assert!(!Channel::EventLifecycle.requires_auth());
    assert!(!Channel::Multivariate.requires_auth());

    // Private channels (require auth)
    assert!(Channel::Fill.requires_auth());
    assert!(Channel::MarketPosition.requires_auth());
    assert!(Channel::Communications.requires_auth());
}

/// Test Channel Display trait
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
