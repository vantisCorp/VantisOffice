//! Vantis Link - P2P collaboration module
//! 
//! Features:
//! - CRDT-based conflict resolution
//! - End-to-end encryption
//! - No central servers
//! - Real-time synchronization
//! - Offline support

pub mod core;
pub mod crdt;
pub mod encryption;
pub mod sync;
pub mod discovery;
pub mod transport;

pub use core::{Session, User, Document, Change};
pub use crdt::{CrdtEngine, CrdtOperation, CrdtType};
pub use encryption::{EncryptionManager, EncryptionKey, EncryptionAlgorithm};
pub use sync::{SyncManager, SyncStatus, SyncConflict};
pub use discovery::{PeerDiscovery, PeerInfo};
pub use transport::{Transport, TransportProtocol};

/// Vantis Link version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize Vantis Link
pub fn init() -> Result<(), LinkError> {
    // Initialize subsystems
    core::init()?;
    crdt::init()?;
    encryption::init()?;
    sync::init()?;
    discovery::init()?;
    transport::init()?;
    
    Ok(())
}

/// Link-specific errors
#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    #[error("Session error: {0}")]
    Session(String),
    
    #[error("CRDT error: {0}")]
    Crdt(String),
    
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("Sync error: {0}")]
    Sync(String),
    
    #[error("Discovery error: {0}")]
    Discovery(String),
    
    #[error("Transport error: {0}")]
    Transport(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("General error: {0}")]
    General(String),
}

impl From<String> for LinkError {
    fn from(s: String) -> Self {
        LinkError::General(s)
    }
}