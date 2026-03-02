//! Vantis Vault: TPM 2.0 encryption and key management for VantisOffice
//!
//! This library provides hardware-level encryption for all VantisOffice documents
//! using TPM 2.0 chip technology.

pub mod tpm;
pub mod crypto;
pub mod recovery;
pub mod api;

use anyhow::Result;

/// Initialize Vantis Vault
pub fn init() -> Result<()> {
    // Initialize TPM provider
    tpm::init()?;
    
    // Initialize crypto primitives
    crypto::init()?;
    
    Ok(())
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
