//! Collaboration module for Vantis Flow
//!
//! Provides real-time collaboration with CRDT-based conflict resolution.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{FlowError, FlowResult};

/// Flow collaboration session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowSession {
    /// Unique identifier
    pub id: Uuid,

    /// Session name
    pub name: String,

    /// Canvas being collaborated on
    pub canvas_id: Uuid,

    /// Users in the session
    pub users: HashMap<Uuid, FlowUser>,

    /// Session owner
    pub owner: Uuid,

    /// CRDT for conflict resolution
    pub crdt: FlowCRDT,

    /// Session status
    pub status: SessionStatus,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last activity timestamp
    pub last_activity: DateTime<Utc>,
}

impl FlowSession {
    /// Create a new session
    pub fn new(name: impl Into<String>, canvas_id: Uuid, owner: Uuid) -> Self {
        let now = Utc::now();
        let mut users = HashMap::new();
        users.insert(owner, FlowUser::new(owner, "Owner".to_string()));

        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            canvas_id,
            users,
            owner,
            crdt: FlowCRDT::new(),
            status: SessionStatus::Active,
            created_at: now,
            last_activity: now,
        }
    }

    /// Add a user to the session
    pub fn add_user(&mut self, user: FlowUser) -> FlowResult<()> {
        self.users.insert(user.id, user);
        self.last_activity = Utc::now();
        Ok(())
    }

    /// Remove a user from the session
    pub fn remove_user(&mut self, user_id: Uuid) -> FlowResult<()> {
        self.users.remove(&user_id);
        self.last_activity = Utc::now();
        Ok(())
    }

    /// Apply a change to the canvas
    pub fn apply_change(&mut self, change: FlowChange) -> FlowResult<()> {
        // Apply change through CRDT
        self.crdt.apply_change(change.clone())?;

        self.last_activity = Utc::now();
        Ok(())
    }

    /// Get all pending changes
    pub fn get_pending_changes(&self) -> Vec<FlowChange> {
        self.crdt.get_pending_changes()
    }
}

/// Session status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionStatus {
    /// Active session
    Active,

    /// Paused session
    Paused,

    /// Ended session
    Ended,
}

/// Flow user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowUser {
    /// Unique identifier
    pub id: Uuid,

    /// User name
    pub name: String,

    /// User color (for cursor highlighting)
    pub color: String,

    /// Cursor position
    pub cursor: Option<Cursor>,

    /// User permissions
    pub permissions: UserPermissions,

    /// Joined timestamp
    pub joined_at: DateTime<Utc>,
}

impl FlowUser {
    /// Create a new user
    pub fn new(id: Uuid, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            color: "#000000".to_string(),
            cursor: None,
            permissions: UserPermissions::default(),
            joined_at: Utc::now(),
        }
    }

    /// Set cursor position
    pub fn set_cursor(&mut self, x: f64, y: f64) {
        self.cursor = Some(Cursor { x, y });
    }
}

/// Cursor position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    /// X coordinate
    pub x: f64,

    /// Y coordinate
    pub y: f64,
}

/// User permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissions {
    /// Can edit canvas
    pub can_edit: bool,

    /// Can add elements
    pub can_add_elements: bool,

    /// Can remove elements
    pub can_remove_elements: bool,

    /// Can add connections
    pub can_add_connections: bool,

    /// Can remove connections
    pub can_remove_connections: bool,

    /// Can change canvas settings
    pub can_change_settings: bool,

    /// Can invite users
    pub can_invite_users: bool,

    /// Can kick users
    pub can_kick_users: bool,
}

impl Default for UserPermissions {
    fn default() -> Self {
        Self {
            can_edit: true,
            can_add_elements: true,
            can_remove_elements: true,
            can_add_connections: true,
            can_remove_connections: true,
            can_change_settings: false,
            can_invite_users: false,
            can_kick_users: false,
        }
    }
}

/// Flow change (CRDT operation)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowChange {
    /// Unique identifier
    pub id: Uuid,

    /// User who made the change
    pub user_id: Uuid,

    /// Change type
    pub change_type: ChangeType,

    /// Change data
    pub data: serde_json::Value,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Logical timestamp (for ordering)
    pub logical_timestamp: u64,

    /// Vector clock (for distributed ordering)
    pub vector_clock: HashMap<Uuid, u64>,
}

impl FlowChange {
    /// Create a new change
    pub fn new(user_id: Uuid, change_type: ChangeType, data: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            change_type,
            data,
            timestamp: Utc::now(),
            logical_timestamp: 0,
            vector_clock: HashMap::new(),
        }
    }
}

/// Change types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    /// Add element
    AddElement,

    /// Remove element
    RemoveElement,

    /// Update element
    UpdateElement,

    /// Add connection
    AddConnection,

    /// Remove connection
    RemoveConnection,

    /// Update connection
    UpdateConnection,

    /// Move element
    MoveElement,

    /// Resize element
    ResizeElement,

    /// Change canvas settings
    ChangeCanvasSettings,
}

/// Flow CRDT for conflict resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowCRDT {
    /// Vector clock for ordering
    pub vector_clock: HashMap<Uuid, u64>,

    /// Pending changes
    pub pending_changes: Vec<FlowChange>,

    /// Applied changes
    pub applied_changes: Vec<FlowChange>,

    /// Conflict resolver
    pub resolver: ConflictResolver,
}

impl FlowCRDT {
    /// Create a new CRDT
    pub fn new() -> Self {
        Self {
            vector_clock: HashMap::new(),
            pending_changes: Vec::new(),
            applied_changes: Vec::new(),
            resolver: ConflictResolver::LastWriteWins,
        }
    }

    /// Apply a change
    pub fn apply_change(&mut self, change: FlowChange) -> FlowResult<()> {
        // Update vector clock
        let user_id = change.user_id;
        let current = *self.vector_clock.get(&user_id).unwrap_or(&0);
        self.vector_clock.insert(user_id, current + 1);

        // Check for conflicts
        let conflicts = self.detect_conflicts(&change);

        if conflicts.is_empty() {
            // No conflicts, apply change
            self.applied_changes.push(change);
        } else {
            // Resolve conflicts
            let resolved = self.resolver.resolve(conflicts, change)?;
            self.applied_changes.push(resolved);
        }

        Ok(())
    }

    /// Detect conflicts
    fn detect_conflicts(&self, change: &FlowChange) -> Vec<FlowChange> {
        let mut conflicts = Vec::new();

        for applied in &self.applied_changes {
            // Check if changes affect the same element
            if self.changes_conflict(applied, change) {
                conflicts.push(applied.clone());
            }
        }

        conflicts
    }

    /// Check if two changes conflict
    fn changes_conflict(&self, change1: &FlowChange, change2: &FlowChange) -> bool {
        // Check if they affect the same element
        if let (Some(id1), Some(id2)) = (
            change1.data.get("element_id").and_then(|v| v.as_str()),
            change2.data.get("element_id").and_then(|v| v.as_str()),
        ) {
            return id1 == id2;
        }

        // Check if they affect the same connection
        if let (Some(id1), Some(id2)) = (
            change1.data.get("connection_id").and_then(|v| v.as_str()),
            change2.data.get("connection_id").and_then(|v| v.as_str()),
        ) {
            return id1 == id2;
        }

        false
    }

    /// Get pending changes
    pub fn get_pending_changes(&self) -> Vec<FlowChange> {
        self.pending_changes.clone()
    }

    /// Merge with another CRDT
    pub fn merge(&mut self, other: FlowCRDT) -> FlowResult<()> {
        // Merge vector clocks
        for (user_id, timestamp) in other.vector_clock {
            let current = *self.vector_clock.get(&user_id).unwrap_or(&0);
            self.vector_clock.insert(user_id, current.max(timestamp));
        }

        // Apply pending changes from other
        for change in other.pending_changes {
            self.apply_change(change)?;
        }

        Ok(())
    }
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolver {
    /// Last write wins (based on timestamp)
    LastWriteWins,

    /// First write wins
    FirstWriteWins,

    /// Merge changes
    Merge,

    /// Manual resolution required
    Manual,
}

impl ConflictResolver {
    /// Resolve conflicts
    pub fn resolve(
        &self,
        conflicts: Vec<FlowChange>,
        new_change: FlowChange,
    ) -> FlowResult<FlowChange> {
        match self {
            ConflictResolver::LastWriteWins => {
                // Use the change with the latest timestamp
                let mut all_changes = conflicts;
                all_changes.push(new_change);

                let latest = all_changes
                    .into_iter()
                    .max_by_key(|c| c.timestamp)
                    .ok_or_else(|| {
                        FlowError::CollaborationError("No changes to resolve".to_string())
                    })?;

                Ok(latest)
            }

            ConflictResolver::FirstWriteWins => {
                // Use the change with the earliest timestamp
                let mut all_changes = conflicts;
                all_changes.push(new_change);

                let earliest = all_changes
                    .into_iter()
                    .min_by_key(|c| c.timestamp)
                    .ok_or_else(|| {
                        FlowError::CollaborationError("No changes to resolve".to_string())
                    })?;

                Ok(earliest)
            }

            ConflictResolver::Merge => {
                // Merge changes (simplified implementation)
                // In a real implementation, this would intelligently merge the changes
                Ok(new_change)
            }

            ConflictResolver::Manual => {
                // Manual resolution required
                Err(FlowError::CollaborationError(
                    "Manual conflict resolution required".to_string(),
                ))
            }
        }
    }
}

/// Real-time synchronization manager
pub struct SyncManager {
    /// Session
    pub session: FlowSession,

    /// Connected peers
    pub peers: HashMap<Uuid, Peer>,
}

/// Peer information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    /// Peer ID
    pub id: Uuid,

    /// Peer address
    pub address: String,

    /// Connection status
    pub status: PeerStatus,

    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
}

/// Peer status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PeerStatus {
    /// Connected
    Connected,

    /// Disconnected
    Disconnected,

    /// Connecting
    Connecting,
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new(session: FlowSession) -> Self {
        Self {
            session,
            peers: HashMap::new(),
        }
    }

    /// Connect to a peer
    pub fn connect_peer(&mut self, peer: Peer) -> FlowResult<()> {
        self.peers.insert(peer.id, peer);
        Ok(())
    }

    /// Disconnect from a peer
    pub fn disconnect_peer(&mut self, peer_id: Uuid) -> FlowResult<()> {
        self.peers.remove(&peer_id);
        Ok(())
    }

    /// Broadcast a change to all peers
    pub fn broadcast_change(&mut self, change: FlowChange) -> FlowResult<()> {
        // Apply change locally
        self.session.apply_change(change.clone())?;

        // In a real implementation, this would send the change to all connected peers
        // For now, we just mark it as applied

        Ok(())
    }

    /// Receive a change from a peer
    pub fn receive_change(&mut self, change: FlowChange) -> FlowResult<()> {
        // Apply change through CRDT
        self.session.apply_change(change)?;

        Ok(())
    }

    /// Sync with all peers
    pub fn sync(&mut self) -> FlowResult<()> {
        // In a real implementation, this would exchange state with all peers
        // For now, we just return success

        Ok(())
    }
}
