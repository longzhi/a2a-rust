set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
  @just --list

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all -- --check

clippy:
  cargo clippy --all-targets --all-features -- -D warnings

test:
  cargo test --all-features

check:
  bash scripts/check.sh

fix: fmt

install-hooks:
  bash scripts/install-git-hooks.sh

doc:
  cargo doc --all-features --no-deps
