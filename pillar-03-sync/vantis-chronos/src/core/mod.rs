//! Core data structures for Vantis Chronos

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc, NaiveDate, NaiveTime, Duration};
use uuid::Uuid;

/// Calendar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Color,
    pub timezone: String,
    pub events: Vec<Event>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Calendar {
    pub fn new(name: String, color: Color) -> Self {
        let now = Utc::now();
        Calendar {
            id: Uuid::new_v4().to_string(),
            name,
            description: None,
            color,
            timezone: "UTC".to_string(),
            events: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
        self.updated_at = Utc::now();
    }
    
    pub fn remove_event(&mut self, event_id: &str) -> Option<Event> {
        let pos = self.events.iter().position(|e| e.id == event_id)?;
        self.updated_at = Utc::now();
        Some(self.events.remove(pos))
    }
    
    pub fn get_event(&self, event_id: &str) -> Option<&Event> {
        self.events.iter().find(|e| e.id == event_id)
    }
    
    pub fn query_events(&self, query: &EventQuery) -> Vec<&Event> {
        self.events.iter()
            .filter(|e| query.matches(e))
            .collect()
    }
}

/// Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub all_day: bool,
    pub location: Option<String>,
    pub attendees: Vec<String>,
    pub recurrence: Option<Recurrence>,
    pub reminders: Vec<Reminder>,
    pub color: Option<Color>,
    pub status: EventStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Event {
    pub fn new(title: String, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        let now = Utc::now();
        Event {
            id: Uuid::new_v4().to_string(),
            title,
            description: None,
            start,
            end,
            all_day: false,
            location: None,
            attendees: Vec::new(),
            recurrence: None,
            reminders: Vec::new(),
            color: None,
            status: EventStatus::Confirmed,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn duration(&self) -> Duration {
        self.end.signed_duration_since(self.start)
    }
    
    pub fn overlaps(&self, other: &Event) -> bool {
        self.start < other.end && self.end > other.start
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventStatus {
    Tentative,
    Confirmed,
    Cancelled,
}

/// Recurrence pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recurrence {
    pub pattern: RecurrencePattern,
    pub interval: u32,
    pub until: Option<DateTime<Utc>>,
    pub count: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurrencePattern {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Reminder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub minutes_before: u32,
    pub method: ReminderMethod,
    pub sent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReminderMethod {
    Email,
    Notification,
    SMS,
}

/// Event query
#[derive(Debug, Clone)]
pub struct EventQuery {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub title_contains: Option<String>,
    pub status: Option<EventStatus>,
}

impl EventQuery {
    pub fn new() -> Self {
        EventQuery {
            start_date: None,
            end_date: None,
            title_contains: None,
            status: None,
        }
    }
    
    pub fn matches(&self, event: &Event) -> bool {
        if let Some(start) = self.start_date {
            if event.end < start {
                return false;
            }
        }
        
        if let Some(end) = self.end_date {
            if event.start > end {
                return false;
            }
        }
        
        if let Some(title) = &self.title_contains {
            if !event.title.to_lowercase().contains(&title.to_lowercase()) {
                return false;
            }
        }
        
        if let Some(status) = self.status {
            if event.status != status {
                return false;
            }
        }
        
        true
    }
}

/// Date range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

/// Time range
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeRange {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

/// Color
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }
    
    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

/// Initialize core module
pub fn init() -> Result<(), String> {
    Ok(())
}