//! Tests for events module functionality including multivariate events.

#[path = "common/mod.rs"]
mod common;
use common::setup_auth_test;

/// Test getting multivariate events without filters
#[tokio::test]
async fn test_get_multivariate_events_basic() {
    let kalshi = setup_auth_test().await.unwrap();

    let result = kalshi
        .get_multivariate_events(Some(10), None, None, None, None)
        .await;

    assert!(
        result.is_ok(),
        "Failed to get multivariate events: {:?}",
        result.err()
    );

    let (cursor, events) = result.unwrap();
    println!(
        "Got {} multivariate events, cursor: {:?}",
        events.len(),
        cursor
    );
}

/// Test get_multivariate_events with series_ticker filter
#[tokio::test]
async fn test_get_multivariate_events_with_series_ticker() {
    let kalshi = setup_auth_test().await.unwrap();

    // First get the series list to find a valid series ticker
    let series_result = kalshi.get_series_list(None, None, None, None).await;
    if let Ok((_, series_list)) = series_result {
        if let Some(first_series) = series_list.first() {
            if let Some(ref ticker) = first_series.ticker {
                let result = kalshi
                    .get_multivariate_events(Some(5), None, Some(ticker.clone()), None, None)
                    .await;

                assert!(
                    result.is_ok(),
                    "Failed to get multivariate events with series_ticker: {:?}",
                    result.err()
                );
            }
        }
    }
}

/// Test get_multivariate_events with with_nested_markets=true
#[tokio::test]
async fn test_get_multivariate_events_with_nested_markets() {
    let kalshi = setup_auth_test().await.unwrap();

    let result = kalshi
        .get_multivariate_events(Some(5), None, None, None, Some(true))
        .await;

    assert!(
        result.is_ok(),
        "Failed to get multivariate events with nested markets: {:?}",
        result.err()
    );
}

/// Test that using both series_ticker and collection_ticker fails
#[tokio::test]
async fn test_get_multivariate_events_invalid_both_filters() {
    let kalshi = setup_auth_test().await.unwrap();

    let result = kalshi
        .get_multivariate_events(
            Some(5),
            None,
            Some("SERIES-TEST".to_string()),
            Some("COLLECTION-TEST".to_string()),
            None,
        )
        .await;

    assert!(
        result.is_err(),
        "Should fail when both series_ticker and collection_ticker provided"
    );

    if let Err(kalshi::KalshiError::UserInputError(msg)) = result {
        assert!(
            msg.contains("series_ticker") || msg.contains("collection_ticker"),
            "Error should mention the conflicting parameters"
        );
    } else {
        panic!("Expected UserInputError but got: {:?}", result);
    }
}

/// Test get_multivariate_events pagination with cursor
#[tokio::test]
async fn test_get_multivariate_events_pagination() {
    let kalshi = setup_auth_test().await.unwrap();

    // First request with small limit
    let result = kalshi
        .get_multivariate_events(Some(2), None, None, None, None)
        .await;

    assert!(
        result.is_ok(),
        "Failed to get multivariate events: {:?}",
        result.err()
    );

    let (cursor, first_events) = result.unwrap();

    // If we got a cursor, try to get the next page
    if let Some(cursor_val) = cursor {
        let second_result = kalshi
            .get_multivariate_events(Some(2), Some(cursor_val), None, None, None)
            .await;

        assert!(
            second_result.is_ok(),
            "Failed to get second page of multivariate events: {:?}",
            second_result.err()
        );

        let (_, second_events) = second_result.unwrap();
        println!(
            "First page: {} events, Second page: {} events",
            first_events.len(),
            second_events.len()
        );
    }
}
