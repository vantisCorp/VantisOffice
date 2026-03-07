//! VantisOffice Post-Quantum Cryptography Module
//!
//! This module provides quantum-resistant cryptographic operations using:
//! - **Kyber** for key encapsulation (KEM)
//! - **Dilithium** for digital signatures
//!
//! # Features
//! - Hybrid encryption combining classical and post-quantum algorithms
//! - Streaming encryption for large files
//! - HSM integration support
//! - Multi-party encryption for group collaboration
//! - Cross-platform support (Windows, macOS, Linux)

pub mod error;
pub mod kyber;
pub mod dilithium;
pub mod hybrid;
pub mod streaming;
pub mod hsm;
pub mod secure_memory;
pub mod kdf;
pub mod multi_party;
pub mod platform;

// Re-exports for convenience
pub use error::{PQCError, Result};
pub use kyber::{KyberKeyPair, KyberSecurityLevel};
pub use dilithium::{DilithiumKeyPair, DilithiumSecurityLevel};
pub use streaming::{StreamingEncryptor, StreamingDecryptor, StreamingHeader};
pub use hsm::{HsmConfig, HsmSession, HsmKeyHandle, HsmType};
pub use multi_party::{MultiPartyManager, GroupState, GroupMember, AccessLevel, MultiRecipientMessage};
pub use platform::{PlatformInfo, SecureStorage, get_secure_storage};

/// Module version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Security levels for PQC operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// NIST Level 1 (equivalent to AES-128)
    Level1,
    /// NIST Level 3 (equivalent to AES-192)
    Level3,
    /// NIST Level 5 (equivalent to AES-256)
    Level5,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        SecurityLevel::Level3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_security_level_default() {
        assert_eq!(SecurityLevel::default(), SecurityLevel::Level3);
    }
}