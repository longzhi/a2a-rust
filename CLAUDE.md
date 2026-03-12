# CLAUDE.md

Read AGENTS.md first — it contains the full project context, code style, and conventions.

## Quick Reference

- **Language**: Rust, edition 2021, MSRV 1.75+
- **Protocol**: A2A v1.0 RC (tag `v1.0.0-rc`, commit `6292104`)
- **This is a library crate** — published to crates.io, no binary
- **Zero Clawhive dependency** — never import `clawhive-*` crates

## Before Every Change

```bash
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

All three must pass. CI treats warnings as errors.

## Critical Rules

1. All JSON field names use `camelCase` (`#[serde(rename_all = "camelCase")]`)
2. All enum values use `SCREAMING_SNAKE_CASE` (`#[serde(rename = "TASK_STATE_COMPLETED")]`)
3. `Part` is a flat struct with optional fields — NOT a tagged enum
4. `SecurityScheme` is externally tagged (proto3 oneof format)
5. Never use `.unwrap()` outside tests
6. Never add Clawhive-specific code
7. The proto file at tag `v1.0.0-rc` is the single source of truth for all types

## Design Doc

The full design specification is at:
`~/Library/Mobile Documents/iCloud~md~obsidian/Documents/obsidian-vault/Projects/clawhive/research/a2a-rust-design.md`

This document contains complete type definitions, API designs, and implementation notes.
