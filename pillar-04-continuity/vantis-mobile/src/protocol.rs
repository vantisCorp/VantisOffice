//! Secure tunnel protocol for mobile-desktop communication
//!
//! Implements WebSocket-based secure tunnel with end-to-end encryption
//! for real-time communication between mobile devices and desktop computers.

use crate::crypto::{Encryptor, EncryptionKey, KeyPair};
use crate::error::{MobileError, Result};
use crate::models::*;
use crate::{DEFAULT_TUNNEL_URL, MAX_MESSAGE_SIZE, PROTOCOL_VERSION};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};
use futures_util::SinkExt;

/// Tunnel configuration
#[derive(Debug, Clone)]
pub struct TunnelConfig {
    /// Tunnel server URL
    pub server_url: String,
    /// Device ID
    pub device_id: DeviceID,
    /// Pre-shared encryption key (for initial connection)
    pub encryption_key: Vec<u8>,
    /// Device type
    pub device_type: DeviceType,
    /// Device name
    pub device_name: String,
    /// OS version
    pub os_version: String,
    /// App version
    pub app_version: String,
}

impl TunnelConfig {
    /// Create a new tunnel configuration
    pub fn new(
        server_url: String,
        device_id: DeviceID,
        encryption_key: &[u8; 32],
    ) -> Self {
        Self {
            server_url,
            device_id,
            encryption_key: encryption_key.to_vec(),
            device_type: DeviceType::Ios,
            device_name: String::new(),
            os_version: String::new(),
            app_version: "1.0.0".to_string(),
        }
    }

    /// Set device type
    pub fn with_device_type(mut self, device_type: DeviceType) -> Self {
        self.device_type = device_type;
        self
    }

    /// Set device name
    pub fn with_device_name(mut self, name: String) -> Self {
        self.device_name = name;
        self
    }

    /// Set OS version
    pub fn with_os_version(mut self, version: String) -> Self {
        self.os_version = version;
        self
    }

    /// Set app version
    pub fn with_app_version(mut self, version: String) -> Self {
        self.app_version = version;
        self
    }
}

/// Protocol message type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum ProtocolMessage {
    /// Authentication handshake
    Handshake {
        device_id: DeviceID,
        protocol_version: u32,
        public_key: String,
        device_info: DeviceInfo,
    },

    /// Key exchange confirmation
    KeyExchange {
        session_id: SessionID,
        shared_secret: String,
    },

    /// Heartbeat/ping
    Ping { timestamp: DateTime<Utc> },

    /// Heartbeat/pong
    Pong { timestamp: DateTime<Utc> },

    /// Document update
    DocumentUpdate {
        document_id: DocumentID,
        metadata: DocumentMetadata,
        content: Option<String>,
    },

    /// Notification
    Notification {
        notification: Notification,
    },

    /// Remote command
    Command {
        command: RemoteCommand,
    },

    /// Command response
    CommandResponse {
        response: CommandResponse,
    },

    /// Sync progress update
    SyncProgress {
        progress: SyncProgress,
    },

    /// Error
    Error {
        code: u32,
        message: String,
    },
}

impl ProtocolMessage {
    /// Create a handshake message
    pub fn handshake(
        device_id: DeviceID,
        public_key: &KeyPair,
        device_info: DeviceInfo,
    ) -> Self {
        Self::Handshake {
            device_id,
            protocol_version: PROTOCOL_VERSION,
            public_key: public_key.public_key_base64(),
            device_info,
        }
    }

    /// Create a ping message
    pub fn ping() -> Self {
        Self::Ping {
            timestamp: Utc::now(),
        }
    }

    /// Create a pong message
    pub fn pong() -> Self {
        Self::Pong {
            timestamp: Utc::now(),
        }
    }
}

/// Encrypted protocol message
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EncryptedProtocolMessage {
    /// Message type (for routing)
    message_type: String,
    /// Encrypted content
    encrypted: crate::crypto::EncryptedMessage,
}

/// Message handler callback type
pub type MessageHandler = Arc<dyn Fn(ProtocolMessage) + Send + Sync>;

/// Secure tunnel connection
pub struct SecureTunnel {
    config: TunnelConfig,
    connection: Arc<RwLock<Option<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>>,
    encryption_key: Arc<RwLock<Option<EncryptionKey>>>,
    key_pair: Arc<RwLock<Option<KeyPair>>>,
    message_sender: mpsc::UnboundedSender<ProtocolMessage>,
    message_handler: Arc<RwLock<Option<MessageHandler>>>,
    session_id: Arc<RwLock<Option<SessionID>>>,
}

impl SecureTunnel {
    /// Create a new secure tunnel instance
    pub fn new(config: TunnelConfig) -> Self {
        let (sender, _receiver) = mpsc::unbounded_channel();

        Self {
            config,
            connection: Arc::new(RwLock::new(None)),
            encryption_key: Arc::new(RwLock::new(None)),
            key_pair: Arc::new(RwLock::new(None)),
            message_sender: sender,
            message_handler: Arc::new(RwLock::new(None)),
            session_id: Arc::new(RwLock::new(None)),
        }
    }

    /// Connect to the tunnel server
    pub async fn connect(&self) -> Result<()> {
        let url = if self.config.server_url.is_empty() {
            DEFAULT_TUNNEL_URL.to_string()
        } else {
            self.config.server_url.clone()
        };

        let (ws_stream, _) = connect_async(&url)
            .await
            .map_err(|e| MobileError::tunnel_connection(format!("Failed to connect: {}", e)))?;

        // Store connection
        let mut conn = self.connection.write().await;
        *conn = Some(ws_stream);
        drop(conn);

        // Generate key pair
        let key_pair = KeyPair::generate()?;
        let mut kp = self.key_pair.write().await;
        *kp = Some(key_pair);
        drop(kp);

        // Send handshake
        let mut device_info = DeviceInfo::new(
            self.config.device_type,
            self.config.device_name.clone(),
            self.config.os_version.clone(),
            self.config.app_version.clone(),
        );
        device_info.update_last_seen();

        let kp = self.key_pair.read().await;
        let kp_ref = kp.as_ref().ok_or_else(|| MobileError::crypto("Key pair not initialized"))?;

        let handshake = ProtocolMessage::handshake(self.config.device_id, kp_ref, device_info);
        self.send_message(handshake).await?;

        Ok(())
    }

    /// Disconnect from the tunnel
    pub async fn disconnect(&self) -> Result<()> {
        let mut conn = self.connection.write().await;
        *conn = None;
        Ok(())
    }

    /// Check if connected
    pub async fn is_connected(&self) -> bool {
        self.connection.read().await.is_some()
    }

    /// Send a protocol message
    pub async fn send_message(&self, message: ProtocolMessage) -> Result<()> {
        let conn = self.connection.read().await;
        let _ws = conn
            .as_ref()
            .ok_or_else(|| MobileError::NotConnected)?;

        let message_type = match &message {
            ProtocolMessage::Handshake { .. } => "handshake",
            ProtocolMessage::KeyExchange { .. } => "key_exchange",
            ProtocolMessage::Ping { .. } => "ping",
            ProtocolMessage::Pong { .. } => "pong",
            ProtocolMessage::DocumentUpdate { .. } => "document_update",
            ProtocolMessage::Notification { .. } => "notification",
            ProtocolMessage::Command { .. } => "command",
            ProtocolMessage::CommandResponse { .. } => "command_response",
            ProtocolMessage::SyncProgress { .. } => "sync_progress",
            ProtocolMessage::Error { .. } => "error",
        };

        // Serialize message
        let serialized = serde_json::to_vec(&message)
            .map_err(|e| MobileError::Serialization(e.to_string()))?;

        // Encrypt if we have an encryption key
        let final_message = if let Some(key) = self.encryption_key.read().await.as_ref() {
            let encryptor = Encryptor::new(key)?;
            let encrypted = encryptor.encrypt(&serialized)?;

            serde_json::to_vec(&EncryptedProtocolMessage {
                message_type: message_type.to_string(),
                encrypted,
            })
            .map_err(|e| MobileError::Serialization(e.to_string()))?
        } else {
            serialized
        };

        // Check message size
        if final_message.len() > MAX_MESSAGE_SIZE {
            return Err(MobileError::Protocol(format!(
                "Message too large: {} bytes (max: {})",
                final_message.len(),
                MAX_MESSAGE_SIZE
            )));
        }

        // Send via WebSocket
        let mut conn_guard = self.connection.write().await;
        if let Some(ws) = conn_guard.as_mut() {
            ws.send(Message::Binary(final_message))
                .await
                .map_err(|e| MobileError::Network(e.to_string()))?;
        }

        Ok(())
    }

    /// Send a document update
    pub async fn send_document_update(
        &self,
        document_id: DocumentID,
        metadata: DocumentMetadata,
        content: Option<String>,
    ) -> Result<()> {
        let message = ProtocolMessage::DocumentUpdate {
            document_id,
            metadata,
            content,
        };
        self.send_message(message).await
    }

    /// Send a notification
    pub async fn send_notification(&self, notification: Notification) -> Result<()> {
        let message = ProtocolMessage::Notification { notification };
        self.send_message(message).await
    }

    /// Send a remote command
    pub async fn send_command(&self, command: RemoteCommand) -> Result<()> {
        let message = ProtocolMessage::Command { command };
        self.send_message(message).await
    }

    /// Send a command response
    pub async fn send_command_response(&self, response: CommandResponse) -> Result<()> {
        let message = ProtocolMessage::CommandResponse { response };
        self.send_message(message).await
    }

    /// Send sync progress
    pub async fn send_sync_progress(&self, progress: SyncProgress) -> Result<()> {
        let message = ProtocolMessage::SyncProgress { progress };
        self.send_message(message).await
    }

    /// Send a ping
    pub async fn ping(&self) -> Result<()> {
        self.send_message(ProtocolMessage::ping()).await
    }

    /// Set message handler
    pub fn set_message_handler(&self, handler: MessageHandler) {
        let mut h = self.message_handler.blocking_write();
        *h = Some(handler);
    }

    /// Get session ID
    pub async fn session_id(&self) -> Option<SessionID> {
        *self.session_id.read().await
    }

    /// Get connection info
    pub async fn connection_info(&self) -> Result<ConnectionInfo> {
        let device_info = DeviceInfo::new(
            self.config.device_type,
            self.config.device_name.clone(),
            self.config.os_version.clone(),
            self.config.app_version.clone(),
        );

        let mut info = ConnectionInfo::new(device_info);

        if self.is_connected().await {
            info.set_connected();
            if let Some(sid) = *self.session_id.read().await {
                info.connection_id = sid;
            }
        }

        Ok(info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_tunnel_config() {
        let config = TunnelConfig::new(
            "wss://test.com".to_string(),
            Uuid::new_v4(),
            &[0u8; 32],
        );
        assert_eq!(config.server_url, "wss://test.com");
    }

    #[test]
    fn test_tunnel_config_builder() {
        let config = TunnelConfig::new(
            "wss://test.com".to_string(),
            Uuid::new_v4(),
            &[0u8; 32],
        )
        .with_device_type(DeviceType::Android)
        .with_device_name("Pixel 7".to_string())
        .with_os_version("Android 14".to_string());

        assert_eq!(config.device_type, DeviceType::Android);
        assert_eq!(config.device_name, "Pixel 7");
    }

    #[test]
    fn test_protocol_message_serialization() {
        let message = ProtocolMessage::Ping {
            timestamp: Utc::now(),
        };
        let json = serde_json::to_string(&message).unwrap();
        let deserialized: ProtocolMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            ProtocolMessage::Ping { .. } => (),
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_protocol_message_handshake() {
        let key_pair = KeyPair::generate().unwrap();
        let device_info = DeviceInfo::new(
            DeviceType::Ios,
            "iPhone 14".to_string(),
            "iOS 17.0".to_string(),
            "1.0.0".to_string(),
        );

        let message = ProtocolMessage::handshake(Uuid::new_v4(), &key_pair, device_info);

        match message {
            ProtocolMessage::Handshake { protocol_version, .. } => {
                assert_eq!(protocol_version, PROTOCOL_VERSION);
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_message_types() {
        let cmd = ProtocolMessage::Command {
            command: RemoteCommand::new(
                AppType::Writer,
                "save".to_string(),
                serde_json::json!({}),
            ),
        };
        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: ProtocolMessage = serde_json::from_str(&json).unwrap();
        match deserialized {
            ProtocolMessage::Command { .. } => (),
            _ => panic!("Wrong message type"),
        }
    }
}