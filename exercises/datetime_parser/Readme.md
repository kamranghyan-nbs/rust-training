cargo run --bin main
Run all tests:
cargo test
Run specific test types:
cargo test --lib                    # Unit tests only
cargo test --test integration_tests # Integration tests only
Generate documentation:
cargo doc --open