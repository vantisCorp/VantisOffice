//! Collaboration module for real-time multi-user editing
//! 
//! Uses CRDT for conflict resolution in canvas editing

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Canvas collaboration manager
pub struct CanvasCollaboration {
    sessions: Arc<RwLock<HashMap<String, CollaborationSession>>>,
    enabled: bool,
}

impl CanvasCollaboration {
    pub fn new() -> Self {
        CanvasCollaboration {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            enabled: true,
        }
    }
    
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Create a new collaboration session
    pub fn create_session(&self, canvas_id: String) -> Result<String, CollaborationError> {
        if !self.enabled {
            return Err(CollaborationError::Disabled);
        }
        
        let session_id = Uuid::new_v4().to_string();
        let session = CollaborationSession::new(canvas_id.clone(), session_id.clone());
        
        let mut sessions = self.sessions.write()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        sessions.insert(session_id.clone(), session);
        
        Ok(session_id)
    }
    
    /// Join an existing collaboration session
    pub fn join_session(&self, session_id: String, user_id: String, user_name: String) 
        -> Result<CollaborationToken, CollaborationError> {
        if !self.enabled {
            return Err(CollaborationError::Disabled);
        }
        
        let mut sessions = self.sessions.write()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        let session = sessions.get_mut(&session_id)
            .ok_or_else(|| CollaborationError::SessionNotFound(session_id.clone()))?;
        
        let token = session.add_user(user_id.clone(), user_name)?;
        
        Ok(token)
    }
    
    /// Leave a collaboration session
    pub fn leave_session(&self, session_id: String, user_id: String) -> Result<(), CollaborationError> {
        let mut sessions = self.sessions.write()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        let session = sessions.get_mut(&session_id)
            .ok_or_else(|| CollaborationError::SessionNotFound(session_id.clone()))?;
        
        session.remove_user(&user_id);
        
        Ok(())
    }
    
    /// Apply a change from a user
    pub fn apply_change(&self, session_id: String, change: CanvasChange) -> Result<AppliedChange, CollaborationError> {
        let mut sessions = self.sessions.write()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        let session = sessions.get_mut(&session_id)
            .ok_or_else(|| CollaborationError::SessionNotFound(session_id.clone()))?;
        
        let applied = session.apply_change(change)?;
        
        Ok(applied)
    }
    
    /// Update user cursor position
    pub fn update_cursor(&self, session_id: String, user_id: String, cursor: Cursor) -> Result<(), CollaborationError> {
        let mut sessions = self.sessions.write()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        let session = sessions.get_mut(&session_id)
            .ok_or_else(|| CollaborationError::SessionNotFound(session_id.clone()))?;
        
        session.update_cursor(&user_id, cursor);
        
        Ok(())
    }
    
    /// Get active users in a session
    pub fn get_active_users(&self, session_id: String) -> Result<Vec<User>, CollaborationError> {
        let sessions = self.sessions.read()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        let session = sessions.get(&session_id)
            .ok_or_else(|| CollaborationError::SessionNotFound(session_id.clone()))?;
        
        Ok(session.get_active_users())
    }
}

impl Default for CanvasCollaboration {
    fn default() -> Self {
        Self::new()
    }
}

/// Collaboration session
#[derive(Debug, Clone)]
pub struct CollaborationSession {
    pub canvas_id: String,
    pub session_id: String,
    pub users: HashMap<String, User>,
    pub changes: Vec<CanvasChange>,
    pub crdt_state: CRDTState,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl CollaborationSession {
    pub fn new(canvas_id: String, session_id: String) -> Self {
        CollaborationSession {
            canvas_id,
            session_id,
            users: HashMap::new(),
            changes: Vec::new(),
            crdt_state: CRDTState::new(),
            created_at: chrono::Utc::now(),
        }
    }
    
    pub fn add_user(&mut self, user_id: String, user_name: String) -> Result<CollaborationToken, CollaborationError> {
        if self.users.contains_key(&user_id) {
            return Err(CollaborationError::UserAlreadyExists(user_id));
        }
        
        let user = User {
            id: user_id.clone(),
            name: user_name,
            cursor: None,
            color: self.generate_color(),
            joined_at: chrono::Utc::now(),
        };
        
        let token = CollaborationToken {
            session_id: self.session_id.clone(),
            user_id: user_id.clone(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
        };
        
        self.users.insert(user_id, user);
        
        Ok(token)
    }
    
    pub fn remove_user(&mut self, user_id: &str) {
        self.users.remove(user_id);
    }
    
    pub fn apply_change(&mut self, change: CanvasChange) -> Result<AppliedChange, CollaborationError> {
        let merged_change = self.crdt_state.merge(change.clone())?;
        self.changes.push(merged_change.clone());
        
        Ok(AppliedChange {
            change: merged_change,
            timestamp: chrono::Utc::now(),
            conflicts_resolved: 0,
        })
    }
    
    pub fn update_cursor(&mut self, user_id: &str, cursor: Cursor) {
        if let Some(user) = self.users.get_mut(user_id) {
            user.cursor = Some(cursor);
        }
    }
    
    pub fn get_active_users(&self) -> Vec<User> {
        self.users.values().cloned().collect()
    }
    
    fn generate_color(&self) -> String {
        let colors = vec![
            "#FF6B6B", "#4ECDC4", "#45B7D1", "#96CEB4", "#FFEAA7",
            "#DDA0DD", "#98D8C8", "#F7DC6F", "#BB8FCE", "#85C1E9"
        ];
        
        let index = self.users.len() % colors.len();
        colors[index].to_string()
    }
}

/// CRDT state for conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRDTState {
    pub version_vector: HashMap<String, u64>,
    pub element_versions: HashMap<String, ElementVersion>,
}

impl CRDTState {
    pub fn new() -> Self {
        CRDTState {
            version_vector: HashMap::new(),
            element_versions: HashMap::new(),
        }
    }
    
    pub fn merge(&mut self, change: CanvasChange) -> Result<CanvasChange, CollaborationError> {
        let current_version = self.version_vector.entry(change.user_id.clone()).or_insert(0);
        *current_version += 1;
        
        let element_version = ElementVersion {
            user_id: change.user_id.clone(),
            version: *current_version,
            timestamp: change.timestamp,
        };
        
        self.element_versions.insert(change.element_id.clone(), element_version);
        
        Ok(change)
    }
}

impl Default for CRDTState {
    fn default() -> Self {
        Self::new()
    }
}

/// Element version for CRDT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementVersion {
    pub user_id: String,
    pub version: u64,
    pub timestamp: i64,
}

/// Canvas change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasChange {
    pub id: String,
    pub user_id: String,
    pub element_id: String,
    pub change_type: CanvasChangeType,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CanvasChangeType {
    AddShape { shape: crate::core::Shape },
    UpdateShape { shape: crate::core::Shape },
    DeleteShape { shape_id: String },
    AddText { text: crate::core::Text },
    UpdateText { text: crate::core::Text },
    DeleteText { text_id: String },
    AddImage { image: crate::core::Image },
    UpdateImage { image: crate::core::Image },
    DeleteImage { image_id: String },
    MoveElement { element_id: String, position: crate::core::Position },
    ResizeElement { element_id: String, size: crate::core::Size },
    RotateElement { element_id: String, rotation: f64 },
}

impl CanvasChange {
    pub fn new(user_id: String, element_id: String, change_type: CanvasChangeType) -> Self {
        CanvasChange {
            id: Uuid::new_v4().to_string(),
            user_id,
            element_id,
            change_type,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}

/// Applied change result
#[derive(Debug, Clone)]
pub struct AppliedChange {
    pub change: CanvasChange,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub conflicts_resolved: usize,
}

/// Collaboration token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationToken {
    pub session_id: String,
    pub user_id: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// User in collaboration session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub cursor: Option<Cursor>,
    pub color: String,
    pub joined_at: chrono::DateTime<chrono::Utc>,
}

/// User cursor position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    pub x: f64,
    pub y: f64,
    pub slide_index: usize,
}

/// Collaboration errors
#[derive(Debug, thiserror::Error)]
pub enum CollaborationError {
    #[error("Collaboration is disabled")]
    Disabled,
    
    #[error("Session not found: {0}")]
    SessionNotFound(String),
    
    #[error("User already exists: {0}")]
    UserAlreadyExists(String),
    
    #[error("Lock error: {0}")]
    LockError(String),
    
    #[error("Conflict resolution error: {0}")]
    ConflictError(String),
    
    #[error("Invalid change: {0}")]
    InvalidChange(String),
}

/// Initialize collaboration module
pub fn init() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_collaboration_creation() {
        let collab = CanvasCollaboration::new();
        assert!(collab.is_enabled());
    }
    
    #[test]
    fn test_session_creation() {
        let collab = CanvasCollaboration::new();
        let session_id = collab.create_session("canvas1".to_string()).unwrap();
        assert!(!session_id.is_empty());
    }
    
    #[test]
    fn test_user_join() {
        let collab = CanvasCollaboration::new();
        let session_id = collab.create_session("canvas1".to_string()).unwrap();
        
        let token = collab.join_session(session_id.clone(), "user1".to_string(), "Alice".to_string()).unwrap();
        assert_eq!(token.user_id, "user1");
    }
}