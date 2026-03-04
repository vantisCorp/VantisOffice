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
pub mod notifications;
pub mod scheduling;
pub mod sync;
pub mod ui;

pub use core::{Calendar, Color, DateRange, Event, EventQuery, Recurrence, Reminder, TimeRange};
pub use encryption::{
    EncryptionLevel, EventEncryption, InvitationSystem, KeyManager, SharingSystem,
};
pub use notifications::{
    ConflictAlertManager, InvitationManager, NotificationManager, ReminderManager,
};
pub use scheduling::{
    Availability, ConflictDetector, ScheduleOptimizer, Suggester, SuggestionCriteria,
};
pub use sync::{ConflictResolver, EventMerger, ExternalSync, FlowSync, LinkSync};
pub use ui::{AgendaView, CalendarView, EventEditor, TimelineView, ViewSettings, ViewType};

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
