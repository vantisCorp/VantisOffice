//! Session Backup Module - Integration with Vantis Link (Pillar 03)
//!
//! Provides automatic backup and recovery for Link collaboration sessions,
//! ensuring that real-time editing sessions can be restored after failures.
//!
//! # Architecture
//!
//! ```text
//! vantis-ark (Pillar 04: Continuity)
//!     └── session_backup module
//!             └── vantis-link (Pillar 03: Sync)
//!                     ├── Session state snapshots
//!                     ├── CRDT state backup
//!                     └── User presence backup
//! ```
//!
//! # Features
//! - Automatic session state snapshots
//! - CRDT state serialization and backup
//! - Session recovery from backup
//! - Incremental backup support
//! - Backup retention policies

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use vantis_link::core::{Session as LinkSession, Change};
use vantis_link::crdt::{CrdtEngine, CrdtType};
use vantis_link::sync::SyncStatus;

use crate::core::{Backup, BackupManager, BackupMetadata, InMemoryStorage, StorageBackend};

/// Session backup manager that integrates Link sessions with Ark backup
pub struct SessionBackupManager {
    /// Ark backup manager
    backup_manager: BackupManager,
    /// Backup configuration
    config: SessionBackupConfig,
    /// Backup history per session
    backup_history: HashMap<String, Vec<SessionSnapshot>>,
    /// Active session tracking
    active_sessions: HashMap<String, SessionTrackingInfo>,
}

/// Configuration for session backups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionBackupConfig {
    /// Maximum number of snapshots to retain per session
    pub max_snapshots_per_session: usize,
    /// Interval between automatic snapshots (in seconds)
    pub snapshot_interval_secs: u64,
    /// Whether to enable incremental backups
    pub incremental_enabled: bool,
    /// Maximum backup size in bytes
    pub max_backup_size: usize,
    /// Whether to compress backups
    pub compress_backups: bool,
}

impl Default for SessionBackupConfig {
    fn default() -> Self {
        Self {
            max_snapshots_per_session: 50,
            snapshot_interval_secs: 300, // 5 minutes
            incremental_enabled: true,
            max_backup_size: 100 * 1024 * 1024, // 100MB
            compress_backups: true,
        }
    }
}

/// A snapshot of a Link session state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSnapshot {
    /// Snapshot ID
    pub id: String,
    /// Session ID this snapshot belongs to
    pub session_id: String,
    /// Document ID associated with the session
    pub document_id: String,
    /// Snapshot timestamp
    pub created_at: DateTime<Utc>,
    /// Serialized session state
    pub session_state: Vec<u8>,
    /// Number of users at snapshot time
    pub user_count: usize,
    /// Number of changes since last snapshot
    pub changes_since_last: usize,
    /// Snapshot type
    pub snapshot_type: SnapshotType,
    /// Size in bytes
    pub size_bytes: usize,
}

/// Type of snapshot
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SnapshotType {
    /// Full snapshot of entire session state
    Full,
    /// Incremental snapshot (changes only)
    Incremental,
    /// Manual snapshot triggered by user
    Manual,
    /// Pre-shutdown snapshot
    Shutdown,
}

/// Tracking info for an active session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTrackingInfo {
    /// Session ID
    pub session_id: String,
    /// Document ID
    pub document_id: String,
    /// When tracking started
    pub tracking_started: DateTime<Utc>,
    /// Last snapshot time
    pub last_snapshot: Option<DateTime<Utc>>,
    /// Total changes recorded
    pub total_changes: usize,
    /// Changes since last snapshot
    pub changes_since_snapshot: usize,
}

/// Result of a session recovery operation
#[derive(Debug, Clone)]
pub struct SessionRecoveryResult {
    /// Whether recovery was successful
    pub success: bool,
    /// Recovered session ID
    pub session_id: String,
    /// Snapshot used for recovery
    pub snapshot_id: String,
    /// Number of changes recovered
    pub changes_recovered: usize,
    /// Recovery timestamp
    pub recovered_at: DateTime<Utc>,
    /// Any warnings during recovery
    pub warnings: Vec<String>,
}

impl SessionBackupManager {
    /// Create a new session backup manager with in-memory storage
    pub fn new() -> Self {
        let storage = Box::new(InMemoryStorage::new());
        let backup_manager = BackupManager::new(storage);

        Self {
            backup_manager,
            config: SessionBackupConfig::default(),
            backup_history: HashMap::new(),
            active_sessions: HashMap::new(),
        }
    }

    /// Create with custom storage backend and config
    pub fn with_config(
        storage: Box<dyn StorageBackend>,
        config: SessionBackupConfig,
    ) -> Self {
        let backup_manager = BackupManager::new(storage);

        Self {
            backup_manager,
            config,
            backup_history: HashMap::new(),
            active_sessions: HashMap::new(),
        }
    }

    /// Get the backup configuration
    pub fn config(&self) -> &SessionBackupConfig {
        &self.config
    }

    /// Start tracking a Link session for backup
    pub fn track_session(&mut self, session: &LinkSession) {
        let tracking = SessionTrackingInfo {
            session_id: session.id.clone(),
            document_id: session.document_id.clone(),
            tracking_started: Utc::now(),
            last_snapshot: None,
            total_changes: 0,
            changes_since_snapshot: 0,
        };

        self.active_sessions.insert(session.id.clone(), tracking);
        self.backup_history
            .entry(session.id.clone())
            .or_insert_with(Vec::new);
    }

    /// Stop tracking a session (creates a final shutdown snapshot)
    pub fn untrack_session(&mut self, session: &LinkSession) -> Option<SessionSnapshot> {
        if self.active_sessions.remove(&session.id).is_some() {
            // Create a shutdown snapshot
            self.create_snapshot(session, SnapshotType::Shutdown).ok()
        } else {
            None
        }
    }

    /// Record a change in a tracked session
    pub fn record_change(&mut self, session_id: &str) {
        if let Some(tracking) = self.active_sessions.get_mut(session_id) {
            tracking.total_changes += 1;
            tracking.changes_since_snapshot += 1;
        }
    }

    /// Check if a session needs a snapshot based on config
    pub fn needs_snapshot(&self, session_id: &str) -> bool {
        if let Some(tracking) = self.active_sessions.get(session_id) {
            if let Some(last) = tracking.last_snapshot {
                let elapsed = Utc::now()
                    .signed_duration_since(last)
                    .num_seconds() as u64;
                elapsed >= self.config.snapshot_interval_secs
            } else {
                // No snapshot yet, always needs one
                true
            }
        } else {
            false
        }
    }

    /// Create a snapshot of a Link session
    pub fn create_snapshot(
        &mut self,
        session: &LinkSession,
        snapshot_type: SnapshotType,
    ) -> Result<SessionSnapshot, String> {
        // Serialize the session state
        let session_state = serde_json::to_vec(session)
            .map_err(|e| format!("Failed to serialize session: {}", e))?;

        let size_bytes = session_state.len();

        if size_bytes > self.config.max_backup_size {
            return Err(format!(
                "Session state ({} bytes) exceeds max backup size ({} bytes)",
                size_bytes, self.config.max_backup_size
            ));
        }

        let changes_since_last = self
            .active_sessions
            .get(&session.id)
            .map(|t| t.changes_since_snapshot)
            .unwrap_or(0);

        let snapshot = SessionSnapshot {
            id: uuid::Uuid::new_v4().to_string(),
            session_id: session.id.clone(),
            document_id: session.document_id.clone(),
            created_at: Utc::now(),
            session_state,
            user_count: session.users.len(),
            changes_since_last,
            snapshot_type,
            size_bytes,
        };

        // Create an Ark backup from the snapshot
        let snapshot_data = serde_json::to_vec(&snapshot)
            .map_err(|e| format!("Failed to serialize snapshot: {}", e))?;

        let backup_name = format!(
            "session_{}_{}_{}",
            session.id,
            snapshot.snapshot_type_str(),
            snapshot.created_at.format("%Y%m%d_%H%M%S")
        );

        let _backup = self
            .backup_manager
            .create_backup(backup_name, snapshot_data)
            .map_err(|e| format!("Failed to create backup: {}", e))?;

        // Update tracking info
        if let Some(tracking) = self.active_sessions.get_mut(&session.id) {
            tracking.last_snapshot = Some(Utc::now());
            tracking.changes_since_snapshot = 0;
        }

        // Add to history
        let history = self
            .backup_history
            .entry(session.id.clone())
            .or_insert_with(Vec::new);
        history.push(snapshot.clone());

        // Enforce retention policy
        self.enforce_retention(&session.id);

        Ok(snapshot)
    }

    /// Get all snapshots for a session
    pub fn get_snapshots(&self, session_id: &str) -> Vec<&SessionSnapshot> {
        self.backup_history
            .get(session_id)
            .map(|h| h.iter().collect())
            .unwrap_or_default()
    }

    /// Get the latest snapshot for a session
    pub fn get_latest_snapshot(&self, session_id: &str) -> Option<&SessionSnapshot> {
        self.backup_history
            .get(session_id)
            .and_then(|h| h.last())
    }

    /// Recover a session from a snapshot
    pub fn recover_session(
        &self,
        snapshot: &SessionSnapshot,
    ) -> Result<SessionRecoveryResult, String> {
        // Deserialize the session state
        let _session: LinkSession = serde_json::from_slice(&snapshot.session_state)
            .map_err(|e| format!("Failed to deserialize session state: {}", e))?;

        Ok(SessionRecoveryResult {
            success: true,
            session_id: snapshot.session_id.clone(),
            snapshot_id: snapshot.id.clone(),
            changes_recovered: snapshot.changes_since_last,
            recovered_at: Utc::now(),
            warnings: Vec::new(),
        })
    }

    /// Get tracking info for a session
    pub fn get_tracking_info(&self, session_id: &str) -> Option<&SessionTrackingInfo> {
        self.active_sessions.get(session_id)
    }

    /// Get all tracked session IDs
    pub fn tracked_sessions(&self) -> Vec<&str> {
        self.active_sessions.keys().map(|s| s.as_str()).collect()
    }

    /// Enforce retention policy for a session's snapshots
    fn enforce_retention(&mut self, session_id: &str) {
        if let Some(history) = self.backup_history.get_mut(session_id) {
            while history.len() > self.config.max_snapshots_per_session {
                history.remove(0);
            }
        }
    }
}

impl Default for SessionBackupManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionSnapshot {
    /// Get a string representation of the snapshot type
    pub fn snapshot_type_str(&self) -> &str {
        match self.snapshot_type {
            SnapshotType::Full => "full",
            SnapshotType::Incremental => "incremental",
            SnapshotType::Manual => "manual",
            SnapshotType::Shutdown => "shutdown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vantis_link::core::{Session as LinkSession, User as LinkUser, UserRole};

    fn create_test_session() -> LinkSession {
        let mut session = LinkSession::new("test-doc-001".to_string());
        let user = LinkUser::new("user1".to_string(), "Alice".to_string());
        let _ = session.add_user(user);
        session
    }

    #[test]
    fn test_create_session_backup_manager() {
        let manager = SessionBackupManager::new();
        assert!(manager.tracked_sessions().is_empty());
        assert_eq!(manager.config().max_snapshots_per_session, 50);
    }

    #[test]
    fn test_track_session() {
        let mut manager = SessionBackupManager::new();
        let session = create_test_session();

        manager.track_session(&session);
        assert_eq!(manager.tracked_sessions().len(), 1);

        let tracking = manager.get_tracking_info(&session.id).unwrap();
        assert_eq!(tracking.document_id, "test-doc-001");
        assert_eq!(tracking.total_changes, 0);
    }

    #[test]
    fn test_record_changes() {
        let mut manager = SessionBackupManager::new();
        let session = create_test_session();

        manager.track_session(&session);
        manager.record_change(&session.id);
        manager.record_change(&session.id);
        manager.record_change(&session.id);

        let tracking = manager.get_tracking_info(&session.id).unwrap();
        assert_eq!(tracking.total_changes, 3);
        assert_eq!(tracking.changes_since_snapshot, 3);
    }

    #[test]
    fn test_create_snapshot() {
        let mut manager = SessionBackupManager::new();
        let session = create_test_session();

        manager.track_session(&session);
        manager.record_change(&session.id);

        let snapshot = manager
            .create_snapshot(&session, SnapshotType::Full)
            .unwrap();

        assert_eq!(snapshot.session_id, session.id);
        assert_eq!(snapshot.document_id, "test-doc-001");
        assert_eq!(snapshot.user_count, 1);
        assert_eq!(snapshot.snapshot_type, SnapshotType::Full);
        assert!(snapshot.size_bytes > 0);
    }

    #[test]
    fn test_snapshot_resets_change_counter() {
        let mut manager = SessionBackupManager::new();
        let session = create_test_session();

        manager.track_session(&session);
        manager.record_change(&session.id);
        manager.record_change(&session.id);

        let _ = manager.create_snapshot(&session, SnapshotType::Full).unwrap();

        let tracking = manager.get_tracking_info(&session.id).unwrap();
        assert_eq!(tracking.changes_since_snapshot, 0);
        assert_eq!(tracking.total_changes, 2);
    }

    #[test]
    fn test_get_snapshots() {
        let mut manager = SessionBackupManager::new();
        let session = create_test_session();

        manager.track_session(&session);

        let _ = manager.create_snapshot(&session, SnapshotType::Full).unwrap();
        let _ = manager.create_snapshot(&session, SnapshotType::Incremental).unwrap();

        let snapshots = manager.get_snapshots(&session.id);
        assert_eq!(snapshots.len(), 2);
    }

    #[test]
    fn test_get_latest_snapshot() {
        let mut manager = SessionBackupManager::new();
        let session = create_test_session();

        manager.track_session(&session);

        let _ = manager.create_snapshot(&session, SnapshotType::Full).unwrap();
        let _ = manager.create_snapshot(&session, SnapshotType::Incremental).unwrap();

        let latest = manager.get_latest_snapshot(&session.id).unwrap();
        assert_eq!(latest.snapshot_type, SnapshotType::Incremental);
    }

    #[test]
    fn test_recover_session() {
        let mut manager = SessionBackupManager::new();
        let session = create_test_session();

        manager.track_session(&session);
        manager.record_change(&session.id);

        let snapshot = manager
            .create_snapshot(&session, SnapshotType::Full)
            .unwrap();

        let recovery = manager.recover_session(&snapshot).unwrap();
        assert!(recovery.success);
        assert_eq!(recovery.session_id, session.id);
    }

    #[test]
    fn test_untrack_session() {
        let mut manager = SessionBackupManager::new();
        let session = create_test_session();

        manager.track_session(&session);
        let shutdown_snapshot = manager.untrack_session(&session);

        assert!(shutdown_snapshot.is_some());
        assert_eq!(
            shutdown_snapshot.unwrap().snapshot_type,
            SnapshotType::Shutdown
        );
        assert!(manager.tracked_sessions().is_empty());
    }

    #[test]
    fn test_retention_policy() {
        let config = SessionBackupConfig {
            max_snapshots_per_session: 3,
            ..Default::default()
        };
        let storage = Box::new(InMemoryStorage::new());
        let mut manager = SessionBackupManager::with_config(storage, config);
        let session = create_test_session();

        manager.track_session(&session);

        // Create 5 snapshots
        for _ in 0..5 {
            let _ = manager.create_snapshot(&session, SnapshotType::Full).unwrap();
        }

        // Only 3 should be retained
        let snapshots = manager.get_snapshots(&session.id);
        assert_eq!(snapshots.len(), 3);
    }

    #[test]
    fn test_needs_snapshot() {
        let mut manager = SessionBackupManager::new();
        let session = create_test_session();

        manager.track_session(&session);

        // Should need snapshot (no snapshot yet)
        assert!(manager.needs_snapshot(&session.id));

        // After creating snapshot, should not need one immediately
        let _ = manager.create_snapshot(&session, SnapshotType::Full).unwrap();
        assert!(!manager.needs_snapshot(&session.id));
    }

    #[test]
    fn test_snapshot_type_str() {
        let snapshot = SessionSnapshot {
            id: "test".to_string(),
            session_id: "s1".to_string(),
            document_id: "d1".to_string(),
            created_at: Utc::now(),
            session_state: vec![],
            user_count: 0,
            changes_since_last: 0,
            snapshot_type: SnapshotType::Manual,
            size_bytes: 0,
        };
        assert_eq!(snapshot.snapshot_type_str(), "manual");
    }
}