//! Vantis Chronos - Privacy-first calendar with PGP encryption
//!
//! Features:
//! - PGP encryption for all calendar data
//! - Meeting suggestions with AI
//! - Conflict detection and resolution
//! - Multiple calendar views
//! - ICS import/export
//! - Privacy-first design

pub mod core;
pub mod encryption;
pub mod scheduling;
pub mod sync;
pub mod ui;
pub mod notifications;

pub use core::{Calendar, Event, Recurrence, Reminder, EventQuery, DateRange, TimeRange, Color};
pub use encryption::{EventEncryption, KeyManager, InvitationSystem, SharingSystem, EncryptionLevel};
pub use scheduling::{Suggester, ConflictDetector, Availability, ScheduleOptimizer, SuggestionCriteria};
pub use sync::{FlowSync, LinkSync, ExternalSync, EventMerger, ConflictResolver};
pub use ui::{CalendarView, ViewType, EventEditor, TimelineView, AgendaView, ViewSettings};
pub use notifications::{NotificationManager, ReminderManager, InvitationManager, ConflictAlertManager};

/// Vantis Chronos version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize Vantis Chronos
pub fn init() -> Result<(), ChronosError> {
    // Initialize subsystems
    core::init()?;
    encryption::init()?;
    scheduling::init()?;
    
    Ok(())
}

/// Chronos-specific errors
#[derive(Debug, thiserror::Error)]
pub enum ChronosError {
    #[error("Event error: {0}")]
    Event(String),
    
    #[error("Calendar error: {0}")]
    Calendar(String),
    
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("Scheduling error: {0}")]
    Scheduling(String),
    
    #[error("Sync error: {0}")]
    Sync(String),
    
    #[error("UI error: {0}")]
    Ui(String),
    
    #[error("Notification error: {0}")]
    Notification(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("General error: {0}")]
    General(String),
}

impl From<String> for ChronosError {
    fn from(s: String) -> Self {
        ChronosError::General(s)
    }
}