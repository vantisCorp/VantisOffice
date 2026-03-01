//! Error types for Vantis Mobile

use thiserror::Error;

/// Vantis Mobile error type
#[derive(Error, Debug)]
pub enum MobileError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Sync error: {0}")]
    Sync(String),

    #[error("Notification error: {0}")]
    Notification(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Not authenticated")]
    NotAuthenticated,

    #[error("Session expired")]
    SessionExpired,

    #[error("Document not found: {0}")]
    DocumentNotFound(String),

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Timeout")]
    Timeout,

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type alias for Vantis Mobile
pub type MobileResult<T> = Result<T, MobileError>;