// Unit tests for Vantis Chronos
use vantis_chronos::{
    Calendar, Event, Recurrence, Reminder, EventQuery, DateRange, TimeRange, Color,
    EventEncryption, KeyManager, InvitationSystem, SharingSystem, EncryptionLevel,
    Suggester, ConflictDetector, Availability, ScheduleOptimizer, SuggestionCriteria,
    ChronosError
};
use chrono::{Utc, Duration, NaiveDateTime, NaiveDate, NaiveTime};
use uuid::Uuid;

#[test]
fn test_calendar_creation() {
    let calendar = Calendar::new("Test Calendar", Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    assert_eq!(calendar.name(), "Test Calendar");
    assert!(calendar.events().is_empty());
}

#[test]
fn test_event_creation() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    
    let event = Event::new(
        "Test Event",
        start,
        end,
        Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }
    );
    
    assert_eq!(event.title(), "Test Event");
    assert_eq!(event.start(), &start);
    assert_eq!(event.end(), &end);
    assert!(!event.all_day());
}

#[test]
fn test_all_day_event() {
    let date = NaiveDate::from_ymd_opt(2025, 3, 3).unwrap();
    
    let event = Event::all_day(
        "All Day Event",
        date,
        Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 }
    );
    
    assert_eq!(event.title(), "All Day Event");
    assert!(event.all_day());
}

#[test]
fn test_calendar_add_event() {
    let mut calendar = Calendar::new("Test Calendar", Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let event = Event::new("Test Event", start, end, Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 });
    
    calendar.add_event(event).unwrap();
    assert_eq!(calendar.events().len(), 1);
}

#[test]
fn test_calendar_remove_event() {
    let mut calendar = Calendar::new("Test Calendar", Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let event = Event::new("Test Event", start, end, Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 });
    
    let event_id = calendar.add_event(event).unwrap();
    assert_eq!(calendar.events().len(), 1);
    
    calendar.remove_event(event_id).unwrap();
    assert!(calendar.events().is_empty());
}

#[test]
fn test_event_query_by_date_range() {
    let mut calendar = Calendar::new("Test Calendar", Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    let now = Utc::now();
    
    let event1 = Event::new(
        "Event 1",
        now - Duration::days(2),
        now - Duration::days(1),
        Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    );
    
    let event2 = Event::new(
        "Event 2",
        now,
        now + Duration::hours(1),
        Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }
    );
    
    let event3 = Event::new(
        "Event 3",
        now + Duration::days(1),
        now + Duration::days(2),
        Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }
    );
    
    calendar.add_event(event1).unwrap();
    calendar.add_event(event2).unwrap();
    calendar.add_event(event3).unwrap();
    
    let query = EventQuery::date_range(
        now - Duration::days(1),
        now + Duration::days(1)
    );
    
    let results = calendar.query(&query);
    assert_eq!(results.len(), 2); // event2 and event3
}

#[test]
fn test_recurring_event_creation() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    
    let recurrence = Recurrence::daily();
    let event = Event::with_recurrence(
        "Recurring Event",
        start,
        end,
        recurrence,
        Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    );
    
    assert_eq!(event.title(), "Recurring Event");
    assert!(event.is_recurring());
}

#[test]
fn test_weekly_recurrence() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    
    let recurrence = Recurrence::weekly();
    let event = Event::with_recurrence(
        "Weekly Event",
        start,
        end,
        recurrence,
        Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    );
    
    let occurrences = event.occurrences(start, start + Duration::weeks(4));
    assert_eq!(occurrences.len(), 5); // Original + 4 weeks
}

#[test]
fn test_monthly_recurrence() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    
    let recurrence = Recurrence::monthly();
    let event = Event::with_recurrence(
        "Monthly Event",
        start,
        end,
        recurrence,
        Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    );
    
    let occurrences = event.occurrences(start, start + Duration::days(90));
    assert_eq!(occurrences.len(), 4); // Original + 3 months
}

#[test]
fn test_reminder_creation() {
    let reminder = Reminder::new(Utc::now(), Duration::minutes(15));
    assert!(reminder.is_due());
}

#[test]
fn test_reminder_future() {
    let reminder = Reminder::new(Utc::now() + Duration::hours(1), Duration::minutes(15));
    assert!(!reminder.is_due());
}

#[test]
fn test_event_encryption() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let event = Event::new("Secret Event", start, end, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    let encryption = EventEncryption::new(EncryptionLevel::Full);
    let key = "test_key".to_string();
    
    let encrypted = encryption.encrypt_event(&event, &key).unwrap();
    let decrypted = encryption.decrypt_event(&encrypted, &key).unwrap();
    
    assert_eq!(decrypted.title(), event.title());
}

#[test]
fn test_key_manager() {
    let mut key_manager = KeyManager::new();
    
    let key_id = key_manager.generate_key().unwrap();
    assert!(!key_id.is_empty());
    
    assert!(key_manager.key_exists(&key_id));
}

#[test]
fn test_invitation_system() {
    let mut invitation_system = InvitationSystem::new();
    
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let event = Event::new("Meeting", start, end, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    let invitee_email = "test@example.com";
    let invitation = invitation_system.create_invitation(&event, invitee_email).unwrap();
    
    assert_eq!(invitation.invitee_email(), invitee_email);
}

#[test]
fn test_conflict_detection() {
    let mut calendar = Calendar::new("Test Calendar", Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    let now = Utc::now();
    
    let event1 = Event::new(
        "Event 1",
        now,
        now + Duration::hours(1),
        Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    );
    
    let event2 = Event::new(
        "Event 2",
        now + Duration::minutes(30),
        now + Duration::hours(2),
        Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }
    );
    
    let event3 = Event::new(
        "Event 3",
        now + Duration::hours(2),
        now + Duration::hours(3),
        Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }
    );
    
    calendar.add_event(event1).unwrap();
    calendar.add_event(event2).unwrap();
    calendar.add_event(event3).unwrap();
    
    let conflict_detector = ConflictDetector::new();
    let conflicts = conflict_detector.detect_conflicts(&calendar);
    
    assert_eq!(conflicts.len(), 1); // event1 and event2 conflict
}

#[test]
fn test_availability() {
    let start = Utc::now();
    let end = start + Duration::days(1);
    
    let availability = Availability::new(start, end);
    
    assert!(availability.is_available_at(start));
    assert!(availability.is_available_at(end));
    assert!(!availability.is_available_at(end + Duration::seconds(1)));
}

#[test]
fn test_schedule_suggestion() {
    let start = Utc::now();
    let end = start + Duration::days(1);
    
    let criteria = SuggestionCriteria {
        duration: Duration::hours(1),
        preferred_start: start,
        preferred_end: end,
        ..Default::default()
    };
    
    let suggester = Suggester::new();
    let suggestions = suggester.suggest(&criteria);
    
    assert!(!suggestions.is_empty());
}

#[test]
fn test_schedule_optimizer() {
    let mut calendar = Calendar::new("Test Calendar", Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    let now = Utc::now();
    
    // Add some events
    for i in 0..5 {
        let start = now + Duration::hours(i * 2);
        let end = start + Duration::hours(1);
        let event = Event::new(
            format!("Event {}", i),
            start,
            end,
            Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
        );
        calendar.add_event(event).unwrap();
    }
    
    let optimizer = ScheduleOptimizer::new();
    let optimized = optimizer.optimize(&calendar);
    
    assert_eq!(optimized.events().len(), calendar.events().len());
}

#[test]
fn test_event_update() {
    let mut calendar = Calendar::new("Test Calendar", Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let event = Event::new("Original Title", start, end, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    let event_id = calendar.add_event(event).unwrap();
    
    calendar.update_event(event_id, |e| {
        e.set_title("Updated Title");
    }).unwrap();
    
    let updated_event = calendar.get_event(event_id).unwrap();
    assert_eq!(updated_event.title(), "Updated Title");
}

#[test]
fn test_multiple_calendars() {
    let cal1 = Calendar::new("Work Calendar", Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    let cal2 = Calendar::new("Personal Calendar", Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 });
    
    let start = Utc::now();
    let end = start + Duration::hours(1);
    
    let work_event = Event::new("Work Event", start, end, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    let personal_event = Event::new("Personal Event", start, end, Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 });
    
    cal1.add_event(work_event).unwrap();
    cal2.add_event(personal_event).unwrap();
    
    assert_eq!(cal1.events().len(), 1);
    assert_eq!(cal2.events().len(), 1);
}

#[test]
fn test_event_description() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let mut event = Event::new("Event", start, end, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    event.set_description("Event description");
    assert_eq!(event.description(), "Event description");
}

#[test]
fn test_event_location() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let mut event = Event::new("Event", start, end, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    event.set_location("Conference Room A");
    assert_eq!(event.location(), "Conference Room A");
}

#[test]
fn test_event_participants() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let mut event = Event::new("Event", start, end, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    event.add_participant("user1@example.com");
    event.add_participant("user2@example.com");
    
    assert_eq!(event.participants().len(), 2);
}

#[test]
fn test_event_color() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let mut event = Event::new("Event", start, end, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    event.set_color(Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 });
    
    assert_eq!(event.color(), &Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 });
}

#[test]
fn test_calendar_query_by_title() {
    let mut calendar = Calendar::new("Test Calendar", Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    let start = Utc::now();
    let end = start + Duration::hours(1);
    
    let event1 = Event::new("Meeting", start, end, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    let event2 = Event::new("Call", start + Duration::hours(2), end + Duration::hours(2), Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 });
    let event3 = Event::new("Meeting", start + Duration::hours(4), end + Duration::hours(4), Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 });
    
    calendar.add_event(event1).unwrap();
    calendar.add_event(event2).unwrap();
    calendar.add_event(event3).unwrap();
    
    let query = EventQuery::title("Meeting");
    let results = calendar.query(&query);
    
    assert_eq!(results.len(), 2);
}

#[test]
fn test_reminder_multiple() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let mut event = Event::new("Event", start, end, Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    
    event.add_reminder(Duration::hours(1));
    event.add_reminder(Duration::minutes(15));
    
    assert_eq!(event.reminders().len(), 2);
}

#[test]
fn test_recurrence_end_date() {
    let start = Utc::now();
    let end = start + Duration::hours(1);
    let recurrence_end = start + Duration::weeks(4);
    
    let recurrence = Recurrence::daily_with_end(recurrence_end);
    let event = Event::with_recurrence(
        "Event",
        start,
        end,
        recurrence,
        Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    );
    
    let occurrences = event.occurrences(start, start + Duration::days(35));
    assert_eq!(occurrences.len(), 5); // Should stop at 4 weeks
}