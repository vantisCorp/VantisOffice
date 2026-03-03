# Vantis Chronos User Guide

## Table of Contents
1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Calendar Views](#calendar-views)
4. [Creating Events](#creating-events)
5. [Managing Events](#managing-events)
6. [Recurring Events](#recurring-events)
7. [Reminders and Notifications](#reminders-and-notifications)
8. [Collaboration](#collaboration)
9. [Import/Export](#importexport)
10. [Privacy and Security](#privacy-and-security)
11. [Keyboard Shortcuts](#keyboard-shortcuts)
12. [Tips and Tricks](#tips-and-tricks)

## Introduction

Vantis Chronos is a privacy-first calendar with PGP encryption, time zone support, and advanced scheduling features.

**Key Features:**
- PGP encryption for all events
- Multiple calendar views (Day, Week, Month, Year)
- Recurring events with complex patterns
- Time zone conversion and support
- Conflict detection
- Import/Export ICS format

## Getting Started

### Launching Vantis Chronos

```bash
cargo run --release -p vantis-chronos
```

### Creating Your First Calendar

1. Launch Vantis Chronos
2. Click "New Calendar" or press `Ctrl+N` / `Cmd+N`
3. Configure calendar settings
4. Add your first event

### Interface Overview

```
┌─────────────────────────────────────────────────────┐
│  File  View  Event  Calendar  Share  Help          │
├─────────────────────────────────────────────────────┤
│  [Today] [Back] [Forward]                          │
├─────────────────────────────────────────────────────┤
│  Sun  Mon  Tue  Wed  Thu  Fri  Sat                │
│  28   29   1    2    3    4    5                   │
│  6    7    8    9    10   11   12                  │
│  13   14   15   16   17   18   19                  │
├─────────────────────────────────────────────────────┤
│  Calendars:                                        │
│  ☑ Personal                                        │
│  ☑ Work                                            │
│  ☐ Family                                          │
├─────────────────────────────────────────────────────┤
│  Upcoming Events:                                  │
│  ├─ 10:00 AM - Team Meeting                       │
│  ├─ 2:00 PM - Client Call                         │
│  └─ 5:00 PM - Gym                                 │
└─────────────────────────────────────────────────────┘
```

## Calendar Views

### Day View

Shows events for single day:

**Navigation:**
- Next/Previous: Arrow buttons or `←`/`→`
- Jump to Today: `T`
- Jump to Date: `G`

**Day View Features:**
- Hourly time slots (30-min intervals)
- All-day events at top
- Color-coded events by calendar
- Time scale on left
- Weather display

### Week View

Shows events for full week:

**Week View Features:**
- 7-day grid
- All-day events row
- Work hours highlighted
- Week number display
- Scroll to see all hours

**Navigation:**
- Previous/Next week
- Jump to today
- Select specific day

### Month View

Shows events for entire month:

**Month View Features:**
- Full month grid
- Events shown as bars
- Event details on hover
- Today highlighted
- Week numbers

**Navigation:**
- Previous/Next month
- Jump to today
- Select specific day

### Year View

Shows overview of entire year:

**Year View Features:**
- 12-month grid
- Event indicators
- Quarter highlights
- Navigate to any month

## Creating Events

### New Event

**Create from Interface:**
1. Click on date/time slot
2. Or press `Ctrl+N` / `Cmd+N`
3. Fill in event details
4. Save event

**Quick Create:**
1. Double-click date/time
2. Enter event title
3. Press Enter to save
4. Edit details later

### Event Properties

**Basic Properties:**
- Title: Event name
- Start: Date and time
- End: Date and time
- All Day: Full day event
- Location: Event location
- Description: Event details

**Advanced Properties:**
- Calendar: Which calendar
- Color: Event color
- Availability: Free/Busy/Tentative
- Visibility: Public/Private/Confidential
- Priority: Low/Normal/High

### Event Encryption

Vantis Chronos encrypts all events with PGP:

**Default Encryption:**
- All events encrypted
- Private key required to decrypt
- End-to-end encryption

**Configure Encryption:**
1. **Calendar** → **Encryption Settings**
2. Select PGP key
3. Choose encryption algorithm
4. Apply to existing events

## Managing Events

### Edit Event

1. Double-click event
2. Or right-click → **Edit**
3. Modify properties
4. Save changes

### Delete Event

1. Select event
2. Press `Del`
3. Confirm deletion
4. Choose to:
   - Delete only this instance
   - Delete all instances (for recurring)

### Move Event

**Drag and Drop:**
1. Drag event to new time/date
2. Event moves automatically
3. Changes sync if shared

**Edit Date/Time:**
1. Open event
2. Change start/end time
3. Save changes

### Copy Event

1. Right-click event
2. **Copy**
3. Paste to new location/time
4. Or duplicate event

### Event Categories

**Add Categories:**
1. **Calendar** → **Categories**
2. Create new category
3. Set color and name
4. Apply to events

**Filter by Category:**
1. Click category in sidebar
2. View events in that category
3. Multiple categories can be selected

## Recurring Events

### Creating Recurring Events

1. Create new event
2. Check **Recurring**
3. Set recurrence pattern
4. Configure recurrence rules

### Recurrence Patterns

**Daily:**
- Every day
- Every N days
- Weekdays only
- Weekends only

**Weekly:**
- Every week
- Every N weeks
- On specific days

**Monthly:**
- Every month
- Every N months
- On specific date
- On specific weekday

**Yearly:**
- Every year
- Every N years
- On specific date
- On specific weekday

**Custom:**
- Build custom recurrence rules
- Set end date or number of occurrences
- Configure exceptions

### Recurrence Examples

**Weekly Meeting:**
- Pattern: Every week on Monday
- End: Never
- Time: 10:00 AM - 11:00 AM

**Monthly Bill:**
- Pattern: Every month on the 15th
- End: Never
- All day event

**Yearly Birthday:**
- Pattern: Every year on March 15
- End: Never
- All day event

## Reminders and Notifications

### Setting Reminders

**Add Reminder to Event:**
1. Open event
2. Click **Add Reminder**
3. Set reminder time:
   - 0 minutes before
   - 5 minutes before
   - 15 minutes before
   - 30 minutes before
   - 1 hour before
   - 1 day before
   - Custom

**Multiple Reminders:**
- Add multiple reminders per event
- Different notification types

### Reminder Types

**Desktop Notification:**
- Pop-up notification
- Sound alert
- Snooze option

**Email Reminder:**
- Send email at reminder time
- Include event details
- Include location map

**SMS Reminder:**
- Send SMS message
- Quick text summary

**Mobile Push:**
- Send to mobile device
- Works with Vantis Mobile

### Notification Settings

1. **Calendar** → **Settings** → **Notifications**
2. Configure:
   - Default reminder time
   - Notification sound
   - Notification display time
   - Snooze duration
   - Do not disturb hours

## Collaboration

### Sharing Calendars

**Share Individual Calendar:**
1. Right-click calendar
2. **Share Calendar**
3. Enter email or username
4. Set permissions:
   - View only
   - Can edit events
   - Can manage calendar
5. Send invitation

**Share Events:**
1. Select event(s)
2. **Event** → **Share**
3. Generate share link
4. Send to collaborators
5. Choose if recipients can edit

### Collaborative Editing

**Real-time Updates:**
- Changes sync instantly
- See collaborators' cursors
- Conflict resolution

**Event Sharing:**
- Invite participants
- Participants can view/edit
- RSVP tracking
- Availability status

### Availability Status

**Show Availability:**
1. **Calendar** → **Availability**
2. Set status:
   - Free
   - Busy
   - Tentative
   - Out of Office

**Busy Time Blocking:**
- Events block time as busy
- Others see availability
- Privacy settings control visibility

## Import/Export

### Import ICS

**Import from File:**
1. **Calendar** → **Import** → **ICS File**
2. Select .ics file
3. Choose destination calendar
4. Import events

**Import from URL:**
1. **Calendar** → **Import** → **URL**
2. Enter ICS URL
3. Set sync frequency
4. Import events

**Import Options:**
- Merge with existing events
- Replace calendar
- Create new calendar

### Export ICS

**Export Calendar:**
1. Right-click calendar
2. **Export** → **ICS**
3. Choose export options:
   - Date range
   - Include private events
   - Include encryption
4. Save file

**Export Events:**
1. Select events
2. **Event** → **Export** → **ICS**
3. Save file

### Calendar Sync

**Sync with External Calendars:**
1. **Calendar** → **Sync Settings**
2. Add external calendar:
   - Google Calendar
   - Outlook
   - iCloud
   - Other CalDAV
3. Enter credentials
4. Configure sync options

**Sync Frequency:**
- Manual
- Every 15 minutes
- Every hour
- Every day

## Privacy and Security

### PGP Encryption

All events are encrypted with PGP:

**How it Works:**
1. Events encrypted with recipient's public key
2. Only recipient's private key can decrypt
3. End-to-end encryption
4. No server-side decryption

**Manage PGP Keys:**
1. **Calendar** → **PGP Keys**
2. View your keys
3. Import public keys
4. Generate new key pair

### Privacy Settings

**Event Visibility:**
- **Public**: Anyone can see details
- **Private**: Show as busy only
- **Confidential**: Show as tentative only

**Calendar Visibility:**
- Share entire calendar or specific events
- Set different visibility for different users
- Hide private events

### Security Settings

**Two-Factor Authentication:**
1. **Calendar** → **Settings** → **Security**
2. Enable 2FA
3. Configure authenticator app

**Session Management:**
1. **Calendar** → **Settings** → **Sessions**
2. View active sessions
3. Revoke sessions

## Keyboard Shortcuts

### Navigation

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Today | `T` | `T` |
| Previous | `←` | `←` |
| Next | `→` | `→` |
| Jump to Date | `G` | `G` |
| Go to Now | `N` | `N` |

### Views

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| Day View | `1` | `1` |
| Week View | `2` | `2` |
| Month View | `3` | `3` |
| Year View | `4` | `4` |

### Events

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Event | `Ctrl+N` | `Cmd+N` |
| Edit Event | `Enter` | `Enter` |
| Delete Event | `Del` | `Del` |
| Copy Event | `Ctrl+C` | `Cmd+C` |
| Paste Event | `Ctrl+V` | `Cmd+V` |

### Calendars

| Action | Windows/Linux | macOS |
|--------|---------------|-------|
| New Calendar | `Ctrl+Shift+N` | `Cmd+Shift+N` |
| Toggle Calendar | Click checkbox | Click checkbox |
| Calendar Settings | `Ctrl+,` | `Cmd+,` |

## Tips and Tricks

### Productivity Tips

1. **Color Coding**: Use consistent colors for event types
2. **Event Templates**: Create templates for common events
3. **Time Blocking**: Block time for focused work
4. **Buffer Time**: Add buffer between events
5. **Review Weekly**: Review upcoming week regularly

### Privacy Tips

1. **Encrypt Sensitive Events**: Use PGP for private events
2. **Set Visibility Carefully**: Choose appropriate visibility
3. **Manage Keys**: Keep PGP keys secure
4. **Share Selectively**: Only share what's necessary
5. **Regular Audits**: Review shared calendars

### Collaboration Tips

1. **Clear Descriptions**: Include all relevant details
2. **Location Maps**: Add location for easy navigation
4. **Invite Early**: Give collaborators notice
5. **Set Reminders**: Set appropriate reminder times

### Scheduling Tips

1. **Use Recurring Events**: For regular meetings
2. **Set Reminders**: Don't forget important events
3. **Check Availability**: Before scheduling meetings
4. **Add Notes**: Include meeting agenda
5. **Follow Up**: Add follow-up tasks as events

## Troubleshooting

### Common Issues

**Events Not Appearing:**
- Check calendar visibility (checkbox)
- Verify date range
- Check for filters
- Refresh calendar

**Sync Issues:**
- Check internet connection
- Verify sync settings
- Refresh external calendars
- Check credentials

**Encryption Issues:**
- Verify PGP keys
- Check key availability
- Ensure correct recipient key
- Test encryption

**Reminder Issues:**
- Check notification settings
- Verify system notifications enabled
- Check do not disturb
- Test with new reminder

### Getting Help

- Check documentation for specific features
- Review keyboard shortcuts
- Report issues at: https://github.com/vantisCorp/VantisOffice/issues

---

**Last Updated**: 2024-03-03  
**VantisChronos Version**: 0.2.0