//! UI module for dashboard and status display

use crate::core::{Backup, Recovery};
use chrono::{DateTime, Utc};

/// Dashboard
#[derive(Debug, Clone)]
pub struct Dashboard {
    pub statistics: Statistics,
    pub recent_backups: Vec<Backup>,
    pub recent_recoveries: Vec<Recovery>,
}

#[derive(Debug, Clone)]
pub struct Statistics {
    pub total_backups: usize,
    pub total_size: usize,
    pub healthy_stations: usize,
    pub pending_tasks: usize,
}

impl Dashboard {
    pub fn new() -> Self {
        Dashboard {
            statistics: Statistics {
                total_backups: 0,
                total_size: 0,
                healthy_stations: 0,
                pending_tasks: 0,
            },
            recent_backups: Vec::new(),
            recent_recoveries: Vec::new(),
        }
    }
    
    pub fn render(&self) -> String {
        format!(
            "=== Vantis Ark Dashboard ===\n\
             Total Backups: {}\n\
             Total Size: {} MB\n\
             Healthy Stations: {}\n\
             Pending Tasks: {}\n",
            self.statistics.total_backups,
            self.statistics.total_size / (1024 * 1024),
            self.statistics.healthy_stations,
            self.statistics.pending_tasks
        )
    }
}

/// Status display
#[derive(Debug, Clone)]
pub struct StatusDisplay {
    pub status: BackupStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupStatus {
    Idle,
    Running,
    Completed,
    Failed,
}

impl StatusDisplay {
    pub fn new(status: BackupStatus) -> Self {
        StatusDisplay {
            status,
        }
    }
    
    pub fn render(&self) -> String {
        match self.status {
            BackupStatus::Idle => "Status: Idle".to_string(),
            BackupStatus::Running => "Status: Running...".to_string(),
            BackupStatus::Completed => "Status: Completed ✓".to_string(),
            BackupStatus::Failed => "Status: Failed ✗".to_string(),
        }
    }
}

/// Recovery UI
#[derive(Debug, Clone)]
pub struct RecoveryUI {
    pub status: RecoveryStatusUI,
    pub progress: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryStatusUI {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}

impl RecoveryUI {
    pub fn new() -> Self {
        RecoveryUI {
            status: RecoveryStatusUI::NotStarted,
            progress: 0.0,
        }
    }
    
    pub fn render(&self) -> String {
        format!(
            "Recovery Status: {:?}\nProgress: {:.1}%",
            self.status,
            self.progress * 100.0
        )
    }
}

/// Settings UI
#[derive(Debug, Clone)]
pub struct SettingsUI {
    pub backup_parts: usize,
    pub backup_threshold: usize,
    pub replication_factor: usize,
}

impl SettingsUI {
    pub fn new() -> Self {
        SettingsUI {
            backup_parts: 10,
            backup_threshold: 3,
            replication_factor: 3,
        }
    }
    
    pub fn render(&self) -> String {
        format!(
            "=== Backup Settings ===\n\
             Parts: {}\n\
             Threshold: {}\n\
             Replication Factor: {}",
            self.backup_parts,
            self.backup_threshold,
            self.replication_factor
        )
    }
}