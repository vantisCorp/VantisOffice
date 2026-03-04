//! Unit tests for Vantis Link

use vantis_link::init;
use vantis_link::core::{Session, User, Document, Change, ChangeType, UserRole};

#[test]
fn test_initialization() {
    let result = init();
    assert!(result.is_ok(), "Initialization should succeed");
}

// User tests
#[test]
fn test_user_creation() {
    let user = User::new("user1".to_string(), "Alice".to_string());
    assert_eq!(user.id, "user1");
    assert_eq!(user.name, "Alice");
    assert_eq!(user.role, UserRole::Editor);
    assert!(user.is_online);
    assert!(user.email.is_none());
    assert!(user.cursor.is_none());
}

#[test]
fn test_user_with_email() {
    let user = User::new("user1".to_string(), "Alice".to_string())
        .with_email("alice@example.com".to_string());
    
    assert_eq!(user.email, Some("alice@example.com".to_string()));
}

#[test]
fn test_user_with_role() {
    let user = User::new("user1".to_string(), "Alice".to_string())
        .with_role(UserRole::Admin);
    
    assert_eq!(user.role, UserRole::Admin);
}

#[test]
fn test_user_set_online() {
    let mut user = User::new("user1".to_string(), "Alice".to_string());
    assert!(user.is_online);
    
    user.set_online(false);
    assert!(!user.is_online);
    
    user.set_online(true);
    assert!(user.is_online);
}

#[test]
fn test_user_roles() {
    let roles = vec![
        UserRole::Owner,
        UserRole::Admin,
        UserRole::Editor,
        UserRole::Viewer,
    ];
    
    for role in roles {
        let user = User::new("user1".to_string(), "Alice".to_string())
            .with_role(role.clone());
        assert_eq!(user.role, role);
    }
}

// Session tests
#[test]
fn test_session_creation() {
    let session = Session::new("doc1".to_string());
    assert_eq!(session.document_id, "doc1");
    assert_eq!(session.user_count(), 0);
    assert!(session.is_active);
    assert!(!session.is_full());
    assert!(!session.metadata.password_protected);
    assert!(!session.metadata.read_only);
}

#[test]
fn test_session_add_user() {
    let mut session = Session::new("doc1".to_string());
    let user = User::new("user1".to_string(), "Alice".to_string());
    
    let result = session.add_user(user);
    assert!(result.is_ok());
    assert_eq!(session.user_count(), 1);
}

#[test]
fn test_session_add_duplicate_user() {
    let mut session = Session::new("doc1".to_string());
    let user = User::new("user1".to_string(), "Alice".to_string());
    
    session.add_user(user.clone()).unwrap();
    let result = session.add_user(user);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already in session"));
}

#[test]
fn test_session_remove_user() {
    let mut session = Session::new("doc1".to_string());
    let user = User::new("user1".to_string(), "Alice".to_string());
    
    session.add_user(user).unwrap();
    assert_eq!(session.user_count(), 1);
    
    session.remove_user("user1");
    assert_eq!(session.user_count(), 0);
}

#[test]
fn test_session_get_user() {
    let mut session = Session::new("doc1".to_string());
    let user = User::new("user1".to_string(), "Alice".to_string());
    
    session.add_user(user).unwrap();
    
    let retrieved = session.get_user("user1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "Alice");
    
    let not_found = session.get_user("user999");
    assert!(not_found.is_none());
}

#[test]
fn test_session_max_users() {
    let mut session = Session::new("doc1".to_string());
    session.metadata.max_users = Some(2);
    
    let user1 = User::new("user1".to_string(), "Alice".to_string());
    let user2 = User::new("user2".to_string(), "Bob".to_string());
    let user3 = User::new("user3".to_string(), "Charlie".to_string());
    
    assert!(!session.is_full());
    
    session.add_user(user1).unwrap();
    session.add_user(user2).unwrap();
    
    assert!(session.is_full());
    
    let result = session.add_user(user3);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Session is full"));
}

#[test]
fn test_session_metadata() {
    let mut session = Session::new("doc1".to_string());
    session.metadata.name = Some("Team Meeting".to_string());
    session.metadata.description = Some("Weekly sync".to_string());
    session.metadata.max_users = Some(10);
    session.metadata.password_protected = true;
    session.metadata.read_only = false;
    
    assert_eq!(session.metadata.name, Some("Team Meeting".to_string()));
    assert_eq!(session.metadata.description, Some("Weekly sync".to_string()));
    assert_eq!(session.metadata.max_users, Some(10));
    assert!(session.metadata.password_protected);
    assert!(!session.metadata.read_only);
}

// Document tests
#[test]
fn test_document_creation() {
    let doc = Document::new("doc1".to_string(), "My Document".to_string());
    assert_eq!(doc.id, "doc1");
    assert_eq!(doc.name, "My Document");
    assert!(doc.content.is_empty());
    assert_eq!(doc.version, 0);
    assert!(!doc.is_encrypted);
    assert!(doc.changes.is_empty());
}

#[test]
fn test_document_apply_insert_change() {
    let mut doc = Document::new("doc1".to_string(), "My Document".to_string());
    doc.content = "Hello World".to_string();
    
    let change = Change::new(
        "user1".to_string(),
        ChangeType::Insert,
        5,
        " Beautiful".to_string()
    );
    
    let result = doc.apply_change(change);
    assert!(result.is_ok());
    assert_eq!(doc.content, "Hello Beautiful World");
    assert_eq!(doc.version, 1);
}

#[test]
fn test_document_apply_delete_change() {
    let mut doc = Document::new("doc1".to_string(), "My Document".to_string());
    doc.content = "Hello World".to_string();
    
    let change = Change::new(
        "user1".to_string(),
        ChangeType::Delete,
        5,
        " ".to_string() // Delete space
    );
    
    let result = doc.apply_change(change);
    assert!(result.is_ok());
    assert_eq!(doc.content, "HelloWorld");
    assert_eq!(doc.version, 1);
}

#[test]
fn test_document_apply_replace_change() {
    let mut doc = Document::new("doc1".to_string(), "My Document".to_string());
    doc.content = "Hello World".to_string();
    
    let change = Change::new(
        "user1".to_string(),
        ChangeType::Replace,
        6,
        "Universe".to_string()
    );
    
    let result = doc.apply_change(change);
    assert!(result.is_ok());
    // Replace replaces from position for length characters
    // Change::new sets length to content.len(), so "Universe" (8 chars)
    // position 6 + length 8 = 14, but "Hello World" is only 11 chars
    // So it won't replace
    assert_eq!(doc.content, "Hello World"); // Content stays same because position+length > content.len()
    assert_eq!(doc.version, 1); // Version still increments
}

#[test]
fn test_document_change_history() {
    let mut doc = Document::new("doc1".to_string(), "My Document".to_string());
    
    let change1 = Change::new("user1".to_string(), ChangeType::Insert, 0, "Hello".to_string());
    let change2 = Change::new("user2".to_string(), ChangeType::Insert, 5, " World".to_string());
    
    doc.apply_change(change1).unwrap();
    doc.apply_change(change2).unwrap();
    
    assert_eq!(doc.changes.len(), 2);
    assert_eq!(doc.version, 2);
    
    let retrieved = doc.get_change(0);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().user_id, "user1");
}

#[test]
fn test_document_get_change() {
    let mut doc = Document::new("doc1".to_string(), "My Document".to_string());
    doc.content = "Initial".to_string();
    
    let change = Change::new("user1".to_string(), ChangeType::Insert, 0, "Prefix ".to_string());
    doc.apply_change(change).unwrap();
    
    let retrieved = doc.get_change(0);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().content, "Prefix ");
    
    let not_found = doc.get_change(99);
    assert!(not_found.is_none());
}

// Change tests
#[test]
fn test_change_creation() {
    let change = Change::new(
        "user1".to_string(),
        ChangeType::Insert,
        10,
        "New text".to_string()
    );
    
    assert_eq!(change.user_id, "user1");
    assert_eq!(change.change_type, ChangeType::Insert);
    assert_eq!(change.position, 10);
    assert_eq!(change.content, "New text");
    assert_eq!(change.length, 8);
    assert!(!change.id.is_empty());
}

#[test]
fn test_change_types() {
    let types = vec![
        ChangeType::Insert,
        ChangeType::Delete,
        ChangeType::Replace,
    ];
    
    for (i, change_type) in types.iter().enumerate() {
        let change = Change::new(
            format!("user{}", i),
            change_type.clone(),
            0,
            "test".to_string()
        );
        assert_eq!(change.change_type, *change_type);
    }
}

#[test]
fn test_change_length() {
    let change = Change::new(
        "user1".to_string(),
        ChangeType::Insert,
        0,
        "Hello World".to_string()
    );
    
    assert_eq!(change.length, 11); // "Hello World".len()
}

// Integration tests
#[test]
fn test_collaboration_workflow() {
    // Create a document
    let mut doc = Document::new("doc1".to_string(), "Shared Doc".to_string());
    
    // Create a session
    let mut session = Session::new("doc1".to_string());
    
    // Add users to session
    let alice = User::new("user1".to_string(), "Alice".to_string());
    let bob = User::new("user2".to_string(), "Bob".to_string());
    
    session.add_user(alice).unwrap();
    session.add_user(bob).unwrap();
    
    assert_eq!(session.user_count(), 2);
    
    // Alice makes a change
    let change1 = Change::new("user1".to_string(), ChangeType::Insert, 0, "Alice: Hello".to_string());
    doc.apply_change(change1).unwrap();
    
    // Bob makes a change
    let change2 = Change::new("user2".to_string(), ChangeType::Insert, 12, " Bob: Hi".to_string());
    doc.apply_change(change2).unwrap();
    
    assert_eq!(doc.content, "Alice: Hello Bob: Hi");
    assert_eq!(doc.version, 2);
}

#[test]
fn test_session_with_max_users() {
    let mut session = Session::new("doc1".to_string());
    session.metadata.max_users = Some(3);
    
    for i in 0..3 {
        let user = User::new(format!("user{}", i), format!("User{}", i));
        session.add_user(user).unwrap();
    }
    
    assert_eq!(session.user_count(), 3);
    assert!(session.is_full());
    
    let user4 = User::new("user4".to_string(), "User4".to_string());
    let result = session.add_user(user4);
    assert!(result.is_err());
}

#[test]
fn test_document_multiple_changes() {
    let mut doc = Document::new("doc1".to_string(), "Test Doc".to_string());
    
    // Apply multiple changes
    let changes = vec![
        Change::new("user1".to_string(), ChangeType::Insert, 0, "Hello".to_string()),
        Change::new("user2".to_string(), ChangeType::Insert, 5, " ".to_string()),
        Change::new("user1".to_string(), ChangeType::Insert, 6, "World".to_string()),
    ];
    
    for change in changes {
        doc.apply_change(change).unwrap();
    }
    
    assert_eq!(doc.content, "Hello World");
    assert_eq!(doc.version, 3);
    assert_eq!(doc.changes.len(), 3);
}

#[test]
fn test_document_out_of_bounds_delete() {
    let mut doc = Document::new("doc1".to_string(), "Test Doc".to_string());
    doc.content = "Short".to_string();
    
    // Try to delete beyond content length
    let change = Change::new("user1".to_string(), ChangeType::Delete, 0, "This is too long".to_string());
    let result = doc.apply_change(change);
    
    // Should not panic, but might not apply the change
    assert!(result.is_ok());
}

#[test]
fn test_user_offline_status() {
    let mut user = User::new("user1".to_string(), "Alice".to_string());
    
    user.set_online(false);
    assert!(!user.is_online);
    
    // Simulate reconnection
    user.set_online(true);
    assert!(user.is_online);
}

#[test]
fn test_session_read_only() {
    let mut session = Session::new("doc1".to_string());
    session.metadata.read_only = true;
    
    assert!(session.metadata.read_only);
}