# Vantis Chronos

## Overview

Vantis Chronos is a privacy-first calendar application that never shares your plans with anyone unless you explicitly send an encrypted invitation. It uses PGP encryption for all calendar sharing and offers complete control over your scheduling data.

## Key Features

- **Privacy-First**: Zero data sharing by default
- **PGP Encryption**: All sharing encrypted with PGP
- **Invitation System**: Encrypted calendar invitations
- **Local Storage**: All data stored locally
- **Smart Scheduling**: AI-powered meeting suggestions
- **Integration**: Syncs with Vantis Flow and Vantis Link

## Architecture

```
vantis-chronos/
├── src/
│   ├── core/
│   │   ├── calendar.rs        # Calendar model
│   │   ├── event.rs           # Event model
│   │   ├── recurrence.rs      # Recurrence patterns
│   │   └── reminder.rs        # Reminder system
│   ├── encryption/
│   │   ├── pgp.rs             # PGP encryption
│   │   ├── keys.rs            # Key management
│   │   ├── invitation.rs      # Encrypted invitations
│   │   └── sharing.rs         # Sharing system
│   ├── scheduling/
│   │   ├── suggester.rs       # Meeting suggestions
│   │   ├── conflict.rs        # Conflict detection
│   │   ├── optimizer.rs       # Schedule optimization
│   │   └── availability.rs    # Availability management
│   ├── sync/
│   │   ├── vantis_flow.rs     # Sync with Vantis Flow
│   │   ├── vantis_link.rs     # Sync with Vantis Link
│   │   └── external.rs        # External calendar sync (optional)
│   ├── ui/
│   │   ├── calendar_view.rs   # Calendar views
│   │   ├── event_editor.rs    # Event editor
│   │   ├── timeline.rs        # Timeline view
│   │   └── agenda.rs          # Agenda view
│   └── notifications/
│       ├── manager.rs         # Notification manager
│       ├── reminders.rs       # Reminder notifications
│       ├── invitations.rs     # Invitation notifications
│       └── conflicts.rs       # Conflict alerts
├── keys/
│   ├── public/                # Public keys
│   └── private/               # Private keys (encrypted)
├── data/
│   ├── calendars/             # Calendar storage
│   └── events/                # Event storage
└── tests/
    ├── encryption/            # Encryption tests
    └── scheduling/            # Scheduling tests
```

## PGP Encryption

### Key Management

```rust
use vantis_chronos::encryption::{KeyManager, KeyPair};

let manager = KeyManager::new()?;

// Generate new key pair
let key_pair = manager.generate_key_pair(
    "user@example.com",
    "John Doe",
    4096  // Key size
)?;

// Export public key
let public_key = key_pair.public_key()?;
manager.save_public_key("public.asc", &public_key)?;

// Import public key
let imported = manager.import_public_key("friend_public.asc")?;
```

### Event Encryption

```rust
use vantis_chronos::encryption::EventEncryption;

let encryption = EventEncryption::new()?;

// Encrypt event for specific recipient
let encrypted = encryption.encrypt_event(
    &event,
    &recipient_public_key
)?;

// Decrypt event
let decrypted = encryption.decrypt_event(
    &encrypted,
    &private_key
)?;
```

### Invitation System

```rust
use vantis_chronos::encryption::{Invitation, InvitationType};

let invitation = Invitation::new()
    .with_type(InvitationType::Event)
    .with_event(event.clone())
    .with_sender_key(&public_key)
    .with_recipient_key(&recipient_public_key)
    .with_expiration(Duration::from_days(7))
    .with_response_required(true);

// Serialize and send
let invitation_data = invitation.serialize()?;

// Decrypt invitation
let decrypted = Invitation::decrypt(
    &invitation_data,
    &private_key
)?;
```

## Calendar Core

### Creating Events

```rust
use vantis_chronos::core::{Event, EventBuilder};

let event = EventBuilder::new()
    .with_title("Team Meeting")
    .with_start(DateTime::parse_from_rfc3339("2024-01-15T10:00:00Z")?)
    .with_duration(Duration::hours(1))
    .with_location("Conference Room A")
    .with_description("Weekly team sync")
    .with_attendees(vec![alice, bob])
    .with_reminders(vec![
        Reminder::new(Duration::minutes(15)),
        Reminder::new(Duration::hours(1))
    ])
    .build()?;

calendar.add_event(event)?;
```

### Recurring Events

```rust
use vantis_chronos::core::{Recurrence, RecurrencePattern};

let recurrence = Recurrence::new(
    RecurrencePattern::Weekly,
    DateTime::parse_from_rfc3339("2024-01-01T09:00:00Z")?,
)
    .with_interval(1)  // Every week
    .with_until(Some(DateTime::parse_from_rfc3339("2024-12-31T09:00:00Z")?))
    .with_weekdays(vec![Weekday::Mon, Weekday::Wed, Weekday::Fri]);

event.set_recurrence(recurrence)?;
```

### Calendar Views

```rust
use vantis_chronos::ui::{CalendarView, ViewType};

// Month view
let month_view = CalendarView::new(ViewType::Month, calendar.clone())?
    .with_show_weekends(true)
    .with_show_week_numbers(true);

// Week view
let week_view = CalendarView::new(ViewType::Week, calendar.clone())?
    .with_start_day(Weekday::Monday)
    .with_hour_height(60);

// Day view
let day_view = CalendarView::new(ViewType::Day, calendar.clone())?
    .with_time_range(TimeRange::from_hours(8, 18));
```

## Smart Scheduling

### Meeting Suggestions

```rust
use vantis_chronos::scheduling::{Suggester, SuggestionCriteria};

let criteria = SuggestionCriteria::new()
    .with_duration(Duration::hours(1))
    .with_attendees(vec![alice, bob, charlie])
    .with_preferred_time_range(TimeRange::from_hours(9, 17))
    .with_preferred_days(vec![Weekday::Mon, Weekday::Tue, Weekday::Wed])
    .with_availability_check(true);

let suggester = Suggester::new(calendar.clone());
let suggestions = suggester.suggest(criteria)?;

for suggestion in suggestions {
    println!("Suggested: {}", suggestion.time_slot);
    println!("Conflicts: {}", suggestion.conflicts.len());
    println!("Preference: {}", suggestion.score);
}
```

### Conflict Detection

```rust
use vantis_chronos::scheduling::ConflictDetector;

let detector = ConflictDetector::new()?;

let conflicts = detector.detect_conflicts(
    calendar,
    DateRange::this_week()
)?;

for conflict in conflicts {
    println!("Conflict detected:");
    println!("  Event 1: {}", conflict.event1.title);
    println!("  Event 2: {}", conflict.event2.title);
    println!("  Overlap: {}", conflict.overlap_duration);
}
```

### Availability Management

```rust
use vantis_chronos::scheduling::Availability;

let availability = Availability::new()?
    .add_working_hours(
        Weekday::Mon,
        TimeRange::from_hours(9, 17)
    )?
    .add_working_hours(
        Weekday::Tue,
        TimeRange::from_hours(9, 17)
    )?
    .add_working_hours(
        Weekday::Wed,
        TimeRange::from_hours(9, 17)
    )?
    .add_busy_slot(
        DateTime::parse_from_rfc3339("2024-01-15T14:00:00Z")?,
        Duration::hours(2)
    )?;

calendar.set_availability(availability)?;
```

## Integration

### Vantis Flow Sync

```rust
use vantis_chronos::sync::FlowSync;

let sync = FlowSync::new()?;

// Sync tasks to calendar
sync.sync_tasks(
    gantt_chart.tasks(),
    calendar.clone()
)?;

// Get calendar events as tasks
let tasks = sync.get_tasks_as_events(
    calendar,
    DateRange::this_month()
)?;
```

### Vantis Link Collaboration

```rust
use vantis_chronos::sync::LinkSync;

let sync = LinkSync::new(session)?;

// Share encrypted calendar
sync.share_calendar(
    calendar,
    vec![alice, bob],
    EncryptionLevel::PGP
)?;

// Receive shared calendar
let shared_calendar = sync.receive_shared_calendar(invitation)?;
```

## Notification System

### Reminders

```rust
use vantis_chronos::notifications::{ReminderManager, Reminder};

let manager = ReminderManager::new()?

// Add reminder
manager.add_reminder(Reminder::new(event.id, Duration::minutes(15)))?;

// Custom reminder
manager.add_custom_reminder(
    event.id,
    ReminderType::Email,
    Duration::hours(1),
    "Don't forget the meeting!"
)?;
```

### Invitation Notifications

```rust
use vantis_chronos::notifications::InvitationManager;

let manager = InvitationManager::new()?;

// Receive invitation
manager.on_invitation(|invitation| {
    println!("Received invitation from {}", invitation.sender);
    println!("Event: {}", invitation.event.title);
    
    // Accept or decline
    if user_wants_to_accept() {
        invitation.accept()?;
    } else {
        invitation.decline()?;
    }
    Ok(())
})?;
```

### Conflict Alerts

```rust
use vantis_chronos::notifications::ConflictAlert;

let alert = ConflictAlert::new(conflict)?
    .with_severity(Severity::High)
    .with_message("This event conflicts with another meeting")
    .with_suggestions(vec![
        "Move to 11:00 AM",
        "Reschedule conflicting event"
    ]);

manager.send_alert(alert)?;
```

## API Examples

### Creating a Calendar

```rust
use vantis_chronos::core::{Calendar, CalendarBuilder};

let calendar = CalendarBuilder::new()
    .with_name("Work")
    .with_color(Color::rgb(0x2196F3))
    .with_timezone("America/New_York")
    .build()?;

calendar.save()?;
```

### Querying Events

```rust
use vantis_chronos::core::EventQuery;

let query = EventQuery::new()
    .with_date_range(DateRange::this_week())
    .with_calendar(calendar.id)
    .with_title_contains("meeting")
    .with_attendee(alice.id);

let events = calendar.query_events(query)?;

for event in events {
    println!("{}: {}", event.start, event.title);
}
```

### Exporting Calendar

```rust
use vantis_chronos::export::{Exporter, ExportFormat};

// Export as ICS
Exporter::export(calendar, "calendar.ics", ExportFormat::ICS)?;

// Export as encrypted backup
Exporter::export_encrypted(
    calendar,
    "calendar.vcal",
    &public_key
)?;
```

## Integration Points

- **Vantis Flow**: Task synchronization
- **Vantis Link**: Encrypted sharing
- **Vantis Vault**: Calendar encryption
- **Vantis Ark**: Calendar backup
- **Vantis Writer**: Event documentation

## Configuration

```toml
# chronos.toml
[calendar]
default_timezone = "America/New_York"
start_of_week = "Monday"
show_weekends = true
show_week_numbers = false

[encryption]
default_algorithm = "rsa4096"
auto_generate_keys = true
key_expiration_days = 365

[sharing]
encryption_required = true
default_encryption = "pgp"
invitation_expiration_days = 7

[scheduling]
check_conflicts = true
suggestion_count = 5
include_weekends = false

[reminders]
default_reminder = "15m"
reminder_types = ["notification", "email"]
snooze_duration = "5m"

[notifications]
enabled = true
sound_enabled = true
notification_sound = "chime"
```

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| New Event | Ctrl+N |
| Go to Today | Ctrl+T |
| Next Day | Ctrl+→ |
| Previous Day | Ctrl+← |
| Next Week | Ctrl+↓ |
| Previous Week | Ctrl+↑ |
| Search | Ctrl+F |
| Sync | Ctrl+R |
| Export | Ctrl+E |

## Performance Metrics

- **Event Load**: 100ms for 10,000 events
- **Query Time**: 10ms for complex queries
- **Suggestion Generation**: 500ms
- **Encryption/Decryption**: 50ms per event
- **Conflict Detection**: 100ms per week
- **Sync Time**: 1s for full sync

## Security Features

1. **PGP Encryption**: All sharing encrypted
2. **Local Storage**: No cloud storage
3. **Key Management**: Secure key storage
4. **Private by Default**: No automatic sharing
5. **Audit Trail**: Complete access log
6. **Secure Deletion**: Wipe calendar data

## Privacy Guarantees

- **Zero Data Collection**: No telemetry or analytics
- **No Cloud Storage**: All data stored locally
- **Encrypted Sharing**: Only intended recipients can access
- **No Metadata Tracking**: Minimal metadata retention
- **User Control**: Complete control over data
- **Open Source**: Auditable codebase

## Future Roadmap

- [ ] Voice event creation
- [ ] AI-powered scheduling assistant
- [ ] Natural language event creation
- [ ] Advanced time zone support
- [ ] Multiple calendar views
- [ ] Mobile companion app

## Build Requirements

- Rust 1.70+
- Flux Vector Engine
- sequoia-pgp (PGP library)
- chrono (date/time handling)
- icalendar (ICS support)

---

**Part of VantisOffice Pillar III - Ecosystem & Collaboration**