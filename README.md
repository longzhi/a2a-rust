# a2a-rust

[![CI](https://github.com/longzhi/a2a-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/longzhi/a2a-rust/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/a2a-rust.svg)](https://crates.io/crates/a2a-rust)
[![docs.rs](https://docs.rs/a2a-rust/badge.svg)](https://docs.rs/a2a-rust)
[![License](https://img.shields.io/crates/l/a2a-rust.svg)](LICENSE-MIT)

A Rust SDK for the [Google A2A (Agent-to-Agent)](https://a2a-protocol.org/) protocol. Provides complete type definitions, a server framework, and a client library for building A2A-compatible agents.

**First Rust A2A SDK. First v1.0 implementation in any language.**

## Features

- **Complete type system** — All A2A v1.0 RC types (AgentCard, Task, Message, Part, SecurityScheme, etc.) with serde serialization
- **Server framework** — axum-based router supporting both REST and JSON-RPC 2.0 bindings, with SSE streaming
- **Client library** — AgentCard discovery with caching, and full A2A client (send, get, cancel, list, subscribe)
- **TaskStore trait** — Pluggable task persistence with built-in `InMemoryTaskStore` (TTL + LRU eviction)
- **Protocol compliant** — Strict alignment with A2A v1.0 RC spec (tag `v1.0.0-rc`, commit `6292104`)
- **Feature-gated** — `server` and `client` features can be enabled independently

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
a2a-rust = "0.1"
```

### Build an A2A Server

Implement the `A2AHandler` trait and mount the router:

```rust
use a2a_rust::server::{A2AHandler, router};
use a2a_rust::types::*;

struct EchoAgent;

#[async_trait::async_trait]
impl A2AHandler for EchoAgent {
    async fn get_agent_card(&self) -> Result<AgentCard, a2a_rust::A2AError> {
        // Return your agent's card
        todo!()
    }

    async fn handle_send_message(
        &self,
        req: SendMessageRequest,
    ) -> Result<SendMessageResponse, a2a_rust::A2AError> {
        // Process the message and return a Task or Message
        todo!()
    }

    // ... implement other required methods
}

#[tokio::main]
async fn main() {
    let app = router(EchoAgent);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Your agent is now discoverable at `GET /.well-known/agent-card.json` and accepts requests via both REST and JSON-RPC endpoints.

### Use the A2A Client

```rust
use a2a_rust::client::A2AClient;
use a2a_rust::types::*;

#[tokio::main]
async fn main() -> Result<(), a2a_rust::A2AError> {
    let client = A2AClient::new();

    // Discover an agent
    let card = client.discover("https://agent.example.com").await?;
    println!("Agent: {} — {}", card.name, card.description);

    // Send a message
    let response = client.send_message("https://agent.example.com", request).await?;
    Ok(())
}
```

## Protocol Bindings

A2A v1.0 RC defines three protocol bindings. This crate implements JSON-RPC and REST:

| Binding | Endpoint | Status |
|---------|----------|--------|
| **JSON-RPC 2.0** | `POST /jsonrpc` | Implemented |
| **REST (HTTP+JSON)** | Multiple endpoints | Implemented |
| **gRPC** | protobuf service | Not yet (structure reserved) |

### REST Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/.well-known/agent-card.json` | Agent discovery |
| POST | `/message:send` | Send message |
| POST | `/message:stream` | Send message (SSE streaming) |
| GET | `/tasks/{id}` | Get task |
| GET | `/tasks` | List tasks (cursor pagination) |
| POST | `/tasks/{id}:cancel` | Cancel task |
| GET | `/tasks/{id}:subscribe` | Subscribe to task (SSE) |
| GET | `/extendedAgentCard` | Extended agent card (authenticated) |

### JSON-RPC Methods

| Method | Description |
|--------|-------------|
| `message/send` | Send message, returns Task or Message |
| `message/stream` | Send message with SSE streaming |
| `tasks/get` | Get task by ID |
| `tasks/list` | List tasks with cursor pagination |
| `tasks/cancel` | Cancel a task |
| `tasks/subscribe` | Subscribe to existing task (SSE) |
| `agent/getExtendedCard` | Get extended agent card |

## Project Structure

```
src/
├── lib.rs               # Public API re-exports
├── types/
│   ├── agent_card.rs    # AgentCard, AgentSkill, AgentCapabilities
│   ├── task.rs          # Task, TaskState, TaskStatus
│   ├── message.rs       # Message, Part (unified), Artifact
│   ├── streaming.rs     # StreamResponse, SSE event types
│   ├── security.rs      # SecurityScheme (5 variants), SecurityRequirement
│   └── jsonrpc.rs       # JSON-RPC 2.0 Request/Response/Error
├── server/
│   ├── handler.rs       # A2AHandler trait (implement this)
│   ├── router.rs        # axum Router builder
│   ├── rest.rs          # REST endpoint handlers
│   ├── jsonrpc.rs       # JSON-RPC 2.0 dispatcher
│   └── streaming.rs     # SSE streaming
├── client/
│   ├── discovery.rs     # AgentCard discovery + caching
│   └── client.rs        # A2AClient
└── error.rs             # A2AError type
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `server` | Yes | A2A server framework (axum-based) |
| `client` | Yes | A2A HTTP client (reqwest-based) |

To use only the types:

```toml
[dependencies]
a2a-rust = { version = "0.1", default-features = false }
```

## Protocol Version

This crate strictly targets **A2A Protocol v1.0 RC** (git tag `v1.0.0-rc`, commit `6292104`, 2026-01-29). Key differences from v0.3.0:

- `Part` is a unified flat struct (not tagged enum)
- `AgentCard.url` removed — replaced by `supported_interfaces: Vec<AgentInterface>`
- Enum values use `SCREAMING_SNAKE_CASE` (e.g., `TASK_STATE_COMPLETED`)
- REST endpoints changed (e.g., `POST /message:send` instead of `POST /tasks/send`)
- JSON-RPC methods renamed (e.g., `message/send` instead of `tasks/send`)
- Well-known path: `agent-card.json` (was `agent.json`)
- TaskState has 9 states including `REJECTED` and `AUTH_REQUIRED`

See [What's New in V1](https://a2a-protocol.org/latest/whats-new-v1/) for the full changelog.

## References

- [A2A Protocol Spec v1.0 RC](https://a2a-protocol.org/latest/specification/)
- [A2A Proto Source (v1.0.0-rc)](https://github.com/a2aproject/A2A/blob/v1.0.0-rc/specification/a2a.proto)
- [A2A Agent Discovery](https://a2a-protocol.org/latest/topics/agent-discovery/)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
