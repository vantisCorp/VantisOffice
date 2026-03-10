//! Document Encryption Module - Integration with Vantis Vault (Pillar 01)
//!
//! Provides transparent encryption/decryption for Writer documents using
//! post-quantum cryptography from vantis-vault and vantis-pqc.
//!
//! # Architecture
//!
//! ```text
//! vantis-writer (Pillar 02: Logic)
//!     └── encryption module
//!             └── vantis-vault (Pillar 01: Iron)
//!                     └── vantis-pqc (Post-Quantum Cryptography)
//! ```
//!
//! # Features
//! - Transparent document encryption/decryption
//! - Post-quantum key management via Vault
//! - Encrypted document serialization format
//! - Security level selection (Standard, High, Maximum)
//! - Document integrity verification with digital signatures

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use vantis_vault::api::{EncryptionProfile, KeySlot, Vault};
use vantis_vault::crypto::{
    decrypt_document as pqc_decrypt, encrypt_document as pqc_encrypt, PqcEncryptedDocument,
    PqcKeyBundle,
};

use crate::core::Document;

/// Security level for document encryption
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentSecurityLevel {
    /// Standard security - Kyber768 encryption
    Standard,
    /// High security - Kyber768 with Dilithium3 signatures
    High,
    /// Maximum security - Kyber1024 with Dilithium5 signatures
    Maximum,
}

impl Default for DocumentSecurityLevel {
    fn default() -> Self {
        DocumentSecurityLevel::High
    }
}

/// Encrypted document wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedDocument {
    /// Original document ID
    pub document_id: String,
    /// Original document title (unencrypted for indexing)
    pub title: String,
    /// Security level used for encryption
    pub security_level: DocumentSecurityLevel,
    /// PQC encrypted content
    pub encrypted_content: PqcEncryptedDocument,
    /// Encryption timestamp
    pub encrypted_at: chrono::DateTime<chrono::Utc>,
    /// Encryption metadata
    pub metadata: EncryptionMetadata,
}

/// Metadata about the encryption operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionMetadata {
    /// Algorithm used
    pub algorithm: String,
    /// Whether the document is signed
    pub is_signed: bool,
    /// Key bundle identifier
    pub key_bundle_id: Option<String>,
    /// Encryption version
    pub version: String,
}

/// Document encryption manager
///
/// Integrates vantis-vault encryption capabilities into the Writer module,
/// providing seamless document encryption and decryption.
pub struct DocumentEncryptionManager {
    /// Vault instance for key management
    vault: Vault,
    /// Default security level
    default_security_level: DocumentSecurityLevel,
    /// Key bundle for encryption operations
    key_bundle: Option<PqcKeyBundle>,
}

impl DocumentEncryptionManager {
    /// Create a new encryption manager with default settings
    pub fn new() -> Result<Self> {
        let vault = Vault::new().context("Failed to initialize Vault")?;
        Ok(DocumentEncryptionManager {
            vault,
            default_security_level: DocumentSecurityLevel::High,
            key_bundle: None,
        })
    }

    /// Create with a specific security level
    pub fn with_security_level(security_level: DocumentSecurityLevel) -> Result<Self> {
        let vault = Vault::new().context("Failed to initialize Vault")?;
        Ok(DocumentEncryptionManager {
            vault,
            default_security_level: security_level,
            key_bundle: None,
        })
    }

    /// Initialize key bundle for encryption operations
    pub fn initialize_keys(&mut self) -> Result<()> {
        let bundle = match self.default_security_level {
            DocumentSecurityLevel::Standard => PqcKeyBundle::new_kyber768()
                .context("Failed to create standard key bundle")?,
            DocumentSecurityLevel::High => PqcKeyBundle::new_with_signing()
                .context("Failed to create high-security key bundle")?,
            DocumentSecurityLevel::Maximum => PqcKeyBundle::new_high_security()
                .context("Failed to create maximum-security key bundle")?,
        };
        self.key_bundle = Some(bundle);
        Ok(())
    }

    /// Get the public key for sharing with collaborators
    pub fn public_key(&self) -> Result<Vec<u8>> {
        self.key_bundle
            .as_ref()
            .map(|kb| kb.public_key())
            .ok_or_else(|| anyhow::anyhow!("Keys not initialized. Call initialize_keys() first."))
    }

    /// Get the signing public key (if available)
    pub fn signing_public_key(&self) -> Option<Vec<u8>> {
        self.key_bundle
            .as_ref()
            .and_then(|kb| kb.signing_public_key())
    }

    /// Encrypt a document using PQC encryption
    ///
    /// The document is serialized to JSON and then encrypted using the
    /// recipient's public key via Kyber key encapsulation.
    pub fn encrypt_document(
        &self,
        document: &Document,
        recipient_public_key: &[u8],
    ) -> Result<EncryptedDocument> {
        // Serialize document to JSON
        let document_json = serde_json::to_vec(document)
            .context("Failed to serialize document")?;

        // Encrypt using PQC
        let encrypted = pqc_encrypt(
            &document.id,
            &document_json,
            recipient_public_key,
            self.key_bundle.as_ref(),
        )
        .context("Failed to encrypt document with PQC")?;

        let is_signed = encrypted.signature.is_some();

        Ok(EncryptedDocument {
            document_id: document.id.clone(),
            title: document.title.clone(),
            security_level: self.default_security_level,
            encrypted_content: encrypted,
            encrypted_at: chrono::Utc::now(),
            metadata: EncryptionMetadata {
                algorithm: format!("PQC-Kyber-{:?}", self.default_security_level),
                is_signed,
                key_bundle_id: None,
                version: "1.0".to_string(),
            },
        })
    }

    /// Decrypt an encrypted document
    ///
    /// Uses the private key to decapsulate the shared secret and decrypt
    /// the document content.
    pub fn decrypt_document(
        &self,
        encrypted_doc: &EncryptedDocument,
        sender_public_key: Option<&[u8]>,
    ) -> Result<Document> {
        let key_bundle = self
            .key_bundle
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Keys not initialized. Call initialize_keys() first."))?;

        // Decrypt using PQC
        let decrypted_bytes = pqc_decrypt(
            &encrypted_doc.encrypted_content,
            key_bundle.kyber_keypair.private_key(),
            sender_public_key,
        )
        .context("Failed to decrypt document")?;

        // Deserialize document from JSON
        let document: Document = serde_json::from_slice(&decrypted_bytes)
            .context("Failed to deserialize decrypted document")?;

        Ok(document)
    }

    /// Encrypt document using Vault's TPM-backed encryption
    ///
    /// This provides an additional layer of hardware-backed encryption
    /// on top of the PQC encryption.
    pub fn vault_encrypt(
        &self,
        document: &Document,
        profile: EncryptionProfile,
    ) -> Result<Vec<u8>> {
        let document_json = serde_json::to_vec(document)
            .context("Failed to serialize document")?;

        self.vault
            .encrypt_document(&document_json, profile, KeySlot::Primary)
            .context("Vault encryption failed")
    }

    /// Decrypt document using Vault's TPM-backed decryption
    pub fn vault_decrypt(&self, ciphertext: &[u8]) -> Result<Document> {
        let decrypted_bytes = self
            .vault
            .decrypt_document(ciphertext, KeySlot::Primary)
            .context("Vault decryption failed")?;

        let document: Document = serde_json::from_slice(&decrypted_bytes)
            .context("Failed to deserialize decrypted document")?;

        Ok(document)
    }

    /// Get the current security level
    pub fn security_level(&self) -> DocumentSecurityLevel {
        self.default_security_level
    }

    /// Set the security level (requires re-initialization of keys)
    pub fn set_security_level(&mut self, level: DocumentSecurityLevel) {
        self.default_security_level = level;
        self.key_bundle = None; // Force re-initialization
    }

    /// Check if keys are initialized
    pub fn is_initialized(&self) -> bool {
        self.key_bundle.is_some()
    }
}

/// Convenience function to encrypt a document with default settings
pub fn quick_encrypt(
    document: &Document,
    recipient_public_key: &[u8],
) -> Result<EncryptedDocument> {
    let manager = DocumentEncryptionManager::new()?;
    manager.encrypt_document(document, recipient_public_key)
}

/// Convenience function to create a key bundle for document exchange
pub fn create_key_bundle(
    security_level: DocumentSecurityLevel,
) -> Result<PqcKeyBundle> {
    match security_level {
        DocumentSecurityLevel::Standard => {
            PqcKeyBundle::new_kyber768().context("Failed to create standard key bundle")
        }
        DocumentSecurityLevel::High => {
            PqcKeyBundle::new_with_signing().context("Failed to create high-security key bundle")
        }
        DocumentSecurityLevel::Maximum => {
            PqcKeyBundle::new_high_security().context("Failed to create maximum-security key bundle")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Paragraph;

    fn create_test_document() -> Document {
        let mut doc = Document::new("Test Document".to_string());
        doc.add_paragraph(Paragraph::new("Hello, encrypted world!".to_string()))
            .unwrap();
        doc.add_paragraph(Paragraph::new("This is a secret document.".to_string()))
            .unwrap();
        doc
    }

    #[test]
    fn test_encryption_manager_creation() {
        let manager = DocumentEncryptionManager::new();
        assert!(manager.is_ok());
        let manager = manager.unwrap();
        assert_eq!(manager.security_level(), DocumentSecurityLevel::High);
        assert!(!manager.is_initialized());
    }

    #[test]
    fn test_encryption_manager_with_security_level() {
        let manager =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::Maximum);
        assert!(manager.is_ok());
        let manager = manager.unwrap();
        assert_eq!(manager.security_level(), DocumentSecurityLevel::Maximum);
    }

    #[test]
    fn test_key_initialization_standard() {
        let mut manager =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::Standard)
                .unwrap();
        assert!(manager.initialize_keys().is_ok());
        assert!(manager.is_initialized());
        assert!(manager.public_key().is_ok());
        assert!(manager.signing_public_key().is_none()); // Standard has no signing
    }

    #[test]
    fn test_key_initialization_high() {
        let mut manager =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::High).unwrap();
        assert!(manager.initialize_keys().is_ok());
        assert!(manager.is_initialized());
        assert!(manager.public_key().is_ok());
        assert!(manager.signing_public_key().is_some()); // High has signing
    }

    #[test]
    fn test_key_initialization_maximum() {
        let mut manager =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::Maximum).unwrap();
        assert!(manager.initialize_keys().is_ok());
        assert!(manager.is_initialized());
        assert!(manager.public_key().is_ok());
        assert!(manager.signing_public_key().is_some()); // Maximum has signing
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let doc = create_test_document();

        // Create sender (with signing)
        let mut sender =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::High).unwrap();
        sender.initialize_keys().unwrap();

        // Create recipient
        let mut recipient =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::High).unwrap();
        recipient.initialize_keys().unwrap();

        // Encrypt
        let encrypted = sender
            .encrypt_document(&doc, &recipient.public_key().unwrap())
            .unwrap();

        assert_eq!(encrypted.document_id, doc.id);
        assert_eq!(encrypted.title, doc.title);
        assert_eq!(encrypted.security_level, DocumentSecurityLevel::High);
        assert!(encrypted.metadata.is_signed);

        // Decrypt
        let decrypted = recipient
            .decrypt_document(&encrypted, sender.signing_public_key().as_deref())
            .unwrap();

        assert_eq!(decrypted.id, doc.id);
        assert_eq!(decrypted.title, doc.title);
        assert_eq!(decrypted.paragraphs.len(), doc.paragraphs.len());
        assert_eq!(decrypted.paragraphs[0].text, "Hello, encrypted world!");
        assert_eq!(decrypted.paragraphs[1].text, "This is a secret document.");
    }

    #[test]
    fn test_encrypt_without_signing() {
        let doc = create_test_document();

        // Create sender without signing (Standard level)
        let sender =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::Standard)
                .unwrap();

        // Create recipient
        let mut recipient =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::Standard)
                .unwrap();
        recipient.initialize_keys().unwrap();

        // Encrypt (sender has no keys, so no signing)
        let encrypted = sender
            .encrypt_document(&doc, &recipient.public_key().unwrap())
            .unwrap();

        assert!(!encrypted.metadata.is_signed);

        // Decrypt
        let decrypted = recipient.decrypt_document(&encrypted, None).unwrap();
        assert_eq!(decrypted.title, doc.title);
    }

    #[test]
    fn test_vault_encrypt_decrypt() {
        let doc = create_test_document();
        let manager = DocumentEncryptionManager::new().unwrap();

        // Encrypt with Vault (software profile)
        let encrypted = manager
            .vault_encrypt(&doc, EncryptionProfile::Software)
            .unwrap();

        // Decrypt with Vault
        let decrypted = manager.vault_decrypt(&encrypted).unwrap();
        assert_eq!(decrypted.title, doc.title);
        assert_eq!(decrypted.paragraphs.len(), doc.paragraphs.len());
    }

    #[test]
    fn test_security_level_change() {
        let mut manager = DocumentEncryptionManager::new().unwrap();
        manager.initialize_keys().unwrap();
        assert!(manager.is_initialized());

        // Change security level should invalidate keys
        manager.set_security_level(DocumentSecurityLevel::Maximum);
        assert!(!manager.is_initialized());
        assert_eq!(manager.security_level(), DocumentSecurityLevel::Maximum);

        // Re-initialize
        manager.initialize_keys().unwrap();
        assert!(manager.is_initialized());
    }

    #[test]
    fn test_create_key_bundle() {
        let bundle = create_key_bundle(DocumentSecurityLevel::Standard).unwrap();
        assert!(!bundle.public_key().is_empty());

        let bundle = create_key_bundle(DocumentSecurityLevel::High).unwrap();
        assert!(!bundle.public_key().is_empty());
        assert!(bundle.signing_public_key().is_some());

        let bundle = create_key_bundle(DocumentSecurityLevel::Maximum).unwrap();
        assert!(!bundle.public_key().is_empty());
        assert!(bundle.signing_public_key().is_some());
    }

    #[test]
    fn test_encrypted_document_serialization() {
        let doc = create_test_document();

        let mut sender =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::High).unwrap();
        sender.initialize_keys().unwrap();

        let mut recipient =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::High).unwrap();
        recipient.initialize_keys().unwrap();

        let encrypted = sender
            .encrypt_document(&doc, &recipient.public_key().unwrap())
            .unwrap();

        // Serialize to JSON
        let json = serde_json::to_string(&encrypted).unwrap();
        assert!(!json.is_empty());

        // Deserialize back
        let deserialized: EncryptedDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.document_id, encrypted.document_id);
        assert_eq!(deserialized.title, encrypted.title);
    }

    #[test]
    fn test_public_key_without_initialization() {
        let manager = DocumentEncryptionManager::new().unwrap();
        assert!(manager.public_key().is_err());
    }

    #[test]
    fn test_decrypt_without_initialization() {
        let doc = create_test_document();

        let mut sender =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::High).unwrap();
        sender.initialize_keys().unwrap();

        let mut recipient =
            DocumentEncryptionManager::with_security_level(DocumentSecurityLevel::High).unwrap();
        recipient.initialize_keys().unwrap();

        let encrypted = sender
            .encrypt_document(&doc, &recipient.public_key().unwrap())
            .unwrap();

        // Try to decrypt with uninitialized manager
        let uninitialized = DocumentEncryptionManager::new().unwrap();
        assert!(uninitialized.decrypt_document(&encrypted, None).is_err());
    }
}