#!/usr/bin/env just --justfile

check:
    cargo fmt
    cargo clippy --all-targets --all-features -- -D warnings
    cargo doc --no-deps --workspace --all-features
    cargo nextest run --all-targets --all-features
