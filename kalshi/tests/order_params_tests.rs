//! Tests for Phase 1.1 and 1.2
#[path = "common/mod.rs"]
mod common;

use kalshi::{Action, AmendOrderResponse, OrderType, SelfTradePreventionType, Side, TimeInForce};

#[test]
fn test_time_in_force_enum_serialization() {
    let fok = TimeInForce::FillOrKill;
    let gtc = TimeInForce::GoodTillCanceled;
    let ioc = TimeInForce::ImmediateOrCancel;
    let fok_json = serde_json::to_string(&fok).unwrap();
    let gtc_json = serde_json::to_string(&gtc).unwrap();
    let ioc_json = serde_json::to_string(&ioc).unwrap();
    assert_eq!(fok_json, "\"fill_or_kill\"");
    assert_eq!(gtc_json, "\"good_till_canceled\"");
    assert_eq!(ioc_json, "\"immediate_or_cancel\"");
}

#[test]
fn test_self_trade_prevention_type_serialization() {
    let taker = SelfTradePreventionType::TakerAtCross;
    let maker = SelfTradePreventionType::Maker;
    let taker_json = serde_json::to_string(&taker).unwrap();
    let maker_json = serde_json::to_string(&maker).unwrap();
    assert_eq!(taker_json, "\"taker_at_cross\"");
    assert_eq!(maker_json, "\"maker\"");
}

#[test]
fn test_amend_order_response_deserialization() {
    let json = r#"{"old_order": {"order_id": "old-123", "ticker": "TEST", "status": "resting", "yes_price": 50, "no_price": 50, "action": "buy", "side": "yes", "type": "limit", "client_order_id": "orig"}, "order": {"order_id": "new-456", "ticker": "TEST", "status": "resting", "yes_price": 55, "no_price": 45, "action": "buy", "side": "yes", "type": "limit", "client_order_id": "new"}}"#;
    let response: AmendOrderResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.old_order.order_id, "old-123");
    assert_eq!(response.order.order_id, "new-456");
}

#[test]
fn test_order_creation_field_new_params() {
    use kalshi::OrderCreationField;
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
        time_in_force: Some(TimeInForce::GoodTillCanceled),
        post_only: Some(true),
        reduce_only: None,
        self_trade_prevention_type: Some(SelfTradePreventionType::Maker),
        order_group_id: Some("group-123".to_string()),
        cancel_order_on_pause: Some(false),
    };
    assert_eq!(field.time_in_force, Some(TimeInForce::GoodTillCanceled));
    assert_eq!(field.post_only, Some(true));
}
