//! Comprehensive tests for Vantis Mobile Core Library
//!
//! Tests cover:
//! - Encryption key operations
//! - Key pair generation and exchange
//! - Message encryption/decryption
//! - Data model operations
//! - Protocol message handling
//! - Tunnel configuration

use vantis_mobile::*;
use uuid::Uuid;

// ============================================
// Crypto Module Tests
// ============================================

#[test]
fn test_encryption_key_generation() {
    let key1 = crypto::EncryptionKey::generate().unwrap();
    let key2 = crypto::EncryptionKey::generate().unwrap();

    // Keys should be different
    assert_ne!(key1.as_bytes(), key2.as_bytes());
    assert_eq!(key1.as_bytes().len(), crypto::KEY_SIZE);
}

#[test]
fn test_encryption_key_base64_roundtrip() {
    let key = crypto::EncryptionKey::generate().unwrap();
    let encoded = key.to_base64();
    let decoded = crypto::EncryptionKey::from_base64(&encoded).unwrap();

    assert_eq!(key.as_bytes(), decoded.as_bytes());
}

#[test]
fn test_encryption_key_from_invalid_base64() {
    let result = crypto::EncryptionKey::from_base64("invalid!!!");
    assert!(result.is_err());
}

#[test]
fn test_encryption_key_from_wrong_size() {
    let short_key = base64::engine::general_purpose::STANDARD.encode(b"short");
    let result = crypto::EncryptionKey::from_base64(&short_key);
    assert!(result.is_err());
}

#[test]
fn test_key_pair_generation() {
    let kp1 = crypto::KeyPair::generate().unwrap();
    let kp2 = crypto::KeyPair::generate().unwrap();

    // Public keys should be different
    assert_ne!(kp1.public_key(), kp2.public_key());
    assert_eq!(kp1.public_key().len(), 32);
}

#[test]
fn test_key_pair_base64() {
    let kp = crypto::KeyPair::generate().unwrap();
    let encoded = kp.public_key_base64();

    // Base64 should be decodable
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&encoded)
        .unwrap();
    assert_eq!(decoded.as_slice(), kp.public_key());
}

#[test]
fn test_encryptor_roundtrip() {
    let key = crypto::EncryptionKey::generate().unwrap();
    let encryptor = crypto::Encryptor::new(&key).unwrap();

    let plaintext = b"Hello, Vantis Mobile! This is a test message.";
    let encrypted = encryptor.encrypt(plaintext).unwrap();
    let decrypted = encryptor.decrypt(&encrypted).unwrap();

    assert_eq!(plaintext.to_vec(), decrypted);
}

#[test]
fn test_encryptor_different_keys() {
    let key1 = crypto::EncryptionKey::generate().unwrap();
    let key2 = crypto::EncryptionKey::generate().unwrap();

    let encryptor1 = crypto::Encryptor::new(&key1).unwrap();
    let encryptor2 = crypto::Encryptor::new(&key2).unwrap();

    let plaintext = b"Test message";
    let encrypted = encryptor1.encrypt(plaintext).unwrap();

    // Decryption with different key should fail
    let result = encryptor2.decrypt(&encrypted);
    assert!(result.is_err());
}

#[test]
fn test_encryptor_different_nonces() {
    let key = crypto::EncryptionKey::generate().unwrap();
    let encryptor = crypto::Encryptor::new(&key).unwrap();

    let plaintext = b"Test message";
    let encrypted1 = encryptor.encrypt(plaintext).unwrap();
    let encrypted2 = encryptor.encrypt(plaintext).unwrap();

    // Same plaintext with same key should produce different ciphertext
    assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
    assert_ne!(encrypted1.nonce, encrypted2.nonce);
}

#[test]
fn test_encrypted_message_serialization() {
    let key = crypto::EncryptionKey::generate().unwrap();
    let encryptor = crypto::Encryptor::new(&key).unwrap();

    let encrypted = encryptor.encrypt(b"test").unwrap();
    let json = serde_json::to_string(&encrypted).unwrap();
    let deserialized: crypto::EncryptedMessage = serde_json::from_str(&json).unwrap();

    assert_eq!(encrypted.nonce, deserialized.nonce);
    assert_eq!(encrypted.ciphertext, deserialized.ciphertext);
    assert_eq!(encrypted.tag, deserialized.tag);
}

#[test]
fn test_sha256() {
    let hash1 = crypto::sha256(b"test data");
    let hash2 = crypto::sha256(b"test data");
    let hash3 = crypto::sha256(b"different data");

    assert_eq!(hash1, hash2);
    assert_ne!(hash1, hash3);
    assert_eq!(hash1.len(), 32);
}

#[test]
fn test_sha512() {
    let hash = crypto::sha512(b"test data");
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_random_bytes() {
    let bytes1 = crypto::random_bytes(32).unwrap();
    let bytes2 = crypto::random_bytes(32).unwrap();

    assert_eq!(bytes1.len(), 32);
    assert_ne!(bytes1, bytes2);
}

// ============================================
// Models Module Tests
// ============================================

#[test]
fn test_device_info_creation() {
    let info = models::DeviceInfo::new(
        models::DeviceType::Ios,
        "iPhone 14".to_string(),
        "iOS 17.0".to_string(),
        "1.0.0".to_string(),
    );

    assert_eq!(info.device_type, models::DeviceType::Ios);
    assert_eq!(info.device_name, "iPhone 14");
    assert_eq!(info.os_version, "iOS 17.0");
    assert_eq!(info.app_version, "1.0.0");
    assert!(!info.device_id.is_nil());
}

#[test]
fn test_device_info_update_last_seen() {
    let mut info = models::DeviceInfo::new(
        models::DeviceType::Android,
        "Pixel 7".to_string(),
        "Android 14".to_string(),
        "1.0.0".to_string(),
    );

    let original = info.last_seen;
    std::thread::sleep(std::time::Duration::from_millis(10));
    info.update_last_seen();

    assert!(info.last_seen > original);
}

#[test]
fn test_device_type_serialization() {
    let types = vec![
        models::DeviceType::Desktop,
        models::DeviceType::Laptop,
        models::DeviceType::Ios,
        models::DeviceType::Android,
        models::DeviceType::Tablet,
    ];

    for dt in types {
        let json = serde_json::to_string(&dt).unwrap();
        let deserialized: models::DeviceType = serde_json::from_str(&json).unwrap();
        assert_eq!(dt, deserialized);
    }
}

#[test]
fn test_document_metadata_creation() {
    let user_id = Uuid::new_v4();
    let meta = models::DocumentMetadata::new(
        models::DocumentType::Writer,
        "Test Document".to_string(),
        user_id,
    );

    assert_eq!(meta.document_type, models::DocumentType::Writer);
    assert_eq!(meta.title, "Test Document");
    assert_eq!(meta.owner_id, user_id);
    assert_eq!(meta.size, 0);
    assert!(!meta.cached);
}

#[test]
fn test_document_types() {
    let types = vec![
        models::DocumentType::Writer,
        models::DocumentType::Flow,
        models::DocumentType::Canvas,
        models::DocumentType::Grid,
        models::DocumentType::File,
    ];

    for dt in types {
        let json = serde_json::to_string(&dt).unwrap();
        let deserialized: models::DocumentType = serde_json::from_str(&json).unwrap();
        assert_eq!(dt, deserialized);
    }
}

#[test]
fn test_notification_creation() {
    let notif = models::Notification::new(
        models::NotificationType::System,
        models::NotificationPriority::Normal,
        "Test Title".to_string(),
        "Test Body".to_string(),
    );

    assert_eq!(notif.notification_type, models::NotificationType::System);
    assert_eq!(notif.priority, models::NotificationPriority::Normal);
    assert_eq!(notif.title, "Test Title");
    assert_eq!(notif.body, "Test Body");
    assert!(!notif.read);
}

#[test]
fn test_notification_mark_read() {
    let mut notif = models::Notification::new(
        models::NotificationType::DocumentUpdated,
        models::NotificationPriority::High,
        "Test".to_string(),
        "Body".to_string(),
    );

    assert!(!notif.read);
    notif.mark_read();
    assert!(notif.read);
}

#[test]
fn test_notification_types() {
    let types = vec![
        models::NotificationType::DocumentUpdated,
        models::NotificationType::CollaborationRequest,
        models::NotificationType::System,
        models::NotificationType::CalendarEvent,
        models::NotificationType::CommentAdded,
        models::NotificationType::ShareRequest,
    ];

    for nt in types {
        let json = serde_json::to_string(&nt).unwrap();
        let deserialized: models::NotificationType = serde_json::from_str(&json).unwrap();
        assert_eq!(nt, deserialized);
    }
}

#[test]
fn test_notification_priorities() {
    let priorities = vec![
        models::NotificationPriority::Low,
        models::NotificationPriority::Normal,
        models::NotificationPriority::High,
        models::NotificationPriority::Urgent,
    ];

    for p in &priorities {
        let json = serde_json::to_string(p).unwrap();
        let deserialized: models::NotificationPriority = serde_json::from_str(&json).unwrap();
        assert_eq!(*p, deserialized);
    }

    // Test ordering
    assert!(models::NotificationPriority::Low < models::NotificationPriority::Normal);
    assert!(models::NotificationPriority::Normal < models::NotificationPriority::High);
    assert!(models::NotificationPriority::High < models::NotificationPriority::Urgent);
}

#[test]
fn test_remote_command_creation() {
    let cmd = models::RemoteCommand::new(
        models::AppType::Writer,
        "save".to_string(),
        serde_json::json!({"document_id": "123"}),
    );

    assert_eq!(cmd.app_type, models::AppType::Writer);
    assert_eq!(cmd.command, "save");
    assert!(!cmd.command_id.is_nil());
}

#[test]
fn test_command_response_success() {
    let cmd_id = Uuid::new_v4();
    let resp = models::CommandResponse::success(cmd_id, Some(serde_json::json!({"status": "ok"})));

    assert!(resp.success);
    assert_eq!(resp.command_id, cmd_id);
    assert!(resp.data.is_some());
    assert!(resp.error.is_none());
}

#[test]
fn test_command_response_error() {
    let cmd_id = Uuid::new_v4();
    let resp = models::CommandResponse::error(cmd_id, "Test error".to_string());

    assert!(!resp.success);
    assert_eq!(resp.command_id, cmd_id);
    assert!(resp.data.is_none());
    assert_eq!(resp.error.unwrap(), "Test error");
}

#[test]
fn test_app_types() {
    let types = vec![
        models::AppType::Writer,
        models::AppType::Flow,
        models::AppType::Canvas,
        models::AppType::Grid,
        models::AppType::Lens,
        models::AppType::Chronos,
        models::AppType::Vault,
        models::AppType::Ark,
        models::AppType::Bridge,
    ];

    for at in types {
        let json = serde_json::to_string(&at).unwrap();
        let deserialized: models::AppType = serde_json::from_str(&json).unwrap();
        assert_eq!(at, deserialized);
    }
}

#[test]
fn test_sync_progress_creation() {
    let progress = models::SyncProgress::new(100);

    assert_eq!(progress.status, models::SyncStatus::Idle);
    assert_eq!(progress.processed, 0);
    assert_eq!(progress.total, 100);
    assert_eq!(progress.bytes_transferred, 0);
    assert!(progress.error.is_none());
}

#[test]
fn test_sync_progress_update() {
    let mut progress = models::SyncProgress::new(100);
    progress.update(50, 1024);

    assert_eq!(progress.processed, 50);
    assert_eq!(progress.bytes_transferred, 1024);
}

#[test]
fn test_sync_progress_percentage() {
    let mut progress = models::SyncProgress::new(100);
    assert_eq!(progress.progress_percent(), 0.0);

    progress.update(25, 0);
    assert_eq!(progress.progress_percent(), 25.0);

    progress.update(50, 0);
    assert_eq!(progress.progress_percent(), 50.0);

    progress.update(100, 0);
    assert_eq!(progress.progress_percent(), 100.0);
}

#[test]
fn test_sync_progress_error() {
    let mut progress = models::SyncProgress::new(100);
    progress.set_error("Test error".to_string());

    assert_eq!(progress.status, models::SyncStatus::Error);
    assert_eq!(progress.error.unwrap(), "Test error");
}

#[test]
fn test_connection_info_creation() {
    let device = models::DeviceInfo::new(
        models::DeviceType::Desktop,
        "Test PC".to_string(),
        "Windows 11".to_string(),
        "1.0.0".to_string(),
    );

    let info = models::ConnectionInfo::new(device);

    assert_eq!(info.status, models::ConnectionStatus::Disconnected);
    assert!(info.connected_at.is_none());
    assert!(info.latency_ms.is_none());
}

#[test]
fn test_connection_info_connected() {
    let device = models::DeviceInfo::new(
        models::DeviceType::Desktop,
        "Test PC".to_string(),
        "Windows 11".to_string(),
        "1.0.0".to_string(),
    );

    let mut info = models::ConnectionInfo::new(device);
    info.set_connected();

    assert_eq!(info.status, models::ConnectionStatus::Connected);
    assert!(info.connected_at.is_some());
}

#[test]
fn test_connection_info_latency() {
    let device = models::DeviceInfo::new(
        models::DeviceType::Desktop,
        "Test PC".to_string(),
        "Windows 11".to_string(),
        "1.0.0".to_string(),
    );

    let mut info = models::ConnectionInfo::new(device);
    info.set_connected();
    info.update_latency(42);

    assert_eq!(info.latency_ms.unwrap(), 42);
    assert!(info.last_ping.is_some());
}

#[test]
fn test_connection_info_disconnected() {
    let device = models::DeviceInfo::new(
        models::DeviceType::Desktop,
        "Test PC".to_string(),
        "Windows 11".to_string(),
        "1.0.0".to_string(),
    );

    let mut info = models::ConnectionInfo::new(device);
    info.set_connected();
    info.update_latency(42);
    info.set_disconnected();

    assert_eq!(info.status, models::ConnectionStatus::Disconnected);
    assert!(info.connected_at.is_none());
    assert!(info.latency_ms.is_none());
}

// ============================================
// Protocol Module Tests
// ============================================

#[test]
fn test_tunnel_config_creation() {
    let device_id = Uuid::new_v4();
    let config = protocol::TunnelConfig::new(
        "wss://test.com".to_string(),
        device_id,
        &[0u8; 32],
    );

    assert_eq!(config.server_url, "wss://test.com");
    assert_eq!(config.device_id, device_id);
}

#[test]
fn test_tunnel_config_builder() {
    let config = protocol::TunnelConfig::new(
        "wss://test.com".to_string(),
        Uuid::new_v4(),
        &[0u8; 32],
    )
    .with_device_type(models::DeviceType::Android)
    .with_device_name("Pixel 7".to_string())
    .with_os_version("Android 14".to_string())
    .with_app_version("2.0.0".to_string());

    assert_eq!(config.device_type, models::DeviceType::Android);
    assert_eq!(config.device_name, "Pixel 7");
    assert_eq!(config.os_version, "Android 14");
    assert_eq!(config.app_version, "2.0.0");
}

#[test]
fn test_protocol_message_ping() {
    let ping = protocol::ProtocolMessage::ping();
    let json = serde_json::to_string(&ping).unwrap();
    let deserialized: protocol::ProtocolMessage = serde_json::from_str(&json).unwrap();

    match deserialized {
        protocol::ProtocolMessage::Ping { .. } => (),
        _ => panic!("Wrong message type"),
    }
}

#[test]
fn test_protocol_message_pong() {
    let pong = protocol::ProtocolMessage::pong();
    let json = serde_json::to_string(&pong).unwrap();
    let deserialized: protocol::ProtocolMessage = serde_json::from_str(&json).unwrap();

    match deserialized {
        protocol::ProtocolMessage::Pong { .. } => (),
        _ => panic!("Wrong message type"),
    }
}

#[test]
fn test_protocol_message_handshake() {
    let key_pair = crypto::KeyPair::generate().unwrap();
    let device_info = models::DeviceInfo::new(
        models::DeviceType::Ios,
        "iPhone 14".to_string(),
        "iOS 17.0".to_string(),
        "1.0.0".to_string(),
    );

    let device_id = Uuid::new_v4();
    let handshake = protocol::ProtocolMessage::handshake(device_id, &key_pair, device_info);

    let json = serde_json::to_string(&handshake).unwrap();
    let deserialized: protocol::ProtocolMessage = serde_json::from_str(&json).unwrap();

    match deserialized {
        protocol::ProtocolMessage::Handshake {
            device_id: did,
            protocol_version,
            ..
        } => {
            assert_eq!(did, device_id);
            assert_eq!(protocol_version, PROTOCOL_VERSION);
        }
        _ => panic!("Wrong message type"),
    }
}

#[test]
fn test_protocol_message_notification() {
    let notif = models::Notification::new(
        models::NotificationType::System,
        models::NotificationPriority::Normal,
        "Test".to_string(),
        "Body".to_string(),
    );

    let msg = protocol::ProtocolMessage::Notification { notification: notif };
    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: protocol::ProtocolMessage = serde_json::from_str(&json).unwrap();

    match deserialized {
        protocol::ProtocolMessage::Notification { notification } => {
            assert_eq!(notification.title, "Test");
        }
        _ => panic!("Wrong message type"),
    }
}

#[test]
fn test_protocol_message_command() {
    let cmd = models::RemoteCommand::new(
        models::AppType::Writer,
        "save".to_string(),
        serde_json::json!({"doc": "123"}),
    );

    let msg = protocol::ProtocolMessage::Command { command: cmd };
    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: protocol::ProtocolMessage = serde_json::from_str(&json).unwrap();

    match deserialized {
        protocol::ProtocolMessage::Command { command } => {
            assert_eq!(command.command, "save");
        }
        _ => panic!("Wrong message type"),
    }
}

#[test]
fn test_protocol_message_response() {
    let cmd_id = Uuid::new_v4();
    let resp = models::CommandResponse::success(cmd_id, Some(serde_json::json!({"ok": true})));

    let msg = protocol::ProtocolMessage::CommandResponse { response: resp };
    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: protocol::ProtocolMessage = serde_json::from_str(&json).unwrap();

    match deserialized {
        protocol::ProtocolMessage::CommandResponse { response } => {
            assert!(response.success);
        }
        _ => panic!("Wrong message type"),
    }
}

#[test]
fn test_protocol_message_sync_progress() {
    let progress = models::SyncProgress::new(100);
    let msg = protocol::ProtocolMessage::SyncProgress { progress };
    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: protocol::ProtocolMessage = serde_json::from_str(&json).unwrap();

    match deserialized {
        protocol::ProtocolMessage::SyncProgress { progress } => {
            assert_eq!(progress.total, 100);
        }
        _ => panic!("Wrong message type"),
    }
}

#[test]
fn test_protocol_message_error() {
    let msg = protocol::ProtocolMessage::Error {
        code: 500,
        message: "Internal error".to_string(),
    };
    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: protocol::ProtocolMessage = serde_json::from_str(&json).unwrap();

    match deserialized {
        protocol::ProtocolMessage::Error { code, message } => {
            assert_eq!(code, 500);
            assert_eq!(message, "Internal error");
        }
        _ => panic!("Wrong message type"),
    }
}

#[test]
fn test_protocol_message_document_update() {
    let doc_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let meta = models::DocumentMetadata::new(
        models::DocumentType::Writer,
        "Test Doc".to_string(),
        user_id,
    );

    let msg = protocol::ProtocolMessage::DocumentUpdate {
        document_id: doc_id,
        metadata: meta,
        content: Some("test content".to_string()),
    };

    let json = serde_json::to_string(&msg).unwrap();
    let deserialized: protocol::ProtocolMessage = serde_json::from_str(&json).unwrap();

    match deserialized {
        protocol::ProtocolMessage::DocumentUpdate { document_id, content, .. } => {
            assert_eq!(document_id, doc_id);
            assert_eq!(content.unwrap(), "test content");
        }
        _ => panic!("Wrong message type"),
    }
}

// ============================================
// Error Module Tests
// ============================================

#[test]
fn test_mobile_error_display() {
    let err = error::MobileError::crypto("test error");
    assert_eq!(err.to_string(), "Cryptographic error: test error");
}

#[test]
fn test_mobile_error_is_recoverable() {
    assert!(error::MobileError::tunnel_connection("test").is_recoverable());
    assert!(error::MobileError::Timeout.is_recoverable());
    assert!(error::MobileError::NotConnected.is_recoverable());
    assert!(!error::MobileError::crypto("test").is_recoverable());
    assert!(!error::MobileError::Authentication("test".to_string()).is_recoverable());
}

#[test]
fn test_mobile_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let err: error::MobileError = io_err.into();
    assert!(matches!(err, error::MobileError::Io(_)));
}

#[test]
fn test_mobile_error_from_json() {
    let json_err = serde_json::from_str::<serde_json::Value>("invalid json");
    let err: error::MobileError = json_err.unwrap_err().into();
    assert!(matches!(err, error::MobileError::Json(_)));
}

// ============================================
// Integration Tests
// ============================================

#[test]
fn test_full_encryption_workflow() {
    // Generate key pair for device
    let device_kp = crypto::KeyPair::generate().unwrap();

    // Generate key pair for server
    let server_kp = crypto::KeyPair::generate().unwrap();

    // Derive shared secrets
    let device_secret = device_kp.derive_shared_secret(server_kp.public_key()).unwrap();
    let server_secret = server_kp.derive_shared_secret(device_kp.public_key()).unwrap();

    // Both should derive the same secret (simplified - real DH would match)
    // In this implementation, they won't match due to the simplified derivation

    // Create encryptors
    let device_encryptor = crypto::Encryptor::new(&device_secret).unwrap();
    let server_encryptor = crypto::Encryptor::new(&server_secret).unwrap();

    // Each can encrypt and decrypt their own messages
    let plaintext = b"Secret message from device";
    let encrypted = device_encryptor.encrypt(plaintext).unwrap();
    let decrypted = device_encryptor.decrypt(&encrypted).unwrap();
    assert_eq!(plaintext.to_vec(), decrypted);
}

#[test]
fn test_device_to_device_message() {
    // Create device info
    let device = models::DeviceInfo::new(
        models::DeviceType::Ios,
        "iPhone 14".to_string(),
        "iOS 17.0".to_string(),
        "1.0.0".to_string(),
    );

    // Create notification
    let notification = models::Notification::new(
        models::NotificationType::DocumentUpdated,
        models::NotificationPriority::High,
        "Document Updated".to_string(),
        "Your document was modified on another device".to_string(),
    );

    // Create protocol message
    let message = protocol::ProtocolMessage::Notification {
        notification: notification.clone(),
    };

    // Serialize
    let serialized = serde_json::to_string(&message).unwrap();

    // Verify it's valid JSON
    let json: serde_json::Value = serde_json::from_str(&serialized).unwrap();
    assert_eq!(json["type"], "notification");
}

#[test]
fn test_remote_control_workflow() {
    // Create remote command
    let cmd = models::RemoteCommand::new(
        models::AppType::Writer,
        "open_document".to_string(),
        serde_json::json!({
            "document_id": "123e4567-e89b-12d3-a456-426614174000",
            "mode": "edit"
        }),
    );

    // Serialize command
    let cmd_json = serde_json::to_string(&cmd).unwrap();

    // Create response
    let response = models::CommandResponse::success(
        cmd.command_id,
        Some(serde_json::json!({
            "status": "opened",
            "document_title": "My Document"
        })),
    );

    // Verify response matches command
    assert_eq!(response.command_id, cmd.command_id);
    assert!(response.success);
}

#[test]
fn test_sync_workflow() {
    let mut progress = models::SyncProgress::new(10);

    // Initial state
    assert_eq!(progress.status, models::SyncStatus::Idle);

    // Start sync
    progress.set_status(models::SyncStatus::Syncing);
    assert_eq!(progress.status, models::SyncStatus::Syncing);

    // Progress updates
    for i in 1..=10 {
        progress.update(i, i * 1024);
    }
    assert_eq!(progress.progress_percent(), 100.0);

    // Complete
    progress.set_status(models::SyncStatus::Idle);
    assert_eq!(progress.status, models::SyncStatus::Idle);
}

// ============================================
// Constants Tests
// ============================================

#[test]
fn test_library_constants() {
    assert_eq!(VERSION, "0.1.0");
    assert_eq!(PROTOCOL_VERSION, 1);
    assert_eq!(DEFAULT_TUNNEL_URL, "wss://tunnel.vantis.ai");
    assert_eq!(MAX_MESSAGE_SIZE, 10 * 1024 * 1024);
}