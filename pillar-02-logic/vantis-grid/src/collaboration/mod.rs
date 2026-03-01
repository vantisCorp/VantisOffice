//! Collaboration module for real-time multi-user editing
//! 
//! Uses CRDT (Conflict-free Replicated Data Types) for conflict resolution

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Collaboration manager for real-time editing
pub struct CollaborationManager {
    sessions: Arc<RwLock<HashMap<String, CollaborationSession>>>,
    change_tracker: ChangeTracker,
    enabled: bool,
}

impl CollaborationManager {
    pub fn new() -> Self {
        CollaborationManager {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            change_tracker: ChangeTracker::new(),
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
    pub fn create_session(&self, document_id: String) -> Result<String, CollaborationError> {
        if !self.enabled {
            return Err(CollaborationError::Disabled);
        }
        
        let session_id = Uuid::new_v4().to_string();
        let session = CollaborationSession::new(document_id.clone(), session_id.clone());
        
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
    pub fn apply_change(&self, session_id: String, change: CellChange) -> Result<AppliedChange, CollaborationError> {
        let mut sessions = self.sessions.write()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        let session = sessions.get_mut(&session_id)
            .ok_or_else(|| CollaborationError::SessionNotFound(session_id.clone()))?;
        
        // Track the change
        self.change_tracker.track_change(&session_id, &change)?;
        
        // Apply change using CRDT
        let applied = session.apply_change(change)?;
        
        Ok(applied)
    }
    
    /// Get pending changes for a user
    pub fn get_pending_changes(&self, session_id: String, user_id: String) 
        -> Result<Vec<CellChange>, CollaborationError> {
        let sessions = self.sessions.read()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        let session = sessions.get(&session_id)
            .ok_or_else(|| CollaborationError::SessionNotFound(session_id.clone()))?;
        
        session.get_changes_for_user(&user_id)
    }
    
    /// Get active users in a session
    pub fn get_active_users(&self, session_id: String) -> Result<Vec<User>, CollaborationError> {
        let sessions = self.sessions.read()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        let session = sessions.get(&session_id)
            .ok_or_else(|| CollaborationError::SessionNotFound(session_id.clone()))?;
        
        Ok(session.get_active_users())
    }
    
    /// Get session statistics
    pub fn get_session_stats(&self, session_id: String) -> Result<SessionStats, CollaborationError> {
        let sessions = self.sessions.read()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        let session = sessions.get(&session_id)
            .ok_or_else(|| CollaborationError::SessionNotFound(session_id.clone()))?;
        
        Ok(session.get_stats())
    }
}

impl Default for CollaborationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Collaboration session
#[derive(Debug, Clone)]
pub struct CollaborationSession {
    pub document_id: String,
    pub session_id: String,
    pub users: HashMap<String, User>,
    pub changes: Vec<CellChange>,
    pub crdt_state: CRDTState,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl CollaborationSession {
    pub fn new(document_id: String, session_id: String) -> Self {
        CollaborationSession {
            document_id,
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
    
    pub fn apply_change(&mut self, change: CellChange) -> Result<AppliedChange, CollaborationError> {
        // Apply CRDT merge
        let merged_change = self.crdt_state.merge(change.clone())?;
        
        // Store change
        self.changes.push(merged_change.clone());
        
        Ok(AppliedChange {
            change: merged_change,
            timestamp: chrono::Utc::now(),
            conflicts_resolved: 0,
        })
    }
    
    pub fn get_changes_for_user(&self, user_id: &str) -> Result<Vec<CellChange>, CollaborationError> {
        let user_changes: Vec<CellChange> = self.changes.iter()
            .filter(|c| c.user_id != user_id)
            .cloned()
            .collect();
        
        Ok(user_changes)
    }
    
    pub fn get_active_users(&self) -> Vec<User> {
        self.users.values().cloned().collect()
    }
    
    pub fn get_stats(&self) -> SessionStats {
        SessionStats {
            active_users: self.users.len(),
            total_changes: self.changes.len(),
            created_at: self.created_at,
            duration: chrono::Utc::now() - self.created_at,
        }
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
    pub cell_versions: HashMap<(usize, usize), CellVersion>,
}

impl CRDTState {
    pub fn new() -> Self {
        CRDTState {
            version_vector: HashMap::new(),
            cell_versions: HashMap::new(),
        }
    }
    
    pub fn merge(&mut self, change: CellChange) -> Result<CellChange, CollaborationError> {
        // Update version vector
        let current_version = self.version_vector.entry(change.user_id.clone()).or_insert(0);
        *current_version += 1;
        
        // Create cell version
        let cell_version = CellVersion {
            user_id: change.user_id.clone(),
            version: *current_version,
            timestamp: change.timestamp,
        };
        
        // Check for conflicts
        let cell_key = (change.row, change.column);
        if let Some(existing_version) = self.cell_versions.get(&cell_key) {
            // Resolve conflict using last-write-wins with timestamp
            if change.timestamp > existing_version.timestamp {
                self.cell_versions.insert(cell_key, cell_version);
            } else {
                // Conflict - keep existing value
                return Ok(change);
            }
        } else {
            self.cell_versions.insert(cell_key, cell_version);
        }
        
        Ok(change)
    }
}

impl Default for CRDTState {
    fn default() -> Self {
        Self::new()
    }
}

/// Cell version for CRDT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellVersion {
    pub user_id: String,
    pub version: u64,
    pub timestamp: i64,
}

/// Cell change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellChange {
    pub id: String,
    pub user_id: String,
    pub row: usize,
    pub column: usize,
    pub old_value: Option<String>,
    pub new_value: String,
    pub change_type: ChangeType,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Insert,
    Update,
    Delete,
    Format,
}

impl CellChange {
    pub fn new(user_id: String, row: usize, column: usize, new_value: String) -> Self {
        CellChange {
            id: Uuid::new_v4().to_string(),
            user_id,
            row,
            column,
            old_value: None,
            new_value,
            change_type: ChangeType::Update,
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}

/// Applied change result
#[derive(Debug, Clone)]
pub struct AppliedChange {
    pub change: CellChange,
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
    pub row: usize,
    pub column: usize,
    pub worksheet: String,
}

/// Session statistics
#[derive(Debug, Clone)]
pub struct SessionStats {
    pub active_users: usize,
    pub total_changes: usize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
}

/// Change tracker for audit and history
pub struct ChangeTracker {
    history: Arc<RwLock<HashMap<String, Vec<CellChange>>>>,
}

impl ChangeTracker {
    pub fn new() -> Self {
        ChangeTracker {
            history: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn track_change(&self, session_id: &str, change: &CellChange) -> Result<(), CollaborationError> {
        let mut history = self.history.write()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        history.entry(session_id.to_string())
            .or_insert_with(Vec::new)
            .push(change.clone());
        
        Ok(())
    }
    
    pub fn get_history(&self, session_id: &str) -> Result<Vec<CellChange>, CollaborationError> {
        let history = self.history.read()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        Ok(history.get(session_id).cloned().unwrap_or_default())
    }
    
    pub fn clear_history(&self, session_id: &str) -> Result<(), CollaborationError> {
        let mut history = self.history.write()
            .map_err(|e| CollaborationError::LockError(e.to_string()))?;
        
        history.remove(session_id);
        
        Ok(())
    }
}

impl Default for ChangeTracker {
    fn default() -> Self {
        Self::new()
    }
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
    fn test_collaboration_manager_creation() {
        let manager = CollaborationManager::new();
        assert!(manager.is_enabled());
    }
    
    #[test]
    fn test_session_creation() {
        let manager = CollaborationManager::new();
        let session_id = manager.create_session("doc1".to_string()).unwrap();
        assert!(!session_id.is_empty());
    }
    
    #[test]
    fn test_user_join() {
        let manager = CollaborationManager::new();
        let session_id = manager.create_session("doc1".to_string()).unwrap();
        
        let token = manager.join_session(session_id.clone(), "user1".to_string(), "Alice".to_string()).unwrap();
        assert_eq!(token.user_id, "user1");
    }
    
    #[test]
    fn test_change_application() {
        let manager = CollaborationManager::new();
        let session_id = manager.create_session("doc1".to_string()).unwrap();
        manager.join_session(session_id.clone(), "user1".to_string(), "Alice".to_string()).unwrap();
        
        let change = CellChange::new("user1".to_string(), 0, 0, "100".to_string());
        let applied = manager.apply_change(session_id.clone(), change).unwrap();
        
        assert_eq!(applied.change.new_value, "100");
    }
}