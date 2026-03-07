// Key management for PQC operations
// Handles key storage, retrieval, and lifecycle management

use crate::error::Result;
use crate::kyber::KyberKeyPair;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Key identifier
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KeyId(String);

impl KeyId {
    /// Create a new key ID
    pub fn new(id: String) -> Self {
        KeyId(id)
    }

    /// Generate a random key ID
    pub fn generate() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        let random: u32 = rand::random();
        KeyId(format!("key_{}_{}", timestamp, random))
    }

    /// Get the key ID as string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Key metadata
#[derive(Debug, Clone)]
pub struct KeyMetadata {
    /// Key ID
    pub id: KeyId,
    /// Creation timestamp
    pub created_at: u64,
    /// Expiration timestamp (0 if never expires)
    pub expires_at: u64,
    /// Key usage count
    pub usage_count: u64,
    /// Custom attributes
    pub attributes: HashMap<String, String>,
}

impl KeyMetadata {
    /// Create new key metadata
    pub fn new(id: KeyId) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        KeyMetadata {
            id,
            created_at: now,
            expires_at: 0,
            usage_count: 0,
            attributes: HashMap::new(),
        }
    }

    /// Check if the key has expired
    pub fn is_expired(&self) -> bool {
        if self.expires_at == 0 {
            return false;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        now > self.expires_at
    }

    /// Increment usage count
    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
    }
}

/// Key storage backend
pub trait KeyStorage: Send + Sync {
    /// Store a key pair
    fn store_key(&self, key_pair: &KyberKeyPair, metadata: KeyMetadata) -> Result<KeyId>;

    /// Retrieve a key pair
    fn retrieve_key(&self, key_id: &KeyId) -> Result<Option<(KyberKeyPair, KeyMetadata)>>;

    /// Delete a key
    fn delete_key(&self, key_id: &KeyId) -> Result<()>;

    /// List all key IDs
    fn list_keys(&self) -> Result<Vec<KeyId>>;
}

/// In-memory key storage (for testing and development)
pub struct MemoryKeyStorage {
    keys: Arc<Mutex<HashMap<KeyId, (KyberKeyPair, KeyMetadata)>>>,
}

impl MemoryKeyStorage {
    /// Create a new in-memory key storage
    pub fn new() -> Self {
        MemoryKeyStorage {
            keys: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for MemoryKeyStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyStorage for MemoryKeyStorage {
    fn store_key(&self, key_pair: &KyberKeyPair, metadata: KeyMetadata) -> Result<KeyId> {
        let mut keys = self.keys.lock().unwrap();
        let key_id = metadata.id.clone();
        keys.insert(key_id.clone(), (key_pair.clone(), metadata));
        Ok(key_id)
    }

    fn retrieve_key(&self, key_id: &KeyId) -> Result<Option<(KyberKeyPair, KeyMetadata)>> {
        let keys = self.keys.lock().unwrap();
        Ok(keys.get(key_id).cloned())
    }

    fn delete_key(&self, key_id: &KeyId) -> Result<()> {
        let mut keys = self.keys.lock().unwrap();
        keys.remove(key_id);
        Ok(())
    }

    fn list_keys(&self) -> Result<Vec<KeyId>> {
        let keys = self.keys.lock().unwrap();
        Ok(keys.keys().cloned().collect())
    }
}

/// Key manager for handling PQC key lifecycle
pub struct KeyManager {
    storage: Box<dyn KeyStorage>,
    default_ttl: Option<u64>, // Time to live in seconds
}

impl KeyManager {
    /// Create a new key manager
    pub fn new(storage: Box<dyn KeyStorage>) -> Self {
        KeyManager {
            storage,
            default_ttl: None,
        }
    }

    /// Create a new key manager with in-memory storage
    pub fn with_memory_storage() -> Self {
        KeyManager::new(Box::new(MemoryKeyStorage::new()))
    }

    /// Set default TTL for keys
    pub fn with_ttl(mut self, ttl: u64) -> Self {
        self.default_ttl = Some(ttl);
        self
    }

    /// Generate and store a new key
    pub fn generate_key(&self) -> Result<KeyId> {
        use crate::kyber::{generate_keypair, KyberSecurityLevel};
        
        let key_pair = generate_keypair(KyberSecurityLevel::Level2)?;
        let key_id = KeyId::generate();
        
        let mut metadata = KeyMetadata::new(key_id.clone());
        
        if let Some(ttl) = self.default_ttl {
            let expires_at = metadata.created_at + ttl;
            metadata.expires_at = expires_at;
        }

        self.storage.store_key(&key_pair, metadata)?;
        Ok(key_id)
    }

    /// Retrieve a key by ID
    pub fn get_key(&self, key_id: &KeyId) -> Result<Option<KyberKeyPair>> {
        match self.storage.retrieve_key(key_id)? {
            Some((key_pair, metadata)) => {
                if metadata.is_expired() {
                    return Ok(None);
                }
                
                // Increment usage count (this would need a mutable storage in real implementation)
                Ok(Some(key_pair))
            }
            None => Ok(None),
        }
    }

    /// Delete a key
    pub fn delete_key(&self, key_id: &KeyId) -> Result<()> {
        self.storage.delete_key(key_id)
    }

    /// List all keys
    pub fn list_keys(&self) -> Result<Vec<KeyId>> {
        self.storage.list_keys()
    }

    /// Clean up expired keys
    pub fn cleanup_expired(&self) -> Result<usize> {
        let keys = self.list_keys()?;
        let mut count = 0;
        
        for key_id in keys {
            if let Some((_, metadata)) = self.storage.retrieve_key(&key_id)? {
                if metadata.is_expired() {
                    self.delete_key(&key_id)?;
                    count += 1;
                }
            }
        }
        
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kyber::KyberSecurityLevel;

    #[test]
    fn test_key_id_generation() {
        let id1 = KeyId::generate();
        let id2 = KeyId::generate();
        
        assert_ne!(id1, id2);
        assert!(id1.as_str().starts_with("key_"));
    }

    #[test]
    fn test_key_metadata() {
        let id = KeyId::new("test_key".to_string());
        let mut metadata = KeyMetadata::new(id.clone());
        
        assert!(!metadata.is_expired());
        metadata.expires_at = 1; // Very old timestamp
        assert!(metadata.is_expired());
        
        assert_eq!(metadata.usage_count, 0);
        metadata.increment_usage();
        assert_eq!(metadata.usage_count, 1);
    }

    #[test]
    fn test_memory_storage() {
        let storage = MemoryKeyStorage::new();
        use crate::kyber::generate_keypair;
        
        let key_pair = generate_keypair(KyberSecurityLevel::Level2).unwrap();
        let metadata = KeyMetadata::new(KeyId::generate());
        let key_id = metadata.id.clone();
        
        // Store key
        let stored_id = storage.store_key(&key_pair, metadata).unwrap();
        assert_eq!(stored_id, key_id);
        
        // Retrieve key
        let retrieved = storage.retrieve_key(&key_id).unwrap();
        assert!(retrieved.is_some());
        
        // List keys
        let keys = storage.list_keys().unwrap();
        assert!(keys.contains(&key_id));
        
        // Delete key
        storage.delete_key(&key_id).unwrap();
        let retrieved = storage.retrieve_key(&key_id).unwrap();
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_key_manager() {
        let manager = KeyManager::with_memory_storage();
        
        // Generate key
        let key_id = manager.generate_key().unwrap();
        
        // Retrieve key
        let key = manager.get_key(&key_id).unwrap();
        assert!(key.is_some());
        
        // List keys
        let keys = manager.list_keys().unwrap();
        assert!(keys.contains(&key_id));
        
        // Delete key
        manager.delete_key(&key_id).unwrap();
        let key = manager.get_key(&key_id).unwrap();
        assert!(key.is_none());
    }

    #[test]
    fn test_key_manager_with_ttl() {
        let manager = KeyManager::with_memory_storage().with_ttl(1); // 1 second TTL
        
        let key_id = manager.generate_key().unwrap();
        
        // Key should be available immediately
        let key = manager.get_key(&key_id).unwrap();
        assert!(key.is_some());
        
        // Wait for expiration
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        // Key should be expired
        let key = manager.get_key(&key_id).unwrap();
        assert!(key.is_none());
    }
}