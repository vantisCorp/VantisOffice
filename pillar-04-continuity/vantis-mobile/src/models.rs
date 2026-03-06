//! Shared data models for mobile-desktop communication
//!
//! Defines common data structures used for synchronization,
//! document transfer, and notification management.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique device identifier
pub type DeviceID = Uuid;

/// Document identifier
pub type DocumentID = Uuid;

/// User identifier
pub type UserID = Uuid;

/// Session identifier
pub type SessionID = Uuid;

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    /// Desktop computer
    Desktop,
    /// Laptop
    Laptop,
    /// iOS device
    Ios,
    /// Android device
    Android,
    /// Tablet
    Tablet,
}

/// Device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Unique device ID
    pub device_id: DeviceID,
    /// Device type
    pub device_type: DeviceType,
    /// Device name
    pub device_name: String,
    /// OS version
    pub os_version: String,
    /// App version
    pub app_version: String,
    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
}

/// Document type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentType {
    /// Vantis Writer document
    Writer,
    /// Vantis Flow diagram
    Flow,
    /// Vantis Canvas drawing
    Canvas,
    /// Vantis Grid spreadsheet
    Grid,
    /// Generic file
    File,
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    /// Document ID
    pub document_id: DocumentID,
    /// Document type
    pub document_type: DocumentType,
    /// Document title
    pub title: String,
    /// Owner user ID
    pub owner_id: UserID,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// File size in bytes
    pub size: u64,
    /// Whether document is cached offline
    pub cached: bool,
    /// Last sync timestamp
    pub last_sync: Option<DateTime<Utc>>,
}

/// Notification type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    /// Document updated
    DocumentUpdated,
    /// Collaboration request
    CollaborationRequest,
    /// System notification
    System,
    /// Calendar event
    CalendarEvent,
    /// Comment added
    CommentAdded,
    /// Share request
    ShareRequest,
}

/// Notification priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID
    pub notification_id: Uuid,
    /// Notification type
    pub notification_type: NotificationType,
    /// Priority
    pub priority: NotificationPriority,
    /// Title
    pub title: String,
    /// Message body
    pub body: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Whether notification was read
    pub read: bool,
    /// Associated document ID (optional)
    pub document_id: Option<DocumentID>,
    /// Associated user ID (optional)
    pub user_id: Option<UserID>,
    /// Action URL (optional)
    pub action_url: Option<String>,
}

/// App type for remote control
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppType {
    Writer,
    Flow,
    Canvas,
    Grid,
    Lens,
    Chronos,
    Vault,
    Ark,
    Bridge,
}

/// Remote command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteCommand {
    /// Command identifier
    pub command_id: Uuid,
    /// App type
    pub app_type: AppType,
    /// Command name
    pub command: String,
    /// Command parameters
    pub parameters: serde_json::Value,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Command response
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandResponse {
    /// Command ID being responded to
    pub command_id: Uuid,
    /// Success flag
    pub success: bool,
    /// Response data
    pub data: Option<serde_json::Value>,
    /// Error message (if any)
    pub error: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Sync status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncStatus {
    Idle,
    Syncing,
    Error,
    Conflict,
}

/// Sync progress
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncProgress {
    /// Sync operation ID
    pub sync_id: Uuid,
    /// Current status
    pub status: SyncStatus,
    /// Items processed
    pub processed: u32,
    /// Total items
    pub total: u32,
    /// Bytes transferred
    pub bytes_transferred: u64,
    /// Total bytes
    pub total_bytes: u64,
    /// Error message (if any)
    pub error: Option<String>,
}

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

/// Connection info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    /// Connection ID
    pub connection_id: SessionID,
    /// Current status
    pub status: ConnectionStatus,
    /// Connected device
    pub device: DeviceInfo,
    /// Connection timestamp
    pub connected_at: Option<DateTime<Utc>>,
    /// Latency in milliseconds
    pub latency_ms: Option<u64>,
    /// Last ping timestamp
    pub last_ping: Option<DateTime<Utc>>,
}

impl DeviceInfo {
    /// Create a new device info
    pub fn new(
        device_type: DeviceType,
        device_name: String,
        os_version: String,
        app_version: String,
    ) -> Self {
        Self {
            device_id: Uuid::new_v4(),
            device_type,
            device_name,
            os_version,
            app_version,
            last_seen: Utc::now(),
        }
    }

    /// Update last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }
}

impl DocumentMetadata {
    /// Create new document metadata
    pub fn new(document_type: DocumentType, title: String, owner_id: UserID) -> Self {
        let now = Utc::now();
        Self {
            document_id: Uuid::new_v4(),
            document_type,
            title,
            owner_id,
            created_at: now,
            modified_at: now,
            size: 0,
            cached: false,
            last_sync: None,
        }
    }
}

impl Notification {
    /// Create a new notification
    pub fn new(
        notification_type: NotificationType,
        priority: NotificationPriority,
        title: String,
        body: String,
    ) -> Self {
        Self {
            notification_id: Uuid::new_v4(),
            notification_type,
            priority,
            title,
            body,
            timestamp: Utc::now(),
            read: false,
            document_id: None,
            user_id: None,
            action_url: None,
        }
    }

    /// Mark notification as read
    pub fn mark_read(&mut self) {
        self.read = true;
    }
}

impl RemoteCommand {
    /// Create a new remote command
    pub fn new(app_type: AppType, command: String, parameters: serde_json::Value) -> Self {
        Self {
            command_id: Uuid::new_v4(),
            app_type,
            command,
            parameters,
            timestamp: Utc::now(),
        }
    }
}

impl CommandResponse {
    /// Create a successful response
    pub fn success(command_id: Uuid, data: Option<serde_json::Value>) -> Self {
        Self {
            command_id,
            success: true,
            data,
            error: None,
            timestamp: Utc::now(),
        }
    }

    /// Create an error response
    pub fn error(command_id: Uuid, error: String) -> Self {
        Self {
            command_id,
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}

impl SyncProgress {
    /// Create new sync progress
    pub fn new(total: u32) -> Self {
        Self {
            sync_id: Uuid::new_v4(),
            status: SyncStatus::Idle,
            processed: 0,
            total,
            bytes_transferred: 0,
            total_bytes: 0,
            error: None,
        }
    }

    /// Update progress
    pub fn update(&mut self, processed: u32, bytes: u64) {
        self.processed = processed;
        self.bytes_transferred = bytes;
    }

    /// Set status
    pub fn set_status(&mut self, status: SyncStatus) {
        self.status = status;
    }

    /// Set error
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.set_status(SyncStatus::Error);
    }

    /// Calculate progress percentage
    pub fn progress_percent(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.processed as f32 / self.total as f32) * 100.0
        }
    }
}

impl ConnectionInfo {
    /// Create new connection info
    pub fn new(device: DeviceInfo) -> Self {
        Self {
            connection_id: Uuid::new_v4(),
            status: ConnectionStatus::Disconnected,
            device,
            connected_at: None,
            latency_ms: None,
            last_ping: None,
        }
    }

    /// Set connected status
    pub fn set_connected(&mut self) {
        self.status = ConnectionStatus::Connected;
        self.connected_at = Some(Utc::now());
    }

    /// Set disconnected
    pub fn set_disconnected(&mut self) {
        self.status = ConnectionStatus::Disconnected;
        self.connected_at = None;
        self.latency_ms = None;
    }

    /// Update latency
    pub fn update_latency(&mut self, latency_ms: u64) {
        self.latency_ms = Some(latency_ms);
        self.last_ping = Some(Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_info_creation() {
        let info = DeviceInfo::new(
            DeviceType::Ios,
            "iPhone 14".to_string(),
            "iOS 17.0".to_string(),
            "1.0.0".to_string(),
        );
        assert_eq!(info.device_type, DeviceType::Ios);
        assert_eq!(info.device_name, "iPhone 14");
    }

    #[test]
    fn test_document_metadata_creation() {
        let user_id = Uuid::new_v4();
        let meta = DocumentMetadata::new(
            DocumentType::Writer,
            "Test Document".to_string(),
            user_id,
        );
        assert_eq!(meta.document_type, DocumentType::Writer);
        assert_eq!(meta.owner_id, user_id);
    }

    #[test]
    fn test_notification_creation() {
        let notif = Notification::new(
            NotificationType::System,
            NotificationPriority::Normal,
            "Test".to_string(),
            "Test body".to_string(),
        );
        assert_eq!(notif.notification_type, NotificationType::System);
        assert!(!notif.read);

        let mut marked = notif.clone();
        marked.mark_read();
        assert!(marked.read);
    }

    #[test]
    fn test_remote_command_creation() {
        let cmd = RemoteCommand::new(
            AppType::Writer,
            "save".to_string(),
            serde_json::json!({"document_id": "123"}),
        );
        assert_eq!(cmd.app_type, AppType::Writer);
        assert_eq!(cmd.command, "save");
    }

    #[test]
    fn test_command_response_success() {
        let cmd_id = Uuid::new_v4();
        let resp = CommandResponse::success(cmd_id, Some(serde_json::json!({"status": "ok"})));
        assert!(resp.success);
        assert_eq!(resp.command_id, cmd_id);
    }

    #[test]
    fn test_command_response_error() {
        let cmd_id = Uuid::new_v4();
        let resp = CommandResponse::error(cmd_id, "Test error".to_string());
        assert!(!resp.success);
        assert_eq!(resp.error.unwrap(), "Test error");
    }

    #[test]
    fn test_sync_progress() {
        let mut progress = SyncProgress::new(100);
        progress.update(50, 1024);
        assert_eq!(progress.progress_percent(), 50.0);
        progress.set_error("Test error".to_string());
        assert_eq!(progress.status, SyncStatus::Error);
    }

    #[test]
    fn test_connection_info() {
        let device = DeviceInfo::new(
            DeviceType::Desktop,
            "Test Desktop".to_string(),
            "Windows 11".to_string(),
            "1.0.0".to_string(),
        );
        let mut conn = ConnectionInfo::new(device);
        conn.set_connected();
        assert_eq!(conn.status, ConnectionStatus::Connected);
        assert!(conn.connected_at.is_some());
    }

    #[test]
    fn test_serialization() {
        let info = DeviceInfo::new(
            DeviceType::Android,
            "Pixel 7".to_string(),
            "Android 14".to_string(),
            "1.0.0".to_string(),
        );
        let json = serde_json::to_string(&info).unwrap();
        let deserialized: DeviceInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(info.device_id, deserialized.device_id);
    }
}