//! Communication protocol for Vantis Mobile

use crate::error::{MobileError, MobileResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Protocol version
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProtocolVersion {
    V1_0,
    V1_1,
    V2_0,
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        Self::V1_1
    }
}

/// Message type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// Authentication
    Authenticate,
    
    /// Document operations
    GetDocument,
    ListDocuments,
    CreateDocument,
    UpdateDocument,
    DeleteDocument,
    
    /// Sync operations
    SyncStart,
    SyncChanges,
    SyncComplete,
    
    /// Notification operations
    RegisterNotifications,
    GetNotifications,
    MarkNotificationRead,
    
    /// Remote control
    OpenApp,
    ExecuteCommand,
    GetAppState,
    
    /// Tunnel operations
    TunnelConnect,
    TunnelDisconnect,
    TunnelPing,
    
    /// Error
    Error,
}

/// Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID
    pub message_id: String,
    
    /// Message type
    pub message_type: MessageType,
    
    /// Protocol version
    pub protocol_version: ProtocolVersion,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Payload
    pub payload: Option<serde_json::Value>,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl Message {
    /// Create a new message
    pub fn new(message_type: MessageType, payload: Option<serde_json::Value>) -> Self {
        Self {
            message_id: Uuid::new_v4().to_string(),
            message_type,
            protocol_version: ProtocolVersion::default(),
            timestamp: chrono::Utc::now(),
            payload,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Serialize to JSON
    pub fn to_json(&self) -> MobileResult<String> {
        serde_json::to_string(self).map_err(MobileError::Serialization)
    }
    
    /// Deserialize from JSON
    pub fn from_json(json: &str) -> MobileResult<Self> {
        serde_json::from_str(json).map_err(MobileError::Serialization)
    }
}

/// Command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Command ID
    pub command_id: String,
    
    /// Command type
    pub command_type: CommandType,
    
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Target app
    pub target_app: Option<String>,
}

/// Command type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    /// Open document
    OpenDocument { document_id: String },
    
    /// Save document
    SaveDocument { document_id: String },
    
    /// Close document
    CloseDocument { document_id: String },
    
    /// Create new document
    NewDocument { format: String },
    
    /// Sync
    Sync,
    
    /// Backup
    Backup,
    
    /// Custom command
    Custom { command: String },
}

/// Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// Response ID
    pub response_id: String,
    
    /// Message ID this is responding to
    pub message_id: String,
    
    /// Success status
    pub success: bool,
    
    /// Data
    pub data: Option<serde_json::Value>,
    
    /// Error message
    pub error: Option<String>,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Response {
    /// Create a successful response
    pub fn success(message_id: String, data: Option<serde_json::Value>) -> Self {
        Self {
            response_id: Uuid::new_v4().to_string(),
            message_id,
            success: true,
            data,
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Create an error response
    pub fn error(message_id: String, error: String) -> Self {
        Self {
            response_id: Uuid::new_v4().to_string(),
            message_id,
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now(),
        }
    }
    
    /// Serialize to JSON
    pub fn to_json(&self) -> MobileResult<String> {
        serde_json::to_string(self).map_err(MobileError::Serialization)
    }
    
    /// Deserialize from JSON
    pub fn from_json(json: &str) -> MobileResult<Self> {
        serde_json::from_str(json).map_err(MobileError::Serialization)
    }
}

/// Protocol handler
pub struct ProtocolHandler {
    version: ProtocolVersion,
}

impl ProtocolHandler {
    /// Create a new protocol handler
    pub fn new(version: ProtocolVersion) -> Self {
        Self { version }
    }
    
    /// Create message
    pub fn create_message(
        &self,
        message_type: MessageType,
        payload: Option<serde_json::Value>,
    ) -> Message {
        let mut message = Message::new(message_type, payload);
        message.protocol_version = self.version.clone();
        message
    }
    
    /// Create response
    pub fn create_response(
        &self,
        message_id: String,
        success: bool,
        data: Option<serde_json::Value>,
        error: Option<String>,
    ) -> Response {
        if success {
            Response::success(message_id, data)
        } else {
            Response::error(message_id, error.unwrap_or_default())
        }
    }
    
    /// Parse message
    pub fn parse_message(&self, json: &str) -> MobileResult<Message> {
        let message = Message::from_json(json)?;
        
        // Validate protocol version
        if message.protocol_version != self.version {
            return Err(MobileError::Protocol(format!(
                "Protocol version mismatch: expected {:?}, got {:?}",
                self.version, message.protocol_version
            )));
        }
        
        Ok(message)
    }
    
    /// Parse response
    pub fn parse_response(&self, json: &str) -> MobileResult<Response> {
        Response::from_json(json)
    }
    
    /// Handle message
    pub async fn handle_message(&self, message: &Message) -> MobileResult<Response> {
        match message.message_type {
            MessageType::Authenticate => {
                // Handle authentication
                Ok(Response::success(
                    message.message_id.clone(),
                    Some(serde_json::json!({
                        "authenticated": true,
                        "session_id": Uuid::new_v4().to_string()
                    })),
                ))
            }
            MessageType::GetDocument => {
                // Handle get document
                Ok(Response::success(
                    message.message_id.clone(),
                    Some(serde_json::json!({
                        "document_id": "doc-123",
                        "title": "Test Document"
                    })),
                ))
            }
            MessageType::ListDocuments => {
                // Handle list documents
                Ok(Response::success(
                    message.message_id.clone(),
                    Some(serde_json::json!({
                        "documents": []
                    })),
                ))
            }
            MessageType::Error => {
                // Handle error message
                Ok(Response::error(
                    message.message_id.clone(),
                    "Received error message".to_string(),
                ))
            }
            _ => {
                // Unknown message type
                Ok(Response::error(
                    message.message_id.clone(),
                    "Unknown message type".to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message::new(MessageType::Authenticate, None);
        assert_eq!(message.message_type, MessageType::Authenticate);
    }

    #[test]
    fn test_message_serialization() {
        let message = Message::new(
            MessageType::GetDocument,
            Some(serde_json::json!({"document_id": "doc-123"})),
        );
        
        let json = message.to_json().unwrap();
        let parsed = Message::from_json(&json).unwrap();
        
        assert_eq!(parsed.message_id, message.message_id);
    }

    #[test]
    fn test_response_creation() {
        let response = Response::success(
            "msg-123".to_string(),
            Some(serde_json::json!({"status": "ok"})),
        );
        
        assert!(response.success);
        assert_eq!(response.message_id, "msg-123");
    }

    #[test]
    fn test_response_error() {
        let response = Response::error("msg-123".to_string(), "Test error".to_string());
        
        assert!(!response.success);
        assert_eq!(response.error, Some("Test error".to_string()));
    }

    #[tokio::test]
    async fn test_protocol_handler() {
        let handler = ProtocolHandler::new(ProtocolVersion::V1_1);
        
        let message = handler.create_message(MessageType::Authenticate, None);
        let response = handler.handle_message(&message).await.unwrap();
        
        assert!(response.success);
    }

    #[test]
    fn test_protocol_version_mismatch() {
        let handler = ProtocolHandler::new(ProtocolVersion::V1_1);
        
        let mut message = Message::new(MessageType::Authenticate, None);
        message.protocol_version = ProtocolVersion::V1_0;
        
        let json = message.to_json().unwrap();
        let result = handler.parse_message(&json);
        
        assert!(result.is_err());
    }
}