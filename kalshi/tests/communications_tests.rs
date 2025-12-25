#[path = "common/mod.rs"]
mod common;

// ========== Task 2.3: get_communications_id() ==========

/// Test getting the user public communications ID.
/// This endpoint returns a communications ID that is used to identify
/// the user in communications with other traders.
#[tokio::test]
async fn test_get_communications_id() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    let result = kalshi.get_communications_id().await;
    assert!(
        result.is_ok(),
        "Failed to get communications ID: {:?}",
        result.err()
    );
    let comm_id = result.unwrap();
    assert!(!comm_id.is_empty(), "Communications ID should not be empty");
    println!("Communications ID: {}", comm_id);
}

// ========== Task 3.1: create_rfq() ==========

/// Test create_rfq API structure (will likely fail on API without valid market)
#[tokio::test]
async fn test_create_rfq_signature() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // This tests the method signature exists, actual API call may fail
    // due to invalid market ticker
    let result = kalshi
        .create_rfq("INVALID-MARKET", false, Some(10), None, None, None)
        .await;

    // We expect this to fail due to invalid market ticker, but method should exist
    assert!(result.is_err(), "Should fail with invalid market ticker");
    println!("Expected error: {:?}", result.err());
}

// ========== Task 3.2: create_quote() ==========

/// Test create_quote API structure (will fail without valid RFQ)
#[tokio::test]
async fn test_create_quote_signature() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // This tests the method signature exists, actual API call may fail
    // due to invalid RFQ ID
    let result = kalshi
        .create_quote("INVALID-RFQ-ID", "0.5000", "0.5000", false)
        .await;

    // We expect this to fail due to invalid RFQ ID, but method should exist
    assert!(result.is_err(), "Should fail with invalid RFQ ID");
    println!("Expected error: {:?}", result.err());
}

// ========== Task 3.3: get_rfqs() and get_quotes() with pagination ==========

/// Test getting RFQs with pagination
#[tokio::test]
async fn test_get_rfqs_with_pagination() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    let result = kalshi
        .get_rfqs(
            None,     // cursor
            None,     // event_ticker
            None,     // market_ticker
            Some(10), // limit
            None,     // status
            None,     // creator_user_id
        )
        .await;

    assert!(result.is_ok(), "Failed to get RFQs: {:?}", result.err());
    let (cursor, rfqs) = result.unwrap();
    println!("Got {} RFQs, cursor: {:?}", rfqs.len(), cursor);
}

/// Test getting quotes with pagination
#[tokio::test]
async fn test_get_quotes_with_pagination() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    let result = kalshi
        .get_quotes(
            None,     // cursor
            None,     // event_ticker
            None,     // market_ticker
            Some(10), // limit
            None,     // status
            None,     // quote_creator_user_id
            None,     // rfq_creator_user_id
            None,     // rfq_id
        )
        .await;

    assert!(result.is_ok(), "Failed to get quotes: {:?}", result.err());
    let (cursor, quotes) = result.unwrap();
    println!("Got {} quotes, cursor: {:?}", quotes.len(), cursor);
}

// ========== Task 3.4: accept_quote() with accepted_side ==========

/// Test accept_quote API structure (will fail without valid quote)
#[tokio::test]
async fn test_accept_quote_signature() {
    use kalshi::Side;

    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // This tests the method signature exists, actual API call may fail
    // due to invalid quote ID
    let result = kalshi.accept_quote("INVALID-QUOTE-ID", Side::Yes).await;

    // We expect this to fail due to invalid quote ID, but method should exist
    assert!(result.is_err(), "Should fail with invalid quote ID");
    println!("Expected error: {:?}", result.err());
}
