//! The core data types and traits for the Multi-Capability Protocol (MCP).
//! This crate defines the MCP specification elements and abstract service interfaces,
//! without depending on specific I/O or async runtimes like Tokio.

mod error;
pub use error::Error;

/// Basic data types in MCP specification
pub mod model;

/// Core service traits (ServiceRole, Service, DynService, ID Providers)
pub mod service_traits;

// Re-export key types and traits for easier use
pub use model::*;
pub use service_traits::*;

// Potentially re-export common dependencies like serde if desired
// pub use serde;
// pub use serde_json; 