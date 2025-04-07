// Re-export core types and traits from rmcp-core
pub use rmcp_core::*;

// error is now in rmcp_core, we might need a new error type here or re-export specific variants?
// For now, just remove the direct mod and pub use.
// mod error;
// pub use error::Error; // Re-exported from rmcp_core

// model is now in rmcp_core
// /// Basic data types in MCP specification
// pub mod model;

#[cfg(any(feature = "client", feature = "server"))]
pub mod service;
#[cfg(feature = "client")]
pub use handler::client::ClientHandler;
#[cfg(feature = "server")]
pub use handler::server::ServerHandler;
#[cfg(any(feature = "client", feature = "server"))]
// Note: Service trait itself is now in rmcp_core, but ServiceExt, Peer, ServiceError are runtime-specific
pub use service::{Peer, /* Service, */ ServiceError, ServiceExt};
#[cfg(feature = "client")]
pub use service::{RoleClient, serve_client};
#[cfg(feature = "server")]
pub use service::{RoleServer, serve_server};

pub mod handler;
pub mod transport;

// re-export
#[cfg(all(feature = "macros", feature = "server"))]
pub use paste::paste;
#[cfg(all(feature = "macros", feature = "server"))]
pub use rmcp_macros::tool;
#[cfg(all(feature = "macros", feature = "server"))]
// schemars is used by model types in rmcp-core, need to adjust features?
// Let's keep this re-export for now, assuming rmcp might still use it directly.
pub use schemars;

// serde and serde_json are dependencies of rmcp-core, re-exporting here might be redundant
// #[cfg(feature = "macros")]
// pub use serde;
// #[cfg(feature = "macros")]
// pub use serde_json;
