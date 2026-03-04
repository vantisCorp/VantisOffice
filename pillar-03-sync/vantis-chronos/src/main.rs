//! Demo application for Vantis Chronos

use chrono::{Duration, Timelike, Utc};
use vantis_chronos::*;

fn main() -> Result<(), ChronosError> {
    println!("=== Vantis Chronos Demo ===\n");

    // Initialize Chronos
    init()?;
    println!("✓ Vantis Chronos initialized\n");

    // Create a calendar
    let mut calendar = Calendar::new("Work Calendar".to_string(), Color::new(0x21, 0x96, 0xF3));
    println!("✓ Created calendar: {}\n", calendar.name);

    // Create events
    let now = Utc::now();
    let meeting_start = now + Duration::hours(2);
    let meeting_end = meeting_start + Duration::hours(1);

    let mut meeting = Event::new("Team Meeting".to_string(), meeting_start, meeting_end);
    meeting.description = Some("Weekly team sync".to_string());
    meeting.location = Some("Conference Room A".to_string());
    meeting.attendees.push("alice@example.com".to_string());
    meeting.attendees.push("bob@example.com".to_string());

    calendar.add_event(meeting);
    println!("✓ Added event: Team Meeting\n");

    // Create another event
    let lunch_start = now + Duration::hours(4);
    let lunch_end = lunch_start + Duration::minutes(30);

    let lunch = Event::new("Lunch Break".to_string(), lunch_start, lunch_end);

    calendar.add_event(lunch);
    println!("✓ Added event: Lunch Break\n");

    // Query events
    let query = EventQuery {
        start_date: Some(now),
        end_date: Some(now + Duration::days(1)),
        title_contains: None,
        status: None,
    };

    let events = calendar.query_events(&query);
    println!("✓ Found {} events today\n", events.len());

    // Test encryption
    let encryption = EventEncryption::new(EncryptionLevel::AES256);
    let event_data = "Team Meeting at 2 PM";
    let encrypted = encryption.encrypt_event(event_data, "public_key")?;
    println!("✓ Encrypted event data\n");

    let decrypted = encryption.decrypt_event(&encrypted, "private_key")?;
    println!("✓ Decrypted event data: {}\n", decrypted);

    // Test key management
    let mut key_manager = KeyManager::new();
    let key_pair = key_manager.generate_key_pair("user@example.com")?;
    println!("✓ Generated PGP key pair for user@example.com\n");

    // Test scheduling
    let working_hours = TimeRange {
        start: chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        end: chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
    };

    let working_days = vec![
        chrono::Weekday::Mon,
        chrono::Weekday::Tue,
        chrono::Weekday::Wed,
        chrono::Weekday::Thu,
        chrono::Weekday::Fri,
    ];

    let suggester = Suggester::new(working_hours, working_days);
    let suggestions = suggester.suggest_meeting_times(
        &[&calendar],
        Duration::hours(1),
        now + Duration::days(1),
        now + Duration::days(7),
        &SuggestionCriteria::default(),
    );

    println!("✓ Found {} meeting suggestions\n", suggestions.len());
    for (i, suggestion) in suggestions.iter().take(3).enumerate() {
        println!(
            "  {}. {} - {} (score: {:.2})",
            i + 1,
            suggestion.start.format("%Y-%m-%d %H:%M"),
            suggestion.end.format("%H:%M"),
            suggestion.score
        );
    }

    // Test conflict detection
    let conflicts = ConflictDetector::detect_conflicts(&calendar);
    println!("\n✓ Found {} conflicts\n", conflicts.len());

    // Test ICS export
    let external_sync = ExternalSync;
    let ics = external_sync.export_to_ics(&calendar)?;
    println!("✓ Exported calendar to ICS format ({} bytes)\n", ics.len());

    // Test calendar views
    let day_view = CalendarView::new(calendar.clone(), ViewType::Day);
    println!("=== Day View ===");
    println!("{}", day_view.render());

    let week_view = CalendarView::new(calendar.clone(), ViewType::Week);
    println!("=== Week View ===");
    println!("{}", week_view.render());

    // Test notifications
    let mut reminder_manager = ReminderManager::new();
    let reminder = reminder_manager.schedule_reminder(&calendar.events[0].id, meeting_start, 15);
    println!("✓ Scheduled reminder 15 minutes before event\n");

    // Test invitations
    let mut invitation_manager = InvitationManager::new();
    let invitation = invitation_manager.create_invitation(
        &calendar.events[0].id,
        "newuser@example.com",
        "user@example.com",
    );
    println!("✓ Created invitation for newuser@example.com\n");

    println!("=== Demo Complete ===");
    Ok(())
}
