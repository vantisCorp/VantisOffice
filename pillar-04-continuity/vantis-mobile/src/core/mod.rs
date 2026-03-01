//! Core data structures and types for Vantis Mobile

use crate::error::{MobileError, MobileResult};
use crate::tunnel::TunnelConfig;
use crate::sync::SyncConfig;
use crate::notification::NotificationConfig;
use crate::auth::AuthConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Server URL
    pub server_url: String,
    
    /// Device ID
    pub device_id: String,
    
    /// Device name
    pub device_name: String,
    
    /// Encryption key
    pub encryption_key: String,
    
    /// Tunnel configuration
    pub tunnel_config: TunnelConfig,
    
    /// Sync configuration
    pub sync_config: SyncConfig,
    
    /// Notification configuration
    pub notification_config: NotificationConfig,
    
    /// Authentication configuration
    pub auth_config: AuthConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_url: "https://api.vantis.ai".to_string(),
            device_id: Uuid::new_v4().to_string(),
            device_name: "Vantis Mobile Device".to_string(),
            encryption_key: String::new(),
            tunnel_config: TunnelConfig::default(),
            sync_config: SyncConfig::default(),
            notification_config: NotificationConfig::default(),
            auth_config: AuthConfig::default(),
        }
    }
}

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Device ID
    pub device_id: String,
    
    /// Device name
    pub device_name: String,
    
    /// Device type (iOS, Android)
    pub device_type: DeviceType,
    
    /// OS version
    pub os_version: String,
    
    /// App version
    pub app_version: String,
    
    /// Device capabilities
    pub capabilities: DeviceCapabilities,
}

/// Device type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    IOS,
    Android,
}

/// Device capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    /// Supports biometric authentication
    pub biometric_auth: bool,
    
    /// Supports push notifications
    pub push_notifications: bool,
    
    /// Supports offline mode
    pub offline_mode: bool,
    
    /// Maximum cache size in MB
    pub max_cache_size_mb: u64,
}

/// User session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session ID
    pub session_id: String,
    
    /// User ID
    pub user_id: String,
    
    /// Device ID
    pub device_id: String,
    
    /// Authentication token
    pub auth_token: String,
    
    /// Session expiration time
    pub expires_at: chrono::DateTime<chrono::Utc>,
    
    /// Session metadata
    pub metadata: HashMap<String, String>,
}

impl Session {
    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }
    
    /// Create a new session
    pub fn new(user_id: String, device_id: String, duration_hours: i64) -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            user_id,
            device_id,
            auth_token: Uuid::new_v4().to_string(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(duration_hours),
            metadata: HashMap::new(),
        }
    }
}

/// Document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Document ID
    pub document_id: String,
    
    /// Document title
    pub title: String,
    
    /// Document format
    pub format: DocumentFormat,
    
    /// Document content
    pub content: Vec<u8>,
    
    /// Document metadata
    pub metadata: DocumentMetadata,
    
    /// Document size in bytes
    pub size: u64,
    
    /// Last modified timestamp
    pub last_modified: chrono::DateTime<chrono::Utc>,
    
    /// Document version
    pub version: u32,
}

/// Document format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentFormat {
    /// Vantis Writer format
    VDoc,
    
    /// Vantis Grid format
    VSheet,
    
    /// Vantis Canvas format
    VSlide,
    
    /// PDF format
    Pdf,
    
    /// Plain text
    Text,
    
    /// Markdown
    Markdown,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Author
    pub author: String,
    
    /// Creation date
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Tags
    pub tags: Vec<String>,
    
    /// Custom metadata
    pub custom: HashMap<String, String>,
    
    /// Is cached locally
    pub is_cached: bool,
    
    /// Cache path
    pub cache_path: Option<String>,
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        Self {
            author: String::new(),
            created_at: chrono::Utc::now(),
            tags: Vec::new(),
            custom: HashMap::new(),
            is_cached: false,
            cache_path: None,
        }
    }
}

impl Document {
    /// Create a new document
    pub fn new(
        title: String,
        format: DocumentFormat,
        content: Vec<u8>,
        author: String,
    ) -> Self {
        let size = content.len() as u64;
        Self {
            document_id: Uuid::new_v4().to_string(),
            title,
            format,
            content,
            metadata: DocumentMetadata {
                author,
                created_at: chrono::Utc::now(),
                ..Default::default()
            },
            size,
            last_modified: chrono::Utc::now(),
            version: 1,
        }
    }
    
    /// Update document content
    pub fn update_content(&mut self, content: Vec<u8>) -> MobileResult<()> {
        self.content = content;
        self.size = self.content.len() as u64;
        self.last_modified = chrono::Utc::now();
        self.version += 1;
        Ok(())
    }
    
    /// Get document as JSON
    pub fn to_json(&self) -> MobileResult<String> {
        serde_json::to_string(self).map_err(MobileError::Serialization)
    }
    
    /// Parse document from JSON
    pub fn from_json(json: &str) -> MobileResult<Self> {
        serde_json::from_str(json).map_err(MobileError::Serialization)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_expiration() {
        let session = Session::new(
            "user123".to_string(),
            "device456".to_string(),
            1, // 1 hour
        );
        assert!(!session.is_expired());
    }

    #[test]
    fn test_document_creation() {
        let doc = Document::new(
            "Test Document".to_string(),
            DocumentFormat::VDoc,
            b"Test content".to_vec(),
            "Test Author".to_string(),
        );
        assert_eq!(doc.title, "Test Document");
        assert_eq!(doc.size, 12);
        assert_eq!(doc.version, 1);
    }

    #[test]
    fn test_document_update() {
        let mut doc = Document::new(
            "Test Document".to_string(),
            DocumentFormat::VDoc,
            b"Test content".to_vec(),
            "Test Author".to_string(),
        );
        doc.update_content(b"Updated content".to_vec()).unwrap();
        assert_eq!(doc.size, 15);
        assert_eq!(doc.version, 2);
    }
}