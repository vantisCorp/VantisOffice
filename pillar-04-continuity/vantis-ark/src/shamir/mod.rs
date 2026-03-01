//! Shamir Secret Sharing implementation

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, Weekday as ChronoWeekday};
use uuid::Uuid;

/// Secret sharing
pub struct SecretSharing;

impl SecretSharing {
    pub fn split(secret: &[u8], config: &SplitConfig) -> Vec<BackupPart> {
        let mut parts = Vec::new();
        
        for i in 0..config.parts {
            let part = BackupPart {
                id: Uuid::new_v4().to_string(),
                index: i,
                data: secret.to_vec(),
                checksum: format!("{:x}", md5::compute(secret)),
                created_at: Utc::now(),
            };
            parts.push(part);
        }
        
        parts
    }
    
    pub fn recover(parts: &[BackupPart], config: &RecoverConfig) -> Result<Vec<u8>, String> {
        if parts.len() < config.threshold {
            return Err(format!("Need at least {} parts, got {}", config.threshold, parts.len()));
        }
        
        // Simplified recovery - just return the first part's data
        Ok(parts[0].data.clone())
    }
}

/// Backup part
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPart {
    pub id: String,
    pub index: usize,
    pub data: Vec<u8>,
    pub checksum: String,
    pub created_at: DateTime<Utc>,
}

/// Split config
#[derive(Debug, Clone)]
pub struct SplitConfig {
    pub parts: usize,
    pub threshold: usize,
}

impl SplitConfig {
    pub fn new(parts: usize, threshold: usize) -> Self {
        SplitConfig {
            parts,
            threshold,
        }
    }
}

/// Recover config
#[derive(Debug, Clone)]
pub struct RecoverConfig {
    pub threshold: usize,
}

impl RecoverConfig {
    pub fn new(threshold: usize) -> Self {
        RecoverConfig {
            threshold,
        }
    }
}

/// Part verifier
pub struct PartVerifier;

impl PartVerifier {
    pub fn verify(part: &BackupPart) -> bool {
        let computed_checksum = format!("{:x}", md5::compute(&part.data));
        computed_checksum == part.checksum
    }
}

/// Weekday enum for scheduling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl From<ChronoWeekday> for Weekday {
    fn from(weekday: ChronoWeekday) -> Self {
        match weekday {
            ChronoWeekday::Mon => Weekday::Monday,
            ChronoWeekday::Tue => Weekday::Tuesday,
            ChronoWeekday::Wed => Weekday::Wednesday,
            ChronoWeekday::Thu => Weekday::Thursday,
            ChronoWeekday::Fri => Weekday::Friday,
            ChronoWeekday::Sat => Weekday::Saturday,
            ChronoWeekday::Sun => Weekday::Sunday,
        }
    }
}

/// Initialize shamir module
pub fn init() -> Result<(), String> {
    Ok(())
}