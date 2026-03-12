# AGENTS.md

This file provides guidance to AI coding agents working in this repository.

## Project Overview

`a2a-rust` is a generic Rust SDK for A2A (Agent-to-Agent) Protocol v1.0 RC.

Current implementation status:

- implemented now: transport-neutral core types, JSON-RPC envelopes/constants, and `A2AError`
- planned next: `server`, `client`, and `store` layers

This crate has zero Clawhive-specific logic. Keep it generic and protocol-focused.

## Source of Truth

Protocol lock:

- tag: `v1.0.0-rc`
- commit: `6292104`
- proto package: `a2a.v1`

Normative source precedence:

1. tagged proto
2. current spec prose
3. repository-local design docs

Use the repo-local design contract at [docs/proto-first-design.md](docs/proto-first-design.md).

Do not treat the old external planning note as the implementation contract.

## Build, Test, and Lint

Use these commands before considering a change done:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --no-default-features -- -D warnings
cargo check --all-features
cargo test --no-default-features
cargo test --all-features
```

Useful variants:

```bash
# Types-only compile
cargo check --no-default-features

# Feature combinations
cargo check --no-default-features --features server
cargo check --no-default-features --features client
cargo check --no-default-features --features server,client
```

CI now checks:

- all-features build, test, clippy, docs
- no-default-features build, test, clippy, docs
- feature-combination compile matrix

## Git Hooks

Install local hooks with:

```bash
just install-hooks
```

Current hooks:

- `pre-commit`: format check, minimal clippy, all-features compile check
- `commit-msg`: Conventional Commits enforcement
- `pre-push`: no-default-features tests and all-features tests

## Current Project Structure

```text
src/
├── lib.rs
├── error.rs
├── jsonrpc.rs
└── types/
    ├── mod.rs
    ├── agent_card.rs
    ├── message.rs
    ├── push.rs
    ├── requests.rs
    ├── responses.rs
    ├── security.rs
    └── task.rs
```

Planned but not implemented yet:

- `src/server/*`
- `src/client/*`
- `src/store.rs`

## Key Source Files

- `src/error.rs` - `A2AError` and protocol/HTTP error mapping
- `src/jsonrpc.rs` - JSON-RPC 2.0 envelopes, method constants, error-code constants
- `src/types/agent_card.rs` - Agent card and capability model
- `src/types/message.rs` - `Message`, `Part`, `Artifact`, `Role`
- `src/types/task.rs` - `Task`, `TaskStatus`, `TaskState`
- `src/types/security.rs` - security schemes and OAuth flow types
- `src/types/requests.rs` - protocol request types
- `src/types/responses.rs` - protocol response and stream-event types
- `docs/proto-first-design.md` - implementation contract for future phases

## Feature Flags

```toml
[features]
default = ["server", "client"]
server = ["dep:axum"]
client = ["dep:reqwest"]
```

At the moment, the feature flags reserve dependency boundaries for the planned transport layers. The current public API is the core crate surface in `lib.rs`.

## Protocol and Serialization Rules

### JSON field naming

Use `camelCase` JSON field names via `#[serde(rename_all = "camelCase")]`.

### Enum values

Use proto enum strings exactly, for example:

- `TASK_STATE_COMPLETED`
- `ROLE_AGENT`

### JSON-RPC method names

Use PascalCase v1.0 RC method names, for example:

- `SendMessage`
- `GetTask`
- `ListTasks`
- `SubscribeToTask`

Do not introduce slash-style method names.

### Part

`Part` is a flat struct, not a tagged enum.

- exactly one of `text`, `raw`, `url`, or `data` should be set
- `raw` is modeled as `Vec<u8>` in Rust and serialized as base64 in JSON
- use `validate()` when you need an explicit semantic check

### SecurityScheme

`SecurityScheme` is externally tagged to match proto-style `oneof` JSON:

```json
{"apiKeySecurityScheme":{"location":"header","name":"X-API-Key"}}
```

Important:

- the field is `location`, not OpenAPI's `in`
- `OAuthFlows` is modeled as a oneof-style enum
- deprecated OAuth flows still exist in the tagged proto and remain part of the wire model

### Required shape corrections already reflected in code

- `Task.context_id` is required
- `TaskStatusUpdateEvent.context_id` is required
- `TaskArtifactUpdateEvent.context_id` is required
- `ListTaskPushNotificationConfigResponse.next_page_token` is a string, with empty string meaning no next page

### Error codes

Use:

- standard JSON-RPC codes (`-32700` to `-32603`) where appropriate
- A2A-specific codes `-32001` through `-32009`

Do not invent new A2A error codes.

## Code Style

- Derive order: `Debug, Clone, Serialize, Deserialize`
- Imports: std, external crates, crate-local
- No `unwrap()` outside tests
- No `unsafe`
- No Clawhive-specific imports or terminology
- Avoid unnecessary dependencies

## Testing Guidance

The most important tests right now are serde and wire-shape tests.

Prefer:

- inline `#[cfg(test)]` modules at the bottom of each file
- canonical spec/proto JSON examples
- explicit invalid-shape tests for `Part`, `SendMessageResponse`, and `StreamResponse`

The current test suite is still narrow. Expanding canonical serde coverage is high-value work.

## References

- [Proto-first design](docs/proto-first-design.md)
- [A2A Protocol Spec v1.0 RC](https://a2a-protocol.org/latest/specification/)
- [A2A Proto v1.0.0-rc](https://github.com/a2aproject/A2A/blob/v1.0.0-rc/specification/a2a.proto)
- [JSON-RPC 2.0 Spec](https://www.jsonrpc.org/specification)
