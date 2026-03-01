//! Secure tunnel for encrypted communication

use crate::error::{MobileError, MobileResult};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret};

/// Tunnel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConfig {
    /// Server URL
    pub server_url: String,
    
    /// Reconnect interval in seconds
    pub reconnect_interval: u64,
    
    /// Keepalive interval in seconds
    pub keepalive_interval: u64,
    
    /// Encryption algorithm
    pub encryption_algorithm: EncryptionAlgorithm,
    
    /// Connection timeout in seconds
    pub timeout: u64,
}

impl Default for TunnelConfig {
    fn default() -> Self {
        Self {
            server_url: "https://tunnel.vantis.ai".to_string(),
            reconnect_interval: 5,
            keepalive_interval: 30,
            encryption_algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            timeout: 30,
        }
    }
}

/// Encryption algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    ChaCha20Poly1305,
    Aes256Gcm,
}

/// Tunnel state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TunnelState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error(String),
}

/// Encryption key
#[derive(Debug, Clone)]
pub struct EncryptionKey {
    /// Key bytes
    pub key: Vec<u8>,
    /// Key creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl EncryptionKey {
    /// Generate a new encryption key
    pub fn generate() -> Self {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        Self {
            key: key.to_vec(),
            created_at: chrono::Utc::now(),
        }
    }
    
    /// Get key as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }
}

/// Key exchange for E2EE
pub struct KeyExchange {
    secret: EphemeralSecret,
    public_key: PublicKey,
}

impl KeyExchange {
    /// Generate a new key pair
    pub fn generate_key_pair() -> Self {
        let secret = EphemeralSecret::new(OsRng);
        let public_key = PublicKey::from(&secret);
        Self { secret, public_key }
    }
    
    /// Get public key
    pub fn public_key(&self) -> PublicKey {
        self.public_key
    }
    
    /// Derive shared secret from peer's public key (consumes self)
    pub fn derive_shared_secret(self, peer_public_key: PublicKey) -> SharedSecret {
        self.secret.diffie_hellman(&peer_public_key)
    }
}

/// Secure tunnel for encrypted communication
pub struct SecureTunnel {
    config: TunnelConfig,
    state: Arc<RwLock<TunnelState>>,
    encryption_key: Arc<RwLock<Option<EncryptionKey>>>,
    shared_secret: Arc<RwLock<Option<SharedSecret>>>,
}

impl SecureTunnel {
    /// Create a new secure tunnel
    pub fn new(config: TunnelConfig) -> MobileResult<Self> {
        Ok(Self {
            config,
            state: Arc::new(RwLock::new(TunnelState::Disconnected)),
            encryption_key: Arc::new(RwLock::new(None)),
            shared_secret: Arc::new(RwLock::new(None)),
        })
    }
    
    /// Connect to the server
    pub async fn connect(&mut self) -> MobileResult<()> {
        *self.state.write().await = TunnelState::Connecting;
        
        // Perform key exchange
        let key_exchange = KeyExchange::generate_key_pair();
        let public_key = key_exchange.public_key();
        
        // In a real implementation, we would:
        // 1. Send public key to server
        // 2. Receive server's public key
        // 3. Derive shared secret
        // 4. Establish encrypted connection
        
        // For demo purposes, we'll simulate a successful connection
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        *self.state.write().await = TunnelState::Connected;
        
        Ok(())
    }
    
    /// Disconnect from the server
    pub async fn disconnect(&mut self) -> MobileResult<()> {
        *self.state.write().await = TunnelState::Disconnected;
        *self.encryption_key.write().await = None;
        *self.shared_secret.write().await = None;
        Ok(())
    }
    
    /// Get current tunnel state
    pub async fn state(&self) -> TunnelState {
        self.state.read().await.clone()
    }
    
    /// Send encrypted message
    pub async fn send_message(&self, message: &[u8]) -> MobileResult<Vec<u8>> {
        let key_guard = self.encryption_key.read().await;
        
        if let Some(key) = key_guard.as_ref() {
            self.encrypt(message, key.as_bytes())
        } else {
            Err(MobileError::Encryption("No encryption key set".to_string()))
        }
    }
    
    /// Receive and decrypt message
    pub async fn receive_message(&self, encrypted: &[u8]) -> MobileResult<Vec<u8>> {
        let key_guard = self.encryption_key.read().await;
        
        if let Some(key) = key_guard.as_ref() {
            self.decrypt(encrypted, key.as_bytes())
        } else {
            Err(MobileError::Encryption("No encryption key set".to_string()))
        }
    }
    
    /// Set encryption key
    pub async fn set_encryption_key(&self, key: EncryptionKey) {
        *self.encryption_key.write().await = Some(key);
    }
    
    /// Set shared secret
    pub async fn set_shared_secret(&self, secret: SharedSecret) {
        *self.shared_secret.write().await = Some(secret);
    }
    
    /// Encrypt data using ChaCha20-Poly1305
    fn encrypt(&self, plaintext: &[u8], key: &[u8]) -> MobileResult<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| MobileError::Encryption(e.to_string()))?;
        
        let nonce = Nonce::from_slice(b"unique nonce"); // In production, use random nonce
        
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| MobileError::Encryption(e.to_string()))?;
        
        Ok(ciphertext)
    }
    
    /// Decrypt data using ChaCha20-Poly1305
    fn decrypt(&self, ciphertext: &[u8], key: &[u8]) -> MobileResult<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| MobileError::Encryption(e.to_string()))?;
        
        let nonce = Nonce::from_slice(b"unique nonce"); // Must match encryption nonce
        
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| MobileError::Encryption(e.to_string()))?;
        
        Ok(plaintext)
    }
    
    /// Send public key to peer
    pub async fn send_public_key(&self, _public_key: PublicKey) -> MobileResult<()> {
        // In real implementation, send to server
        Ok(())
    }
    
    /// Handle received public key
    pub async fn on_public_key_received<F>(
        &self,
        _key_exchange: KeyExchange,
        _callback: F,
    ) -> MobileResult<()>
    where
        F: FnOnce(PublicKey) + Send + 'static,
    {
        // In real implementation, receive from server and call callback
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_exchange() {
        let alice = KeyExchange::generate_key_pair();
        let bob = KeyExchange::generate_key_pair();
        
        let alice_public = alice.public_key();
        let bob_public = bob.public_key();
        
        let alice_shared = alice.derive_shared_secret(bob_public);
        let bob_shared = bob.derive_shared_secret(alice_public);
        
        assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());
    }

    #[test]
    fn test_encryption_key_generation() {
        let key = EncryptionKey::generate();
        assert_eq!(key.as_bytes().len(), 32); // ChaCha20-Poly1305 key size
    }

    #[tokio::test]
    async fn test_tunnel_creation() {
        let config = TunnelConfig::default();
        let tunnel = SecureTunnel::new(config).unwrap();
        assert_eq!(tunnel.state().await, TunnelState::Disconnected);
    }

    #[tokio::test]
    async fn test_encryption_decryption() {
        let config = TunnelConfig::default();
        let tunnel = SecureTunnel::new(config).unwrap();
        
        let key = EncryptionKey::generate();
        tunnel.set_encryption_key(key).await;
        
        let plaintext = b"Hello, Vantis!";
        let encrypted = tunnel.send_message(plaintext).await.unwrap();
        let decrypted = tunnel.receive_message(&encrypted).await.unwrap();
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }
}