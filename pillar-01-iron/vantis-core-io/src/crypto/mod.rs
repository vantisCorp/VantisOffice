//! Cryptographic primitives for Vantis-Core-IO

use anyhow::Result;

/// Encryption levels for file operations
#[derive(Debug, Clone, Copy)]
pub enum EncryptionLevel {
    /// No encryption
    None,
    /// Software-based encryption
    Software,
    /// TPM 2.0 hardware encryption
    TPM2_0,
}

/// Integrity check result
#[derive(Debug, Clone, Copy)]
pub enum IntegrityCheck {
    /// Check passed
    Valid,
    /// Check failed
    Invalid,
    /// Check not performed
    NotPerformed,
}

/// Initialize cryptographic primitives
pub fn init() -> Result<()> {
    Ok(())
}

/// Compute SHA-3 hash of data
pub fn sha3_hash(data: &[u8]) -> Result<Vec<u8>> {
    use sha3::{Digest, Sha3_256};
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    Ok(hasher.finalize().to_vec())
}

/// Verify data integrity
pub fn verify_integrity(data: &[u8], expected_hash: &[u8]) -> IntegrityCheck {
    match sha3_hash(data) {
        Ok(hash) if hash == expected_hash => IntegrityCheck::Valid,
        Ok(_) => IntegrityCheck::Invalid,
        Err(_) => IntegrityCheck::NotPerformed,
    }
}
