//! Vantis Link - P2P collaboration module
//! 
//! Main entry point for the Vantis Link application

use vantis_link::{Session, User, Document, Change, CrdtEngine, CrdtType, EncryptionManager, EncryptionAlgorithm, SyncManager, PeerDiscovery, Transport, TransportProtocol};
use vantis_link::core::{UserRole, ChangeType};
use vantis_link::crdt::CrdtOperationType;
use std::path::PathBuf;

fn main() {
    println!("Vantis Link v{}", vantis_link::VERSION);
    println!("P2P collaboration for VantisOffice\n");
    
    // Initialize subsystems
    if let Err(e) = vantis_link::init() {
        eprintln!("Initialization error: {}", e);
        std::process::exit(1);
    }
    
    println!("✓ Vantis Link initialized successfully\n");
    
    // Create a collaboration session
    let mut session = Session::new("document_123".to_string());
    session.metadata.name = Some("Team Meeting Notes".to_string());
    println!("✓ Created collaboration session: {}", session.id);
    
    // Add users to session
    let user1 = User::new("user_1".to_string(), "Alice".to_string())
        .with_email("alice@example.com".to_string())
        .with_role(UserRole::Owner);
    session.add_user(user1.clone()).unwrap();
    println!("✓ Added user: {} ({})", user1.name, user1.id);
    
    let user2 = User::new("user_2".to_string(), "Bob".to_string())
        .with_email("bob@example.com".to_string())
        .with_role(UserRole::Editor);
    session.add_user(user2.clone()).unwrap();
    println!("✓ Added user: {} ({})", user2.name, user2.id);
    
    let user3 = User::new("user_3".to_string(), "Charlie".to_string())
        .with_email("charlie@example.com".to_string())
        .with_role(UserRole::Viewer);
    session.add_user(user3.clone()).unwrap();
    println!("✓ Added user: {} ({})", user3.name, user3.id);
    
    println!("  Total users in session: {}\n", session.user_count());
    
    // Create a document
    let mut document = Document::new("document_123".to_string(), "Meeting Notes".to_string());
    document.content = "Welcome to the meeting notes.\n\nAgenda:\n1. Project update\n2. Review milestones\n3. Next steps".to_string();
    println!("✓ Created document: {}", document.name);
    println!("  Content length: {} characters\n", document.content.len());
    
    // Test CRDT Engine
    println!("Testing CRDT Engine:");
    let crdt_engine = CrdtEngine::new(CrdtType::Rga);
    println!("  ✓ CRDT engine created (type: RGA)");
    
    // Create and apply CRDT operations
    let op1 = crdt_engine.create_operation(
        user1.id.clone(),
        CrdtOperationType::Insert,
        document.content.len(),
        "\n4. Q&A session".to_string()
    ).unwrap();
    println!("  ✓ Created CRDT operation: {}", op1.id);
    
    crdt_engine.apply_operation(&mut document, &op1).unwrap();
    println!("  ✓ Applied CRDT operation");
    println!("  New content length: {} characters\n", document.content.len());
    
    // Test Encryption Manager
    println!("Testing Encryption Manager:");
    let enc_manager = EncryptionManager::new(EncryptionAlgorithm::ChaCha20Poly1305);
    println!("  ✓ Encryption manager created (algorithm: ChaCha20Poly1305)");
    
    let key = enc_manager.generate_key("session_key".to_string()).unwrap();
    println!("  ✓ Generated encryption key: {}", key.id);
    
    let plaintext = "This is a secret message";
    let encrypted = enc_manager.encrypt(plaintext, "session_key").unwrap();
    println!("  ✓ Encrypted message: {}...", &encrypted[..40]);
    
    let decrypted = enc_manager.decrypt(&encrypted, "session_key").unwrap();
    println!("  ✓ Decrypted message: {}", decrypted);
    println!();
    
    // Test Sync Manager
    println!("Testing Sync Manager:");
    let sync_manager = SyncManager::new();
    println!("  ✓ Sync manager created");
    
    let sync_session_id = sync_manager.create_session(document.id.clone()).unwrap();
    println!("  ✓ Created sync session: {}", sync_session_id);
    
    sync_manager.join_session(sync_session_id.clone(), user1.id.clone()).unwrap();
    println!("  ✓ User {} joined sync session", user1.name);
    
    sync_manager.join_session(sync_session_id.clone(), user2.id.clone()).unwrap();
    println!("  ✓ User {} joined sync session", user2.name);
    
    let sync_status = sync_manager.get_sync_status(sync_session_id.clone()).unwrap();
    println!("  ✓ Sync status: {:?}\n", sync_status);
    
    // Test Peer Discovery
    println!("Testing Peer Discovery:");
    let mut peer_discovery = PeerDiscovery::new();
    println!("  ✓ Peer discovery created");
    
    let local_peer = vantis_link::discovery::PeerInfo::new(
        "peer_1".to_string(),
        "Alice's Device".to_string(),
        "192.168.1.100".to_string(),
        8080
    ).with_capabilities(vec!["editing".to_string(), "sync".to_string()]);
    peer_discovery.set_local_peer(local_peer.clone());
    println!("  ✓ Set local peer: {} ({})", local_peer.name, local_peer.endpoint());
    
    let remote_peer = vantis_link::discovery::PeerInfo::new(
        "peer_2".to_string(),
        "Bob's Device".to_string(),
        "192.168.1.101".to_string(),
        8080
    ).with_capabilities(vec!["editing".to_string(), "sync".to_string()]);
    peer_discovery.add_peer(remote_peer.clone()).unwrap();
    println!("  ✓ Added remote peer: {} ({})", remote_peer.name, remote_peer.endpoint());
    
    let discovered_peers = peer_discovery.discover_peers().unwrap();
    println!("  ✓ Discovered {} peers\n", discovered_peers.len());
    
    // Test Transport
    println!("Testing Transport:");
    let transport = Transport::new(TransportProtocol::Tcp);
    println!("  ✓ Transport created (protocol: TCP)");
    
    let connection_id = transport.connect(
        remote_peer.id.clone(),
        remote_peer.address.clone(),
        remote_peer.port
    ).unwrap();
    println!("  ✓ Connected to peer: {}", connection_id);
    
    let connections = transport.get_all_connections();
    println!("  ✓ Active connections: {}\n", connections.len());
    
    println!("─────────────────────────────────");
    println!("Vantis Link demo completed successfully!");
    println!("─────────────────────────────────");
    println!();
    println!("Session Statistics:");
    println!("  Session ID: {}", session.id);
    println!("  Document ID: {}", session.document_id);
    println!("  Total users: {}", session.user_count());
    println!("  Is active: {}", session.is_active);
    println!();
    println!("Security Features:");
    println!("  ✓ CRDT-based conflict resolution");
    println!("  ✓ End-to-end encryption");
    println!("  ✓ No central servers");
    println!("  ✓ Real-time synchronization");
    println!("  ✓ Offline support");
}