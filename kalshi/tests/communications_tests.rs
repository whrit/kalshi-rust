#[path = "common/mod.rs"]
mod common;

// NOTE: The following test is for Phase 2 feature (get_communications_id)
// It has been commented out until Phase 2.3 is implemented.
// See API-Parity-Plan.md Phase 2.3 for details.

// /// Test getting the user public communications ID.
// /// This endpoint returns a communications ID that is used to identify
// /// the user in communications with other traders.
// #[tokio::test]
// async fn test_get_communications_id() {
//     use common::setup_auth_test;
//     let kalshi = setup_auth_test().await.unwrap();
//     let result = kalshi.get_communications_id().await;
//     assert!(result.is_ok(), "Failed to get communications ID: {:?}", result.err());
//     let comm_id = result.unwrap();
//     assert!(!comm_id.is_empty(), "Communications ID should not be empty");
//     println!("Communications ID: {}", comm_id);
// }
