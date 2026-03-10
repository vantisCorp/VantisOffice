//! Offline Sync Module - Integration with Vantis Link (Pillar 03)
//!
//! Provides offline editing capabilities for mobile devices by caching
//! Link session state and queuing changes for sync when connectivity
//! is restored.
//!
//! # Architecture
//!
//! ```text
//! vantis-mobile (Pillar 04: Continuity)
//!     └── offline_sync module
//!             └── vantis-link (Pillar 03: Sync)
//!                     ├── Session state caching
//!                     ├── Change queue management
//!                     └── Conflict resolution on reconnect
//! ```
//!
//! # Features
//! - Offline change queue with automatic sync on reconnect
//! - Session state caching for offline access
//! - Conflict detection and resolution
//! - Bandwidth-aware sync strategies
//! - Progress tracking for sync operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;

use vantis_link::core::{Change, ChangeType, Session as LinkSession, User as LinkUser};
use vantis_link::sync::SyncStatus;

use crate::models::{ConnectionStatus, DocumentID, DocumentType, SyncProgress, SyncStatus as MobileSyncStatus};

/// Offline sync manager for mobile devices
pub struct OfflineSyncManager {
    /// Device ID
    device_id: Uuid,
    /// Pending changes queue per document
    change_queues: HashMap<String, VecDeque<OfflineChange>>,
    /// Cached session states
    cached_sessions: HashMap<String, CachedSession>,
    /// Sync configuration
    config: OfflineSyncConfig,
    /// Current connection status
    connection_status: ConnectionStatus,
    /// Sync history
    sync_history: Vec<SyncRecord>,
}

/// Configuration for offline sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineSyncConfig {
    /// Maximum number of queued changes per document
    pub max_queue_size: usize,
    /// Maximum cache size in bytes
    pub max_cache_size: usize,
    /// Whether to auto-sync on reconnect
    pub auto_sync_on_reconnect: bool,
    /// Sync batch size
    pub batch_size: usize,
    /// Conflict resolution strategy
    pub conflict_strategy: ConflictStrategy,
}

impl Default for OfflineSyncConfig {
    fn default() -> Self {
        Self {
            max_queue_size: 1000,
            max_cache_size: 50 * 1024 * 1024, // 50MB
            auto_sync_on_reconnect: true,
            batch_size: 50,
            conflict_strategy: ConflictStrategy::LastWriterWins,
        }
    }
}

/// Strategy for resolving conflicts on reconnect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictStrategy {
    /// Last writer wins (based on timestamp)
    LastWriterWins,
    /// Local changes take priority
    LocalFirst,
    /// Remote changes take priority
    RemoteFirst,
    /// Queue conflicts for manual resolution
    Manual,
}

/// A change made while offline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineChange {
    /// Change ID
    pub id: String,
    /// Document ID
    pub document_id: String,
    /// User who made the change
    pub user_id: String,
    /// Type of change
    pub change_type: OfflineChangeType,
    /// Position in document
    pub position: usize,
    /// Content of the change
    pub content: String,
    /// When the change was made
    pub created_at: DateTime<Utc>,
    /// Whether this change has been synced
    pub synced: bool,
}

/// Type of offline change
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfflineChangeType {
    /// Text insertion
    Insert,
    /// Text deletion
    Delete,
    /// Format change
    Format,
    /// Cell update (for Grid)
    CellUpdate,
    /// Shape modification (for Canvas)
    ShapeModify,
}

/// A cached Link session for offline access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedSession {
    /// Session ID
    pub session_id: String,
    /// Document ID
    pub document_id: String,
    /// When the session was cached
    pub cached_at: DateTime<Utc>,
    /// Serialized session state
    pub session_data: Vec<u8>,
    /// Number of users at cache time
    pub user_count: usize,
    /// Cache size in bytes
    pub size_bytes: usize,
    /// Whether the cache is stale
    pub is_stale: bool,
}

/// A conflict detected during sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    /// Conflict ID
    pub id: String,
    /// Document ID
    pub document_id: String,
    /// Local change
    pub local_change: OfflineChange,
    /// Remote change description
    pub remote_description: String,
    /// Whether the conflict has been resolved
    pub resolved: bool,
    /// Resolution (if resolved)
    pub resolution: Option<ConflictResolution>,
}

/// How a conflict was resolved
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Kept local version
    KeepLocal,
    /// Kept remote version
    KeepRemote,
    /// Merged both versions
    Merged,
    /// Discarded both
    Discarded,
}

/// Record of a sync operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRecord {
    /// Sync ID
    pub sync_id: Uuid,
    /// When sync started
    pub started_at: DateTime<Utc>,
    /// When sync completed
    pub completed_at: Option<DateTime<Utc>>,
    /// Number of changes synced
    pub changes_synced: usize,
    /// Number of conflicts detected
    pub conflicts_detected: usize,
    /// Whether sync was successful
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

impl OfflineSyncManager {
    /// Create a new offline sync manager
    pub fn new(device_id: Uuid) -> Self {
        Self {
            device_id,
            change_queues: HashMap::new(),
            cached_sessions: HashMap::new(),
            config: OfflineSyncConfig::default(),
            connection_status: ConnectionStatus::Disconnected,
            sync_history: Vec::new(),
        }
    }

    /// Create with custom configuration
    pub fn with_config(device_id: Uuid, config: OfflineSyncConfig) -> Self {
        Self {
            device_id,
            change_queues: HashMap::new(),
            cached_sessions: HashMap::new(),
            config,
            connection_status: ConnectionStatus::Disconnected,
            sync_history: Vec::new(),
        }
    }

    /// Get the device ID
    pub fn device_id(&self) -> Uuid {
        self.device_id
    }

    /// Get the configuration
    pub fn config(&self) -> &OfflineSyncConfig {
        &self.config
    }

    /// Get current connection status
    pub fn connection_status(&self) -> ConnectionStatus {
        self.connection_status
    }

    /// Update connection status
    pub fn set_connection_status(&mut self, status: ConnectionStatus) {
        self.connection_status = status;
    }

    /// Cache a Link session for offline access
    pub fn cache_session(&mut self, session: &LinkSession) -> Result<CachedSession, String> {
        let session_data = serde_json::to_vec(session)
            .map_err(|e| format!("Failed to serialize session: {}", e))?;

        let size_bytes = session_data.len();

        // Check cache size limit
        let total_cache: usize = self.cached_sessions.values().map(|c| c.size_bytes).sum();
        if total_cache + size_bytes > self.config.max_cache_size {
            return Err(format!(
                "Cache size limit exceeded ({} + {} > {})",
                total_cache, size_bytes, self.config.max_cache_size
            ));
        }

        let cached = CachedSession {
            session_id: session.id.clone(),
            document_id: session.document_id.clone(),
            cached_at: Utc::now(),
            session_data,
            user_count: session.users.len(),
            size_bytes,
            is_stale: false,
        };

        self.cached_sessions
            .insert(session.id.clone(), cached.clone());

        Ok(cached)
    }

    /// Get a cached session
    pub fn get_cached_session(&self, session_id: &str) -> Option<&CachedSession> {
        self.cached_sessions.get(session_id)
    }

    /// Mark a cached session as stale
    pub fn mark_session_stale(&mut self, session_id: &str) {
        if let Some(cached) = self.cached_sessions.get_mut(session_id) {
            cached.is_stale = true;
        }
    }

    /// Remove a cached session
    pub fn remove_cached_session(&mut self, session_id: &str) -> Option<CachedSession> {
        self.cached_sessions.remove(session_id)
    }

    /// Queue an offline change
    pub fn queue_change(
        &mut self,
        document_id: &str,
        user_id: &str,
        change_type: OfflineChangeType,
        position: usize,
        content: String,
    ) -> Result<OfflineChange, String> {
        let queue = self
            .change_queues
            .entry(document_id.to_string())
            .or_insert_with(VecDeque::new);

        if queue.len() >= self.config.max_queue_size {
            return Err(format!(
                "Change queue full for document {} (max: {})",
                document_id, self.config.max_queue_size
            ));
        }

        let change = OfflineChange {
            id: Uuid::new_v4().to_string(),
            document_id: document_id.to_string(),
            user_id: user_id.to_string(),
            change_type,
            position,
            content,
            created_at: Utc::now(),
            synced: false,
        };

        queue.push_back(change.clone());
        Ok(change)
    }

    /// Get pending changes for a document
    pub fn pending_changes(&self, document_id: &str) -> Vec<&OfflineChange> {
        self.change_queues
            .get(document_id)
            .map(|q| q.iter().filter(|c| !c.synced).collect())
            .unwrap_or_default()
    }

    /// Get total pending change count across all documents
    pub fn total_pending_changes(&self) -> usize {
        self.change_queues
            .values()
            .flat_map(|q| q.iter())
            .filter(|c| !c.synced)
            .count()
    }

    /// Convert offline changes to Link changes for sync
    pub fn prepare_sync_batch(
        &self,
        document_id: &str,
    ) -> Vec<Change> {
        let pending = self.pending_changes(document_id);
        let batch_size = self.config.batch_size.min(pending.len());

        pending[..batch_size]
            .iter()
            .map(|offline_change| {
                let change_type = match offline_change.change_type {
                    OfflineChangeType::Insert => ChangeType::Insert,
                    OfflineChangeType::Delete => ChangeType::Delete,
                    OfflineChangeType::Format => ChangeType::Replace,
                    OfflineChangeType::CellUpdate => ChangeType::Replace,
                    OfflineChangeType::ShapeModify => ChangeType::Replace,
                };

                Change::new(
                    offline_change.user_id.clone(),
                    change_type,
                    offline_change.position,
                    offline_change.content.clone(),
                )
            })
            .collect()
    }

    /// Mark changes as synced
    pub fn mark_synced(
        &mut self,
        document_id: &str,
        count: usize,
    ) {
        if let Some(queue) = self.change_queues.get_mut(document_id) {
            let mut marked = 0;
            for change in queue.iter_mut() {
                if !change.synced && marked < count {
                    change.synced = true;
                    marked += 1;
                }
            }
        }
    }

    /// Clear synced changes from queue
    pub fn clear_synced(&mut self, document_id: &str) {
        if let Some(queue) = self.change_queues.get_mut(document_id) {
            queue.retain(|c| !c.synced);
        }
    }

    /// Record a sync operation
    pub fn record_sync(
        &mut self,
        changes_synced: usize,
        conflicts: usize,
        success: bool,
        error: Option<String>,
    ) -> SyncRecord {
        let record = SyncRecord {
            sync_id: Uuid::new_v4(),
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
            changes_synced,
            conflicts_detected: conflicts,
            success,
            error,
        };

        self.sync_history.push(record.clone());
        record
    }

    /// Get sync history
    pub fn sync_history(&self) -> &[SyncRecord] {
        &self.sync_history
    }

    /// Get sync progress for a document
    pub fn get_sync_progress(&self, document_id: &str) -> SyncProgress {
        let total = self
            .change_queues
            .get(document_id)
            .map(|q| q.len() as u32)
            .unwrap_or(0);

        let synced = self
            .change_queues
            .get(document_id)
            .map(|q| q.iter().filter(|c| c.synced).count() as u32)
            .unwrap_or(0);

        SyncProgress {
            sync_id: Uuid::new_v4(),
            status: if total == 0 {
                MobileSyncStatus::Idle
            } else if synced == total {
                MobileSyncStatus::Idle
            } else {
                MobileSyncStatus::Syncing
            },
            processed: synced,
            total,
            bytes_transferred: 0,
            total_bytes: 0,
            error: None,
        }
    }

    /// Get all cached session IDs
    pub fn cached_session_ids(&self) -> Vec<&str> {
        self.cached_sessions.keys().map(|s| s.as_str()).collect()
    }

    /// Get total cache size in bytes
    pub fn total_cache_size(&self) -> usize {
        self.cached_sessions.values().map(|c| c.size_bytes).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vantis_link::core::{Session as LinkSession, User as LinkUser};

    fn create_test_manager() -> OfflineSyncManager {
        OfflineSyncManager::new(Uuid::new_v4())
    }

    fn create_test_session() -> LinkSession {
        let mut session = LinkSession::new("doc-001".to_string());
        let user = LinkUser::new("user1".to_string(), "Alice".to_string());
        let _ = session.add_user(user);
        session
    }

    #[test]
    fn test_create_manager() {
        let manager = create_test_manager();
        assert_eq!(manager.total_pending_changes(), 0);
        assert!(manager.cached_session_ids().is_empty());
        assert_eq!(manager.connection_status(), ConnectionStatus::Disconnected);
    }

    #[test]
    fn test_cache_session() {
        let mut manager = create_test_manager();
        let session = create_test_session();

        let cached = manager.cache_session(&session).unwrap();
        assert_eq!(cached.document_id, "doc-001");
        assert_eq!(cached.user_count, 1);
        assert!(!cached.is_stale);
        assert!(cached.size_bytes > 0);
    }

    #[test]
    fn test_get_cached_session() {
        let mut manager = create_test_manager();
        let session = create_test_session();

        manager.cache_session(&session).unwrap();

        let cached = manager.get_cached_session(&session.id).unwrap();
        assert_eq!(cached.document_id, "doc-001");
    }

    #[test]
    fn test_mark_session_stale() {
        let mut manager = create_test_manager();
        let session = create_test_session();

        manager.cache_session(&session).unwrap();
        manager.mark_session_stale(&session.id);

        let cached = manager.get_cached_session(&session.id).unwrap();
        assert!(cached.is_stale);
    }

    #[test]
    fn test_queue_change() {
        let mut manager = create_test_manager();

        let change = manager
            .queue_change("doc-001", "user1", OfflineChangeType::Insert, 0, "Hello".to_string())
            .unwrap();

        assert_eq!(change.document_id, "doc-001");
        assert!(!change.synced);
        assert_eq!(manager.total_pending_changes(), 1);
    }

    #[test]
    fn test_multiple_changes() {
        let mut manager = create_test_manager();

        manager
            .queue_change("doc-001", "user1", OfflineChangeType::Insert, 0, "Hello".to_string())
            .unwrap();
        manager
            .queue_change("doc-001", "user1", OfflineChangeType::Insert, 5, " World".to_string())
            .unwrap();
        manager
            .queue_change("doc-002", "user1", OfflineChangeType::Delete, 0, "x".to_string())
            .unwrap();

        assert_eq!(manager.pending_changes("doc-001").len(), 2);
        assert_eq!(manager.pending_changes("doc-002").len(), 1);
        assert_eq!(manager.total_pending_changes(), 3);
    }

    #[test]
    fn test_queue_size_limit() {
        let config = OfflineSyncConfig {
            max_queue_size: 2,
            ..Default::default()
        };
        let mut manager = OfflineSyncManager::with_config(Uuid::new_v4(), config);

        manager
            .queue_change("doc-001", "user1", OfflineChangeType::Insert, 0, "a".to_string())
            .unwrap();
        manager
            .queue_change("doc-001", "user1", OfflineChangeType::Insert, 1, "b".to_string())
            .unwrap();

        let result = manager.queue_change(
            "doc-001", "user1", OfflineChangeType::Insert, 2, "c".to_string(),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_prepare_sync_batch() {
        let mut manager = create_test_manager();

        manager
            .queue_change("doc-001", "user1", OfflineChangeType::Insert, 0, "Hello".to_string())
            .unwrap();
        manager
            .queue_change("doc-001", "user1", OfflineChangeType::Delete, 3, "lo".to_string())
            .unwrap();

        let batch = manager.prepare_sync_batch("doc-001");
        assert_eq!(batch.len(), 2);
    }

    #[test]
    fn test_mark_synced_and_clear() {
        let mut manager = create_test_manager();

        manager
            .queue_change("doc-001", "user1", OfflineChangeType::Insert, 0, "a".to_string())
            .unwrap();
        manager
            .queue_change("doc-001", "user1", OfflineChangeType::Insert, 1, "b".to_string())
            .unwrap();

        manager.mark_synced("doc-001", 1);
        assert_eq!(manager.pending_changes("doc-001").len(), 1);

        manager.clear_synced("doc-001");
        assert_eq!(manager.total_pending_changes(), 1);
    }

    #[test]
    fn test_record_sync() {
        let mut manager = create_test_manager();

        let record = manager.record_sync(10, 0, true, None);
        assert!(record.success);
        assert_eq!(record.changes_synced, 10);
        assert_eq!(manager.sync_history().len(), 1);
    }

    #[test]
    fn test_sync_progress() {
        let mut manager = create_test_manager();

        manager
            .queue_change("doc-001", "user1", OfflineChangeType::Insert, 0, "a".to_string())
            .unwrap();
        manager
            .queue_change("doc-001", "user1", OfflineChangeType::Insert, 1, "b".to_string())
            .unwrap();

        let progress = manager.get_sync_progress("doc-001");
        assert_eq!(progress.total, 2);
        assert_eq!(progress.processed, 0);
        assert_eq!(progress.status, MobileSyncStatus::Syncing);
    }

    #[test]
    fn test_connection_status() {
        let mut manager = create_test_manager();

        manager.set_connection_status(ConnectionStatus::Connected);
        assert_eq!(manager.connection_status(), ConnectionStatus::Connected);

        manager.set_connection_status(ConnectionStatus::Reconnecting);
        assert_eq!(manager.connection_status(), ConnectionStatus::Reconnecting);
    }

    #[test]
    fn test_remove_cached_session() {
        let mut manager = create_test_manager();
        let session = create_test_session();

        manager.cache_session(&session).unwrap();
        assert_eq!(manager.cached_session_ids().len(), 1);

        let removed = manager.remove_cached_session(&session.id);
        assert!(removed.is_some());
        assert!(manager.cached_session_ids().is_empty());
    }

    #[test]
    fn test_total_cache_size() {
        let mut manager = create_test_manager();
        let session = create_test_session();

        let cached = manager.cache_session(&session).unwrap();
        assert_eq!(manager.total_cache_size(), cached.size_bytes);
    }
}