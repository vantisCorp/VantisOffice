//! Sync manager for document synchronization

use crate::core::{Document, DocumentFormat};
use crate::error::{MobileError, MobileResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Sync configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Auto-sync enabled
    pub auto_sync: bool,
    
    /// Sync interval in seconds
    pub sync_interval: u64,
    
    /// Maximum retry attempts
    pub max_retries: u32,
    
    /// Conflict resolution strategy
    pub conflict_resolution: ConflictResolution,
    
    /// Sync direction
    pub sync_direction: SyncDirection,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            auto_sync: true,
            sync_interval: 60,
            max_retries: 3,
            conflict_resolution: ConflictResolution::LastWriteWins,
            sync_direction: SyncDirection::Bidirectional,
        }
    }
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    LastWriteWins,
    FirstWriteWins,
    Manual,
    Merge,
}

/// Sync direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncDirection {
    UploadOnly,
    DownloadOnly,
    Bidirectional,
}

/// Sync status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Error(String),
    Completed,
}

/// Document sync information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSync {
    /// Document ID
    pub document_id: String,
    
    /// Local version
    pub local_version: u32,
    
    /// Remote version
    pub remote_version: u32,
    
    /// Last sync timestamp
    pub last_sync: chrono::DateTime<chrono::Utc>,
    
    /// Sync status
    pub status: SyncStatus,
    
    /// Needs sync
    pub needs_sync: bool,
}

/// Change set for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeSet {
    /// Change set ID
    pub change_id: String,
    
    /// Document ID
    pub document_id: String,
    
    /// Changes
    pub changes: Vec<Change>,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Author
    pub author: String,
}

/// Individual change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    /// Change type
    pub change_type: ChangeType,
    
    /// Position
    pub position: usize,
    
    /// Content
    pub content: String,
    
    /// Length
    pub length: usize,
}

/// Change type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Insert,
    Delete,
    Replace,
}

/// Sync manager for document synchronization
pub struct SyncManager {
    config: SyncConfig,
    status: Arc<RwLock<SyncStatus>>,
    documents: Arc<RwLock<HashMap<String, DocumentSync>>>,
    change_sets: Arc<RwLock<Vec<ChangeSet>>>,
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new(config: SyncConfig) -> Self {
        Self {
            config,
            status: Arc::new(RwLock::new(SyncStatus::Idle)),
            documents: Arc::new(RwLock::new(HashMap::new())),
            change_sets: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Get current sync status
    pub async fn status(&self) -> SyncStatus {
        self.status.read().await.clone()
    }
    
    /// Start sync
    pub async fn start_sync(&self) -> MobileResult<()> {
        *self.status.write().await = SyncStatus::Syncing;
        
        // In a real implementation, this would:
        // 1. Fetch remote document list
        // 2. Compare with local documents
        // 3. Upload changes
        // 4. Download changes
        // 5. Resolve conflicts
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        *self.status.write().await = SyncStatus::Completed;
        
        Ok(())
    }
    
    /// Stop sync
    pub async fn stop_sync(&self) -> MobileResult<()> {
        *self.status.write().await = SyncStatus::Idle;
        Ok(())
    }
    
    /// Register document for sync
    pub async fn register_document(&self, document_id: String, version: u32) -> MobileResult<()> {
        let mut docs = self.documents.write().await;
        docs.insert(
            document_id.clone(),
            DocumentSync {
                document_id,
                local_version: version,
                remote_version: version,
                last_sync: chrono::Utc::now(),
                status: SyncStatus::Idle,
                needs_sync: false,
            },
        );
        Ok(())
    }
    
    /// Unregister document from sync
    pub async fn unregister_document(&self, document_id: &str) -> MobileResult<()> {
        let mut docs = self.documents.write().await;
        docs.remove(document_id);
        Ok(())
    }
    
    /// Get document sync info
    pub async fn get_document_sync(&self, document_id: &str) -> Option<DocumentSync> {
        let docs = self.documents.read().await;
        docs.get(document_id).cloned()
    }
    
    /// Get all document sync info
    pub async fn get_all_documents(&self) -> Vec<DocumentSync> {
        let docs = self.documents.read().await;
        docs.values().cloned().collect()
    }
    
    /// Create change set
    pub async fn create_change_set(
        &self,
        document_id: String,
        changes: Vec<Change>,
        author: String,
    ) -> MobileResult<String> {
        let change_set = ChangeSet {
            change_id: Uuid::new_v4().to_string(),
            document_id,
            changes,
            timestamp: chrono::Utc::now(),
            author,
        };
        
        let change_id = change_set.change_id.clone();
        self.change_sets.write().await.push(change_set);
        
        Ok(change_id)
    }
    
    /// Get change set
    pub async fn get_change_set(&self, change_id: &str) -> Option<ChangeSet> {
        let sets = self.change_sets.read().await;
        sets.iter().find(|cs| cs.change_id == change_id).cloned()
    }
    
    /// Apply change set to document
    pub async fn apply_change_set(&self, document: &mut Document, change_set: &ChangeSet) -> MobileResult<()> {
        for change in &change_set.changes {
            match change.change_type {
                ChangeType::Insert => {
                    let content_bytes = change.content.as_bytes();
                    document.content.splice(
                        change.position..change.position,
                        content_bytes.iter().cloned(),
                    );
                }
                ChangeType::Delete => {
                    document.content.drain(change.position..change.position + change.length);
                }
                ChangeType::Replace => {
                    let content_bytes = change.content.as_bytes();
                    document.content.splice(
                        change.position..change.position + change.length,
                        content_bytes.iter().cloned(),
                    );
                }
            }
        }
        
        document.last_modified = chrono::Utc::now();
        document.version += 1;
        
        Ok(())
    }
    
    /// Sync document
    pub async fn sync_document(&self, document_id: &str) -> MobileResult<()> {
        let mut docs = self.documents.write().await;
        
        if let Some(doc_sync) = docs.get_mut(document_id) {
            doc_sync.status = SyncStatus::Syncing;
            doc_sync.last_sync = chrono::Utc::now();
            
            // Simulate sync
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            
            doc_sync.remote_version = doc_sync.local_version;
            doc_sync.status = SyncStatus::Completed;
            doc_sync.needs_sync = false;
        }
        
        Ok(())
    }
    
    /// Mark document as needing sync
    pub async fn mark_needs_sync(&self, _document_id: &str) -> MobileResult<()> {
        let mut docs = self.documents.write().await;
        
        if let Some(doc_sync) = docs.get_mut(_document_id) {
            doc_sync.needs_sync = true;
        }
        
        Ok(())
    }
    
    /// Resolve conflict
    pub async fn resolve_conflict(
        &self,
        document_id: &str,
        local_doc: &Document,
        remote_doc: &Document,
    ) -> MobileResult<Document> {
        match self.config.conflict_resolution {
            ConflictResolution::LastWriteWins => {
                if local_doc.last_modified > remote_doc.last_modified {
                    Ok(local_doc.clone())
                } else {
                    Ok(remote_doc.clone())
                }
            }
            ConflictResolution::FirstWriteWins => {
                if local_doc.last_modified < remote_doc.last_modified {
                    Ok(local_doc.clone())
                } else {
                    Ok(remote_doc.clone())
                }
            }
            ConflictResolution::Manual => {
                Err(MobileError::Sync("Manual conflict resolution required".to_string()))
            }
            ConflictResolution::Merge => {
                // Simple merge: concatenate content
                let mut merged = local_doc.clone();
                merged.content.extend_from_slice(&remote_doc.content);
                merged.version = local_doc.version.max(remote_doc.version) + 1;
                Ok(merged)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_config_default() {
        let config = SyncConfig::default();
        assert!(config.auto_sync);
        assert_eq!(config.sync_interval, 60);
    }

    #[tokio::test]
    async fn test_sync_manager_creation() {
        let config = SyncConfig::default();
        let manager = SyncManager::new(config);
        assert_eq!(manager.status().await, SyncStatus::Idle);
    }

    #[tokio::test]
    async fn test_register_document() {
        let config = SyncConfig::default();
        let manager = SyncManager::new(config);
        
        manager
            .register_document("doc123".to_string(), 1)
            .await
            .unwrap();
        
        let doc_sync = manager.get_document_sync("doc123").await;
        assert!(doc_sync.is_some());
        assert_eq!(doc_sync.unwrap().local_version, 1);
    }

    #[tokio::test]
    async fn test_change_set_creation() {
        let config = SyncConfig::default();
        let manager = SyncManager::new(config);
        
        let changes = vec![Change {
            change_type: ChangeType::Insert,
            position: 0,
            content: "Hello".to_string(),
            length: 5,
        }];
        
        let change_id = manager
            .create_change_set("doc123".to_string(), changes, "user1".to_string())
            .await
            .unwrap();
        
        let change_set = manager.get_change_set(&change_id).await;
        assert!(change_set.is_some());
    }

    #[tokio::test]
    async fn test_apply_change_set() {
        let config = SyncConfig::default();
        let manager = SyncManager::new(config);
        
        let mut doc = Document::new(
            "Test".to_string(),
            DocumentFormat::VDoc,
            b"Hello World".to_vec(),
            "Author".to_string(),
        );
        
        let changes = vec![Change {
            change_type: ChangeType::Insert,
            position: 5,
            content: " Beautiful".to_string(),
            length: 10,
        }];
        
        let change_set = ChangeSet {
            change_id: Uuid::new_v4().to_string(),
            document_id: doc.document_id.clone(),
            changes,
            timestamp: chrono::Utc::now(),
            author: "user1".to_string(),
        };
        
        manager.apply_change_set(&mut doc, &change_set).await.unwrap();
        
        let content = String::from_utf8(doc.content).unwrap();
        assert_eq!(content, "Hello Beautiful World");
    }
}