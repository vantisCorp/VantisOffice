//! Cryptographic operations for Vantis Vault

use anyhow::Result;

pub mod pqc_integration;

// Re-export key PQC integration types
pub use pqc_integration::{
    PqcEncryptedDocument,
    PqcKeyBundle,
    encrypt_document,
    decrypt_document,
    hybrid_encrypt,
};

/// Initialize crypto subsystem
pub fn init() -> Result<()> {
    Ok(())
}