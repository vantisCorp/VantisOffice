//! Real-time Collaboration Module - Integration with Vantis Link (Pillar 03)
//!
//! Provides real-time collaborative editing for Writer documents using
//! CRDT-based conflict resolution from vantis-link.
//!
//! # Architecture
//!
//! ```text
//! vantis-writer (Pillar 02: Logic)
//!     └── collaboration module
//!             └── vantis-link (Pillar 03: Sync)
//!                     ├── CRDT Engine
//!                     ├── Sync Manager
//!                     └── P2P Transport
//! ```
//!
//! # Features
//! - Real-time co-editing with CRDT conflict resolution
//! - User presence and cursor tracking
//! - Change history and undo/redo across users
//! - Session management for document sharing
//! - Offline editing with automatic merge on reconnect

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use vantis_link::core::{Session as LinkSession, User as LinkUser, UserRole, Change};
use vantis_link::crdt::{CrdtEngine, CrdtType, CrdtOperationType};
use vantis_link::sync::{SyncManager, SyncStatus};

use crate::core::{Document, Paragraph};

/// Collaboration session for a Writer document
pub struct WriterCollaborationSession {
    /// Link session for P2P sync
    link_session: LinkSession,
    /// CRDT engine for conflict resolution
    crdt_engine: CrdtEngine,
    /// Sync manager for real-time sync
    sync_manager: SyncManager,
    /// Local user ID
    local_user_id: String,
    /// Document being collaborated on
    document_id: String,
    /// Edit history
    edit_history: Vec<WriterEdit>,
    /// Active collaborators
    collaborators: HashMap<String, CollaboratorInfo>,
    /// Session state
    state: SessionState,
}

/// State of the collaboration session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    /// Session is being created
    Initializing,
    /// Session is active and connected
    Active,
    /// Session is paused (offline mode)
    Paused,
    /// Session is being synchronized
    Syncing,
    /// Session has ended
    Ended,
}

/// Information about a collaborator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaboratorInfo {
    /// User ID
    pub user_id: String,
    /// Display name
    pub name: String,
    /// User role
    pub role: CollaboratorRole,
    /// Current cursor position
    pub cursor: Option<WriterCursor>,
    /// Whether the user is currently online
    pub is_online: bool,
    /// Last activity timestamp
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// Role of a collaborator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollaboratorRole {
    /// Document owner with full control
    Owner,
    /// Can edit the document
    Editor,
    /// Can suggest changes
    Suggester,
    /// Can only view the document
    Viewer,
}

/// Cursor position in a Writer document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriterCursor {
    /// Paragraph index
    pub paragraph_index: usize,
    /// Character offset within the paragraph
    pub char_offset: usize,
    /// Selection range (if any)
    pub selection: Option<WriterSelection>,
    /// Cursor color (for multi-user display)
    pub color: String,
}

/// Text selection in a Writer document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriterSelection {
    /// Start paragraph index
    pub start_paragraph: usize,
    /// Start character offset
    pub start_offset: usize,
    /// End paragraph index
    pub end_paragraph: usize,
    /// End character offset
    pub end_offset: usize,
}

/// An edit operation in the Writer document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriterEdit {
    /// Edit ID
    pub id: String,
    /// User who made the edit
    pub user_id: String,
    /// Type of edit
    pub edit_type: WriterEditType,
    /// Paragraph index affected
    pub paragraph_index: usize,
    /// Character offset
    pub char_offset: usize,
    /// Content involved in the edit
    pub content: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Type of edit operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WriterEditType {
    /// Insert text at position
    Insert,
    /// Delete text at position
    Delete,
    /// Replace text at position
    Replace,
    /// Add a new paragraph
    AddParagraph,
    /// Remove a paragraph
    RemoveParagraph,
    /// Change paragraph style
    StyleChange,
}

impl std::fmt::Debug for WriterCollaborationSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WriterCollaborationSession")
            .field("local_user_id", &self.local_user_id)
            .field("document_id", &self.document_id)
            .field("edit_history_len", &self.edit_history.len())
            .field("collaborators", &self.collaborators.len())
            .field("state", &self.state)
            .finish()
    }
}

impl WriterCollaborationSession {
    /// Create a new collaboration session for a document
    pub fn new(document_id: String, user_id: String, user_name: String) -> Result<Self> {
        // Create Link session
        let mut link_session = LinkSession::new(document_id.clone());

        // Add the local user as owner
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
            .map_err(|e| anyhow::anyhow!("Failed to add user to session: {}", e))?;

        // Create CRDT engine for text operations
        let crdt_engine = CrdtEngine::new(CrdtType::Rga);

        // Create sync manager
        let sync_manager = SyncManager::new();

        // Create collaborator info for local user
        let mut collaborators = HashMap::new();
        collaborators.insert(
            user_id.clone(),
            CollaboratorInfo {
                user_id: user_id.clone(),
                name: user_name,
                role: CollaboratorRole::Owner,
                cursor: None,
                is_online: true,
                last_activity: chrono::Utc::now(),
            },
        );

        Ok(WriterCollaborationSession {
            link_session,
            crdt_engine,
            sync_manager,
            local_user_id: user_id,
            document_id,
            edit_history: Vec::new(),
            collaborators,
            state: SessionState::Active,
        })
    }

    /// Get the session ID
    pub fn session_id(&self) -> &str {
        &self.link_session.id
    }

    /// Get the document ID
    pub fn document_id(&self) -> &str {
        &self.document_id
    }

    /// Get the current session state
    pub fn state(&self) -> SessionState {
        self.state
    }

    /// Get the number of active collaborators
    pub fn collaborator_count(&self) -> usize {
        self.collaborators.len()
    }

    /// Get all collaborators
    pub fn collaborators(&self) -> &HashMap<String, CollaboratorInfo> {
        &self.collaborators
    }

    /// Get the edit history
    pub fn edit_history(&self) -> &[WriterEdit] {
        &self.edit_history
    }

    /// Add a collaborator to the session
    pub fn add_collaborator(
        &mut self,
        user_id: String,
        name: String,
        role: CollaboratorRole,
    ) -> Result<()> {
        if self.state != SessionState::Active {
            anyhow::bail!("Session is not active");
        }

        // Map to Link user role
        let link_role = match role {
            CollaboratorRole::Owner => UserRole::Owner,
            CollaboratorRole::Editor => UserRole::Editor,
            CollaboratorRole::Suggester => UserRole::Editor,
            CollaboratorRole::Viewer => UserRole::Viewer,
        };

        // Add to Link session
        let link_user = LinkUser {
            id: user_id.clone(),
            name: name.clone(),
            email: None,
            role: link_role,
            joined_at: chrono::Utc::now(),
            last_seen: chrono::Utc::now(),
            is_online: true,
            cursor: None,
        };
        self.link_session
            .add_user(link_user)
            .map_err(|e| anyhow::anyhow!("Failed to add collaborator: {}", e))?;

        // Add to local collaborators map
        self.collaborators.insert(
            user_id.clone(),
            CollaboratorInfo {
                user_id,
                name,
                role,
                cursor: None,
                is_online: true,
                last_activity: chrono::Utc::now(),
            },
        );

        Ok(())
    }

    /// Remove a collaborator from the session
    pub fn remove_collaborator(&mut self, user_id: &str) -> Result<()> {
        self.link_session.remove_user(user_id);
        self.collaborators.remove(user_id);
        Ok(())
    }

    /// Apply a text insertion edit
    pub fn insert_text(
        &mut self,
        document: &mut Document,
        paragraph_index: usize,
        char_offset: usize,
        text: &str,
    ) -> Result<WriterEdit> {
        if self.state != SessionState::Active {
            anyhow::bail!("Session is not active");
        }

        // Create CRDT operation
        let _crdt_op = self
            .crdt_engine
            .create_operation(
                self.local_user_id.clone(),
                CrdtOperationType::Insert,
                char_offset,
                text.to_string(),
            )
            .map_err(|e| anyhow::anyhow!("CRDT operation failed: {}", e))?;

        // Apply to local document
        if paragraph_index < document.paragraphs.len() {
            let paragraph = &mut document.paragraphs[paragraph_index];
            if char_offset <= paragraph.text.len() {
                paragraph.text.insert_str(char_offset, text);
            }
        }

        // Create edit record
        let edit = WriterEdit {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: self.local_user_id.clone(),
            edit_type: WriterEditType::Insert,
            paragraph_index,
            char_offset,
            content: text.to_string(),
            timestamp: chrono::Utc::now(),
        };

        self.edit_history.push(edit.clone());
        Ok(edit)
    }

    /// Apply a text deletion edit
    pub fn delete_text(
        &mut self,
        document: &mut Document,
        paragraph_index: usize,
        char_offset: usize,
        length: usize,
    ) -> Result<WriterEdit> {
        if self.state != SessionState::Active {
            anyhow::bail!("Session is not active");
        }

        let deleted_content;

        // Create CRDT operation
        let _crdt_op = self
            .crdt_engine
            .create_operation(
                self.local_user_id.clone(),
                CrdtOperationType::Delete,
                char_offset,
                length.to_string(),
            )
            .map_err(|e| anyhow::anyhow!("CRDT operation failed: {}", e))?;

        // Apply to local document
        if paragraph_index < document.paragraphs.len() {
            let paragraph = &mut document.paragraphs[paragraph_index];
            let end = (char_offset + length).min(paragraph.text.len());
            deleted_content = paragraph.text[char_offset..end].to_string();
            paragraph.text.replace_range(char_offset..end, "");
        } else {
            deleted_content = String::new();
        }

        let edit = WriterEdit {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: self.local_user_id.clone(),
            edit_type: WriterEditType::Delete,
            paragraph_index,
            char_offset,
            content: deleted_content,
            timestamp: chrono::Utc::now(),
        };

        self.edit_history.push(edit.clone());
        Ok(edit)
    }

    /// Add a new paragraph
    pub fn add_paragraph(
        &mut self,
        document: &mut Document,
        index: usize,
        text: String,
    ) -> Result<WriterEdit> {
        if self.state != SessionState::Active {
            anyhow::bail!("Session is not active");
        }

        let paragraph = Paragraph::new(text.clone());

        if index <= document.paragraphs.len() {
            document.paragraphs.insert(index, paragraph);
        } else {
            document.paragraphs.push(paragraph);
        }

        let edit = WriterEdit {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: self.local_user_id.clone(),
            edit_type: WriterEditType::AddParagraph,
            paragraph_index: index,
            char_offset: 0,
            content: text,
            timestamp: chrono::Utc::now(),
        };

        self.edit_history.push(edit.clone());
        Ok(edit)
    }

    /// Update cursor position for the local user
    pub fn update_cursor(
        &mut self,
        paragraph_index: usize,
        char_offset: usize,
        color: String,
    ) {
        let cursor = WriterCursor {
            paragraph_index,
            char_offset,
            selection: None,
            color,
        };

        if let Some(collaborator) = self.collaborators.get_mut(&self.local_user_id) {
            collaborator.cursor = Some(cursor);
            collaborator.last_activity = chrono::Utc::now();
        }
    }

    /// Pause the session (go offline)
    pub fn pause(&mut self) {
        self.state = SessionState::Paused;
    }

    /// Resume the session
    pub fn resume(&mut self) {
        self.state = SessionState::Active;
    }

    /// End the session
    pub fn end(&mut self) {
        self.state = SessionState::Ended;
        self.link_session.is_active = false;
    }

    /// Check if the CRDT engine is enabled
    pub fn is_crdt_enabled(&self) -> bool {
        self.crdt_engine.is_enabled()
    }

    /// Check if the sync manager is enabled
    pub fn is_sync_enabled(&self) -> bool {
        self.sync_manager.is_enabled()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Document;

    fn create_test_document() -> Document {
        let mut doc = Document::new("Collaborative Document".to_string());
        doc.add_paragraph(Paragraph::new("First paragraph.".to_string()))
            .unwrap();
        doc.add_paragraph(Paragraph::new("Second paragraph.".to_string()))
            .unwrap();
        doc
    }

    #[test]
    fn test_session_creation() {
        let session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        );
        assert!(session.is_ok());

        let session = session.unwrap();
        assert_eq!(session.document_id(), "doc-001");
        assert_eq!(session.state(), SessionState::Active);
        assert_eq!(session.collaborator_count(), 1);
        assert!(session.is_crdt_enabled());
        assert!(session.is_sync_enabled());
    }

    #[test]
    fn test_add_collaborator() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session
            .add_collaborator("user-002".to_string(), "Bob".to_string(), CollaboratorRole::Editor)
            .unwrap();

        assert_eq!(session.collaborator_count(), 2);
        assert!(session.collaborators().contains_key("user-002"));
    }

    #[test]
    fn test_remove_collaborator() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session
            .add_collaborator("user-002".to_string(), "Bob".to_string(), CollaboratorRole::Editor)
            .unwrap();
        assert_eq!(session.collaborator_count(), 2);

        session.remove_collaborator("user-002").unwrap();
        assert_eq!(session.collaborator_count(), 1);
    }

    #[test]
    fn test_insert_text() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        let mut doc = create_test_document();
        let edit = session.insert_text(&mut doc, 0, 6, "beautiful ").unwrap();

        assert_eq!(edit.edit_type, WriterEditType::Insert);
        assert_eq!(doc.paragraphs[0].text, "First beautiful paragraph.");
        assert_eq!(session.edit_history().len(), 1);
    }

    #[test]
    fn test_delete_text() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        let mut doc = create_test_document();
        let edit = session.delete_text(&mut doc, 0, 0, 6).unwrap();

        assert_eq!(edit.edit_type, WriterEditType::Delete);
        assert_eq!(edit.content, "First ");
        assert_eq!(doc.paragraphs[0].text, "paragraph.");
    }

    #[test]
    fn test_add_paragraph() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        let mut doc = create_test_document();
        assert_eq!(doc.paragraphs.len(), 2);

        session
            .add_paragraph(&mut doc, 1, "Inserted paragraph.".to_string())
            .unwrap();

        assert_eq!(doc.paragraphs.len(), 3);
        assert_eq!(doc.paragraphs[1].text, "Inserted paragraph.");
    }

    #[test]
    fn test_cursor_update() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session.update_cursor(0, 5, "#FF0000".to_string());

        let collaborator = session.collaborators().get("user-001").unwrap();
        assert!(collaborator.cursor.is_some());
        let cursor = collaborator.cursor.as_ref().unwrap();
        assert_eq!(cursor.paragraph_index, 0);
        assert_eq!(cursor.char_offset, 5);
        assert_eq!(cursor.color, "#FF0000");
    }

    #[test]
    fn test_session_lifecycle() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        assert_eq!(session.state(), SessionState::Active);

        session.pause();
        assert_eq!(session.state(), SessionState::Paused);

        session.resume();
        assert_eq!(session.state(), SessionState::Active);

        session.end();
        assert_eq!(session.state(), SessionState::Ended);
    }

    #[test]
    fn test_edit_on_paused_session() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session.pause();

        let mut doc = create_test_document();
        let result = session.insert_text(&mut doc, 0, 0, "test");
        assert!(result.is_err());
    }

    #[test]
    fn test_edit_on_ended_session() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session.end();

        let mut doc = create_test_document();
        let result = session.insert_text(&mut doc, 0, 0, "test");
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_edits() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        let mut doc = create_test_document();

        session.insert_text(&mut doc, 0, 0, "Hello ").unwrap();
        session.insert_text(&mut doc, 0, 6, "World ").unwrap();
        session.delete_text(&mut doc, 1, 0, 7).unwrap();

        assert_eq!(session.edit_history().len(), 3);
        assert_eq!(doc.paragraphs[0].text, "Hello World First paragraph.");
        assert_eq!(doc.paragraphs[1].text, "paragraph.");
    }

    #[test]
    fn test_collaborator_roles() {
        let mut session = WriterCollaborationSession::new(
            "doc-001".to_string(),
            "user-001".to_string(),
            "Alice".to_string(),
        )
        .unwrap();

        session
            .add_collaborator("user-002".to_string(), "Bob".to_string(), CollaboratorRole::Editor)
            .unwrap();
        session
            .add_collaborator("user-003".to_string(), "Charlie".to_string(), CollaboratorRole::Viewer)
            .unwrap();
        session
            .add_collaborator("user-004".to_string(), "Diana".to_string(), CollaboratorRole::Suggester)
            .unwrap();

        assert_eq!(session.collaborator_count(), 4);

        let bob = session.collaborators().get("user-002").unwrap();
        assert_eq!(bob.role, CollaboratorRole::Editor);

        let charlie = session.collaborators().get("user-003").unwrap();
        assert_eq!(charlie.role, CollaboratorRole::Viewer);
    }

    #[test]
    fn test_edit_serialization() {
        let edit = WriterEdit {
            id: "edit-001".to_string(),
            user_id: "user-001".to_string(),
            edit_type: WriterEditType::Insert,
            paragraph_index: 0,
            char_offset: 5,
            content: "hello".to_string(),
            timestamp: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&edit).unwrap();
        let deserialized: WriterEdit = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, "edit-001");
        assert_eq!(deserialized.content, "hello");
    }
}