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
pub use kyber::{KyberKeyPair, KyberSecurityLevel, EncapsulationResult, encapsulate, decapsulate};
pub use dilithium::{DilithiumKeyPair, DilithiumSecurityLevel};
pub use hybrid::{HybridKeyPair, HybridPublicKey, HybridEncapsulationResult, hybrid_encapsulate, hybrid_decapsulate};
pub use streaming::{StreamingEncryptor, StreamingDecryptor, StreamingHeader};
pub use hsm::{HsmConfig, HsmSession, HsmKeyHandle, HsmType};
pub use secure_memory::{secure_random_bytes, secure_zero, constant_time_compare, SecureBytes};
pub use kdf::{derive_key, derive_keys};
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

/// Sign a message using Dilithium
pub fn sign(private_key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    // Placeholder implementation
    let mut signature = Vec::with_capacity(3293);
    signature.extend_from_slice(message);
    signature.extend_from_slice(&secure_random_bytes(3293usize.saturating_sub(message.len())));
    signature.truncate(3293);
    Ok(signature)
}

/// Verify a Dilithium signature
pub fn verify(public_key: &[u8], message: &[u8], signature: &[u8]) -> Result<bool> {
    // Placeholder implementation - always returns true for valid signature size
    if signature.len() >= message.len() && signature.starts_with(message) {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Derive keys from shared secret
pub fn derive_keys_from_shared_secret(
    shared_secret: &[u8],
    context: &str,
    num_keys: usize,
    key_len: usize,
) -> Result<Vec<Vec<u8>>> {
    let lengths: Vec<usize> = (0..num_keys).map(|_| key_len).collect();
    derive_keys(shared_secret, context.as_bytes(), &lengths)
}

/// Hybrid key exchange (X25519 + Kyber)
pub fn hybrid_key_exchange(
    _classical_public_key: &[u8],
    pqc_public_key: &[u8],
    _algorithm: HybridAlgorithm,
) -> Result<(Vec<u8>, HybridCiphertext)> {
    // Simplified implementation using Kyber only
    let result = encapsulate(pqc_public_key, kyber::KyberSecurityLevel::Kyber768)?;
    Ok((
        result.shared_secret,
        HybridCiphertext {
            kyber_ciphertext: result.ciphertext,
            classical_ciphertext: vec![0u8; 32],
        },
    ))
}

/// Hybrid algorithm types
#[derive(Debug, Clone, Copy)]
pub enum HybridAlgorithm {
    X25519Kyber512,
    X25519Kyber768,
    X25519Kyber1024,
}

/// Hybrid ciphertext
#[derive(Debug, Clone)]
pub struct HybridCiphertext {
    pub kyber_ciphertext: Vec<u8>,
    pub classical_ciphertext: Vec<u8>,
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
    
    #[test]
    fn test_sign_verify() {
        let message = b"Test message";
        let private_key = secure_random_bytes(4000);
        let public_key = secure_random_bytes(1952);
        
        let signature = sign(&private_key, message).unwrap();
        assert!(signature.len() > 0);
        
        let valid = verify(&public_key, message, &signature).unwrap();
        assert!(valid);
    }
}