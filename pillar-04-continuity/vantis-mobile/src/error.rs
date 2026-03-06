//! Error types for Vantis Mobile

use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, MobileError>;

/// Errors that can occur in Vantis Mobile
#[derive(Error, Debug)]
pub enum MobileError {
    /// Tunnel connection errors
    #[error("Tunnel connection error: {0}")]
    TunnelConnection(String),

    /// Cryptographic errors
    #[error("Cryptographic error: {0}")]
    Crypto(String),

    /// Protocol errors
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Network errors
    #[error("Network error: {0}")]
    Network(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Authentication(String),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Timeout
    #[error("Operation timed out")]
    Timeout,

    /// Not connected
    #[error("Not connected to tunnel")]
    NotConnected,

    /// Key exchange failed
    #[error("Key exchange failed: {0}")]
    KeyExchange(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

impl MobileError {
    /// Create a tunnel connection error
    pub fn tunnel_connection(msg: impl Into<String>) -> Self {
        Self::TunnelConnection(msg.into())
    }

    /// Create a cryptographic error
    pub fn crypto(msg: impl Into<String>) -> Self {
        Self::Crypto(msg.into())
    }

    /// Create a protocol error
    pub fn protocol(msg: impl Into<String>) -> Self {
        Self::Protocol(msg.into())
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::TunnelConnection(_)
            | Self::Network(_)
            | Self::Timeout
            | Self::NotConnected
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = MobileError::crypto("test error");
        assert_eq!(err.to_string(), "Cryptographic error: test error");
    }

    #[test]
    fn test_error_is_recoverable() {
        assert!(MobileError::tunnel_connection("test").is_recoverable());
        assert!(MobileError::Timeout.is_recoverable());
        assert!(!MobileError::crypto("test").is_recoverable());
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: MobileError = io_err.into();
        assert!(matches!(err, MobileError::Io(_)));
    }
}