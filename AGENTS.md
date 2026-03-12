# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## Project Overview

`a2a-rust` is a generic Rust SDK for the Google A2A (Agent-to-Agent) protocol. It provides a complete type system, an axum-based server framework, and an HTTP client library. This is the **first Rust A2A SDK** and the **first v1.0 RC implementation** in any language.

**This crate has ZERO Clawhive-specific logic.** It is a pure protocol SDK intended for any Rust project.

## Protocol Version Lock

All types and behavior strictly align with **A2A Protocol v1.0 RC**:
- Git tag: `v1.0.0-rc`, commit `6292104`, dated 2026-01-29
- Proto package: `lf.a2a.v1`
- Spec: https://a2a-protocol.org/latest/specification/
- Proto source: https://github.com/a2aproject/A2A/blob/v1.0.0-rc/specification/a2a.proto

**The proto file is the single source of truth.** When in doubt about a type, field, or behavior, check the proto definition.

## Build / Test / Lint

```bash
# Build
cargo build

# Run all tests
cargo test

# Lint (zero warnings required)
cargo clippy --all-targets -- -D warnings

# Format check
cargo fmt -- --check

# Generate docs
cargo doc --no-deps --open

# Run a single test by name
cargo test -- test_agent_card_deserialization -v

# Run tests for a specific module
cargo test types::

# Build with specific features
cargo build --no-default-features                  # types only
cargo build --no-default-features --features server # server only
cargo build --no-default-features --features client # client only
```

CI runs 4 parallel jobs: check, test, clippy, fmt. `RUSTFLAGS=-Dwarnings` is set — **all warnings are errors**.

## Project Structure

```
src/
├── lib.rs               # Public API re-exports
├── types/
│   ├── mod.rs
│   ├── agent_card.rs    # AgentCard, AgentSkill, AgentCapabilities, AgentProvider, AgentInterface
│   ├── task.rs          # Task, TaskState, TaskStatus, TaskStatusUpdateEvent
│   ├── message.rs       # Message, Part (unified flat struct), Artifact, Role
│   ├── streaming.rs     # StreamResponse, TaskArtifactUpdateEvent
│   ├── security.rs      # SecurityScheme (5 variants), SecurityRequirement
│   └── jsonrpc.rs       # JSON-RPC 2.0 Request/Response/Error, method constants
├── server/
│   ├── mod.rs
│   ├── handler.rs       # A2AHandler trait — users implement this
│   ├── router.rs        # axum Router builder
│   ├── rest.rs          # REST endpoint handlers (v1.0 RC paths)
│   ├── jsonrpc.rs       # JSON-RPC 2.0 dispatcher
│   └── streaming.rs     # SSE streaming implementation
├── client/
│   ├── mod.rs
│   ├── discovery.rs     # AgentCard discovery + TTL cache
│   └── client.rs        # A2AClient (send, get, cancel, list, subscribe)
├── store.rs             # TaskStore trait + InMemoryTaskStore
└── error.rs             # A2AError unified error type
```

### Key Source Files

- `src/server/handler.rs` — `A2AHandler` trait: the core trait users implement to build an A2A agent
- `src/server/router.rs` — `router()` function: builds axum Router with all A2A endpoints
- `src/client/client.rs` — `A2AClient`: HTTP client for calling remote A2A agents
- `src/store.rs` — `TaskStore` trait: pluggable task persistence
- `src/error.rs` — `A2AError`: all error types with JSON-RPC code mapping

## Feature Flags

```toml
[features]
default = ["server", "client"]
server = ["axum"]      # A2A server framework
client = ["reqwest"]   # A2A HTTP client
```

Users can depend on types-only by disabling defaults. Server and client are independently toggleable.

## Code Style

### Serde Conventions (CRITICAL)

A2A spec uses `camelCase` JSON fields. All struct/enum types must use:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SomeType {
    pub some_field: String,       // serializes as "someField"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_field: Option<String>,
}
```

### Enum Value Naming

v1.0 RC uses `SCREAMING_SNAKE_CASE` for all enum values:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskState {
    #[serde(rename = "TASK_STATE_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "TASK_STATE_SUBMITTED")]
    Submitted,
    #[serde(rename = "TASK_STATE_WORKING")]
    Working,
    // ...
}
```

### Part Type

`Part` is a **flat struct** (NOT a tagged enum). Content type is determined by which field is `Some`:

```rust
pub struct Part {
    pub text: Option<String>,
    pub raw: Option<String>,
    pub url: Option<String>,
    pub data: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub filename: Option<String>,
    pub media_type: Option<String>,
}
```

### SecurityScheme Serialization

Externally tagged to match proto3 JSON oneof mapping:

```rust
#[serde(rename_all = "camelCase")]
pub enum SecurityScheme {
    ApiKeySecurityScheme(ApiKeySecurityScheme),
    HttpAuthSecurityScheme(HttpAuthSecurityScheme),
    // ...
}
// JSON: {"apiKeySecurityScheme": {"in": "header", "name": "X-API-KEY"}}
```

**Note:** Python SDK uses a different format with `type` discriminator. For interop, implement a custom deserializer that accepts both formats.

### Imports

Order: std → external crates → crate-local. One blank line between groups.

```rust
use std::collections::HashMap;
use std::sync::Arc;

use axum::{Router, routing::get};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::types::*;
use crate::error::A2AError;
```

### Error Handling

- `thiserror` for all public error types (this is a library crate)
- `A2AError` has methods `to_jsonrpc_error()` and `status_code()` for protocol mapping
- Use A2A-defined error codes: `-32001` to `-32005` (do NOT invent new codes)
- Never use `.unwrap()` in non-test code

### Derive Order

Always: `Debug, Clone, Serialize, Deserialize` (consistent across codebase).

### Naming

- Modules: `snake_case`
- Structs/Enums: `PascalCase`
- Functions: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`

### Logging

Use `tracing` crate, not `log` or `println!`:

```rust
tracing::info!(agent = %card.name, "agent card served");
```

### Tests

- Inline tests in `#[cfg(test)] mod tests { }` at bottom of each file
- Integration tests in `tests/` directory
- Test function names describe behavior: `fn agent_card_round_trip_serialization()`
- **Type serde tests are the most critical** — use A2A spec JSON examples as test data

### Async

- Tokio runtime
- `async_trait` for async trait methods
- `Arc<T>` for shared state

## Key Patterns

### A2AHandler Trait

The core trait users implement. All methods have default implementations returning `UnsupportedOperation` except `get_agent_card` and `handle_send_message`.

### Router Builder

```rust
let app = a2a_rust::server::router(my_handler);
// Mounts all REST + JSON-RPC + well-known endpoints
```

### TaskStore Trait

```rust
pub trait TaskStore: Send + Sync + 'static {
    async fn get(&self, task_id: &str) -> Result<Option<Task>, A2AError>;
    async fn put(&self, task: &Task) -> Result<(), A2AError>;
    async fn list(&self, req: &ListTasksRequest) -> Result<ListTasksResponse, A2AError>;
    async fn delete(&self, task_id: &str) -> Result<bool, A2AError>;
}
```

Built-in `InMemoryTaskStore` provides TTL-based expiration and LRU eviction. Downstream projects (clawhive-a2a, clawhive-hub) implement `SqliteTaskStore`.

### JSON-RPC Constants

```rust
pub const METHOD_MESSAGE_SEND: &str = "message/send";
pub const METHOD_TASKS_GET: &str = "tasks/get";
// etc.
pub const TASK_NOT_FOUND: i32 = -32001;
pub const TASK_NOT_CANCELABLE: i32 = -32002;
// etc.
```

## Critical Implementation Notes

1. **JSON field names** — Always `camelCase` via `#[serde(rename_all = "camelCase")]`
2. **SecurityScheme serde** — Externally tagged (proto3 oneof), with optional interop deserializer for Python SDK format
3. **Enum values** — `SCREAMING_SNAKE_CASE` (e.g., `TASK_STATE_COMPLETED`, `ROLE_AGENT`)
4. **Part structure** — Flat struct with optional fields, NOT a tagged enum
5. **SSE format** — Standard `data:` prefix + double newline, payload is `StreamResponse` wrapper
6. **JSON-RPC strictness** — `jsonrpc` must be `"2.0"`, `id` must be echoed back
7. **Error codes** — Use only A2A-defined codes (`-32001` to `-32005`)
8. **Default values** — Missing `Option` fields → `None`, not empty string
9. **No Clawhive** — Zero imports from any `clawhive-*` crate, no Clawhive-specific terminology
10. **Proto is truth** — Proto definition is the canonical reference, locked to tag `v1.0.0-rc`

## Don'ts

- **No `unsafe`** without explicit justification
- **No `.unwrap()`** outside of tests
- **No `println!`** — use `tracing::*`
- **No suppressing clippy** with `#[allow(...)]` without a comment explaining why
- **No new dependencies** without checking if existing deps provide equivalent functionality
- **No Clawhive imports** — this is a generic SDK
- **No `as any`** type assertions in doc examples
- **No inventing error codes** — stick to A2A spec
- **No `agent.json`** — well-known path is `agent-card.json` (v1.0 RC)
- **No lowercase enum values** — all enum serialization uses `SCREAMING_SNAKE_CASE`

## Dependencies

### Runtime

- `serde`, `serde_json` — serialization
- `axum` (feature: `server`) — HTTP server
- `reqwest` (feature: `client`) — HTTP client
- `tokio` — async runtime
- `futures`, `tokio-stream` — streaming
- `thiserror`, `anyhow` — error handling
- `async-trait` — async trait support
- `tracing` — structured logging

### Dev

- `tower` — testing axum handlers
- `wiremock` — HTTP mocking
- `tempfile` — filesystem tests

## References

- [A2A Protocol Spec v1.0 RC](https://a2a-protocol.org/latest/specification/)
- [A2A What's New in V1](https://a2a-protocol.org/latest/whats-new-v1/)
- [A2A Proto (v1.0.0-rc)](https://github.com/a2aproject/A2A/blob/v1.0.0-rc/specification/a2a.proto)
- [A2A Agent Discovery](https://a2a-protocol.org/latest/topics/agent-discovery/)
- [JSON-RPC 2.0 Spec](https://www.jsonrpc.org/specification)
- [OpenAPI 3.2 Security Scheme](https://spec.openapis.org/oas/v3.2.0.html#security-scheme-object)
