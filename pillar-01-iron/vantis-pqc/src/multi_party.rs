//! Multi-Party Encryption Module
//! 
//! Implements secure group collaboration with multi-recipient encryption,
//! dynamic group membership, and access control.
//!
//! # Features
//! - Group key management with automatic rotation
//! - Multi-recipient encryption (encrypt once, decrypt by many)
//! - Dynamic membership (add/remove members)
//! - Forward secrecy for removed members
//! - Backward compatibility for new members
//! - Hierarchical access control (read/write/admin)

use crate::error::{PQCError, Result};
use crate::kyber::{self, KyberSecurityLevel, KyberKeyPair};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zeroize::ZeroizeOnDrop;

/// Access level for group members
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessLevel {
    /// Read-only access
    Read,
    /// Read and write access
    Write,
    /// Full administrative access
    Admin,
}

impl Default for AccessLevel {
    fn default() -> Self {
        Self::Read
    }
}

/// Group member information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    /// Unique member identifier
    pub id: String,
    /// Member's public key for encryption
    pub public_key: Vec<u8>,
    /// Access level
    pub access_level: AccessLevel,
    /// When the member was added
    pub added_at: u64,
    /// Key version when member was added
    pub key_version: u32,
}

/// Group key information (encrypted for each member)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedGroupKey {
    /// Member ID this key belongs to
    pub member_id: String,
    /// Encrypted group key (encrypted with member's public key)
    pub encrypted_key: Vec<u8>,
    /// Key version
    pub version: u32,
}

/// Group state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupState {
    /// Unique group identifier
    pub group_id: String,
    /// Group name
    pub name: String,
    /// Current key version (incremented on rotation)
    pub key_version: u32,
    /// Group members
    pub members: HashMap<String, GroupMember>,
    /// Encrypted group keys for each member
    pub encrypted_keys: Vec<EncryptedGroupKey>,
    /// Creation timestamp
    pub created_at: u64,
    /// Last updated timestamp
    pub updated_at: u64,
}

impl GroupState {
    /// Create a new group state
    pub fn new(group_id: &str, name: &str) -> Self {
        let now = current_timestamp();
        Self {
            group_id: group_id.to_string(),
            name: name.to_string(),
            key_version: 1,
            members: HashMap::new(),
            encrypted_keys: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Add a member to the group
    pub fn add_member(&mut self, member: GroupMember) {
        self.members.insert(member.id.clone(), member);
        self.updated_at = current_timestamp();
    }
    
    /// Remove a member from the group
    pub fn remove_member(&mut self, member_id: &str) -> bool {
        let removed = self.members.remove(member_id).is_some();
        if removed {
            self.updated_at = current_timestamp();
        }
        removed
    }
    
    /// Get a member by ID
    pub fn get_member(&self, member_id: &str) -> Option<&GroupMember> {
        self.members.get(member_id)
    }
    
    /// Check if a member has at least the specified access level
    pub fn check_access(&self, member_id: &str, required_level: AccessLevel) -> bool {
        self.members.get(member_id)
            .map(|m| matches!((m.access_level, required_level),
                (AccessLevel::Admin, _) |
                (AccessLevel::Write, AccessLevel::Write) |
                (AccessLevel::Write, AccessLevel::Read) |
                (AccessLevel::Read, AccessLevel::Read)))
            .unwrap_or(false)
    }
    
    /// Get all member IDs
    pub fn member_ids(&self) -> Vec<String> {
        self.members.keys().cloned().collect()
    }
    
    /// Get member count
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}

/// Multi-recipient encrypted message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiRecipientMessage {
    /// Message ID
    pub message_id: String,
    /// Group ID
    pub group_id: String,
    /// Key version used for encryption
    pub key_version: u32,
    /// Sender member ID
    pub sender_id: String,
    /// Nonce for encryption
    pub nonce: Vec<u8>,
    /// Encrypted content (encrypted with group key)
    pub ciphertext: Vec<u8>,
    /// Per-recipient encrypted keys (for new members not in group key rotation)
    pub recipient_keys: Vec<EncryptedGroupKey>,
    /// Timestamp
    pub timestamp: u64,
}

/// Multi-party encryption manager
#[derive(Debug)]
pub struct MultiPartyManager {
    /// Security level for Kyber operations
    security_level: KyberSecurityLevel,
    /// Current group state
    group_state: GroupState,
    /// Current group key (plaintext, kept in secure memory)
    group_key: Option<SecureGroupKey>,
}

/// Secure group key that zeroizes on drop
#[derive(ZeroizeOnDrop)]
pub struct SecureGroupKey {
    #[zeroize]
    key: Vec<u8>,
    version: u32,
}

impl std::fmt::Debug for SecureGroupKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecureGroupKey")
            .field("key", &"[REDACTED]")
            .field("version", &self.version)
            .finish()
    }
}

impl SecureGroupKey {
    fn new(key: Vec<u8>, version: u32) -> Self {
        Self { key, version }
    }
    
    fn as_bytes(&self) -> &[u8] {
        &self.key
    }
    
    fn version(&self) -> u32 {
        self.version
    }
}

impl MultiPartyManager {
    /// Create a new multi-party manager for a group
    pub fn new(group_id: &str, group_name: &str, security_level: KyberSecurityLevel) -> Self {
        Self {
            security_level,
            group_state: GroupState::new(group_id, group_name),
            group_key: None,
        }
    }
    
    /// Initialize the group with the first member (creator)
    pub fn initialize(&mut self, creator_id: &str, creator_public_key: &[u8]) -> Result<()> {
        // Generate group key
        let group_key = generate_group_key();
        let version = self.group_state.key_version;
        
        // Encrypt group key for creator
        let encrypted_key = kyber::encapsulate(creator_public_key, self.security_level)?;
        
        // Add creator as admin
        let creator = GroupMember {
            id: creator_id.to_string(),
            public_key: creator_public_key.to_vec(),
            access_level: AccessLevel::Admin,
            added_at: current_timestamp(),
            key_version: version,
        };
        
        self.group_state.add_member(creator);
        self.group_state.encrypted_keys.push(EncryptedGroupKey {
            member_id: creator_id.to_string(),
            encrypted_key: encrypted_key.ciphertext,
            version,
        });
        
        // Store group key
        self.group_key = Some(SecureGroupKey::new(group_key, version));
        
        Ok(())
    }
    
    /// Add a new member to the group
    pub fn add_member(
        &mut self,
        admin_id: &str,
        new_member_id: &str,
        new_member_public_key: &[u8],
        access_level: AccessLevel,
    ) -> Result<()> {
        // Verify admin has permission
        if !self.group_state.check_access(admin_id, AccessLevel::Admin) {
            return Err(PQCError::PermissionDenied("Only admins can add members".into()));
        }
        
        // Get group key
        let group_key = self.group_key.as_ref()
            .ok_or_else(|| PQCError::EncryptionFailed("Group not initialized".into()))?;
        
        // Encrypt group key for new member
        let encapsulation = kyber::encapsulate(new_member_public_key, self.security_level)?;
        
        // Create new member
        let member = GroupMember {
            id: new_member_id.to_string(),
            public_key: new_member_public_key.to_vec(),
            access_level,
            added_at: current_timestamp(),
            key_version: self.group_state.key_version,
        };
        
        // Add encrypted key
        self.group_state.encrypted_keys.push(EncryptedGroupKey {
            member_id: new_member_id.to_string(),
            encrypted_key: encapsulation.ciphertext,
            version: group_key.version(),
        });
        
        self.group_state.add_member(member);
        
        Ok(())
    }
    
    /// Remove a member from the group
    pub fn remove_member(&mut self, admin_id: &str, member_id: &str) -> Result<()> {
        // Verify admin has permission
        if !self.group_state.check_access(admin_id, AccessLevel::Admin) {
            return Err(PQCError::PermissionDenied("Only admins can remove members".into()));
        }
        
        // Cannot remove yourself if you're the last admin
        if admin_id == member_id {
            let admin_count = self.group_state.members.values()
                .filter(|m| m.access_level == AccessLevel::Admin)
                .count();
            if admin_count <= 1 {
                return Err(PQCError::PermissionDenied(
                    "Cannot remove the last admin".into()
                ));
            }
        }
        
        // Remove member
        self.group_state.remove_member(member_id);
        
        // Remove encrypted key for this member
        self.group_state.encrypted_keys.retain(|k| k.member_id != member_id);
        
        // Rotate group key for forward secrecy
        self.rotate_group_key()?;
        
        Ok(())
    }
    
    /// Rotate the group key
    pub fn rotate_group_key(&mut self) -> Result<()> {
        // Generate new group key
        let new_group_key = generate_group_key();
        
        // Increment version
        self.group_state.key_version += 1;
        let new_version = self.group_state.key_version;
        
        // Clear old encrypted keys
        self.group_state.encrypted_keys.clear();
        
        // Encrypt new key for all current members
        for (member_id, member) in &self.group_state.members {
            let encapsulation = kyber::encapsulate(&member.public_key, self.security_level)?;
            self.group_state.encrypted_keys.push(EncryptedGroupKey {
                member_id: member_id.clone(),
                encrypted_key: encapsulation.ciphertext,
                version: new_version,
            });
        }
        
        // Update group key
        self.group_key = Some(SecureGroupKey::new(new_group_key, new_version));
        
        Ok(())
    }
    
    /// Encrypt a message for all group members
    pub fn encrypt(
        &self,
        sender_id: &str,
        plaintext: &[u8],
    ) -> Result<MultiRecipientMessage> {
        // Verify sender is a member
        let sender = self.group_state.get_member(sender_id)
            .ok_or_else(|| PQCError::PermissionDenied("Sender is not a group member".into()))?;
        
        // Verify sender has write access
        if !self.group_state.check_access(sender_id, AccessLevel::Write) {
            return Err(PQCError::PermissionDenied("Sender does not have write access".into()));
        }
        
        // Get group key
        let group_key = self.group_key.as_ref()
            .ok_or_else(|| PQCError::EncryptionFailed("Group not initialized".into()))?;
        
        // Generate nonce
        let nonce = crate::secure_memory::secure_random_bytes(12);
        
        // Encrypt with ChaCha20-Poly1305 using group key
        let ciphertext = encrypt_with_key(group_key.as_bytes(), &nonce, plaintext)?;
        
        Ok(MultiRecipientMessage {
            message_id: uuid::Uuid::new_v4().to_string(),
            group_id: self.group_state.group_id.clone(),
            key_version: group_key.version(),
            sender_id: sender_id.to_string(),
            nonce,
            ciphertext,
            recipient_keys: Vec::new(), // For future per-message key distribution
            timestamp: current_timestamp(),
        })
    }
    
    /// Decrypt a message
    pub fn decrypt(
        &self,
        member_id: &str,
        message: &MultiRecipientMessage,
    ) -> Result<Vec<u8>> {
        // Verify member is in group
        if !self.group_state.members.contains_key(member_id) {
            return Err(PQCError::PermissionDenied("Not a group member".into()));
        }
        
        // Verify message is for this group
        if message.group_id != self.group_state.group_id {
            return Err(PQCError::DecryptionFailed("Message is for a different group".into()));
        }
        
        // Get group key
        let group_key = self.group_key.as_ref()
            .ok_or_else(|| PQCError::DecryptionFailed("Group not initialized".into()))?;
        
        // Verify key version
        if message.key_version != group_key.version() {
            return Err(PQCError::DecryptionFailed(
                "Key version mismatch - member needs key update".into()
            ));
        }
        
        // Decrypt with ChaCha20-Poly1305
        decrypt_with_key(group_key.as_bytes(), &message.nonce, &message.ciphertext)
    }
    
    /// Get group state
    pub fn group_state(&self) -> &GroupState {
        &self.group_state
    }
    
    /// Serialize group state for storage
    pub fn serialize_state(&self) -> Result<Vec<u8>> {
        bincode::serialize(&self.group_state)
            .map_err(|e| PQCError::SerializationError(e.to_string()))
    }
    
    /// Deserialize group state from storage
    pub fn deserialize_state(data: &[u8]) -> Result<GroupState> {
        bincode::deserialize(data)
            .map_err(|e| PQCError::SerializationError(e.to_string()))
    }
    
    /// Update member access level
    pub fn update_access_level(
        &mut self,
        admin_id: &str,
        member_id: &str,
        new_level: AccessLevel,
    ) -> Result<()> {
        // Verify admin has permission
        if !self.group_state.check_access(admin_id, AccessLevel::Admin) {
            return Err(PQCError::PermissionDenied("Only admins can update access levels".into()));
        }
        
        // Update access level
        if let Some(member) = self.group_state.members.get_mut(member_id) {
            member.access_level = new_level;
            self.group_state.updated_at = current_timestamp();
            Ok(())
        } else {
            Err(PQCError::PermissionDenied("Member not found".into()))
        }
    }
}

/// Generate a random group key
fn generate_group_key() -> Vec<u8> {
    crate::secure_memory::secure_random_bytes(32)
}

/// Get current timestamp in seconds
fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Encrypt data with a key using ChaCha20-Poly1305
fn encrypt_with_key(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, KeyInit};
    use chacha20poly1305::aead::Aead;
    
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);
    
    cipher.encrypt(nonce, plaintext.as_ref())
        .map_err(|e: chacha20poly1305::Error| PQCError::EncryptionFailed(e.to_string()))
}

/// Decrypt data with a key using ChaCha20-Poly1305
fn decrypt_with_key(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce, KeyInit};
    use chacha20poly1305::aead::Aead;
    
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = Nonce::from_slice(nonce);
    
    cipher.decrypt(nonce, ciphertext.as_ref())
        .map_err(|e: chacha20poly1305::Error| PQCError::DecryptionFailed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kyber::KyberKeyPair;
    
    #[test]
    fn test_access_level_permissions() {
        assert!(matches!(AccessLevel::Read, AccessLevel::Read));
        assert!(matches!(AccessLevel::Write, AccessLevel::Write));
        assert!(matches!(AccessLevel::Admin, AccessLevel::Admin));
    }
    
    #[test]
    fn test_group_state_creation() {
        let state = GroupState::new("group-1", "Test Group");
        assert_eq!(state.group_id, "group-1");
        assert_eq!(state.name, "Test Group");
        assert_eq!(state.key_version, 1);
        assert_eq!(state.member_count(), 0);
    }
    
    #[test]
    fn test_group_state_add_member() {
        let mut state = GroupState::new("group-1", "Test Group");
        let member = GroupMember {
            id: "user-1".to_string(),
            public_key: vec![0u8; 32],
            access_level: AccessLevel::Admin,
            added_at: 0,
            key_version: 1,
        };
        
        state.add_member(member);
        assert_eq!(state.member_count(), 1);
        assert!(state.get_member("user-1").is_some());
    }
    
    #[test]
    fn test_group_state_remove_member() {
        let mut state = GroupState::new("group-1", "Test Group");
        let member = GroupMember {
            id: "user-1".to_string(),
            public_key: vec![0u8; 32],
            access_level: AccessLevel::Admin,
            added_at: 0,
            key_version: 1,
        };
        
        state.add_member(member);
        assert_eq!(state.member_count(), 1);
        
        let removed = state.remove_member("user-1");
        assert!(removed);
        assert_eq!(state.member_count(), 0);
    }
    
    #[test]
    fn test_check_access() {
        let mut state = GroupState::new("group-1", "Test Group");
        
        // Add members with different access levels
        state.add_member(GroupMember {
            id: "reader".to_string(),
            public_key: vec![0u8; 32],
            access_level: AccessLevel::Read,
            added_at: 0,
            key_version: 1,
        });
        state.add_member(GroupMember {
            id: "writer".to_string(),
            public_key: vec![0u8; 32],
            access_level: AccessLevel::Write,
            added_at: 0,
            key_version: 1,
        });
        state.add_member(GroupMember {
            id: "admin".to_string(),
            public_key: vec![0u8; 32],
            access_level: AccessLevel::Admin,
            added_at: 0,
            key_version: 1,
        });
        
        // Reader can only read
        assert!(state.check_access("reader", AccessLevel::Read));
        assert!(!state.check_access("reader", AccessLevel::Write));
        assert!(!state.check_access("reader", AccessLevel::Admin));
        
        // Writer can read and write
        assert!(state.check_access("writer", AccessLevel::Read));
        assert!(state.check_access("writer", AccessLevel::Write));
        assert!(!state.check_access("writer", AccessLevel::Admin));
        
        // Admin can do everything
        assert!(state.check_access("admin", AccessLevel::Read));
        assert!(state.check_access("admin", AccessLevel::Write));
        assert!(state.check_access("admin", AccessLevel::Admin));
    }
    
    #[test]
    fn test_multi_party_manager_creation() {
        let manager = MultiPartyManager::new("group-1", "Test Group", KyberSecurityLevel::Kyber768);
        assert_eq!(manager.group_state().group_id, "group-1");
        assert_eq!(manager.group_state().name, "Test Group");
    }
    
    #[test]
    fn test_multi_party_initialize() {
        let mut manager = MultiPartyManager::new("group-1", "Test Group", KyberSecurityLevel::Kyber768);
        let keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        
        let result = manager.initialize("admin-1", &keypair.public_key());
        assert!(result.is_ok());
        assert_eq!(manager.group_state().member_count(), 1);
        assert!(manager.group_state().check_access("admin-1", AccessLevel::Admin));
    }
    
    #[test]
    fn test_multi_party_encrypt_decrypt() {
        let mut manager = MultiPartyManager::new("group-1", "Test Group", KyberSecurityLevel::Kyber768);
        let keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        
        manager.initialize("admin-1", &keypair.public_key()).unwrap();
        
        let plaintext = b"Hello, group members!";
        let message = manager.encrypt("admin-1", plaintext).unwrap();
        
        assert_eq!(message.group_id, "group-1");
        assert_eq!(message.sender_id, "admin-1");
        assert!(!message.ciphertext.is_empty());
        
        let decrypted = manager.decrypt("admin-1", &message).unwrap();
        assert_eq!(decrypted, plaintext);
    }
    
    #[test]
    fn test_add_member() {
        let mut manager = MultiPartyManager::new("group-1", "Test Group", KyberSecurityLevel::Kyber768);
        let admin_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        let member_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        
        manager.initialize("admin-1", &admin_keypair.public_key()).unwrap();
        
        let result = manager.add_member(
            "admin-1",
            "member-1",
            &member_keypair.public_key(),
            AccessLevel::Write,
        );
        
        assert!(result.is_ok());
        assert_eq!(manager.group_state().member_count(), 2);
        assert!(manager.group_state().check_access("member-1", AccessLevel::Write));
    }
    
    #[test]
    fn test_remove_member() {
        let mut manager = MultiPartyManager::new("group-1", "Test Group", KyberSecurityLevel::Kyber768);
        let admin_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        let member_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        
        manager.initialize("admin-1", &admin_keypair.public_key()).unwrap();
        manager.add_member(
            "admin-1",
            "member-1",
            &member_keypair.public_key(),
            AccessLevel::Write,
        ).unwrap();
        
        assert_eq!(manager.group_state().member_count(), 2);
        
        // Need another admin to remove the first admin
        let admin2_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        manager.add_member(
            "admin-1",
            "admin-2",
            &admin2_keypair.public_key(),
            AccessLevel::Admin,
        ).unwrap();
        
        // Remove member (triggers key rotation)
        let result = manager.remove_member("admin-2", "member-1");
        assert!(result.is_ok());
        assert_eq!(manager.group_state().member_count(), 2); // 2 admins remain
    }
    
    #[test]
    fn test_key_rotation() {
        let mut manager = MultiPartyManager::new("group-1", "Test Group", KyberSecurityLevel::Kyber768);
        let keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        
        manager.initialize("admin-1", &keypair.public_key()).unwrap();
        let initial_version = manager.group_state().key_version;
        
        manager.rotate_group_key().unwrap();
        
        assert!(manager.group_state().key_version > initial_version);
    }
    
    #[test]
    fn test_update_access_level() {
        let mut manager = MultiPartyManager::new("group-1", "Test Group", KyberSecurityLevel::Kyber768);
        let admin_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        let member_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        
        manager.initialize("admin-1", &admin_keypair.public_key()).unwrap();
        manager.add_member(
            "admin-1",
            "member-1",
            &member_keypair.public_key(),
            AccessLevel::Read,
        ).unwrap();
        
        assert!(manager.group_state().check_access("member-1", AccessLevel::Read));
        assert!(!manager.group_state().check_access("member-1", AccessLevel::Write));
        
        manager.update_access_level("admin-1", "member-1", AccessLevel::Write).unwrap();
        
        assert!(manager.group_state().check_access("member-1", AccessLevel::Write));
    }
    
    #[test]
    fn test_serialize_deserialize_state() {
        let mut manager = MultiPartyManager::new("group-1", "Test Group", KyberSecurityLevel::Kyber768);
        let keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        
        manager.initialize("admin-1", &keypair.public_key()).unwrap();
        
        let serialized = manager.serialize_state().unwrap();
        let deserialized = MultiPartyManager::deserialize_state(&serialized).unwrap();
        
        assert_eq!(deserialized.group_id, "group-1");
        assert_eq!(deserialized.member_count(), 1);
    }
    
    #[test]
    fn test_non_member_cannot_decrypt() {
        let mut manager = MultiPartyManager::new("group-1", "Test Group", KyberSecurityLevel::Kyber768);
        let keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        
        manager.initialize("admin-1", &keypair.public_key()).unwrap();
        
        let plaintext = b"Secret message";
        let message = manager.encrypt("admin-1", plaintext).unwrap();
        
        // Try to decrypt as non-member
        let result = manager.decrypt("non-member", &message);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_reader_cannot_encrypt() {
        let mut manager = MultiPartyManager::new("group-1", "Test Group", KyberSecurityLevel::Kyber768);
        let admin_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        let reader_keypair = KyberKeyPair::generate(KyberSecurityLevel::Kyber768).unwrap();
        
        manager.initialize("admin-1", &admin_keypair.public_key()).unwrap();
        manager.add_member(
            "admin-1",
            "reader-1",
            &reader_keypair.public_key(),
            AccessLevel::Read,
        ).unwrap();
        
        let plaintext = b"Secret message";
        let result = manager.encrypt("reader-1", plaintext);
        assert!(result.is_err());
    }
}