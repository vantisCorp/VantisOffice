//! Notifications module for reminders, invitations, and conflict alerts

// Notifications module - imports used when notification features are enabled
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

/// Notification manager
pub struct NotificationManager {
    notifications: HashMap<String, Notification>,
}

impl NotificationManager {
    pub fn new() -> Self {
        NotificationManager {
            notifications: HashMap::new(),
        }
    }

    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications
            .insert(notification.id.clone(), notification);
    }

    pub fn get_notifications(&self) -> Vec<&Notification> {
        self.notifications.values().collect()
    }

    pub fn mark_as_read(&mut self, notification_id: &str) -> Result<(), String> {
        let notification = self
            .notifications
            .get_mut(notification_id)
            .ok_or("Notification not found")?;
        notification.read = true;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: String,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub read: bool,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Reminder,
    Invitation,
    Conflict,
}

/// Reminder manager
pub struct ReminderManager {
    reminders: HashMap<String, Reminder>,
}

impl ReminderManager {
    pub fn new() -> Self {
        ReminderManager {
            reminders: HashMap::new(),
        }
    }

    pub fn schedule_reminder(
        &mut self,
        event_id: &str,
        event_start: DateTime<Utc>,
        minutes_before: u32,
    ) -> Reminder {
        let reminder = Reminder {
            id: uuid::Uuid::new_v4().to_string(),
            event_id: event_id.to_string(),
            scheduled_time: event_start - Duration::minutes(minutes_before as i64),
            minutes_before,
            sent: false,
        };

        self.reminders.insert(reminder.id.clone(), reminder.clone());
        reminder
    }

    pub fn check_due_reminders(&mut self, now: DateTime<Utc>) -> Vec<Reminder> {
        let mut due = Vec::new();

        for reminder in self.reminders.values_mut() {
            if !reminder.sent && reminder.scheduled_time <= now {
                reminder.sent = true;
                due.push(reminder.clone());
            }
        }

        due
    }
}

#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: String,
    pub event_id: String,
    pub scheduled_time: DateTime<Utc>,
    pub minutes_before: u32,
    pub sent: bool,
}

/// Invitation manager
pub struct InvitationManager {
    invitations: HashMap<String, Invitation>,
}

impl InvitationManager {
    pub fn new() -> Self {
        InvitationManager {
            invitations: HashMap::new(),
        }
    }

    pub fn create_invitation(
        &mut self,
        event_id: &str,
        invitee: &str,
        inviter: &str,
    ) -> Invitation {
        let invitation = Invitation {
            id: uuid::Uuid::new_v4().to_string(),
            event_id: event_id.to_string(),
            invitee: invitee.to_string(),
            inviter: inviter.to_string(),
            status: InvitationStatus::Pending,
            created_at: Utc::now(),
        };

        self.invitations
            .insert(invitation.id.clone(), invitation.clone());
        invitation
    }

    pub fn accept_invitation(&mut self, invitation_id: &str) -> Result<(), String> {
        let invitation = self
            .invitations
            .get_mut(invitation_id)
            .ok_or("Invitation not found")?;
        invitation.status = InvitationStatus::Accepted;
        Ok(())
    }

    pub fn decline_invitation(&mut self, invitation_id: &str) -> Result<(), String> {
        let invitation = self
            .invitations
            .get_mut(invitation_id)
            .ok_or("Invitation not found")?;
        invitation.status = InvitationStatus::Declined;
        Ok(())
    }

    pub fn get_pending_invitations(&self, user_id: &str) -> Vec<&Invitation> {
        self.invitations
            .values()
            .filter(|i| i.invitee == user_id && i.status == InvitationStatus::Pending)
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct Invitation {
    pub id: String,
    pub event_id: String,
    pub invitee: String,
    pub inviter: String,
    pub status: InvitationStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
}

/// Conflict alert manager
pub struct ConflictAlertManager {
    alerts: HashMap<String, ConflictAlert>,
}

impl ConflictAlertManager {
    pub fn new() -> Self {
        ConflictAlertManager {
            alerts: HashMap::new(),
        }
    }

    pub fn create_alert(
        &mut self,
        event1_id: &str,
        event2_id: &str,
        severity: ConflictSeverity,
    ) -> ConflictAlert {
        let alert = ConflictAlert {
            id: uuid::Uuid::new_v4().to_string(),
            event1_id: event1_id.to_string(),
            event2_id: event2_id.to_string(),
            severity,
            created_at: Utc::now(),
            resolved: false,
        };

        self.alerts.insert(alert.id.clone(), alert.clone());
        alert
    }

    pub fn resolve_alert(&mut self, alert_id: &str) -> Result<(), String> {
        let alert = self.alerts.get_mut(alert_id).ok_or("Alert not found")?;
        alert.resolved = true;
        Ok(())
    }

    pub fn get_unresolved_alerts(&self) -> Vec<&ConflictAlert> {
        self.alerts.values().filter(|a| !a.resolved).collect()
    }
}

#[derive(Debug, Clone)]
pub struct ConflictAlert {
    pub id: String,
    pub event1_id: String,
    pub event2_id: String,
    pub severity: ConflictSeverity,
    pub created_at: DateTime<Utc>,
    pub resolved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictSeverity {
    Low,
    Medium,
    High,
}
