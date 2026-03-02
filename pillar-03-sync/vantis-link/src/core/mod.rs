//! Core data structures for Vantis Link

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Collaboration Session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub document_id: String,
    pub users: HashMap<String, User>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub is_active: bool,
    pub metadata: SessionMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_users: Option<usize>,
    pub password_protected: bool,
    pub read_only: bool,
}

impl Session {
    pub fn new(document_id: String) -> Self {
        let now = chrono::Utc::now();
        Session {
            id: Uuid::new_v4().to_string(),
            document_id,
            users: HashMap::new(),
            created_at: now,
            is_active: true,
            metadata: SessionMetadata {
                name: None,
                description: None,
                max_users: None,
                password_protected: false,
                read_only: false,
            },
        }
    }
    
    pub fn add_user(&mut self, user: User) -> Result<(), String> {
        if self.users.contains_key(&user.id) {
            return Err(format!("User {} already in session", user.id));
        }
        
        if let Some(max) = self.metadata.max_users {
            if self.users.len() >= max {
                return Err("Session is full".to_string());
            }
        }
        
        self.users.insert(user.id.clone(), user);
        Ok(())
    }
    
    pub fn remove_user(&mut self, user_id: &str) {
        self.users.remove(user_id);
    }
    
    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.get(user_id)
    }
    
    pub fn user_count(&self) -> usize {
        self.users.len()
    }
    
    pub fn is_full(&self) -> bool {
        if let Some(max) = self.metadata.max_users {
            self.users.len() >= max
        } else {
            false
        }
    }
}

/// User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub role: UserRole,
    pub joined_at: chrono::DateTime<chrono::Utc>,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub is_online: bool,
    pub cursor: Option<Cursor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Owner,
    Admin,
    Editor,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    pub x: f64,
    pub y: f64,
    pub page: Option<usize>,
    pub selection: Option<Selection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selection {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl User {
    pub fn new(id: String, name: String) -> Self {
        let now = chrono::Utc::now();
        User {
            id,
            name,
            email: None,
            role: UserRole::Editor,
            joined_at: now,
            last_seen: now,
            is_online: true,
            cursor: None,
        }
    }
    
    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    
    pub fn with_role(mut self, role: UserRole) -> Self {
        self.role = role;
        self
    }
    
    pub fn update_cursor(&mut self, cursor: Cursor) {
        self.cursor = Some(cursor);
        self.last_seen = chrono::Utc::now();
    }
    
    pub fn set_online(&mut self, online: bool) {
        self.is_online = online;
        self.last_seen = chrono::Utc::now();
    }
}

/// Document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub content: String,
    pub version: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
    pub changes: Vec<Change>,
    pub is_encrypted: bool,
}

impl Document {
    pub fn new(id: String, name: String) -> Self {
        let now = chrono::Utc::now();
        Document {
            id,
            name,
            content: String::new(),
            version: 0,
            created_at: now,
            modified_at: now,
            changes: Vec::new(),
            is_encrypted: false,
        }
    }
    
    pub fn apply_change(&mut self, change: Change) -> Result<(), String> {
        // Apply change to content
        match change.change_type {
            ChangeType::Insert => {
                self.content.insert_str(change.position, &change.content);
            }
            ChangeType::Delete => {
                let end = change.position + change.length;
                if end <= self.content.len() {
                    self.content.replace_range(change.position..end, "");
                }
            }
            ChangeType::Replace => {
                let end = change.position + change.length;
                if end <= self.content.len() {
                    self.content.replace_range(change.position..end, &change.content);
                }
            }
        }
        
        self.version += 1;
        self.modified_at = chrono::Utc::now();
        self.changes.push(change);
        
        Ok(())
    }
    
    pub fn get_change(&self, index: usize) -> Option<&Change> {
        self.changes.get(index)
    }
}

/// Change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub id: String,
    pub user_id: String,
    pub change_type: ChangeType,
    pub position: usize,
    pub length: usize,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub crdt_operation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Insert,
    Delete,
    Replace,
}

impl Change {
    pub fn new(user_id: String, change_type: ChangeType, position: usize, content: String) -> Self {
        let now = chrono::Utc::now();
        Change {
            id: Uuid::new_v4().to_string(),
            user_id,
            change_type,
            position,
            length: content.len(),
            content,
            timestamp: now,
            crdt_operation: None,
        }
    }
}

/// Initialize core module
pub fn init() -> Result<(), String> {
    Ok(())
}