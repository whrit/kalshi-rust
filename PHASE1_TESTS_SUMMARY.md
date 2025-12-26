# Phase 1 API Parity - TDD Test Suite Summary

## Overview

Comprehensive Test-Driven Development (TDD) tests have been written for Phase 1 of the Kalshi Rust API parity plan. These tests are designed to **FAIL** until the implementation is complete, following the red-green-refactor TDD cycle.

## Test File Location

`/Users/beckett/Projects/github_clones/kalshi-rust/kalshi/tests/phase1_api_parity_tests.rs`

## Running the Tests

```bash
cd kalshi
cargo test --test phase1_api_parity_tests
```

**Expected Result**: All tests will fail with compilation errors until Phase 1 implementation is complete.

## Test Coverage by Feature

### 1.1 create_order() - New Parameters (13 tests)

#### Enum Serialization Tests
- ✅ `test_time_in_force_enum_serialization` - Tests TimeInForce enum serializes to snake_case
- ✅ `test_time_in_force_enum_deserialization` - Tests TimeInForce roundtrip serialization
- ✅ `test_self_trade_prevention_type_serialization` - Tests SelfTradePreventionType enum
- ✅ `test_self_trade_prevention_type_deserialization` - Tests SelfTradePreventionType roundtrip

#### Payload Serialization Tests
- ✅ `test_create_order_payload_serialization_all_new_fields` - Verifies skip_serializing_if works for None
- ✅ `test_create_order_payload_serialization_with_new_fields` - Verifies all fields included when Some
- ✅ `test_order_creation_field_new_params` - Tests OrderCreationField struct with new parameters

#### Integration Tests
- ✅ `test_create_order_with_time_in_force` - End-to-end test with new parameters

**New Types Required**:
```rust
pub enum TimeInForce {
    FillOrKill,
    GoodTillCanceled,
    ImmediateOrCancel,
}

pub enum SelfTradePreventionType {
    TakerAtCross,
    Maker,
}
```

**New Parameters**:
- `time_in_force: Option<TimeInForce>`
- `post_only: Option<bool>`
- `reduce_only: Option<bool>`
- `self_trade_prevention_type: Option<SelfTradePreventionType>`
- `order_group_id: Option<String>`
- `cancel_order_on_pause: Option<bool>`

### 1.2 amend_order() - Complete Rewrite (7 tests)

#### Serialization/Deserialization Tests
- ✅ `test_amend_order_request_serialization` - Tests AmendOrderRequest serializes correctly
- ✅ `test_amend_order_request_optional_fields_skipped` - Verifies skip_serializing_if
- ✅ `test_amend_order_response_deserialization` - Tests AmendOrderResponse with old_order and order

#### Validation Tests
- ✅ `test_amend_order_price_validation_multiple_prices` - Validates error when >1 price field provided
- ✅ `test_amend_order_price_validation_zero_prices` - Validates 0 price fields allowed (count-only)
- ✅ `test_amend_order_price_validation_one_price` - Validates exactly 1 price field allowed

**New Types Required**:
```rust
struct AmendOrderRequest {
    ticker: String,
    side: Side,
    action: Action,
    client_order_id: String,
    updated_client_order_id: String,
    yes_price: Option<i32>,
    no_price: Option<i32>,
    yes_price_dollars: Option<String>,
    no_price_dollars: Option<String>,
    count: Option<i32>,
}

pub struct AmendOrderResponse {
    pub old_order: Order,
    pub order: Order,
}
```

**New Signature**:
```rust
pub async fn amend_order(
    &self,
    order_id: &str,
    ticker: &str,
    side: Side,
    action: Action,
    client_order_id: &str,
    updated_client_order_id: &str,
    yes_price: Option<i32>,
    no_price: Option<i32>,
    yes_price_dollars: Option<String>,
    no_price_dollars: Option<String>,
    count: Option<i32>,
) -> Result<AmendOrderResponse, KalshiError>
```

### 1.3 get_settlements() - New Filters (4 tests)

- ✅ `test_get_settlements_with_ticker_filter` - Tests ticker parameter
- ✅ `test_get_settlements_with_event_ticker_filter` - Tests event_ticker parameter
- ✅ `test_get_settlements_with_timestamp_filters` - Tests min_ts and max_ts parameters
- ✅ `test_get_settlements_with_all_new_filters` - Tests all parameters combined

**New Parameters**:
- `ticker: Option<String>`
- `event_ticker: Option<String>`
- `min_ts: Option<i64>`
- `max_ts: Option<i64>`

**Updated Signature**:
```rust
pub async fn get_settlements(
    &self,
    limit: Option<i64>,
    cursor: Option<String>,
    ticker: Option<String>,        // NEW
    event_ticker: Option<String>,  // NEW
    min_ts: Option<i64>,           // NEW
    max_ts: Option<i64>,           // NEW
) -> Result<(Option<String>, Vec<Settlement>), KalshiError>
```

### 1.4 get_positions() - count_filter (4 tests)

- ✅ `test_get_positions_with_count_filter_position` - Tests count_filter="position"
- ✅ `test_get_positions_with_count_filter_total_traded` - Tests count_filter="total_traded"
- ✅ `test_get_positions_with_count_filter_combined` - Tests comma-separated values
- ✅ `test_get_positions_without_count_filter` - Tests backward compatibility

**New Parameter**:
- `count_filter: Option<String>` - Accepts "position", "total_traded", or comma-separated

**Updated Signature**:
```rust
pub async fn get_positions(
    &self,
    limit: Option<i64>,
    cursor: Option<String>,
    settlement_status: Option<String>,
    ticker: Option<String>,
    event_ticker: Option<String>,
    count_filter: Option<String>,  // NEW
) -> Result<(Option<String>, Vec<EventPosition>, Vec<MarketPosition>), KalshiError>
```

### Integration Test (1 test)

- ✅ `test_phase1_integration_create_and_amend_order` - Full workflow: create with new params → amend

## Test Approach

### TDD Principles Applied

1. **Red Phase (Current)**: Tests written first, they fail with compilation errors
2. **Green Phase (Next)**: Implement minimal code to make tests pass
3. **Refactor Phase (Final)**: Optimize and clean up implementation

### Test Patterns Used

1. **Serialization/Deserialization Tests**: Verify JSON encoding matches API spec
2. **Validation Tests**: Ensure business logic validation (e.g., price field constraints)
3. **Integration Tests**: End-to-end workflows combining multiple features
4. **Edge Case Tests**: Zero values, combined filters, backward compatibility

### Authentication Handling

All integration tests use the common test infrastructure:
- Skip tests gracefully if credentials not available
- Use `common::skip_if_no_auth()` pattern
- Show helpful skip messages only once

### Error Handling

Tests verify both:
- **Success paths**: Valid requests succeed
- **Failure paths**: Invalid requests return appropriate `KalshiError::UserInputError`

## Current Test Status

**Total Tests**: 29
**Status**: ❌ All failing (expected - implementation not yet complete)

### Compilation Errors (Expected)

The following types and signatures don't exist yet:
- `TimeInForce` enum
- `SelfTradePreventionType` enum
- `AmendOrderRequest` struct
- `AmendOrderResponse` struct
- Updated `create_order()` signature (13 params → 19 params)
- Updated `amend_order()` signature (3 params → 11 params)
- Updated `get_settlements()` signature (2 params → 6 params)
- Updated `get_positions()` signature (5 params → 6 params)

## Next Steps for Implementation

1. **Define new enums** in `kalshi/src/portfolio/mod.rs`:
   - `TimeInForce`
   - `SelfTradePreventionType`

2. **Update `create_order()`**:
   - Add 6 new parameters to method signature
   - Update `CreateOrderPayload` struct
   - Update `OrderCreationField` struct
   - Update `batch_create_order()`

3. **Rewrite `amend_order()`**:
   - Create `AmendOrderRequest` struct
   - Create `AmendOrderResponse` struct
   - Implement price validation logic
   - Update method to POST to `/portfolio/orders/{order_id}/amend`

4. **Update `get_settlements()`**:
   - Add 4 new optional parameters
   - Add query parameter handling with `add_param!` macro

5. **Update `get_positions()`**:
   - Add `count_filter` parameter
   - Add query parameter handling

6. **Export new types** in `kalshi/src/lib.rs`:
   - Add `pub use portfolio::{TimeInForce, SelfTradePreventionType, AmendOrderResponse};`

## Running Tests After Implementation

Once implementation is complete:

```bash
# Run all Phase 1 tests
cargo test --test phase1_api_parity_tests

# Run specific test category
cargo test --test phase1_api_parity_tests test_time_in_force
cargo test --test phase1_api_parity_tests test_amend_order
cargo test --test phase1_api_parity_tests test_get_settlements
cargo test --test phase1_api_parity_tests test_get_positions

# Run with output visible
cargo test --test phase1_api_parity_tests -- --nocapture

# Run integration test
cargo test --test phase1_api_parity_tests test_phase1_integration
```

## Test Quality Metrics

- **Coverage**: All new parameters and types tested
- **Edge Cases**: Multiple price fields, zero price fields, combined filters
- **Backward Compatibility**: Tests verify existing behavior preserved
- **Error Handling**: Validates both success and failure paths
- **Documentation**: Each test has descriptive name and comments

## References

- API Parity Plan: `/Users/beckett/Projects/github_clones/kalshi-rust/API-Parity-Plan.md`
- Kalshi API Reference: `/Users/beckett/Projects/github_clones/kalshi-rust/kalshi-api-reference.md`
- Test Infrastructure: `/Users/beckett/Projects/github_clones/kalshi-rust/kalshi/tests/common/mod.rs`
