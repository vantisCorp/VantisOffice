//! Vantis Ark - Distributed backup with Shamir Secret Sharing
//!
//! Features:
//! - Shamir Secret Sharing for distributed encryption
//! - Automatic backup scheduling
//! - Health monitoring of backup stations
//! - Multiple storage backends
//! - Recovery from partial backups

pub mod core;
pub mod distribution;
pub mod encryption;
pub mod scheduler;
pub mod shamir;
pub mod ui;

pub use core::{
    Backup, BackupConfig, BackupManager, InMemoryStorage, Recovery, RecoveryConfig, StorageBackend,
};
pub use distribution::{
    DistributionConfig, Distributor, HealthMonitor, HealthStatus, Station, StationManager,
    TrustLevel,
};
pub use encryption::{
    KeyManager, StorageAlgorithm, StorageEncryption, TransportAlgorithm, TransportEncryption,
};
pub use scheduler::{
    BackupScheduler, BackupTask, RetentionPolicy, Schedule, ScheduleConfig, ScheduleFrequency,
    TaskConfig, TaskResult,
};
pub use shamir::{BackupPart, PartVerifier, RecoverConfig, SecretSharing, SplitConfig, Weekday};
pub use ui::{BackupStatus, Dashboard, RecoveryStatusUI, RecoveryUI, SettingsUI, StatusDisplay};

/// Vantis Ark version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize Vantis Ark
pub fn init() -> Result<(), ArkError> {
    // Initialize subsystems
    core::init()?;
    shamir::init()?;
    distribution::init()?;

    Ok(())
}

/// Ark-specific errors
#[derive(Debug, thiserror::Error)]
pub enum ArkError {
    #[error("Backup error: {0}")]
    Backup(String),

    #[error("Recovery error: {0}")]
    Recovery(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Distribution error: {0}")]
    Distribution(String),

    #[error("Scheduler error: {0}")]
    Scheduler(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("General error: {0}")]
    General(String),
}

impl From<String> for ArkError {
    fn from(s: String) -> Self {
        ArkError::General(s)
    }
}
