# Kalshi Rust - Testing Guide

## Test Organization

The test suite is organized into several files:

```
kalshi/tests/
├── common/mod.rs                    # Shared test utilities
├── auth_tests.rs                    # Authentication tests
├── basic_tests.rs                   # Basic functionality tests
├── exchange_tests.rs                # Exchange endpoint tests
├── market_tests.rs                  # Market data tests
├── portfolio_tests.rs               # Portfolio/trading tests
├── order_params_tests.rs            # Order parameter validation tests
└── phase1_api_parity_tests.rs       # Phase 1 API parity TDD tests (NEW)
```

## Running Tests

### Run All Tests
```bash
cd kalshi
cargo test
```

### Run Specific Test File
```bash
cargo test --test phase1_api_parity_tests
cargo test --test portfolio_tests
cargo test --test market_tests
```

### Run Specific Test by Name
```bash
cargo test test_time_in_force_enum_serialization
cargo test test_amend_order_price_validation
```

### Run Tests with Output Visible
```bash
cargo test -- --nocapture
cargo test --test phase1_api_parity_tests -- --nocapture
```

### Run Library Tests Only
```bash
cargo test --lib
```

## Authentication Setup for Tests

Many tests require Kalshi API credentials. Set up environment variables:

```bash
export KALSHI_DEMO_API_KEY="your-demo-api-key"
export KALSHI_DEMO_PEM_PATH="/path/to/your/private-key.pem"
export KALSHI_TEST_ENV="demo"  # or "prod"
```

Or create a `.env` file:

```env
KALSHI_DEMO_API_KEY=your-demo-api-key
KALSHI_DEMO_PEM_PATH=/path/to/your/private-key.pem
KALSHI_TEST_ENV=demo
```

Or specify a custom env file:

```bash
export KALSHI_ENV_FILE=/path/to/custom.env
cargo test
```

## Test Categories

### Unit Tests
Test individual functions and data structures in isolation.
- Example: `test_time_in_force_enum_serialization`

### Integration Tests
Test complete workflows and API interactions.
- Example: `test_phase1_integration_create_and_amend_order`

### Validation Tests
Test business logic validation and error handling.
- Example: `test_amend_order_price_validation_multiple_prices`

## Phase 1 TDD Tests

The Phase 1 tests follow Test-Driven Development principles:

### Current Status
- ❌ **All tests FAILING** (expected - implementation not yet complete)
- Tests written BEFORE implementation
- Compilation errors are expected

### Running Phase 1 Tests
```bash
cd kalshi
cargo test --test phase1_api_parity_tests 2>&1 | less
```

### What's Being Tested
1. **create_order()** - New trading parameters (6 new parameters)
2. **amend_order()** - Complete API rewrite (11 parameters)
3. **get_settlements()** - New filter parameters (4 new filters)
4. **get_positions()** - count_filter parameter

### Test Coverage
- 29 total tests
- Enum serialization/deserialization
- Payload serialization with skip_serializing_if
- Validation logic (e.g., price field constraints)
- Integration workflows
- Edge cases and backward compatibility

## Test Utilities

### Common Test Functions

```rust
use common::{setup_auth_test, skip_if_no_auth, show_skip_message_once};

// Require authentication (panics if not available)
#[tokio::test]
async fn my_test() {
    let kalshi = setup_auth_test().await.unwrap();
    // ... test code
}

// Skip gracefully if no auth
#[tokio::test]
async fn my_test_skip_safe() {
    let kalshi = match skip_if_no_auth() {
        Some(auth) => auth.create_kalshi().await.unwrap(),
        None => {
            show_skip_message_once();
            return;
        }
    };
    // ... test code
}
```

## Debugging Tests

### Run Single Test with Full Output
```bash
cargo test test_name -- --nocapture --test-threads=1
```

### Show Test Names Without Running
```bash
cargo test -- --list
```

### Run Failed Tests Only
```bash
cargo test -- --failed
```

## CI/CD Integration

Tests are designed to run in CI/CD environments:

```yaml
# Example GitHub Actions workflow
- name: Run tests
  run: |
    cd kalshi
    cargo test --all-features
  env:
    KALSHI_DEMO_API_KEY: ${{ secrets.KALSHI_DEMO_API_KEY }}
    KALSHI_DEMO_PEM_PATH: ${{ secrets.KALSHI_DEMO_PEM_PATH }}
    KALSHI_TEST_ENV: demo
```

## Test Best Practices

1. **Isolation**: Each test should be independent
2. **Cleanup**: Tests should not leave side effects
3. **Naming**: Use descriptive test names (test_what_when_expected)
4. **Documentation**: Add comments for complex test logic
5. **Speed**: Keep tests fast; use mocks for external dependencies
6. **Determinism**: Tests should always produce same results

## Troubleshooting

### "Authentication required but not available"
- Set KALSHI_DEMO_API_KEY and KALSHI_DEMO_PEM_PATH
- Check that PEM file exists and is readable

### "Failed to create Kalshi instance"
- Verify API key is valid
- Check PEM file format (should be RSA private key)
- Ensure network connectivity to Kalshi API

### Compilation Errors in phase1_api_parity_tests
- **Expected!** These tests are written for features not yet implemented
- Errors will disappear as Phase 1 implementation progresses

## Next Steps

After implementing Phase 1 features:

1. Run tests to verify implementation:
   ```bash
   cargo test --test phase1_api_parity_tests
   ```

2. All tests should pass

3. Run full test suite:
   ```bash
   cargo test
   ```

4. Update API-Parity-Plan.md to mark Phase 1 as complete

## Resources

- Phase 1 Test Summary: `PHASE1_TESTS_SUMMARY.md`
- API Parity Plan: `API-Parity-Plan.md`
- Kalshi API Reference: `kalshi-api-reference.md`
