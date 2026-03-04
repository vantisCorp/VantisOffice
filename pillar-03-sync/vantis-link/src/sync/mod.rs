//! Sync module for real-time synchronization
//!
//! Provides synchronization between peers

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Sync Manager
pub struct SyncManager {
    sessions: Arc<RwLock<HashMap<String, SyncSession>>>,
    enabled: bool,
}

impl SyncManager {
    pub fn new() -> Self {
        SyncManager {
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

    /// Create a new sync session
    pub fn create_session(&self, document_id: String) -> Result<String, String> {
        if !self.enabled {
            return Err("Sync manager is disabled".to_string());
        }

        let session_id = uuid::Uuid::new_v4().to_string();
        let session = SyncSession::new(document_id.clone(), session_id.clone());

        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;

        sessions.insert(session_id.clone(), session);

        Ok(session_id)
    }

    /// Join a sync session
    pub fn join_session(&self, session_id: String, user_id: String) -> Result<SyncStatus, String> {
        if !self.enabled {
            return Err("Sync manager is disabled".to_string());
        }

        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;

        let session = sessions
            .get_mut(&session_id)
            .ok_or_else(|| format!("Session '{}' not found", session_id))?;

        session.add_peer(user_id.clone());

        Ok(SyncStatus::Connected)
    }

    /// Leave a sync session
    pub fn leave_session(&self, session_id: String, user_id: String) -> Result<(), String> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;

        let session = sessions
            .get_mut(&session_id)
            .ok_or_else(|| format!("Session '{}' not found", session_id))?;

        session.remove_peer(&user_id);

        Ok(())
    }

    /// Sync changes
    pub fn sync_changes(
        &self,
        session_id: String,
        changes: Vec<crate::core::Change>,
    ) -> Result<SyncResult, String> {
        if !self.enabled {
            return Err("Sync manager is disabled".to_string());
        }

        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;

        let session = sessions
            .get_mut(&session_id)
            .ok_or_else(|| format!("Session '{}' not found", session_id))?;

        // Check for conflicts
        let conflicts = self.detect_conflicts(session, &changes)?;

        if !conflicts.is_empty() {
            return Ok(SyncResult::Conflicts(conflicts));
        }

        // Apply changes
        for change in changes {
            session.add_change(change);
        }

        Ok(SyncResult::Success)
    }

    /// Detect conflicts
    fn detect_conflicts(
        &self,
        session: &SyncSession,
        changes: &[crate::core::Change],
    ) -> Result<Vec<SyncConflict>, String> {
        let mut conflicts = Vec::new();

        for change in changes {
            for existing_change in &session.changes {
                // Check for overlapping changes
                if self.changes_overlap(change, existing_change) {
                    conflicts.push(SyncConflict {
                        change_id: change.id.clone(),
                        conflicting_change_id: existing_change.id.clone(),
                        conflict_type: ConflictType::OverlappingEdits,
                    });
                }
            }
        }

        Ok(conflicts)
    }

    /// Check if changes overlap
    fn changes_overlap(
        &self,
        change1: &crate::core::Change,
        change2: &crate::core::Change,
    ) -> bool {
        let start1 = change1.position;
        let end1 = change1.position + change1.length;
        let start2 = change2.position;
        let end2 = change2.position + change2.length;

        // Check if ranges overlap
        !(end1 <= start2 || end2 <= start1)
    }

    /// Get sync status
    pub fn get_sync_status(&self, session_id: String) -> Result<SyncStatus, String> {
        let sessions = self
            .sessions
            .read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;

        let session = sessions
            .get(&session_id)
            .ok_or_else(|| format!("Session '{}' not found", session_id))?;

        if session.peers.is_empty() {
            Ok(SyncStatus::Disconnected)
        } else {
            Ok(SyncStatus::Connected)
        }
    }
}

/// Sync Session
#[derive(Debug, Clone)]
pub struct SyncSession {
    pub id: String,
    pub document_id: String,
    pub peers: Vec<String>,
    pub changes: Vec<crate::core::Change>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_sync: chrono::DateTime<chrono::Utc>,
}

impl SyncSession {
    pub fn new(document_id: String, id: String) -> Self {
        let now = chrono::Utc::now();
        SyncSession {
            id,
            document_id,
            peers: Vec::new(),
            changes: Vec::new(),
            created_at: now,
            last_sync: now,
        }
    }

    pub fn add_peer(&mut self, peer_id: String) {
        if !self.peers.contains(&peer_id) {
            self.peers.push(peer_id);
        }
    }

    pub fn remove_peer(&mut self, peer_id: &str) {
        self.peers.retain(|p| p != peer_id);
    }

    pub fn add_change(&mut self, change: crate::core::Change) {
        self.changes.push(change);
        self.last_sync = chrono::Utc::now();
    }
}

/// Sync Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncStatus {
    Connected,
    Disconnected,
    Syncing,
    Error,
}

/// Sync Result
#[derive(Debug, Clone)]
pub enum SyncResult {
    Success,
    Conflicts(Vec<SyncConflict>),
    Error(String),
}

/// Sync Conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    pub change_id: String,
    pub conflicting_change_id: String,
    pub conflict_type: ConflictType,
}

/// Conflict Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictType {
    OverlappingEdits,
    ConcurrentModifications,
    VersionMismatch,
}

/// Initialize sync module
pub fn init() -> Result<(), String> {
    Ok(())
}
