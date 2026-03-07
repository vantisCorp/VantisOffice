//! Key Rotation and Migration for Post-Quantum Cryptography
//!
//! This module provides key rotation policies and migration utilities for
//! transitioning between cryptographic keys. It supports:
//! - Automatic key rotation based on age and usage
//! - Key versioning for backward compatibility
//! - Migration from classical to post-quantum algorithms
//! - Scheduled key rotation policies

use crate::error::{PQCError, Result};
use crate::kyber::{KyberKeyPair, KyberSecurityLevel};
use crate::dilithium::{DilithiumKeyPair, DilithiumSecurityLevel};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Key version identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyVersion(pub u32);

impl Default for KeyVersion {
    fn default() -> Self {
        Self(1)
    }
}

impl KeyVersion {
    /// Create a new key version
    pub fn new(version: u32) -> Self {
        Self(version)
    }

    /// Increment the version
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// Get the version number
    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

/// Key state in the rotation lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyState {
    /// Key is active and can be used for all operations
    Active,
    /// Key is being phased out, can still decrypt/verify
    Deprecated,
    /// Key is no longer in use, kept for historical purposes
    Archived,
    /// Key is scheduled for deletion
    ScheduledDeletion,
}

impl Default for KeyState {
    fn default() -> Self {
        Self::Active
    }
}

/// Rotation policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationPolicy {
    /// Maximum key age before rotation (in seconds)
    pub max_age_seconds: u64,
    /// Maximum number of uses before rotation
    pub max_uses: Option<u64>,
    /// Grace period for deprecated keys (in seconds)
    pub deprecation_grace_seconds: u64,
    /// Time before archived keys are deleted (in seconds)
    pub archive_retention_seconds: u64,
    /// Enable automatic rotation
    pub auto_rotate: bool,
}

impl Default for RotationPolicy {
    fn default() -> Self {
        Self {
            max_age_seconds: 365 * 24 * 60 * 60, // 1 year
            max_uses: Some(1_000_000),
            deprecation_grace_seconds: 30 * 24 * 60 * 60, // 30 days
            archive_retention_seconds: 90 * 24 * 60 * 60, // 90 days
            auto_rotate: true,
        }
    }
}

impl RotationPolicy {
    /// Create a strict rotation policy (90 days)
    pub fn strict() -> Self {
        Self {
            max_age_seconds: 90 * 24 * 60 * 60, // 90 days
            max_uses: Some(100_000),
            deprecation_grace_seconds: 7 * 24 * 60 * 60, // 7 days
            archive_retention_seconds: 30 * 24 * 60 * 60, // 30 days
            auto_rotate: true,
        }
    }

    /// Create a relaxed rotation policy (2 years)
    pub fn relaxed() -> Self {
        Self {
            max_age_seconds: 2 * 365 * 24 * 60 * 60, // 2 years
            max_uses: None,
            deprecation_grace_seconds: 60 * 24 * 60 * 60, // 60 days
            archive_retention_seconds: 180 * 24 * 60 * 60, // 180 days
            auto_rotate: false,
        }
    }

    /// Check if a key should be rotated based on age
    pub fn should_rotate_by_age(&self, created_at: SystemTime) -> bool {
        let age = SystemTime::now()
            .duration_since(created_at)
            .unwrap_or(Duration::ZERO);
        age.as_secs() >= self.max_age_seconds
    }

    /// Check if a key should be rotated based on usage count
    pub fn should_rotate_by_usage(&self, use_count: u64) -> bool {
        self.max_uses.map_or(false, |max| use_count >= max)
    }
}

/// Key metadata for rotation tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotatableKeyMetadata {
    /// Unique key identifier
    pub key_id: String,
    /// Key version
    pub version: KeyVersion,
    /// Current state
    pub state: KeyState,
    /// Creation timestamp
    pub created_at: u64,
    /// Last rotation timestamp
    pub last_rotated_at: Option<u64>,
    /// Usage counter
    pub use_count: u64,
    /// Security level (algorithm-specific)
    pub security_level: String,
    /// Parent key ID (if rotated from another key)
    pub parent_key_id: Option<String>,
    /// Rotation reason
    pub rotation_reason: Option<String>,
}

impl RotatableKeyMetadata {
    /// Create new key metadata
    pub fn new(key_id: String, security_level: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Self {
            key_id,
            version: KeyVersion::default(),
            state: KeyState::Active,
            created_at: now,
            last_rotated_at: None,
            use_count: 0,
            security_level,
            parent_key_id: None,
            rotation_reason: None,
        }
    }

    /// Record a use of this key
    pub fn record_use(&mut self) {
        self.use_count += 1;
    }

    /// Get the key age in seconds
    pub fn age_seconds(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.created_at)
    }

    /// Transition to a new state
    pub fn transition_to(&mut self, new_state: KeyState) {
        self.state = new_state;
    }
}

/// Rotation event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationEvent {
    /// Event ID
    pub event_id: String,
    /// Old key ID
    pub old_key_id: String,
    /// New key ID
    pub new_key_id: String,
    /// Rotation timestamp
    pub timestamp: u64,
    /// Rotation reason
    pub reason: String,
    /// Old key version
    pub old_version: KeyVersion,
    /// New key version
    pub new_version: KeyVersion,
}

impl RotationEvent {
    /// Create a new rotation event
    pub fn new(
        old_key_id: String,
        new_key_id: String,
        reason: String,
        old_version: KeyVersion,
        new_version: KeyVersion,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Self {
            event_id: format!("rot_{}", now),
            old_key_id,
            new_key_id,
            timestamp: now,
            reason,
            old_version,
            new_version,
        }
    }
}

/// Key rotation manager
pub struct KeyRotationManager {
    /// Rotation policy
    policy: RotationPolicy,
    /// Key metadata store
    key_metadata: std::collections::HashMap<String, RotatableKeyMetadata>,
    /// Rotation history
    rotation_history: Vec<RotationEvent>,
}

impl KeyRotationManager {
    /// Create a new rotation manager with default policy
    pub fn new() -> Self {
        Self {
            policy: RotationPolicy::default(),
            key_metadata: std::collections::HashMap::new(),
            rotation_history: Vec::new(),
        }
    }

    /// Create a rotation manager with custom policy
    pub fn with_policy(policy: RotationPolicy) -> Self {
        Self {
            policy,
            key_metadata: std::collections::HashMap::new(),
            rotation_history: Vec::new(),
        }
    }

    /// Register a key for rotation tracking
    pub fn register_key(&mut self, key_id: String, security_level: String) {
        let metadata = RotatableKeyMetadata::new(key_id.clone(), security_level);
        self.key_metadata.insert(key_id, metadata);
    }

    /// Check if a key needs rotation
    pub fn needs_rotation(&self, key_id: &str) -> Option<String> {
        let metadata = self.key_metadata.get(key_id)?;
        
        // Check age-based rotation
        if self.policy.should_rotate_by_age(
            SystemTime::UNIX_EPOCH + Duration::from_secs(metadata.created_at)
        ) {
            return Some(format!(
                "Key age ({}) exceeds maximum ({})",
                metadata.age_seconds(),
                self.policy.max_age_seconds
            ));
        }
        
        // Check usage-based rotation
        if let Some(reason) = self.policy.max_uses.and_then(|max| {
            if metadata.use_count >= max {
                Some(format!(
                    "Key usage count ({}) exceeds maximum ({})",
                    metadata.use_count, max
                ))
            } else {
                None
            }
        }) {
            return Some(reason);
        }
        
        None
    }

    /// Get all keys that need rotation
    pub fn get_keys_needing_rotation(&self) -> Vec<(String, String)> {
        self.key_metadata
            .keys()
            .filter_map(|key_id| {
                self.needs_rotation(key_id)
                    .map(|reason| (key_id.clone(), reason))
            })
            .collect()
    }

    /// Perform key rotation
    pub fn rotate_key(
        &mut self,
        old_key_id: &str,
        new_key_id: String,
        new_security_level: String,
        reason: String,
    ) -> Result<RotationEvent> {
        let old_metadata = self.key_metadata.get(old_key_id)
            .ok_or_else(|| PQCError::InvalidKeyMaterial(format!("Key {} not found", old_key_id)))?
            .clone();
        
        // Create new key metadata
        let mut new_metadata = RotatableKeyMetadata::new(new_key_id.clone(), new_security_level);
        new_metadata.version = KeyVersion(old_metadata.version.as_u32() + 1);
        new_metadata.parent_key_id = Some(old_key_id.to_string());
        new_metadata.rotation_reason = Some(reason.clone());
        
        // Update old key state
        if let Some(old_meta) = self.key_metadata.get_mut(old_key_id) {
            old_meta.state = KeyState::Deprecated;
            old_meta.last_rotated_at = Some(new_metadata.created_at);
        }
        
        // Create rotation event
        let event = RotationEvent::new(
            old_key_id.to_string(),
            new_key_id.clone(),
            reason,
            old_metadata.version,
            new_metadata.version,
        );
        
        // Store new metadata
        self.key_metadata.insert(new_key_id, new_metadata);
        self.rotation_history.push(event.clone());
        
        Ok(event)
    }

    /// Record key usage
    pub fn record_usage(&mut self, key_id: &str) {
        if let Some(metadata) = self.key_metadata.get_mut(key_id) {
            metadata.record_use();
        }
    }

    /// Get key metadata
    pub fn get_metadata(&self, key_id: &str) -> Option<&RotatableKeyMetadata> {
        self.key_metadata.get(key_id)
    }

    /// Transition key state
    pub fn transition_key_state(&mut self, key_id: &str, new_state: KeyState) -> Result<()> {
        let metadata = self.key_metadata.get_mut(key_id)
            .ok_or_else(|| PQCError::InvalidKeyMaterial(format!("Key {} not found", key_id)))?;
        
        metadata.state = new_state;
        Ok(())
    }

    /// Get rotation history
    pub fn get_rotation_history(&self) -> &[RotationEvent] {
        &self.rotation_history
    }

    /// Clean up archived keys past retention
    pub fn cleanup_expired_keys(&mut self) -> Vec<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let expired_keys: Vec<String> = self.key_metadata
            .iter()
            .filter(|(_, meta)| {
                matches!(meta.state, KeyState::Archived) &&
                meta.last_rotated_at.map_or(false, |rotated| {
                    now - rotated > self.policy.archive_retention_seconds
                })
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        // Transition to ScheduledDeletion instead of removing
        for key_id in &expired_keys {
            if let Some(meta) = self.key_metadata.get_mut(key_id) {
                meta.state = KeyState::ScheduledDeletion;
            }
        }
        
        expired_keys
    }
}

impl Default for KeyRotationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Migration plan for transitioning to PQC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPlan {
    /// Migration phases
    pub phases: Vec<MigrationPhase>,
    /// Current phase index
    pub current_phase: usize,
    /// Migration started
    pub started_at: Option<u64>,
    /// Migration completed
    pub completed_at: Option<u64>,
}

/// A phase in the migration plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPhase {
    /// Phase name
    pub name: String,
    /// Phase description
    pub description: String,
    /// Target algorithm
    pub target_algorithm: String,
    /// Percentage of keys to migrate in this phase
    pub key_percentage: u8,
    /// Phase status
    pub status: MigrationPhaseStatus,
}

/// Status of a migration phase
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationPhaseStatus {
    /// Phase not started
    Pending,
    /// Phase in progress
    InProgress,
    /// Phase completed
    Completed,
    /// Phase failed
    Failed,
}

impl MigrationPlan {
    /// Create a standard migration plan to Kyber768
    pub fn to_kyber768() -> Self {
        Self {
            phases: vec![
                MigrationPhase {
                    name: "Pilot".to_string(),
                    description: "Migrate pilot users to hybrid X25519+Kyber768".to_string(),
                    target_algorithm: "hybrid_x25519_kyber768".to_string(),
                    key_percentage: 5,
                    status: MigrationPhaseStatus::Pending,
                },
                MigrationPhase {
                    name: "Early Adopters".to_string(),
                    description: "Migrate early adopters to hybrid mode".to_string(),
                    target_algorithm: "hybrid_x25519_kyber768".to_string(),
                    key_percentage: 25,
                    status: MigrationPhaseStatus::Pending,
                },
                MigrationPhase {
                    name: "General Availability".to_string(),
                    description: "Roll out hybrid mode to all users".to_string(),
                    target_algorithm: "hybrid_x25519_kyber768".to_string(),
                    key_percentage: 100,
                    status: MigrationPhaseStatus::Pending,
                },
            ],
            current_phase: 0,
            started_at: None,
            completed_at: None,
        }
    }

    /// Create a migration plan to pure Kyber
    pub fn to_pure_kyber() -> Self {
        Self {
            phases: vec![
                MigrationPhase {
                    name: "Hybrid Transition".to_string(),
                    description: "First transition to hybrid X25519+Kyber768".to_string(),
                    target_algorithm: "hybrid_x25519_kyber768".to_string(),
                    key_percentage: 100,
                    status: MigrationPhaseStatus::Pending,
                },
                MigrationPhase {
                    name: "Pure PQC".to_string(),
                    description: "Transition to pure Kyber768 for new keys".to_string(),
                    target_algorithm: "kyber768".to_string(),
                    key_percentage: 100,
                    status: MigrationPhaseStatus::Pending,
                },
            ],
            current_phase: 0,
            started_at: None,
            completed_at: None,
        }
    }

    /// Start the migration
    pub fn start(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        self.started_at = Some(now);
        if !self.phases.is_empty() {
            self.phases[0].status = MigrationPhaseStatus::InProgress;
        }
    }

    /// Advance to the next phase
    pub fn advance_phase(&mut self) -> Result<&MigrationPhase> {
        if self.current_phase >= self.phases.len() {
            return Err(PQCError::ConfigurationError("No more phases in migration plan".to_string()));
        }
        
        // Mark current phase as completed
        self.phases[self.current_phase].status = MigrationPhaseStatus::Completed;
        self.current_phase += 1;
        
        // Start next phase
        if self.current_phase < self.phases.len() {
            self.phases[self.current_phase].status = MigrationPhaseStatus::InProgress;
            Ok(&self.phases[self.current_phase])
        } else {
            // Migration complete
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            self.completed_at = Some(now);
            Err(PQCError::ConfigurationError("Migration complete".to_string()))
        }
    }

    /// Get current phase
    pub fn current_phase(&self) -> Option<&MigrationPhase> {
        self.phases.get(self.current_phase)
    }

    /// Get migration progress (0-100)
    pub fn progress(&self) -> u8 {
        if self.phases.is_empty() {
            return 100;
        }
        
        let completed = self.current_phase;
        let total = self.phases.len();
        ((completed * 100) / total) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_version() {
        let mut version = KeyVersion::new(1);
        assert_eq!(version.as_u32(), 1);
        
        version.increment();
        assert_eq!(version.as_u32(), 2);
    }

    #[test]
    fn test_rotation_policy_default() {
        let policy = RotationPolicy::default();
        assert!(policy.auto_rotate);
        assert!(policy.max_age_seconds > 0);
    }

    #[test]
    fn test_rotation_policy_strict() {
        let policy = RotationPolicy::strict();
        assert!(policy.max_age_seconds < RotationPolicy::default().max_age_seconds);
    }

    #[test]
    fn test_rotation_policy_should_rotate_by_age() {
        let policy = RotationPolicy::default();
        
        // New key should not need rotation
        let now = SystemTime::now();
        assert!(!policy.should_rotate_by_age(now));
        
        // Old key should need rotation
        let old = SystemTime::UNIX_EPOCH + Duration::from_secs(policy.max_age_seconds);
        assert!(policy.should_rotate_by_age(old));
    }

    #[test]
    fn test_rotatable_key_metadata() {
        let metadata = RotatableKeyMetadata::new(
            "key_123".to_string(),
            "kyber768".to_string(),
        );
        
        assert_eq!(metadata.key_id, "key_123");
        assert_eq!(metadata.version.as_u32(), 1);
        assert_eq!(metadata.state, KeyState::Active);
        assert_eq!(metadata.use_count, 0);
    }

    #[test]
    fn test_key_metadata_record_use() {
        let mut metadata = RotatableKeyMetadata::new(
            "key_123".to_string(),
            "kyber768".to_string(),
        );
        
        metadata.record_use();
        metadata.record_use();
        
        assert_eq!(metadata.use_count, 2);
    }

    #[test]
    fn test_key_rotation_manager() {
        let mut manager = KeyRotationManager::new();
        
        manager.register_key("key_1".to_string(), "kyber768".to_string());
        
        assert!(manager.get_metadata("key_1").is_some());
        assert!(manager.get_metadata("nonexistent").is_none());
    }

    #[test]
    fn test_key_rotation_manager_record_usage() {
        let mut manager = KeyRotationManager::new();
        manager.register_key("key_1".to_string(), "kyber768".to_string());
        
        manager.record_usage("key_1");
        manager.record_usage("key_1");
        
        let metadata = manager.get_metadata("key_1").unwrap();
        assert_eq!(metadata.use_count, 2);
    }

    #[test]
    fn test_key_rotation_manager_rotate() {
        let mut manager = KeyRotationManager::new();
        manager.register_key("key_1".to_string(), "kyber512".to_string());
        
        let event = manager.rotate_key(
            "key_1",
            "key_2".to_string(),
            "kyber768".to_string(),
            "Upgrade security level".to_string(),
        ).unwrap();
        
        assert_eq!(event.old_key_id, "key_1");
        assert_eq!(event.new_key_id, "key_2");
        
        // Old key should be deprecated
        let old_meta = manager.get_metadata("key_1").unwrap();
        assert_eq!(old_meta.state, KeyState::Deprecated);
        
        // New key should be active
        let new_meta = manager.get_metadata("key_2").unwrap();
        assert_eq!(new_meta.state, KeyState::Active);
        assert_eq!(new_meta.parent_key_id, Some("key_1".to_string()));
    }

    #[test]
    fn test_migration_plan_to_kyber768() {
        let plan = MigrationPlan::to_kyber768();
        
        assert_eq!(plan.phases.len(), 3);
        assert_eq!(plan.current_phase, 0);
        assert!(plan.started_at.is_none());
    }

    #[test]
    fn test_migration_plan_progress() {
        let mut plan = MigrationPlan::to_kyber768();
        
        assert_eq!(plan.progress(), 0);
        
        plan.start();
        assert_eq!(plan.progress(), 0);
        
        plan.advance_phase().ok();
        assert_eq!(plan.progress(), 33);
    }

    #[test]
    fn test_migration_plan_advance() {
        let mut plan = MigrationPlan::to_kyber768();
        plan.start();
        
        let phase = plan.current_phase().unwrap();
        assert_eq!(phase.status, MigrationPhaseStatus::InProgress);
        
        plan.advance_phase().ok();
        
        let phase = plan.current_phase().unwrap();
        assert_eq!(phase.name, "Early Adopters");
    }

    #[test]
    fn test_transition_key_state() {
        let mut manager = KeyRotationManager::new();
        manager.register_key("key_1".to_string(), "kyber768".to_string());
        
        manager.transition_key_state("key_1", KeyState::Deprecated).unwrap();
        
        let metadata = manager.get_metadata("key_1").unwrap();
        assert_eq!(metadata.state, KeyState::Deprecated);
    }

    #[test]
    fn test_cleanup_expired_keys() {
        let mut manager = KeyRotationManager::with_policy(RotationPolicy {
            archive_retention_seconds: 0,
            ..Default::default()
        });
        
        manager.register_key("key_1".to_string(), "kyber768".to_string());
        manager.transition_key_state("key_1", KeyState::Archived).unwrap();
        
        // Set last_rotated_at to simulate expired key
        if let Some(meta) = manager.key_metadata.get_mut("key_1") {
            meta.last_rotated_at = Some(0);
        }
        
        let expired = manager.cleanup_expired_keys();
        assert!(expired.contains(&"key_1".to_string()));
    }
}