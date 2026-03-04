//! Discovery module for peer discovery
//!
//! Provides peer discovery and network information

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Peer Discovery
pub struct PeerDiscovery {
    peers: Arc<RwLock<HashMap<String, PeerInfo>>>,
    enabled: bool,
    local_peer: Option<PeerInfo>,
}

impl PeerDiscovery {
    pub fn new() -> Self {
        PeerDiscovery {
            peers: Arc::new(RwLock::new(HashMap::new())),
            enabled: true,
            local_peer: None,
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

    /// Set local peer info
    pub fn set_local_peer(&mut self, peer: PeerInfo) {
        self.local_peer = Some(peer.clone());

        let mut peers = self.peers.write().ok();
        if let Some(peers) = &mut peers {
            peers.insert(peer.id.clone(), peer);
        }
    }

    /// Discover peers
    pub fn discover_peers(&self) -> Result<Vec<PeerInfo>, String> {
        if !self.enabled {
            return Err("Peer discovery is disabled".to_string());
        }

        let peers = self
            .peers
            .read()
            .map_err(|e| format!("Failed to acquire read lock: {}", e))?;

        Ok(peers.values().cloned().collect())
    }

    /// Add a peer
    pub fn add_peer(&self, peer: PeerInfo) -> Result<(), String> {
        if !self.enabled {
            return Err("Peer discovery is disabled".to_string());
        }

        let mut peers = self
            .peers
            .write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;

        peers.insert(peer.id.clone(), peer);

        Ok(())
    }

    /// Remove a peer
    pub fn remove_peer(&self, peer_id: &str) -> Result<(), String> {
        let mut peers = self
            .peers
            .write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;

        peers
            .remove(peer_id)
            .ok_or_else(|| format!("Peer '{}' not found", peer_id))?;

        Ok(())
    }

    /// Get peer by ID
    pub fn get_peer(&self, peer_id: &str) -> Option<PeerInfo> {
        let peers = self.peers.read().ok()?;
        peers.get(peer_id).cloned()
    }

    /// Get local peer
    pub fn get_local_peer(&self) -> Option<PeerInfo> {
        self.local_peer.clone()
    }

    /// Update peer status
    pub fn update_peer_status(&self, peer_id: &str, online: bool) -> Result<(), String> {
        let mut peers = self
            .peers
            .write()
            .map_err(|e| format!("Failed to acquire write lock: {}", e))?;

        let peer = peers
            .get_mut(peer_id)
            .ok_or_else(|| format!("Peer '{}' not found", peer_id))?;

        peer.is_online = online;
        peer.last_seen = chrono::Utc::now();

        Ok(())
    }
}

/// Peer Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub is_online: bool,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl PeerInfo {
    pub fn new(id: String, name: String, address: String, port: u16) -> Self {
        let now = chrono::Utc::now();
        PeerInfo {
            id,
            name,
            address,
            port,
            is_online: true,
            last_seen: now,
            capabilities: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_capabilities(mut self, capabilities: Vec<String>) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

/// Initialize discovery module
pub fn init() -> Result<(), String> {
    Ok(())
}
