set positional-arguments
alias f := fmt-native-fix
alias l := lints
alias t := tests

# Fetches the latest Chainlist JSON file and updates the local file.
update:
  @echo "Fetching latest Chainlist JSON file..."
  @curl -s -o static/chainlist.json -H "Accept: application/json" https://chainlist.org/rpcs.json
  @echo "Chainlist JSON file updated."

# Lint the Rust documentation
lint-docs:
  RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --document-private-items

# Fixes the formatting of the workspace
fmt-native-fix:
  cargo +nightly fmt --all

# Check the formatting of the workspace
fmt-native-check:
  cargo +nightly fmt --all -- --check

# Lints the workspace
lints: fmt-native-check lint-docs

# Lint the workspace
clippy:
  cargo clippy --workspace --all-features --all-targets -- -D warnings

# Tests the workspace
tests: test test-docs

# Test the Rust documentation
test-docs:
  cargo test --doc --workspace --locked

# Test for the native target with all features. By default, excludes online tests.
test *args="-E '!test(test_online)'":
  cargo nextest run --workspace --all-features {{args}}
