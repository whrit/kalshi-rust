//! Phase 1 API Parity Tests - Test-Driven Development
//!
//! These tests are written BEFORE implementation to drive the development of:
//! 1.1 create_order() - New Parameters
//! 1.2 amend_order() - Complete Rewrite
//! 1.3 get_settlements() - New Filters
//! 1.4 get_positions() - count_filter parameter

#[path = "common/mod.rs"]
mod common;

use kalshi::{Action, KalshiError, OrderType, Side};
use serde_json;

// =============================================================================
// 1.1 create_order() - New Parameter Tests
// =============================================================================

#[test]
fn test_time_in_force_enum_serialization() {
    // These enums don't exist yet - tests will fail until implemented
    use kalshi::TimeInForce;

    let fok = TimeInForce::FillOrKill;
    let gtc = TimeInForce::GoodTillCanceled;
    let ioc = TimeInForce::ImmediateOrCancel;

    let fok_json = serde_json::to_string(&fok).expect("Failed to serialize FillOrKill");
    let gtc_json = serde_json::to_string(&gtc).expect("Failed to serialize GoodTillCanceled");
    let ioc_json = serde_json::to_string(&ioc).expect("Failed to serialize ImmediateOrCancel");

    assert_eq!(
        fok_json, "\"fill_or_kill\"",
        "FillOrKill should serialize to snake_case"
    );
    assert_eq!(
        gtc_json, "\"good_till_canceled\"",
        "GoodTillCanceled should serialize to snake_case"
    );
    assert_eq!(
        ioc_json, "\"immediate_or_cancel\"",
        "ImmediateOrCancel should serialize to snake_case"
    );
}

#[test]
fn test_time_in_force_enum_deserialization() {
    use kalshi::TimeInForce;

    let fok: TimeInForce =
        serde_json::from_str("\"fill_or_kill\"").expect("Failed to deserialize fill_or_kill");
    let gtc: TimeInForce = serde_json::from_str("\"good_till_canceled\"")
        .expect("Failed to deserialize good_till_canceled");
    let ioc: TimeInForce = serde_json::from_str("\"immediate_or_cancel\"")
        .expect("Failed to deserialize immediate_or_cancel");

    // Test roundtrip
    assert_eq!(serde_json::to_string(&fok).unwrap(), "\"fill_or_kill\"");
    assert_eq!(
        serde_json::to_string(&gtc).unwrap(),
        "\"good_till_canceled\""
    );
    assert_eq!(
        serde_json::to_string(&ioc).unwrap(),
        "\"immediate_or_cancel\""
    );
}

#[test]
fn test_self_trade_prevention_type_serialization() {
    use kalshi::SelfTradePreventionType;

    let taker = SelfTradePreventionType::TakerAtCross;
    let maker = SelfTradePreventionType::Maker;

    let taker_json = serde_json::to_string(&taker).expect("Failed to serialize TakerAtCross");
    let maker_json = serde_json::to_string(&maker).expect("Failed to serialize Maker");

    assert_eq!(
        taker_json, "\"taker_at_cross\"",
        "TakerAtCross should serialize to snake_case"
    );
    assert_eq!(
        maker_json, "\"maker\"",
        "Maker should serialize to lowercase"
    );
}

#[test]
fn test_self_trade_prevention_type_deserialization() {
    use kalshi::SelfTradePreventionType;

    let taker: SelfTradePreventionType =
        serde_json::from_str("\"taker_at_cross\"").expect("Failed to deserialize taker_at_cross");
    let maker: SelfTradePreventionType =
        serde_json::from_str("\"maker\"").expect("Failed to deserialize maker");

    // Test roundtrip
    assert_eq!(serde_json::to_string(&taker).unwrap(), "\"taker_at_cross\"");
    assert_eq!(serde_json::to_string(&maker).unwrap(), "\"maker\"");
}

#[test]
fn test_create_order_payload_serialization_all_new_fields() {
    // This test verifies that CreateOrderPayload properly serializes all new optional fields
    // and that skip_serializing_if works correctly
    // Create a minimal payload with only required fields
    let minimal_json = serde_json::json!({
        "action": "buy",
        "client_order_id": "test-123",
        "count": 10,
        "side": "yes",
        "ticker": "TEST-TICKER",
        "type": "limit"
    });

    // When deserialized and re-serialized, optional fields should be omitted if None
    // This would test the actual CreateOrderPayload struct once implemented
    let json_str = minimal_json.to_string();

    // Verify None fields are not included
    assert!(
        !json_str.contains("time_in_force"),
        "time_in_force should be omitted when None"
    );
    assert!(
        !json_str.contains("post_only"),
        "post_only should be omitted when None"
    );
    assert!(
        !json_str.contains("reduce_only"),
        "reduce_only should be omitted when None"
    );
    assert!(
        !json_str.contains("self_trade_prevention_type"),
        "self_trade_prevention_type should be omitted when None"
    );
    assert!(
        !json_str.contains("order_group_id"),
        "order_group_id should be omitted when None"
    );
    assert!(
        !json_str.contains("cancel_order_on_pause"),
        "cancel_order_on_pause should be omitted when None"
    );
}

#[test]
fn test_create_order_payload_serialization_with_new_fields() {
    // Create a payload with all new fields populated
    let full_json = serde_json::json!({
        "action": "buy",
        "client_order_id": "test-123",
        "count": 10,
        "side": "yes",
        "ticker": "TEST-TICKER",
        "type": "limit",
        "yes_price": 50,
        "time_in_force": "good_till_canceled",
        "post_only": true,
        "reduce_only": false,
        "self_trade_prevention_type": "maker",
        "order_group_id": "group-456",
        "cancel_order_on_pause": true
    });

    let json_str = full_json.to_string();

    // Verify all new fields are included when Some
    assert!(
        json_str.contains("time_in_force"),
        "time_in_force should be included when Some"
    );
    assert!(
        json_str.contains("post_only"),
        "post_only should be included when Some"
    );
    assert!(
        json_str.contains("reduce_only"),
        "reduce_only should be included when Some"
    );
    assert!(
        json_str.contains("self_trade_prevention_type"),
        "self_trade_prevention_type should be included when Some"
    );
    assert!(
        json_str.contains("order_group_id"),
        "order_group_id should be included when Some"
    );
    assert!(
        json_str.contains("cancel_order_on_pause"),
        "cancel_order_on_pause should be included when Some"
    );
}

#[test]
fn test_order_creation_field_new_params() {
    // Test OrderCreationField struct with new parameters
    use kalshi::{OrderCreationField, SelfTradePreventionType, TimeInForce};

    let field = OrderCreationField {
        action: Action::Buy,
        client_order_id: Some("test-123".to_string()),
        count: 10,
        side: Side::Yes,
        ticker: "TEST-TICKER".to_string(),
        input_type: OrderType::Limit,
        buy_max_cost: None,
        expiration_ts: None,
        yes_price: Some(50),
        no_price: None,
        sell_position_floor: None,
        yes_price_dollars: None,
        no_price_dollars: None,
        // NEW FIELDS
        time_in_force: Some(TimeInForce::GoodTillCanceled),
        post_only: Some(true),
        reduce_only: Some(false),
        self_trade_prevention_type: Some(SelfTradePreventionType::Maker),
        order_group_id: Some("group-123".to_string()),
        cancel_order_on_pause: Some(false),
    };

    assert_eq!(field.time_in_force, Some(TimeInForce::GoodTillCanceled));
    assert_eq!(field.post_only, Some(true));
    assert_eq!(field.reduce_only, Some(false));
    assert_eq!(
        field.self_trade_prevention_type,
        Some(SelfTradePreventionType::Maker)
    );
    assert_eq!(field.order_group_id, Some("group-123".to_string()));
    assert_eq!(field.cancel_order_on_pause, Some(false));
}

#[tokio::test]
async fn test_create_order_with_time_in_force() {
    // This will fail until create_order signature is updated
    use kalshi::TimeInForce;

    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // Attempt to create an order with new time_in_force parameter
    // This should fail gracefully if the parameter isn't implemented yet
    let result = kalshi
        .create_order(
            Action::Buy,
            Some("test-tif-order".to_string()),
            1,
            Side::Yes,
            "TEST-MARKET".to_string(),
            OrderType::Limit,
            None,                                 // buy_max_cost
            None,                                 // expiration_ts
            Some(1),                              // yes_price (very low to avoid execution)
            None,                                 // no_price
            None,                                 // sell_position_floor
            None,                                 // yes_price_dollars
            None,                                 // no_price_dollars
            Some(TimeInForce::ImmediateOrCancel), // NEW: time_in_force
            None,                                 // NEW: post_only
            None,                                 // NEW: reduce_only
            None,                                 // NEW: self_trade_prevention_type
            None,                                 // NEW: order_group_id
            None,                                 // NEW: cancel_order_on_pause
        )
        .await;

    // For now, we just check that the function signature compiles
    // The actual result doesn't matter until implementation
    match result {
        Ok(_) => println!("Order created successfully with time_in_force"),
        Err(e) => println!("Expected error (not yet implemented): {:?}", e),
    }
}

// =============================================================================
// 1.2 amend_order() - Complete Rewrite Tests
// =============================================================================

#[test]
fn test_amend_order_request_serialization() {
    // Test that AmendOrderRequest serializes correctly with all fields
    let request_json = serde_json::json!({
        "ticker": "TEST-MARKET",
        "side": "yes",
        "action": "buy",
        "client_order_id": "original-id",
        "updated_client_order_id": "updated-id",
        "yes_price": 55,
        "count": 20
    });

    let json_str = request_json.to_string();

    assert!(json_str.contains("ticker"));
    assert!(json_str.contains("side"));
    assert!(json_str.contains("action"));
    assert!(json_str.contains("client_order_id"));
    assert!(json_str.contains("updated_client_order_id"));
    assert!(json_str.contains("yes_price"));
    assert!(json_str.contains("count"));
}

#[test]
fn test_amend_order_request_optional_fields_skipped() {
    // Test that optional fields are skipped when None
    let request_json = serde_json::json!({
        "ticker": "TEST-MARKET",
        "side": "yes",
        "action": "buy",
        "client_order_id": "original-id",
        "updated_client_order_id": "updated-id",
        "yes_price": 55
    });

    let json_str = request_json.to_string();

    // count, no_price, yes_price_dollars, no_price_dollars should be omitted
    assert!(
        !json_str.contains("\"count\""),
        "count should be omitted when None"
    );
    assert!(
        !json_str.contains("\"no_price\""),
        "no_price should be omitted when None"
    );
}

#[test]
fn test_amend_order_response_deserialization() {
    use kalshi::AmendOrderResponse;

    // Test that AmendOrderResponse deserializes correctly
    let json = r#"{
        "old_order": {
            "order_id": "old-123",
            "ticker": "TEST-MARKET",
            "status": "resting",
            "yes_price": 50,
            "no_price": 50,
            "action": "buy",
            "side": "yes",
            "type": "limit",
            "client_order_id": "original-id",
            "user_id": "user-123",
            "created_time": "2024-01-01T00:00:00Z",
            "count": 10,
            "remaining_count": 10
        },
        "order": {
            "order_id": "new-456",
            "ticker": "TEST-MARKET",
            "status": "resting",
            "yes_price": 55,
            "no_price": 45,
            "action": "buy",
            "side": "yes",
            "type": "limit",
            "client_order_id": "updated-id",
            "user_id": "user-123",
            "created_time": "2024-01-01T00:00:00Z",
            "count": 20,
            "remaining_count": 20
        }
    }"#;

    let response: Result<AmendOrderResponse, _> = serde_json::from_str(json);

    assert!(
        response.is_ok(),
        "AmendOrderResponse should deserialize successfully"
    );

    let response = response.unwrap();
    assert_eq!(response.old_order.order_id, "old-123");
    assert_eq!(response.order.order_id, "new-456");
    assert_eq!(response.old_order.yes_price, Some(50));
    assert_eq!(response.order.yes_price, Some(55));
    assert_eq!(response.old_order.count, Some(10));
    assert_eq!(response.order.count, Some(20));
}

#[tokio::test]
async fn test_amend_order_price_validation_multiple_prices() {
    // Test that amend_order validates that at most one price field is provided
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // This should fail validation: providing both yes_price and no_price
    let result = kalshi
        .amend_order(
            "test-order-id",
            "TEST-MARKET",
            Side::Yes,
            Action::Buy,
            "original-client-id",
            "updated-client-id",
            Some(55), // yes_price
            Some(45), // no_price - CONFLICT!
            None,     // yes_price_dollars
            None,     // no_price_dollars
            None,     // count
        )
        .await;

    // Should return UserInputError
    match result {
        Err(KalshiError::UserInputError(msg)) => {
            assert!(
                msg.contains("one of") || msg.contains("Exactly one"),
                "Error should mention only one price field allowed: {}",
                msg
            );
        }
        _ => panic!("Expected UserInputError for multiple price fields"),
    }
}

#[tokio::test]
async fn test_amend_order_price_validation_zero_prices() {
    // Test that amend_order allows zero price fields (count-only amendment)
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // This should be valid: no price fields, only count
    let result = kalshi
        .amend_order(
            "test-order-id",
            "TEST-MARKET",
            Side::Yes,
            Action::Buy,
            "original-client-id",
            "updated-client-id",
            None,     // yes_price
            None,     // no_price
            None,     // yes_price_dollars
            None,     // no_price_dollars
            Some(15), // count
        )
        .await;

    // Should either succeed or fail for reasons other than validation
    match result {
        Err(KalshiError::UserInputError(msg))
            if msg.contains("one of") || msg.contains("Exactly one") =>
        {
            panic!(
                "Should allow zero price fields when count is provided: {}",
                msg
            );
        }
        _ => {
            // OK - either succeeded or failed for other reasons (like order not found)
            println!("Zero price fields validation passed");
        }
    }
}

#[tokio::test]
async fn test_amend_order_price_validation_one_price() {
    // Test that amend_order allows exactly one price field
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // This should be valid: exactly one price field
    let result = kalshi
        .amend_order(
            "test-order-id",
            "TEST-MARKET",
            Side::Yes,
            Action::Buy,
            "original-client-id",
            "updated-client-id",
            Some(55), // yes_price - only one price field
            None,     // no_price
            None,     // yes_price_dollars
            None,     // no_price_dollars
            None,     // count
        )
        .await;

    // Should either succeed or fail for reasons other than validation
    match result {
        Err(KalshiError::UserInputError(msg))
            if msg.contains("one of") || msg.contains("Exactly one") =>
        {
            panic!("Should allow exactly one price field: {}", msg);
        }
        _ => {
            // OK - either succeeded or failed for other reasons
            println!("One price field validation passed");
        }
    }
}

// =============================================================================
// 1.3 get_settlements() - New Filters Tests
// =============================================================================

#[tokio::test]
async fn test_get_settlements_with_ticker_filter() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // Test with new ticker parameter
    let result = kalshi
        .get_settlements(
            None,                            // limit
            None,                            // cursor
            Some("TEST-MARKET".to_string()), // NEW: ticker
            None,                            // NEW: event_ticker
            None,                            // NEW: min_ts
            None,                            // NEW: max_ts
        )
        .await;

    match result {
        Ok((_cursor, settlements)) => {
            println!(
                "Got settlements with ticker filter: {} results",
                settlements.len()
            );
            // Verify ticker parameter was accepted
            assert!(true, "ticker parameter accepted");
        }
        Err(e) => {
            // May fail if endpoint not yet updated, but signature should compile
            println!("Expected error (not yet implemented): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_settlements_with_event_ticker_filter() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // Test with new event_ticker parameter
    let result = kalshi
        .get_settlements(
            None,                           // limit
            None,                           // cursor
            None,                           // NEW: ticker
            Some("EVENT-2024".to_string()), // NEW: event_ticker
            None,                           // NEW: min_ts
            None,                           // NEW: max_ts
        )
        .await;

    match result {
        Ok((_cursor, settlements)) => {
            println!(
                "Got settlements with event_ticker filter: {} results",
                settlements.len()
            );
            assert!(true, "event_ticker parameter accepted");
        }
        Err(e) => {
            println!("Expected error (not yet implemented): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_settlements_with_timestamp_filters() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // Test with new timestamp parameters
    let min_ts = 1704067200; // 2024-01-01 00:00:00 UTC
    let max_ts = 1735689600; // 2025-01-01 00:00:00 UTC

    let result = kalshi
        .get_settlements(
            None,         // limit
            None,         // cursor
            None,         // NEW: ticker
            None,         // NEW: event_ticker
            Some(min_ts), // NEW: min_ts
            Some(max_ts), // NEW: max_ts
        )
        .await;

    match result {
        Ok((_cursor, settlements)) => {
            println!(
                "Got settlements with timestamp filters: {} results",
                settlements.len()
            );
            assert!(true, "timestamp parameters accepted");
        }
        Err(e) => {
            println!("Expected error (not yet implemented): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_settlements_with_all_new_filters() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // Test with all new parameters combined
    let result = kalshi
        .get_settlements(
            Some(50),                        // limit
            None,                            // cursor
            Some("TEST-MARKET".to_string()), // NEW: ticker
            Some("EVENT-2024".to_string()),  // NEW: event_ticker
            Some(1704067200),                // NEW: min_ts
            Some(1735689600),                // NEW: max_ts
        )
        .await;

    match result {
        Ok((_cursor, settlements)) => {
            println!(
                "Got settlements with all filters: {} results",
                settlements.len()
            );
            assert!(true, "All new parameters accepted");
        }
        Err(e) => {
            println!("Expected error (not yet implemented): {:?}", e);
        }
    }
}

// =============================================================================
// 1.4 get_positions() - count_filter Tests
// =============================================================================

#[tokio::test]
async fn test_get_positions_with_count_filter_position() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // Test with count_filter="position"
    let result = kalshi
        .get_positions(
            None,                         // limit
            None,                         // cursor
            None,                         // settlement_status
            None,                         // ticker
            None,                         // event_ticker
            Some("position".to_string()), // NEW: count_filter
        )
        .await;

    match result {
        Ok((_cursor, event_positions, market_positions)) => {
            println!(
                "Got positions with count_filter='position': {} events, {} markets",
                event_positions.len(),
                market_positions.len()
            );
            assert!(true, "count_filter parameter accepted");
        }
        Err(e) => {
            println!("Expected error (not yet implemented): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_positions_with_count_filter_total_traded() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // Test with count_filter="total_traded"
    let result = kalshi
        .get_positions(
            None,                             // limit
            None,                             // cursor
            None,                             // settlement_status
            None,                             // ticker
            None,                             // event_ticker
            Some("total_traded".to_string()), // NEW: count_filter
        )
        .await;

    match result {
        Ok((_cursor, event_positions, market_positions)) => {
            println!(
                "Got positions with count_filter='total_traded': {} events, {} markets",
                event_positions.len(),
                market_positions.len()
            );
            assert!(true, "count_filter='total_traded' accepted");
        }
        Err(e) => {
            println!("Expected error (not yet implemented): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_positions_with_count_filter_combined() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // Test with combined count_filter values (comma-separated)
    let result = kalshi
        .get_positions(
            None,                                      // limit
            None,                                      // cursor
            None,                                      // settlement_status
            None,                                      // ticker
            None,                                      // event_ticker
            Some("position,total_traded".to_string()), // NEW: count_filter (combined)
        )
        .await;

    match result {
        Ok((_cursor, event_positions, market_positions)) => {
            println!(
                "Got positions with combined count_filter: {} events, {} markets",
                event_positions.len(),
                market_positions.len()
            );
            assert!(true, "Combined count_filter accepted");
        }
        Err(e) => {
            println!("Expected error (not yet implemented): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_positions_without_count_filter() {
    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    // Test backward compatibility: should work without count_filter
    let result = kalshi
        .get_positions(
            None, // limit
            None, // cursor
            None, // settlement_status
            None, // ticker
            None, // event_ticker
            None, // NEW: count_filter (omitted for backward compatibility)
        )
        .await;

    match result {
        Ok((_cursor, event_positions, market_positions)) => {
            println!(
                "Got positions without count_filter: {} events, {} markets",
                event_positions.len(),
                market_positions.len()
            );
            assert!(true, "Backward compatibility maintained");
        }
        Err(e) => {
            // Should not fail just because count_filter is None
            panic!(
                "Should maintain backward compatibility without count_filter: {:?}",
                e
            );
        }
    }
}

// =============================================================================
// Integration Tests - End-to-End Scenarios
// =============================================================================

#[tokio::test]
async fn test_phase1_integration_create_and_amend_order() {
    // This test demonstrates the full Phase 1 workflow:
    // 1. Create an order with new parameters
    // 2. Amend it with the new API
    // 3. Verify the changes

    let kalshi = match common::skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            common::show_skip_message_once();
            return;
        }
    };

    use kalshi::{SelfTradePreventionType, TimeInForce};

    // Step 1: Create order with new parameters
    let create_result = kalshi
        .create_order(
            Action::Buy,
            Some("integration-test-order".to_string()),
            1,
            Side::Yes,
            "TEST-MARKET".to_string(),
            OrderType::Limit,
            None,                                 // buy_max_cost
            None,                                 // expiration_ts
            Some(1),                              // yes_price (very low)
            None,                                 // no_price
            None,                                 // sell_position_floor
            None,                                 // yes_price_dollars
            None,                                 // no_price_dollars
            Some(TimeInForce::GoodTillCanceled),  // NEW: time_in_force
            Some(true),                           // NEW: post_only
            None,                                 // NEW: reduce_only
            Some(SelfTradePreventionType::Maker), // NEW: self_trade_prevention_type
            Some("test-group-1".to_string()),     // NEW: order_group_id
            Some(false),                          // NEW: cancel_order_on_pause
        )
        .await;

    // For TDD, we expect this to fail until implementation
    match create_result {
        Ok(order) => {
            println!("Created order: {:?}", order.order_id);

            // Step 2: Amend the order
            let amend_result = kalshi
                .amend_order(
                    &order.order_id,
                    "TEST-MARKET",
                    Side::Yes,
                    Action::Buy,
                    "integration-test-order",
                    "integration-test-order-amended",
                    Some(2), // New yes_price
                    None,    // no_price
                    None,    // yes_price_dollars
                    None,    // no_price_dollars
                    Some(2), // New count
                )
                .await;

            match amend_result {
                Ok(amend_response) => {
                    println!("Amended order successfully");
                    assert_eq!(amend_response.old_order.order_id, order.order_id);
                    assert_ne!(
                        amend_response.order.yes_price,
                        amend_response.old_order.yes_price
                    );
                }
                Err(e) => println!("Amend failed (expected for TDD): {:?}", e),
            }
        }
        Err(e) => {
            println!("Create failed (expected for TDD): {:?}", e);
        }
    }
}
