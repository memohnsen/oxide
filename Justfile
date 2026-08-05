# No args auto runs CLI list of options
default: 
    @just --choose

# Run the editor opening a test file
run:
    cargo run -- test.txt

# Run the editor opening welcome screen
welcome:
    cargo run

# Cargo fmt and clippy
lint:
    cargo fmt --all -- --check
    cargo clippy --all-targets --locked -- -D warnings
    cargo check

# Check formatting, run lints, and execute tests locally
test:
    cargo fmt --all -- --check
    cargo clippy --all-targets --locked -- -D warnings
    cargo test
    cargo check

# Cargo build
build:
    cargo build

# Audit third-party dependencies for known vulnerabilities
audit:
    cargo audit

# Scrub all cargo build objects and Trunk generation files
clean:
    cargo clean
    rm -rf dist

# Cargo insta review screenshots\
review:
    cargo insta review

# Run cargo test with full backtrace printed out 
backtrace:
    RUST_BACKTRACE=1 cargo test
