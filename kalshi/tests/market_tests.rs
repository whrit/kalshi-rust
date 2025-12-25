#[path = "common/mod.rs"]
mod common;
use common::setup_auth_test;

#[tokio::test]
async fn test_get_exchange_status() {
    let kalshi = setup_auth_test().await.unwrap();

    // Test getting exchange status
    let status = kalshi.get_exchange_status().await;
    assert!(
        status.is_ok(),
        "Failed to get exchange status: {:?}",
        status.err()
    );
}

#[tokio::test]
async fn test_get_exchange_schedule() {
    let kalshi = setup_auth_test().await.unwrap();

    // Test getting exchange schedule
    let schedule = kalshi.get_exchange_schedule().await;
    assert!(
        schedule.is_ok(),
        "Failed to get exchange schedule: {:?}",
        schedule.err()
    );
}

#[tokio::test]
async fn test_get_events() {
    let kalshi = setup_auth_test().await.unwrap();

    // Test getting events with limit
    let result = kalshi
        .get_events(Some(5), None, None, None, None, None, None)
        .await;
    assert!(result.is_ok(), "Failed to get events: {:?}", result.err());

    let (_cursor, events) = result.unwrap();
    assert!(events.len() <= 5, "Should return at most 5 events");
}

#[tokio::test]
async fn test_get_series_list() {
    let kalshi = setup_auth_test().await.unwrap();

    // Test getting series list
    let result = kalshi.get_series_list(None, None, None, None).await;
    match result {
        Ok((cursor, series)) => {
            println!(
                "Series list test successful - cursor: {:?}, series count: {}",
                cursor,
                series.len()
            );
            // Even if no series are returned, that's still a valid response
            assert!(true, "Successfully got series list");
        }
        Err(e) => {
            println!("Series list error: {:?}", e);
            assert!(false, "Failed to get series list: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_markets() {
    let kalshi = setup_auth_test().await.unwrap();

    // Test getting markets
    let result = kalshi
        .get_markets(
            None, None, None, None, None, None, None, None, None, None, None, None, None,
        )
        .await;
    assert!(result.is_ok(), "Failed to get markets: {:?}", result.err());

    let (_cursor, markets) = result.unwrap();
    assert!(!markets.is_empty(), "Should return at least one market");
}

#[tokio::test]
async fn test_get_trades() {
    let kalshi = setup_auth_test().await.unwrap();

    // Test getting trades
    let result = kalshi.get_trades(None, None, None, None, None).await;
    assert!(result.is_ok(), "Failed to get trades: {:?}", result.err());
}

// Phase 2 tests for timestamp filters, mve_filter, and batch candlesticks

/// Test getting batch market candlesticks (Phase 2.1)
#[tokio::test]
async fn test_batch_get_market_candlesticks() {
    let kalshi = setup_auth_test().await.unwrap();

    // Get some markets first to get valid tickers
    let (_, markets) = kalshi
        .get_markets(
            Some(3),
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
        .await
        .unwrap();

    if markets.is_empty() {
        println!("No markets available for testing");
        return;
    }

    let tickers: Vec<String> = markets.iter().take(2).map(|m| m.ticker.clone()).collect();
    let now = chrono::Utc::now().timestamp();
    let start_ts = now - 86400; // 1 day ago

    let result = kalshi
        .batch_get_market_candlesticks(
            tickers.clone(),
            start_ts,
            now,
            60, // hourly candles
            None,
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to get batch candlesticks: {:?}",
        result.err()
    );

    let candlesticks_data = result.unwrap();
    // The result should be a Vec of MarketCandlesticks, one per ticker
    println!(
        "Received {} market candlestick records",
        candlesticks_data.len()
    );
}

/// Test batch candlesticks with include_latest_before_start parameter
#[tokio::test]
async fn test_batch_get_market_candlesticks_with_include_latest() {
    let kalshi = setup_auth_test().await.unwrap();

    // Get some markets first to get valid tickers
    let (_, markets) = kalshi
        .get_markets(
            Some(2),
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
        .await
        .unwrap();

    if markets.is_empty() {
        println!("No markets available for testing");
        return;
    }

    let tickers: Vec<String> = markets.iter().take(2).map(|m| m.ticker.clone()).collect();
    let now = chrono::Utc::now().timestamp();
    let start_ts = now - 86400; // 1 day ago

    let result = kalshi
        .batch_get_market_candlesticks(
            tickers.clone(),
            start_ts,
            now,
            60, // hourly candles
            Some(true),
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to get batch candlesticks with include_latest: {:?}",
        result.err()
    );
}

/// Test get_markets with timestamp filters (Phase 2.4)
#[tokio::test]
async fn test_get_markets_with_created_ts_filters() {
    let kalshi = setup_auth_test().await.unwrap();

    let now = chrono::Utc::now().timestamp();
    let one_week_ago = now - (7 * 86400);

    let result = kalshi
        .get_markets(
            Some(10),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(one_week_ago), // min_created_ts
            Some(now),          // max_created_ts
            None,               // min_settled_ts
            None,               // max_settled_ts
            None,               // mve_filter
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to get markets with ts filters: {:?}",
        result.err()
    );
}

/// Test get_markets with MveFilter::Exclude
#[tokio::test]
async fn test_get_markets_with_mve_filter_exclude() {
    use kalshi::MveFilter;
    let kalshi = setup_auth_test().await.unwrap();

    let result = kalshi
        .get_markets(
            Some(10),
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
            Some(MveFilter::Exclude),
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to get markets with mve_filter exclude: {:?}",
        result.err()
    );
}

/// Test get_markets with MveFilter::Only
#[tokio::test]
async fn test_get_markets_with_mve_filter_only() {
    use kalshi::MveFilter;
    let kalshi = setup_auth_test().await.unwrap();

    let result = kalshi
        .get_markets(
            Some(10),
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
            Some(MveFilter::Only),
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to get markets with mve_filter only: {:?}",
        result.err()
    );
}

/// Test get_markets with settled timestamp filters
#[tokio::test]
async fn test_get_markets_with_settled_ts_filters() {
    let kalshi = setup_auth_test().await.unwrap();

    let now = chrono::Utc::now().timestamp();
    let one_month_ago = now - (30 * 86400);

    let result = kalshi
        .get_markets(
            Some(10),
            None,
            None,
            None,
            Some("settled".to_string()), // Only look at settled markets
            None,
            None,
            None,
            None,
            None,
            Some(one_month_ago), // min_settled_ts
            Some(now),           // max_settled_ts
            None,
        )
        .await;

    assert!(
        result.is_ok(),
        "Failed to get markets with settled ts filters: {:?}",
        result.err()
    );
}
