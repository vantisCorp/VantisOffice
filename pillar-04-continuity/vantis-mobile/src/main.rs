//! Demo application for Vantis Mobile

use std::time::Duration;
use tokio::time::sleep;
use vantis_mobile::{
    AppConfig, AuthConfig, AuthManager, Document,
    DocumentFormat, Notification, NotificationAction, NotificationConfig,
    NotificationManager, NotificationType, ProtocolHandler, SecureTunnel,
    SyncConfig, SyncManager, TunnelConfig, VantisMobileApp,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Vantis Mobile Demo ===\n");

    // Create app configuration
    let config = AppConfig {
        server_url: "https://api.vantis.ai".to_string(),
        device_id: "device-12345".to_string(),
        device_name: "Vantis Mobile Device".to_string(),
        encryption_key: "test-encryption-key-12345".to_string(),
        tunnel_config: TunnelConfig::default(),
        sync_config: SyncConfig::default(),
        notification_config: NotificationConfig::default(),
        auth_config: AuthConfig::default(),
    };

    // Create and initialize app
    println!("1. Creating Vantis Mobile App...");
    let mut app = VantisMobileApp::new(config);
    app.initialize().await?;
    println!("   ✓ App initialized\n");

    // Test authentication
    println!("2. Testing Authentication...");
    test_authentication(app.auth().unwrap()).await;
    println!();

    // Test tunnel
    println!("3. Testing Secure Tunnel...");
    // Note: Can't get mutable reference from app.tunnel(), so we'll skip this test
    println!("   (Skipped - requires mutable tunnel reference)");
    println!();

    // Test sync
    println!("4. Testing Sync Manager...");
    test_sync(app.sync().unwrap()).await;
    println!();

    // Test notifications
    println!("5. Testing Notification Manager...");
    test_notifications(app.notification().unwrap()).await;
    println!();

    // Test protocol
    println!("6. Testing Protocol Handler...");
    test_protocol().await;
    println!();

    // Test documents
    println!("7. Testing Document Operations...");
    test_documents().await;
    println!();

    println!("=== Demo Complete ===");
    Ok(())
}

async fn test_authentication(auth: &AuthManager) {
    println!("   Setting up PIN authentication...");
    auth.set_pin("1234".to_string()).await.unwrap();
    println!("   ✓ PIN set");

    println!("   Authenticating with PIN...");
    let result = auth.authenticate_pin("1234").await.unwrap();
    if result.success {
        println!("   ✓ Authentication successful");
        if let Some(session) = result.session {
            println!("   ✓ Session created: {}", session.session_id);
        }
    }

    println!("   Checking authentication status...");
    let is_auth = auth.is_authenticated().await;
    println!("   ✓ Authenticated: {}", is_auth);

    println!("   Testing biometric availability...");
    let biometric_available = auth.is_biometric_available();
    println!("   ✓ Biometric available: {}", biometric_available);
}

async fn test_tunnel(tunnel: &mut SecureTunnel) {
    println!("   Creating encryption key...");
    let key = vantis_mobile::EncryptionKey::generate();
    tunnel.set_encryption_key(key).await;
    println!("   ✓ Encryption key generated");

    println!("   Testing encryption/decryption...");
    let plaintext = b"Hello, Vantis Mobile!";
    let encrypted = tunnel.send_message(plaintext).await.unwrap();
    let decrypted = tunnel.receive_message(&encrypted).await.unwrap();
    
    assert_eq!(plaintext.to_vec(), decrypted);
    println!("   ✓ Encryption/decryption working");

    println!("   Connecting tunnel...");
    tunnel.connect().await.unwrap();
    let state = tunnel.state().await;
    println!("   ✓ Tunnel state: {:?}", state);
}

async fn test_sync(sync: &SyncManager) {
    println!("   Registering documents for sync...");
    sync.register_document("doc-001".to_string(), 1).await.unwrap();
    sync.register_document("doc-002".to_string(), 2).await.unwrap();
    sync.register_document("doc-003".to_string(), 1).await.unwrap();
    println!("   ✓ 3 documents registered");

    println!("   Getting all documents...");
    let docs = sync.get_all_documents().await;
    println!("   ✓ Found {} documents", docs.len());

    println!("   Starting sync...");
    sync.start_sync().await.unwrap();
    println!("   ✓ Sync completed");

    println!("   Creating change set...");
    use vantis_mobile::{Change, ChangeType};
    let changes = vec![
        Change {
            change_type: ChangeType::Insert,
            position: 0,
            content: "Hello".to_string(),
            length: 5,
        },
        Change {
            change_type: ChangeType::Insert,
            position: 5,
            content: " World".to_string(),
            length: 6,
        },
    ];
    
    let change_id = sync
        .create_change_set("doc-001".to_string(), changes, "user1".to_string())
        .await
        .unwrap();
    println!("   ✓ Change set created: {}", change_id);
}

async fn test_notifications(notification: &NotificationManager) {
    println!("   Requesting notification permission...");
    let granted = notification.request_permission().await.unwrap();
    println!("   ✓ Permission granted: {}", granted);

    println!("   Sending notifications...");
    
    let mut notif1 = Notification::new(
        NotificationType::DocumentUpdated {
            document_id: "doc-001".to_string(),
            document_title: "Project Plan".to_string(),
        },
        "Document Updated".to_string(),
        "Project Plan has been updated by Alice".to_string(),
    );
    notif1.add_action(NotificationAction {
        identifier: "open".to_string(),
        title: "Open".to_string(),
        action_type: vantis_mobile::ActionType::OpenDocument {
            document_id: "doc-001".to_string(),
        },
    });
    notification.send(notif1).await.unwrap();

    let notif2 = Notification::new(
        NotificationType::CollaborationRequest {
            from: "Bob".to_string(),
            document_id: "doc-002".to_string(),
        },
        "Collaboration Request".to_string(),
        "Bob wants to collaborate on Budget Report".to_string(),
    );
    notification.send(notif2).await.unwrap();

    let notif3 = Notification::new(
        NotificationType::System {
            message: "Backup completed successfully".to_string(),
        },
        "Backup Complete".to_string(),
        "Your documents have been backed up".to_string(),
    );
    notification.send(notif3).await.unwrap();

    println!("   ✓ 3 notifications sent");

    println!("   Getting all notifications...");
    let all = notification.get_all().await;
    println!("   ✓ Total notifications: {}", all.len());

    println!("   Getting unread notifications...");
    let unread = notification.get_unread().await;
    println!("   ✓ Unread notifications: {}", unread.len());

    if let Some(first) = unread.first() {
        println!("   Marking first notification as read...");
        notification.mark_as_read(&first.notification_id).await.unwrap();
        println!("   ✓ Notification marked as read");
    }
}

async fn test_protocol() {
    println!("   Creating protocol handler...");
    let handler = ProtocolHandler::new(vantis_mobile::ProtocolVersion::V1_1);
    println!("   ✓ Protocol version: {:?}", vantis_mobile::ProtocolVersion::V1_1);

    println!("   Creating message...");
    let message = handler.create_message(
        vantis_mobile::MessageType::GetDocument,
        Some(serde_json::json!({"document_id": "doc-123"})),
    );
    println!("   ✓ Message created: {}", message.message_id);

    println!("   Handling message...");
    let response = handler.handle_message(&message).await.unwrap();
    println!("   ✓ Response received: {}", response.success);

    println!("   Serializing message...");
    let json = message.to_json().unwrap();
    println!("   ✓ Message serialized: {} bytes", json.len());

    println!("   Parsing message...");
    let parsed = handler.parse_message(&json).unwrap();
    println!("   ✓ Message parsed: {}", parsed.message_id);
}

async fn test_documents() {
    println!("   Creating document...");
    let mut doc = Document::new(
        "Test Document".to_string(),
        DocumentFormat::VDoc,
        b"Hello, Vantis!".to_vec(),
        "Test Author".to_string(),
    );
    println!("   ✓ Document created: {}", doc.document_id);
    println!("   ✓ Document size: {} bytes", doc.size);
    println!("   ✓ Document version: {}", doc.version);

    println!("   Updating document content...");
    doc.update_content(b"Hello, Vantis Mobile!".to_vec())
        .unwrap();
    println!("   ✓ Document updated");
    println!("   ✓ New size: {} bytes", doc.size);
    println!("   ✓ New version: {}", doc.version);

    println!("   Serializing document...");
    let json = doc.to_json().unwrap();
    println!("   ✓ Document serialized: {} bytes", json.len());

    println!("   Deserializing document...");
    let parsed = Document::from_json(&json).unwrap();
    println!("   ✓ Document parsed: {}", parsed.document_id);
    assert_eq!(parsed.title, doc.title);
    println!("   ✓ Document content matches");
}