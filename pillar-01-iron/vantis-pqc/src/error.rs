//! Error handling for PQC operations

use thiserror::Error;

/// PQC operation result type
pub type Result<T> = std::result::Result<T, PQCError>;

/// Errors that can occur during PQC operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PQCError {
    /// Invalid key size
    #[error("Invalid key size: {0}")]
    InvalidKeySize(usize),

    /// Invalid security level
    #[error("Invalid security level: {0}")]
    InvalidSecurityLevel(String),

    /// Key generation failed
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),

    /// Encapsulation failed
    #[error("Encapsulation failed: {0}")]
    EncapsulationFailed(String),

    /// Decapsulation failed
    #[error("Decapsulation failed: {0}")]
    DecapsulationFailed(String),

    /// Invalid public key
    #[error("Invalid public key")]
    InvalidPublicKey,

    /// Invalid private key
    #[error("Invalid private key")]
    InvalidPrivateKey,

    /// Invalid ciphertext
    #[error("Invalid ciphertext: {0}")]
    InvalidCiphertext(String),

    /// Invalid signature
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),

    /// Invalid key material
    #[error("Invalid key material: {0}")]
    InvalidKeyMaterial(String),

    /// Key derivation failed
    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),

    /// Algorithm not supported
    #[error("Algorithm not supported: {0}")]
    AlgorithmNotSupported(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Encryption failed
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    /// Decryption failed
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(String),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Library error
    #[error("PQC library error: {0}")]
    LibraryError(String),

    /// Generic error
    #[error("PQC error: {0}")]
    Generic(String),
}

impl PQCError {
    /// Create a new generic PQC error
    pub fn new(message: impl Into<String>) -> Self {
        PQCError::Generic(message.into())
    }
}

impl From<anyhow::Error> for PQCError {
    fn from(err: anyhow::Error) -> Self {
        PQCError::Generic(err.to_string())
    }
}

impl From<std::io::Error> for PQCError {
    fn from(err: std::io::Error) -> Self {
        PQCError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = PQCError::InvalidKeySize(1024);
        assert!(error.to_string().contains("1024"));
    }

    #[test]
    fn test_error_from_anyhow() {
        let anyhow_err = anyhow::anyhow!("Test error");
        let pqc_err: PQCError = anyhow_err.into();
        assert!(matches!(pqc_err, PQCError::Generic(_)));
    }

    #[test]
    fn test_result_type() {
        let result: Result<()> = Ok(());
        assert!(result.is_ok());

        let error_result: Result<()> = Err(PQCError::InvalidPublicKey);
        assert!(error_result.is_err());
    }
}