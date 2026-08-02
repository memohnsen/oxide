# No args auto runs CLI list of options
default: 
    @just --choose

# Cargo run
run:
    cargo run

# Cargo fmt and clippy
lint:
    cargo fmt --all -- --check
    cargo clippy --all-targets --locked -- -D warnings

# Check formatting, run lints, and execute tests locally
test:
    cargo fmt --all -- --check
    cargo clippy --all-targets --locked -- -D warnings
    cargo test

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
