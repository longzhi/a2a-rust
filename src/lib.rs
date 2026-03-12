#[cfg(feature = "client")]
pub mod client;
pub mod error;
pub mod jsonrpc;
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "server")]
pub mod store;
pub mod types;

#[cfg(feature = "client")]
pub use crate::client::{
    A2AClient, A2AClientConfig, A2AClientStream, AgentCardDiscovery, AgentCardDiscoveryConfig,
};
pub use crate::error::A2AError;
#[cfg(feature = "server")]
pub use crate::store::{InMemoryTaskStore, TaskStore};
pub use crate::types::*;
