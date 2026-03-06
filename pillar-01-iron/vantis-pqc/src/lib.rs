// VantisOffice Post-Quantum Cryptography Module
// Provides quantum-resistant cryptographic algorithms for future-proof security

pub mod kyber;
pub mod error;
pub mod key_management;
pub mod hybrid;

// Re-exports for convenience
pub use error::{PQCError, Result};
pub use kyber::{KyberKeyPair, KyberSecurityLevel, encapsulate, decapsulate};
pub use key_management::{KeyManager, KeyStorage};
pub use hybrid::{hybrid_key_exchange, HybridAlgorithm};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize the PQC module
pub fn init() -> Result<()> {
    // Initialize any required state
    Ok(())
}

/// Check if PQC is available and working
pub fn is_available() -> bool {
    true // PQC is always available in this implementation
}

/// Get supported algorithms
pub fn supported_algorithms() -> Vec<String> {
    vec![
        "kyber512".to_string(),
        "kyber768".to_string(),
        "kyber1024".to_string(),
        "hybrid_x25519_kyber768".to_string(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_init() {
        assert!(init().is_ok());
    }

    #[test]
    fn test_pqc_available() {
        assert!(is_available());
    }

    #[test]
    fn test_supported_algorithms() {
        let algorithms = supported_algorithms();
        assert!(!algorithms.is_empty());
        assert!(algorithms.contains(&"kyber768".to_string()));
    }
}