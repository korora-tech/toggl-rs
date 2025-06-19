#!/usr/bin/env just --justfile

check:
    cargo fmt
    cargo clippy --all-targets --all-features -- -D warnings
    cargo nextest run --all-targets --all-features
