//! Comprehensive tests for Vantis Chronos module
//!
//! Tests cover:
//! - Calendar operations (create, events, query)
//! - Event operations (create, duration, overlap, status)
//! - Recurrence patterns
//! - Reminders
//! - Event query filtering

use vantis_chronos::{
    core::*,
    ChronosError,
};
use chrono::{Utc, Duration, NaiveDate, NaiveTime};
use uuid::Uuid;

// ============================================================================
// Calendar Tests
// ============================================================================

#[test]
fn test_calendar_creation() {
    let calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    assert_eq!(calendar.name, "Work Calendar");
    assert_eq!(calendar.color, Color::new(0, 123, 255));
    assert_eq!(calendar.timezone, "UTC");
    assert!(calendar.events.is_empty());
}

#[test]
fn test_calendar_add_event() {
    let mut calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    let now = Utc::now();
    let event = Event::new(
        "Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    calendar.add_event(event);
    
    assert_eq!(calendar.events.len(), 1);
    assert_eq!(calendar.events[0].title, "Meeting");
}

#[test]
fn test_calendar_remove_event() {
    let mut calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    let now = Utc::now();
    let event = Event::new(
        "Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    let event_id = event.id.clone();
    
    calendar.add_event(event);
    let removed = calendar.remove_event(&event_id);
    
    assert!(removed.is_some());
    assert_eq!(calendar.events.len(), 0);
}

#[test]
fn test_calendar_remove_nonexistent_event() {
    let mut calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    let removed = calendar.remove_event("nonexistent");
    
    assert!(removed.is_none());
}

#[test]
fn test_calendar_get_event() {
    let mut calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    let now = Utc::now();
    let event = Event::new(
        "Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    let event_id = event.id.clone();
    
    calendar.add_event(event);
    
    let retrieved = calendar.get_event(&event_id);
    
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().title, "Meeting");
}

#[test]
fn test_calendar_query_events_empty() {
    let calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    let query = EventQuery::new();
    let results = calendar.query_events(&query);
    
    assert!(results.is_empty());
}

#[test]
fn test_calendar_query_events_by_title() {
    let mut calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    let now = Utc::now();
    calendar.add_event(Event::new(
        "Team Meeting".to_string(),
        now,
        now + Duration::hours(1)
    ));
    calendar.add_event(Event::new(
        "Lunch Break".to_string(),
        now + Duration::hours(2),
        now + Duration::hours(3)
    ));
    
    let mut query = EventQuery::new();
    query.title_contains = Some("Meeting".to_string());
    
    let results = calendar.query_events(&query);
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Team Meeting");
}

#[test]
fn test_calendar_query_events_by_status() {
    let mut calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    let now = Utc::now();
    
    let mut event1 = Event::new(
        "Confirmed Event".to_string(),
        now,
        now + Duration::hours(1)
    );
    event1.status = EventStatus::Confirmed;
    
    let mut event2 = Event::new(
        "Cancelled Event".to_string(),
        now + Duration::hours(2),
        now + Duration::hours(3)
    );
    event2.status = EventStatus::Cancelled;
    
    calendar.add_event(event1);
    calendar.add_event(event2);
    
    let mut query = EventQuery::new();
    query.status = Some(EventStatus::Confirmed);
    
    let results = calendar.query_events(&query);
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, EventStatus::Confirmed);
}

#[test]
fn test_calendar_query_events_by_date_range() {
    let mut calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    let now = Utc::now();
    calendar.add_event(Event::new(
        "Today's Event".to_string(),
        now,
        now + Duration::hours(1)
    ));
    calendar.add_event(Event::new(
        "Tomorrow's Event".to_string(),
        now + Duration::days(1),
        now + Duration::days(1) + Duration::hours(1)
    ));
    
    let mut query = EventQuery::new();
    query.start_date = Some(now);
    query.end_date = Some(now + Duration::hours(2));
    
    let results = calendar.query_events(&query);
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Today's Event");
}

// ============================================================================
// Event Tests
// ============================================================================

#[test]
fn test_event_creation() {
    let now = Utc::now();
    let event = Event::new(
        "Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    assert_eq!(event.title, "Meeting");
    assert_eq!(event.status, EventStatus::Confirmed);
    assert!(!event.all_day);
    assert!(event.attendees.is_empty());
    assert!(event.reminders.is_empty());
}

#[test]
fn test_event_duration() {
    let now = Utc::now();
    let event = Event::new(
        "Meeting".to_string(),
        now,
        now + Duration::hours(2)
    );
    
    assert_eq!(event.duration(), Duration::hours(2));
}

#[test]
fn test_event_overlaps() {
    let now = Utc::now();
    let event1 = Event::new(
        "Event 1".to_string(),
        now,
        now + Duration::hours(2)
    );
    
    let event2 = Event::new(
        "Event 2".to_string(),
        now + Duration::hours(1),
        now + Duration::hours(3)
    );
    
    assert!(event1.overlaps(&event2));
    assert!(event2.overlaps(&event1));
}

#[test]
fn test_event_no_overlap() {
    let now = Utc::now();
    let event1 = Event::new(
        "Event 1".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    let event2 = Event::new(
        "Event 2".to_string(),
        now + Duration::hours(2),
        now + Duration::hours(3)
    );
    
    assert!(!event1.overlaps(&event2));
    assert!(!event2.overlaps(&event1));
}

#[test]
fn test_event_edge_case_overlap() {
    let now = Utc::now();
    let event1 = Event::new(
        "Event 1".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    let event2 = Event::new(
        "Event 2".to_string(),
        now + Duration::hours(1),
        now + Duration::hours(2)
    );
    
    // Events that just touch don't overlap
    assert!(!event1.overlaps(&event2));
    assert!(!event2.overlaps(&event1));
}

// ============================================================================
// Event Status Tests
// ============================================================================

#[test]
fn test_event_status_tentative() {
    let mut event = Event::new(
        "Meeting".to_string(),
        Utc::now(),
        Utc::now() + Duration::hours(1)
    );
    
    event.status = EventStatus::Tentative;
    assert_eq!(event.status, EventStatus::Tentative);
}

#[test]
fn test_event_status_confirmed() {
    let mut event = Event::new(
        "Meeting".to_string(),
        Utc::now(),
        Utc::now() + Duration::hours(1)
    );
    
    event.status = EventStatus::Confirmed;
    assert_eq!(event.status, EventStatus::Confirmed);
}

#[test]
fn test_event_status_cancelled() {
    let mut event = Event::new(
        "Meeting".to_string(),
        Utc::now(),
        Utc::now() + Duration::hours(1)
    );
    
    event.status = EventStatus::Cancelled;
    assert_eq!(event.status, EventStatus::Cancelled);
}

// ============================================================================
// Recurrence Tests
// ============================================================================

#[test]
fn test_recurrence_daily() {
    let now = Utc::now();
    let recurrence = Recurrence {
        pattern: RecurrencePattern::Daily,
        interval: 1,
        until: Some(now + Duration::days(7)),
        count: None,
    };
    
    assert_eq!(recurrence.pattern, RecurrencePattern::Daily);
    assert_eq!(recurrence.interval, 1);
}

#[test]
fn test_recurrence_weekly() {
    let recurrence = Recurrence {
        pattern: RecurrencePattern::Weekly,
        interval: 1,
        until: None,
        count: Some(52),
    };
    
    assert_eq!(recurrence.pattern, RecurrencePattern::Weekly);
    assert_eq!(recurrence.count, Some(52));
}

#[test]
fn test_recurrence_monthly() {
    let recurrence = Recurrence {
        pattern: RecurrencePattern::Monthly,
        interval: 1,
        until: None,
        count: Some(12),
    };
    
    assert_eq!(recurrence.pattern, RecurrencePattern::Monthly);
}

#[test]
fn test_recurrence_yearly() {
    let recurrence = Recurrence {
        pattern: RecurrencePattern::Yearly,
        interval: 1,
        until: None,
        count: None,
    };
    
    assert_eq!(recurrence.pattern, RecurrencePattern::Yearly);
}

// ============================================================================
// Reminder Tests
// ============================================================================

#[test]
fn test_reminder_creation() {
    let reminder = Reminder {
        id: Uuid::new_v4().to_string(),
        minutes_before: 15,
        method: ReminderMethod::Email,
        sent: false,
    };
    
    assert_eq!(reminder.minutes_before, 15);
    assert_eq!(reminder.method, ReminderMethod::Email);
    assert!(!reminder.sent);
}

#[test]
fn test_reminder_method_notification() {
    let reminder = Reminder {
        id: Uuid::new_v4().to_string(),
        minutes_before: 5,
        method: ReminderMethod::Notification,
        sent: false,
    };
    
    assert_eq!(reminder.method, ReminderMethod::Notification);
}

#[test]
fn test_reminder_method_sms() {
    let reminder = Reminder {
        id: Uuid::new_v4().to_string(),
        minutes_before: 30,
        method: ReminderMethod::SMS,
        sent: false,
    };
    
    assert_eq!(reminder.method, ReminderMethod::SMS);
}

// ============================================================================
// Event Query Tests
// ============================================================================

#[test]
fn test_event_query_new() {
    let query = EventQuery::new();
    
    assert!(query.start_date.is_none());
    assert!(query.end_date.is_none());
    assert!(query.title_contains.is_none());
    assert!(query.status.is_none());
}

#[test]
fn test_event_query_with_date_range() {
    let now = Utc::now();
    let query = EventQuery {
        start_date: Some(now),
        end_date: Some(now + Duration::days(1)),
        title_contains: None,
        status: None,
    };
    
    assert!(query.start_date.is_some());
    assert!(query.end_date.is_some());
}

#[test]
fn test_event_query_matches_by_title() {
    let now = Utc::now();
    let event = Event::new(
        "Team Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    let mut query = EventQuery::new();
    query.title_contains = Some("team".to_string());
    
    assert!(query.matches(&event));
}

#[test]
fn test_event_query_not_match_by_title() {
    let now = Utc::now();
    let event = Event::new(
        "Lunch".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    let mut query = EventQuery::new();
    query.title_contains = Some("meeting".to_string());
    
    assert!(!query.matches(&event));
}

#[test]
fn test_event_query_case_insensitive_title() {
    let now = Utc::now();
    let event = Event::new(
        "Team Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    let mut query = EventQuery::new();
    query.title_contains = Some("TEAM MEETING".to_string());
    
    assert!(query.matches(&event));
}

#[test]
fn test_event_query_matches_by_status() {
    let now = Utc::now();
    let mut event = Event::new(
        "Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    event.status = EventStatus::Confirmed;
    
    let mut query = EventQuery::new();
    query.status = Some(EventStatus::Confirmed);
    
    assert!(query.matches(&event));
}

#[test]
fn test_event_query_not_match_by_status() {
    let now = Utc::now();
    let mut event = Event::new(
        "Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    event.status = EventStatus::Cancelled;
    
    let mut query = EventQuery::new();
    query.status = Some(EventStatus::Confirmed);
    
    assert!(!query.matches(&event));
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_complete_calendar_workflow() {
    // Create calendar
    let mut calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    // Add events
    let now = Utc::now();
    
    let morning_meeting = Event::new(
        "Morning Standup".to_string(),
        now,
        now + Duration::minutes(30)
    );
    
    let lunch = Event::new(
        "Lunch Break".to_string(),
        now + Duration::hours(1),
        now + Duration::hours(2)
    );
    
    calendar.add_event(morning_meeting);
    calendar.add_event(lunch);
    
    // Verify events were added
    assert_eq!(calendar.events.len(), 2);
    
    // Query events
    let mut query = EventQuery::new();
    query.title_contains = Some("Standup".to_string());
    
    let results = calendar.query_events(&query);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Morning Standup");
}

#[test]
fn test_calendar_with_multiple_events() {
    let mut calendar = Calendar::new(
        "Personal Calendar".to_string(),
        Color::new(0, 200, 83)
    );
    
    let now = Utc::now();
    let base_date = now.date_naive();
    
    // Add events for the week
    for day in 0..7 {
        let event_start = (base_date + Duration::days(day))
            .and_time(NaiveTime::from_hms_opt(9, 0, 0).unwrap())
            .and_utc();
        
        let event_end = (base_date + Duration::days(day))
            .and_time(NaiveTime::from_hms_opt(10, 0, 0).unwrap())
            .and_utc();
        
        let event = Event::new(
            format!("Daily Task - Day {}", day + 1),
            event_start,
            event_end
        );
        
        calendar.add_event(event);
    }
    
    assert_eq!(calendar.events.len(), 7);
    
    // Query for a specific day
    let day2_start = (base_date + Duration::days(2))
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .and_utc();
    
    let day2_end = (base_date + Duration::days(2))
        .and_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap())
        .and_utc();
    
    let mut query = EventQuery::new();
    query.start_date = Some(day2_start);
    query.end_date = Some(day2_end);
    
    let results = calendar.query_events(&query);
    assert_eq!(results.len(), 1);
}

#[test]
fn test_event_with_attendees() {
    let mut calendar = Calendar::new(
        "Team Calendar".to_string(),
        Color::new(156, 39, 176)
    );
    
    let now = Utc::now();
    let mut event = Event::new(
        "Team Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    event.attendees = vec![
        "alice@example.com".to_string(),
        "bob@example.com".to_string(),
        "charlie@example.com".to_string(),
    ];
    
    calendar.add_event(event);
    
    assert_eq!(calendar.events[0].attendees.len(), 3);
}

#[test]
fn test_event_with_recurrence() {
    let now = Utc::now();
    let mut event = Event::new(
        "Weekly Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    event.recurrence = Some(Recurrence {
        pattern: RecurrencePattern::Weekly,
        interval: 1,
        until: Some(now + Duration::weeks(12)),
        count: None,
    });
    
    assert!(event.recurrence.is_some());
    assert_eq!(event.recurrence.unwrap().pattern, RecurrencePattern::Weekly);
}

#[test]
fn test_event_with_reminders() {
    let now = Utc::now();
    let mut event = Event::new(
        "Important Meeting".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    event.reminders = vec![
        Reminder {
            id: Uuid::new_v4().to_string(),
            minutes_before: 15,
            method: ReminderMethod::Notification,
            sent: false,
        },
        Reminder {
            id: Uuid::new_v4().to_string(),
            minutes_before: 60,
            method: ReminderMethod::Email,
            sent: false,
        },
    ];
    
    assert_eq!(event.reminders.len(), 2);
}

#[test]
fn test_calendar_timezone() {
    let mut calendar = Calendar::new(
        "Work Calendar".to_string(),
        Color::new(0, 123, 255)
    );
    
    calendar.timezone = "America/New_York".to_string();
    
    assert_eq!(calendar.timezone, "America/New_York");
}

#[test]
fn test_event_all_day() {
    let now = Utc::now();
    let mut event = Event::new(
        "Birthday".to_string(),
        now,
        now + Duration::days(1)
    );
    
    event.all_day = true;
    
    assert!(event.all_day);
}

#[test]
fn test_event_with_location() {
    let now = Utc::now();
    let mut event = Event::new(
        "Conference".to_string(),
        now,
        now + Duration::hours(8)
    );
    
    event.location = Some("Convention Center".to_string());
    
    assert_eq!(event.location, Some("Convention Center".to_string()));
}

#[test]
fn test_event_with_color() {
    let now = Utc::now();
    let mut event = Event::new(
        "Important".to_string(),
        now,
        now + Duration::hours(1)
    );
    
    event.color = Some(Color::new(255, 82, 82));
    
    assert_eq!(event.color, Some(Color::new(255, 82, 82)));
}