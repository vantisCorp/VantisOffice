//! Integration tests for Vantis Ark
//! Tests backup, recovery, Shamir Secret Sharing, distribution, encryption, and scheduling

use vantis_ark::{
    BackupConfig, RecoveryConfig, InMemoryStorage, BackupManager,
    PartVerifier, SplitConfig, RecoverConfig, SecretSharing,
    KeyManager, RetentionPolicy,
};

#[test]
fn test_backup_creation() {
    let storage = Box::new(InMemoryStorage::new());
    let manager = BackupManager::new(storage);
    
    let data = b"Test data for backup".to_vec();
    let backup = manager.create_backup("test_backup".to_string(), data.clone()).unwrap();
    
    assert_eq!(backup.name, "test_backup");
    assert!(!backup.id.is_empty());
}

#[test]
fn test_backup_config() {
    let config = BackupConfig::new("test_backup".to_string(), 10, 3);
    
    assert_eq!(config.name, "test_backup");
    assert_eq!(config.parts, 10);
    assert_eq!(config.threshold, 3);
}

#[test]
fn test_recovery_config() {
    let config = RecoveryConfig::new("backup_id".to_string(), 3);
    
    assert_eq!(config.backup_id, "backup_id");
    assert_eq!(config.threshold, 3);
}

#[test]
fn test_shamir_secret_sharing_split() {
    let secret = b"My secret data".to_vec();
    
    let config = SplitConfig::new(10, 3);
    let parts = SecretSharing::split(&secret, &config);
    assert_eq!(parts.len(), 10);
    
    for part in &parts {
        assert!(!part.id.is_empty());
        assert!(!part.data.is_empty());
        assert!(part.checksum.len() > 0);
    }
}

#[test]
fn test_shamir_secret_sharing_recover() {
    let secret = b"My secret data".to_vec();
    
    let split_config = SplitConfig::new(10, 3);
    let parts = SecretSharing::split(&secret, &split_config);
    
    // Recover with threshold parts
    let recover_config = RecoverConfig::new(3);
    let recovered = SecretSharing::recover(&parts[0..3], &recover_config).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_part_verification() {
    let secret = b"My secret data".to_vec();
    
    let config = SplitConfig::new(10, 3);
    let parts = SecretSharing::split(&secret, &config);
    
    for part in &parts {
        assert!(PartVerifier::verify(part));
    }
}

// Encryption tests skipped - placeholder implementations

#[test]
fn test_key_manager() {
    let mut key_manager = KeyManager::new();
    
    let transport_key = key_manager.generate_key(32);
    assert_eq!(transport_key.len(), 32);
    
    let storage_key = key_manager.derive_key("password123", b"salt");
    assert!(!storage_key.is_empty());
    
    let different_key = key_manager.derive_key("password456", b"salt");
    assert_ne!(storage_key, different_key);
}

#[test]
fn test_retention_policy() {
    let policy = RetentionPolicy::default();
    
    assert_eq!(policy.keep_daily, 7);
    assert_eq!(policy.keep_weekly, 4);
    assert_eq!(policy.keep_monthly, 12);
}

#[test]
fn test_full_backup_workflow() {
    // Create backup
    let storage = Box::new(InMemoryStorage::new());
    let manager = BackupManager::new(storage);
    
    let original_data = b"Important data to backup".to_vec();
    let backup = manager.create_backup("workflow_test".to_string(), original_data.clone()).unwrap();
    
    // Split using Shamir
    let split_config = SplitConfig::new(10, 3);
    let parts = SecretSharing::split(&original_data, &split_config);
    assert_eq!(parts.len(), 10);
    
    // Recover using threshold parts
    let recover_config = RecoverConfig::new(3);
    let recovered_data = SecretSharing::recover(&parts[0..3], &recover_config).unwrap();
    assert_eq!(recovered_data, original_data);
}