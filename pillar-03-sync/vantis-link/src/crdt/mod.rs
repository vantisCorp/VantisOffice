//! CRDT (Conflict-free Replicated Data Types) module
//! 
//! Provides conflict resolution for distributed collaboration

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

/// CRDT Engine
pub struct CrdtEngine {
    operations: Arc<RwLock<HashMap<String, CrdtOperation>>>,
    enabled: bool,
    crdt_type: CrdtType,
}

impl CrdtEngine {
    pub fn new(crdt_type: CrdtType) -> Self {
        CrdtEngine {
            operations: Arc::new(RwLock::new(HashMap::new())),
            enabled: true,
            crdt_type,
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
    
    /// Create a new CRDT operation
    pub fn create_operation(&self, user_id: String, operation_type: CrdtOperationType, position: usize, content: String) -> Result<CrdtOperation, String> {
        if !self.enabled {
            return Err("CRDT engine is disabled".to_string());
        }
        
        let operation = CrdtOperation::new(user_id, operation_type, position, content);
        
        let mut operations = self.operations.write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        
        operations.insert(operation.id.clone(), operation.clone());
        
        Ok(operation)
    }
    
    /// Apply an operation
    pub fn apply_operation(&self, document: &mut crate::core::Document, operation: &CrdtOperation) -> Result<(), String> {
        if !self.enabled {
            return Err("CRDT engine is disabled".to_string());
        }
        
        match operation.operation_type {
            CrdtOperationType::Insert => {
                self.apply_insert(document, operation)?;
            }
            CrdtOperationType::Delete => {
                self.apply_delete(document, operation)?;
            }
            CrdtOperationType::Replace => {
                self.apply_replace(document, operation)?;
            }
        }
        
        Ok(())
    }
    
    /// Apply insert operation
    fn apply_insert(&self, document: &mut crate::core::Document, operation: &CrdtOperation) -> Result<(), String> {
        // Resolve conflicts using CRDT
        let resolved_position = self.resolve_conflict(document, operation)?;
        
        // Apply insert
        if resolved_position <= document.content.len() {
            document.content.insert_str(resolved_position, &operation.content);
            document.version += 1;
            document.modified_at = chrono::Utc::now();
        }
        
        Ok(())
    }
    
    /// Apply delete operation
    fn apply_delete(&self, document: &mut crate::core::Document, operation: &CrdtOperation) -> Result<(), String> {
        let resolved_position = self.resolve_conflict(document, operation)?;
        let end = resolved_position + operation.length;
        
        if end <= document.content.len() {
            document.content.replace_range(resolved_position..end, "");
            document.version += 1;
            document.modified_at = chrono::Utc::now();
        }
        
        Ok(())
    }
    
    /// Apply replace operation
    fn apply_replace(&self, document: &mut crate::core::Document, operation: &CrdtOperation) -> Result<(), String> {
        let resolved_position = self.resolve_conflict(document, operation)?;
        let end = resolved_position + operation.length;
        
        if end <= document.content.len() {
            document.content.replace_range(resolved_position..end, &operation.content);
            document.version += 1;
            document.modified_at = chrono::Utc::now();
        }
        
        Ok(())
    }
    
    /// Resolve conflicts using CRDT
    fn resolve_conflict(&self, document: &crate::core::Document, operation: &CrdtOperation) -> Result<usize, String> {
        match self.crdt_type {
            CrdtType::Rga => self.resolve_rga(document, operation),
            CrdtType::Lww => self.resolve_lww(document, operation),
            CrdtType::Orset => self.resolve_orset(document, operation),
        }
    }
    
    /// Resolve using RGA (Replicated Growable Array)
    fn resolve_rga(&self, document: &crate::core::Document, operation: &CrdtOperation) -> Result<usize, String> {
        // RGA uses causal ordering to resolve conflicts
        // For simplicity, we'll use timestamp-based ordering
        
        let mut position = operation.position;
        
        // Check for concurrent operations
        for change in &document.changes {
            if change.timestamp > operation.timestamp {
                // This change happened after our operation, adjust position
                if change.position <= position {
                    position += change.content.len();
                }
            }
        }
        
        Ok(position)
    }
    
    /// Resolve using LWW (Last-Write-Wins)
    fn resolve_lww(&self, _document: &crate::core::Document, operation: &CrdtOperation) -> Result<usize, String> {
        // LWW simply uses the last write
        Ok(operation.position)
    }
    
    /// Resolve using OR-Set (Observed-Remove Set)
    fn resolve_orset(&self, document: &crate::core::Document, operation: &CrdtOperation) -> Result<usize, String> {
        // OR-Set tracks observed and removed elements
        // For simplicity, we'll use RGA resolution
        self.resolve_rga(document, operation)
    }
    
    /// Get operation by ID
    pub fn get_operation(&self, operation_id: &str) -> Option<CrdtOperation> {
        let operations = self.operations.read().ok()?;
        operations.get(operation_id).cloned()
    }
    
    /// Get all operations
    pub fn get_all_operations(&self) -> Vec<CrdtOperation> {
        let operations = self.operations.read().ok();
        match operations {
            Some(ops) => ops.values().cloned().collect(),
            None => Vec::new(),
        }
    }
}

/// CRDT Operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdtOperation {
    pub id: String,
    pub user_id: String,
    pub operation_type: CrdtOperationType,
    pub position: usize,
    pub length: usize,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub vector_clock: HashMap<String, u64>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrdtOperationType {
    Insert,
    Delete,
    Replace,
}

impl CrdtOperation {
    pub fn new(user_id: String, operation_type: CrdtOperationType, position: usize, content: String) -> Self {
        let now = chrono::Utc::now();
        let mut vector_clock = HashMap::new();
        vector_clock.insert(user_id.clone(), 1);
        
        CrdtOperation {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            operation_type,
            position,
            length: content.len(),
            content,
            timestamp: now,
            vector_clock,
            dependencies: Vec::new(),
        }
    }
    
    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }
}

/// CRDT Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrdtType {
    Rga,    // Replicated Growable Array
    Lww,    // Last-Write-Wins
    Orset,  // Observed-Remove Set
}

/// Initialize CRDT module
pub fn init() -> Result<(), String> {
    Ok(())
}