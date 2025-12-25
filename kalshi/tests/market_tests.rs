#[path = "common/mod.rs"]
mod common;
use common::setup_auth_test;

#[tokio::test]
async fn test_get_exchange_status() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting exchange status
    let status = kalshi.get_exchange_status().await;
    assert!(status.is_ok(), "Failed to get exchange status: {:?}", status.err());
}

#[tokio::test]
async fn test_get_exchange_schedule() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting exchange schedule
    let schedule = kalshi.get_exchange_schedule().await;
    assert!(schedule.is_ok(), "Failed to get exchange schedule: {:?}", schedule.err());
}

#[tokio::test]
async fn test_get_events() {
    let kalshi = setup_auth_test().await.unwrap();
    
    // Test getting events with limit
    let result = kalshi.get_events(Some(5), None, None, None, None).await;
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
            println!("Series list test successful - cursor: {:?}, series count: {}", cursor, series.len());
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
    let result = kalshi.get_markets(None, None, None, None, None, None, None, None).await;
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

/// Test getting markets with new timestamp filter parameters
/// Tests min_created_ts, max_created_ts, min_settled_ts, max_settled_ts
#[tokio::test]
async fn test_get_markets_with_created_ts_filters() {
    let kalshi = setup_auth_test().await.unwrap();

    // Get a timestamp from 30 days ago
    let thirty_days_ago = chrono::Utc::now().timestamp() - (30 * 24 * 60 * 60);
    
    // Test getting markets created after a certain date
    let result = kalshi.get_markets(
        Some(10), None, None, None, None, None, None, None,
        Some(thirty_days_ago), // min_created_ts
        None,                  // max_created_ts
        None,                  // min_settled_ts
        None,                  // max_settled_ts
        None,                  // mve_filter
    ).await;
    assert!(result.is_ok(), "Failed to get markets with min_created_ts: {:?}", result.err());
}

/// Test getting markets with MveFilter::Exclude to exclude multivariate events
#[tokio::test]
async fn test_get_markets_with_mve_filter_exclude() {
    use kalshi::MveFilter;
    let kalshi = setup_auth_test().await.unwrap();

    // Test getting markets excluding multivariate events
    let result = kalshi.get_markets(
        Some(10), None, None, None, None, None, None, None,
        None, None, None, None,
        Some(MveFilter::Exclude),
    ).await;
    assert!(result.is_ok(), "Failed to get markets with mve_filter exclude: {:?}", result.err());
    
    let (_cursor, markets) = result.unwrap();
    println!("Got {} markets with mve_filter=exclude", markets.len());
}

/// Test getting markets with MveFilter::Only to get only multivariate events
#[tokio::test]
async fn test_get_markets_with_mve_filter_only() {
    use kalshi::MveFilter;
    let kalshi = setup_auth_test().await.unwrap();

    // Test getting only multivariate event markets
    let result = kalshi.get_markets(
        Some(10), None, None, None, None, None, None, None,
        None, None, None, None,
        Some(MveFilter::Only),
    ).await;
    assert!(result.is_ok(), "Failed to get markets with mve_filter only: {:?}", result.err());
    
    let (_cursor, markets) = result.unwrap();
    println!("Got {} markets with mve_filter=only", markets.len());
}

/// Test getting settled markets within a timestamp range
#[tokio::test]
async fn test_get_markets_with_settled_ts_filters() {
    let kalshi = setup_auth_test().await.unwrap();

    // Get timestamps for a 90-day range
    let now = chrono::Utc::now().timestamp();
    let ninety_days_ago = now - (90 * 24 * 60 * 60);
    
    // Test getting settled markets from the last 90 days
    let result = kalshi.get_markets(
        Some(10), None, None, None, 
        Some("settled".to_string()), // status filter for settled
        None, None, None,
        None,              // min_created_ts
        None,              // max_created_ts
        Some(ninety_days_ago), // min_settled_ts
        Some(now),         // max_settled_ts
        None,              // mve_filter
    ).await;
    assert!(result.is_ok(), "Failed to get markets with settled_ts filters: {:?}", result.err());
    
    let (_cursor, markets) = result.unwrap();
    println!("Got {} settled markets in the last 90 days", markets.len());
}
