//! Real-time Sync Module - Integration with Vantis Link (Pillar 03)
//!
//! Provides real-time collaborative editing for spreadsheets using
//! CRDT-based conflict resolution from vantis-link.
//!
//! # Architecture
//!
//! ```text
//! vantis-grid (Pillar 02: Logic)
//!     └── realtime_sync module
//!             └── vantis-link (Pillar 03: Sync)
//!                     ├── CRDT Engine
//!                     ├── Sync Manager
//!                     └── P2P Transport
//! ```
//!
//! # Features
//! - Real-time cell editing with CRDT conflict resolution
//! - User presence and cell selection tracking
//! - Cell-level locking for exclusive edits
//! - Change propagation across peers
//! - Offline editing with automatic merge

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use vantis_link::core::{Session as LinkSession, User as LinkUser, UserRole};
use vantis_link::crdt::{CrdtEngine, CrdtType, CrdtOperationType};
use vantis_link::sync::SyncManager;

use crate::core::CellValue;
use crate::GridError;

/// Real-time sync session for a spreadsheet
pub struct GridSyncSession {
    /// Link session for P2P sync
    link_session: LinkSession,
    /// CRDT engine for conflict resolution
    crdt_engine: CrdtEngine,
    /// Sync manager
    sync_manager: SyncManager,
    /// Local user ID
    local_user_id: String,
    /// Workbook ID being synced
    workbook_id: String,
    /// Cell change history
    change_history: Vec<CellChange>,
    /// Active users and their selections
    user_selections: HashMap<String, UserCellSelection>,
    /// Cell locks
    cell_locks: HashMap<String, CellLock>,
    /// Session state
    state: GridSyncState,
}

/// State of the sync session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridSyncState {
    /// Connected and syncing
    Connected,
    /// Syncing pending changes
    Syncing,
    /// Offline mode
    Offline,
    /// Disconnected
    Disconnected,
}

/// A cell change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellChange {
    /// Change ID
    pub id: String,
    /// User who made the change
    pub user_id: String,
    /// Worksheet name
    pub worksheet: String,
    /// Cell reference (e.g., "A1")
    pub cell_ref: String,
    /// Row index
    pub row: usize,
    /// Column index
    pub column: usize,
    /// Previous value
    pub old_value: CellValue,
    /// New value
    pub new_value: CellValue,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Change type
    pub change_type: CellChangeType,
}

/// Type of cell change
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellChangeType {
    /// Value was set
    ValueSet,
    /// Value was cleared
    ValueCleared,
    /// Formula was set
    FormulaSet,
    /// Style was changed
    StyleChanged,
    /// Cell was merged
    CellMerged,
}

/// User's current cell selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCellSelection {
    /// User ID
    pub user_id: String,
    /// User display name
    pub user_name: String,
    /// Selected worksheet
    pub worksheet: String,
    /// Selected cell range start (row, col)
    pub selection_start: (usize, usize),
    /// Selected cell range end (row, col)
    pub selection_end: (usize, usize),
    /// User's highlight color
    pub color: String,
    /// Last activity
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// A lock on a cell or range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellLock {
    /// Cell reference
    pub cell_ref: String,
    /// User who holds the lock
    pub locked_by: String,
    /// Lock timestamp
    pub locked_at: chrono::DateTime<chrono::Utc>,
    /// Lock expiry (auto-release)
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl GridSyncSession {
    /// Create a new sync session
    pub fn new(
        workbook_id: String,
        user_id: String,
        user_name: String,
    ) -> Result<Self, GridError> {
        let mut link_session = LinkSession::new(workbook_id.clone());

        let link_user = LinkUser {
            id: user_id.clone(),
            name: user_name.clone(),
            email: None,
            role: UserRole::Owner,
            joined_at: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            is_online: true,
            cursor: None,
        };
        link_session
            .add_user(link_user)
            .map_err(|e| GridError::Collaboration(format!("Failed to create session: {}", e)))?;

        let crdt_engine = CrdtEngine::new(CrdtType::Lww);
        let sync_manager = SyncManager::new();

        let mut user_selections = HashMap::new();
        user_selections.insert(
            user_id.clone(),
            UserCellSelection {
                user_id: user_id.clone(),
                user_name,
                worksheet: "Sheet1".to_string(),
                selection_start: (0, 0),
                selection_end: (0, 0),
                color: "#4285F4".to_string(),
                last_activity: chrono::Utc::now(),
            },
        );

        Ok(GridSyncSession {
            link_session,
            crdt_engine,
            sync_manager,
            local_user_id: user_id,
            workbook_id,
            change_history: Vec::new(),
            user_selections,
            cell_locks: HashMap::new(),
            state: GridSyncState::Connected,
        })
    }

    /// Get session ID
    pub fn session_id(&self) -> &str {
        &self.link_session.id
    }

    /// Get workbook ID
    pub fn workbook_id(&self) -> &str {
        &self.workbook_id
    }

    /// Get current state
    pub fn state(&self) -> GridSyncState {
        self.state
    }

    /// Get user count
    pub fn user_count(&self) -> usize {
        self.user_selections.len()
    }

    /// Get change history
    pub fn change_history(&self) -> &[CellChange] {
        &self.change_history
    }

    /// Get user selections
    pub fn user_selections(&self) -> &HashMap<String, UserCellSelection> {
        &self.user_selections
    }

    /// Add a user to the sync session
    pub fn add_user(
        &mut self,
        user_id: String,
        user_name: String,
        color: String,
    ) -> Result<(), GridError> {
        if self.state == GridSyncState::Disconnected {
            return Err(GridError::Collaboration("Session is disconnected".to_string()));
        }

        let link_user = LinkUser {
            id: user_id.clone(),
            name: user_name.clone(),
            email: None,
            role: UserRole::Editor,
            joined_at: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            is_online: true,
            cursor: None,
        };
        self.link_session
            .add_user(link_user)
            .map_err(|e| GridError::Collaboration(e))?;

        self.user_selections.insert(
            user_id.clone(),
            UserCellSelection {
                user_id,
                user_name,
                worksheet: "Sheet1".to_string(),
                selection_start: (0, 0),
                selection_end: (0, 0),
                color,
                last_activity: chrono::Utc::now(),
            },
        );

        Ok(())
    }

    /// Remove a user from the sync session
    pub fn remove_user(&mut self, user_id: &str) {
        self.link_session.remove_user(user_id);
        self.user_selections.remove(user_id);
        // Release any locks held by this user
        self.cell_locks.retain(|_, lock| lock.locked_by != user_id);
    }

    /// Record a cell change
    pub fn record_cell_change(
        &mut self,
        worksheet: &str,
        row: usize,
        column: usize,
        cell_ref: &str,
        old_value: CellValue,
        new_value: CellValue,
    ) -> Result<CellChange, GridError> {
        if self.state == GridSyncState::Disconnected {
            return Err(GridError::Collaboration("Session is disconnected".to_string()));
        }

        // Create CRDT operation
        let content = serde_json::to_string(&new_value)
            .map_err(|e| GridError::Serialization(e.to_string()))?;

        let _crdt_op = self
            .crdt_engine
            .create_operation(
                self.local_user_id.clone(),
                CrdtOperationType::Replace,
                row * 10000 + column, // Encode position
                content,
            )
            .map_err(|e| GridError::Collaboration(e))?;

        let change_type = if matches!(new_value, CellValue::Empty) {
            CellChangeType::ValueCleared
        } else if matches!(new_value, CellValue::Formula(_)) {
            CellChangeType::FormulaSet
        } else {
            CellChangeType::ValueSet
        };

        let change = CellChange {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: self.local_user_id.clone(),
            worksheet: worksheet.to_string(),
            cell_ref: cell_ref.to_string(),
            row,
            column,
            old_value,
            new_value,
            timestamp: chrono::Utc::now(),
            change_type,
        };

        self.change_history.push(change.clone());
        Ok(change)
    }

    /// Update user's cell selection
    pub fn update_selection(
        &mut self,
        worksheet: &str,
        start: (usize, usize),
        end: (usize, usize),
    ) {
        if let Some(selection) = self.user_selections.get_mut(&self.local_user_id) {
            selection.worksheet = worksheet.to_string();
            selection.selection_start = start;
            selection.selection_end = end;
            selection.last_activity = chrono::Utc::now();
        }
    }

    /// Try to lock a cell for exclusive editing
    pub fn lock_cell(&mut self, cell_ref: &str) -> Result<(), GridError> {
        if let Some(existing_lock) = self.cell_locks.get(cell_ref) {
            if existing_lock.locked_by != self.local_user_id {
                if existing_lock.expires_at > chrono::Utc::now() {
                    return Err(GridError::Collaboration(format!(
                        "Cell {} is locked by user {}",
                        cell_ref, existing_lock.locked_by
                    )));
                }
            }
        }

        let lock = CellLock {
            cell_ref: cell_ref.to_string(),
            locked_by: self.local_user_id.clone(),
            locked_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::minutes(5),
        };

        self.cell_locks.insert(cell_ref.to_string(), lock);
        Ok(())
    }

    /// Unlock a cell
    pub fn unlock_cell(&mut self, cell_ref: &str) -> Result<(), GridError> {
        if let Some(lock) = self.cell_locks.get(cell_ref) {
            if lock.locked_by != self.local_user_id {
                return Err(GridError::Collaboration(
                    "Cannot unlock cell locked by another user".to_string(),
                ));
            }
        }
        self.cell_locks.remove(cell_ref);
        Ok(())
    }

    /// Check if a cell is locked
    pub fn is_cell_locked(&self, cell_ref: &str) -> bool {
        if let Some(lock) = self.cell_locks.get(cell_ref) {
            lock.expires_at > chrono::Utc::now()
        } else {
            false
        }
    }

    /// Go offline
    pub fn go_offline(&mut self) {
        self.state = GridSyncState::Offline;
    }

    /// Reconnect
    pub fn reconnect(&mut self) {
        self.state = GridSyncState::Connected;
    }

    /// Disconnect
    pub fn disconnect(&mut self) {
        self.state = GridSyncState::Disconnected;
        self.link_session.is_active = false;
    }

    /// Check if CRDT engine is enabled
    pub fn is_crdt_enabled(&self) -> bool {
        self.crdt_engine.is_enabled()
    }

    /// Check if sync manager is enabled
    pub fn is_sync_enabled(&self) -> bool {
        self.sync_manager.is_enabled()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        );
        assert!(session.is_ok());

        let session = session.unwrap();
        assert_eq!(session.workbook_id(), "workbook-001");
        assert_eq!(session.state(), GridSyncState::Connected);
        assert_eq!(session.user_count(), 1);
        assert!(session.is_crdt_enabled());
        assert!(session.is_sync_enabled());
    }

    #[test]
    fn test_add_remove_user() {
        let mut session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session
            .add_user("user-002".to_string(), "Bob".to_string(), "#EA4335".to_string())
            .unwrap();
        assert_eq!(session.user_count(), 2);

        session.remove_user("user-002");
        assert_eq!(session.user_count(), 1);
    }

    #[test]
    fn test_record_cell_change() {
        let mut session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        let change = session
            .record_cell_change(
                "Sheet1",
                0,
                0,
                "A1",
                CellValue::Empty,
                CellValue::Number(42.0),
            )
            .unwrap();

        assert_eq!(change.cell_ref, "A1");
        assert_eq!(change.change_type, CellChangeType::ValueSet);
        assert_eq!(session.change_history().len(), 1);
    }

    #[test]
    fn test_record_formula_change() {
        let mut session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        let change = session
            .record_cell_change(
                "Sheet1",
                0,
                2,
                "C1",
                CellValue::Empty,
                CellValue::Formula("=A1+B1".to_string()),
            )
            .unwrap();

        assert_eq!(change.change_type, CellChangeType::FormulaSet);
    }

    #[test]
    fn test_record_clear_change() {
        let mut session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        let change = session
            .record_cell_change(
                "Sheet1",
                0,
                0,
                "A1",
                CellValue::Number(42.0),
                CellValue::Empty,
            )
            .unwrap();

        assert_eq!(change.change_type, CellChangeType::ValueCleared);
    }

    #[test]
    fn test_update_selection() {
        let mut session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session.update_selection("Sheet2", (5, 3), (10, 7));

        let selection = session.user_selections().get("user-001").unwrap();
        assert_eq!(selection.worksheet, "Sheet2");
        assert_eq!(selection.selection_start, (5, 3));
        assert_eq!(selection.selection_end, (10, 7));
    }

    #[test]
    fn test_cell_locking() {
        let mut session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        assert!(!session.is_cell_locked("A1"));

        session.lock_cell("A1").unwrap();
        assert!(session.is_cell_locked("A1"));

        session.unlock_cell("A1").unwrap();
        assert!(!session.is_cell_locked("A1"));
    }

    #[test]
    fn test_cell_lock_conflict() {
        let mut session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session.lock_cell("A1").unwrap();

        // Simulate another user's lock by directly inserting
        session.cell_locks.insert(
            "B1".to_string(),
            CellLock {
                cell_ref: "B1".to_string(),
                locked_by: "user-002".to_string(),
                locked_at: chrono::Utc::now(),
                expires_at: chrono::Utc::now() + chrono::Duration::minutes(5),
            },
        );

        // Should fail because B1 is locked by another user
        assert!(session.lock_cell("B1").is_err());
    }

    #[test]
    fn test_session_lifecycle() {
        let mut session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        assert_eq!(session.state(), GridSyncState::Connected);

        session.go_offline();
        assert_eq!(session.state(), GridSyncState::Offline);

        session.reconnect();
        assert_eq!(session.state(), GridSyncState::Connected);

        session.disconnect();
        assert_eq!(session.state(), GridSyncState::Disconnected);
    }

    #[test]
    fn test_change_on_disconnected_session() {
        let mut session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session.disconnect();

        let result = session.record_cell_change(
            "Sheet1",
            0,
            0,
            "A1",
            CellValue::Empty,
            CellValue::Number(1.0),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_user_releases_locks() {
        let mut session = GridSyncSession::new(
            "workbook-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session
            .add_user("user-002".to_string(), "Bob".to_string(), "#EA4335".to_string())
            .unwrap();

        // Add a lock for user-002
        session.cell_locks.insert(
            "A1".to_string(),
            CellLock {
                cell_ref: "A1".to_string(),
                locked_by: "user-002".to_string(),
                locked_at: chrono::Utc::now(),
                expires_at: chrono::Utc::now() + chrono::Duration::minutes(5),
            },
        );

        assert!(session.is_cell_locked("A1"));

        // Remove user-002 should release their locks
        session.remove_user("user-002");
        assert!(!session.is_cell_locked("A1"));
    }

    #[test]
    fn test_cell_change_serialization() {
        let change = CellChange {
            id: "change-001".to_string(),
            user_id: "user-001".to_string(),
            worksheet: "Sheet1".to_string(),
            cell_ref: "A1".to_string(),
            row: 0,
            column: 0,
            old_value: CellValue::Empty,
            new_value: CellValue::Number(42.0),
            timestamp: chrono::Utc::now(),
            change_type: CellChangeType::ValueSet,
        };

        let json = serde_json::to_string(&change).unwrap();
        let deserialized: CellChange = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.cell_ref, "A1");
        assert_eq!(deserialized.change_type, CellChangeType::ValueSet);
    }
}