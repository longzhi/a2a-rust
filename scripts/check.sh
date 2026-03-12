#!/usr/bin/env bash
set -euo pipefail

echo "[check] cargo fmt --all -- --check"
cargo fmt --all -- --check

echo "[check] cargo clippy --all-targets --all-features -- -D warnings"
cargo clippy --all-targets --all-features -- -D warnings

echo "[check] cargo test --all-features"
cargo test --all-features
