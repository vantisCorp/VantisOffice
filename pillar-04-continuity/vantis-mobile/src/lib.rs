//! # Vantis Mobile Core
//!
//! Core Rust library for Vantis Mobile companion app.
//! Provides secure tunnel protocol, cryptographic utilities,
//! and shared data models for iOS and Android applications.
//!
//! ## Architecture
//!
//! - **Protocol**: WebSocket-based secure tunnel communication
//! - **Crypto**: End-to-end encryption using ChaCha20-Poly1305
//! - **Models**: Shared data structures for mobile-desktop sync
//!
//! ## Example
//!
//! ```no_run
//! use vantis_mobile::{SecureTunnel, TunnelConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = TunnelConfig::new(
//!         "wss://tunnel.vantis.ai".to_string(),
//!         uuid::Uuid::new_v4(),
//!         b"encryption_key_32_bytes_exact_length_",
//!     );
//!
//!     let tunnel = SecureTunnel::connect(config).await?;
//!     println!("Connected to tunnel");
//!
//!     Ok::<(), Box<dyn std::error::Error>>(())
//! }
//! ```

pub mod crypto;
pub mod models;
pub mod protocol;
pub mod error;
pub mod ffi;

pub use crypto::*;
pub use models::*;
pub use protocol::*;
pub use error::*;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Mobile protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Default tunnel server URL
pub const DEFAULT_TUNNEL_URL: &str = "wss://tunnel.vantis.ai";

/// Maximum message size (10MB)
pub const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024;