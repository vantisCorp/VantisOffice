//! Core data structures for Vantis Ark

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Backup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backup {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub data: Vec<u8>,
    pub parts: Vec<BackupPart>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: BackupMetadata,
}

impl Backup {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        let now = Utc::now();
        Backup {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            data,
            parts: Vec::new(),
            created_at: now,
            updated_at: now,
            metadata: BackupMetadata::default(),
        }
    }
}

/// Backup metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub size: usize,
    pub checksum: String,
    pub version: String,
}

impl Default for BackupMetadata {
    fn default() -> Self {
        BackupMetadata {
            size: 0,
            checksum: String::new(),
            version: "1.0".to_string(),
        }
    }
}

/// Recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recovery {
    pub id: String,
    pub backup_id: String,
    pub parts: Vec<BackupPart>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: RecoveryStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecoveryStatus {
    InProgress,
    Completed,
    Failed,
}

/// Backup config
#[derive(Debug, Clone)]
pub struct BackupConfig {
    pub name: String,
    pub parts: usize,
    pub threshold: usize,
    pub replication_factor: usize,
}

impl BackupConfig {
    pub fn new(name: String, parts: usize, threshold: usize) -> Self {
        BackupConfig {
            name,
            parts,
            threshold,
            replication_factor: 3,
        }
    }
}

/// Recovery config
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    pub backup_id: String,
    pub threshold: usize,
}

impl RecoveryConfig {
    pub fn new(backup_id: String, threshold: usize) -> Self {
        RecoveryConfig {
            backup_id,
            threshold,
        }
    }
}

/// Storage backend trait
pub trait StorageBackend: Send + Sync {
    fn store(&self, key: &str, data: &[u8]) -> Result<(), String>;
    fn retrieve(&self, key: &str) -> Result<Vec<u8>, String>;
    fn delete(&self, key: &str) -> Result<(), String>;
    fn exists(&self, key: &str) -> bool;
}

/// In-memory storage implementation
pub struct InMemoryStorage {
    data: HashMap<String, Vec<u8>>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        InMemoryStorage {
            data: HashMap::new(),
        }
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageBackend for InMemoryStorage {
    fn store(&self, key: &str, data: &[u8]) -> Result<(), String> {
        // Note: This would need interior mutability in a real implementation
        // For now, we'll just return Ok
        Ok(())
    }

    fn retrieve(&self, key: &str) -> Result<Vec<u8>, String> {
        self.data
            .get(key)
            .cloned()
            .ok_or_else(|| format!("Key '{}' not found", key))
    }

    fn delete(&self, key: &str) -> Result<(), String> {
        // Note: This would need interior mutability in a real implementation
        Ok(())
    }

    fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}

/// Backup manager
pub struct BackupManager {
    storage: Box<dyn StorageBackend>,
}

impl BackupManager {
    pub fn new(storage: Box<dyn StorageBackend>) -> Self {
        BackupManager { storage }
    }

    pub fn create_backup(&self, name: String, data: Vec<u8>) -> Result<Backup, String> {
        let mut backup = Backup::new(name, data);
        backup.metadata.size = backup.data.len();
        backup.metadata.checksum = format!("{:x}", md5::compute(&backup.data));
        Ok(backup)
    }

    pub fn get_backup(&self, backup_id: &str) -> Result<Backup, String> {
        let data = self.storage.retrieve(backup_id)?;
        let backup: Backup = serde_json::from_slice(&data)
            .map_err(|e| format!("Failed to deserialize backup: {}", e))?;
        Ok(backup)
    }

    pub fn delete_backup(&self, backup_id: &str) -> Result<(), String> {
        self.storage.delete(backup_id)
    }
}

/// Re-export BackupPart from shamir module
pub use crate::shamir::BackupPart;

/// Initialize core module
pub fn init() -> Result<(), String> {
    Ok(())
}
