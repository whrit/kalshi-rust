#[path = "common/mod.rs"]
mod common;
use common::setup_auth_test;

/// Test getting the user public communications ID.
/// This endpoint returns a communications ID that is used to identify
/// the user in communications with other traders.
#[tokio::test]
async fn test_get_communications_id() {
    let kalshi = setup_auth_test().await.unwrap();
    let result = kalshi.get_communications_id().await;
    assert!(result.is_ok(), "Failed to get communications ID: {:?}", result.err());
    let comm_id = result.unwrap();
    assert!(!comm_id.is_empty(), "Communications ID should not be empty");
    println!("Communications ID: {}", comm_id);
}
