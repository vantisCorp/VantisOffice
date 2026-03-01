//! Notification manager for mobile notifications

use crate::error::{MobileError, MobileResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Notifications enabled
    pub enabled: bool,
    
    /// Sound enabled
    pub sound_enabled: bool,
    
    /// Vibration enabled
    pub vibration_enabled: bool,
    
    /// Badge enabled
    pub badge_enabled: bool,
    
    /// Quiet hours enabled
    pub quiet_hours_enabled: bool,
    
    /// Quiet hours start (HH:MM)
    pub quiet_hours_start: String,
    
    /// Quiet hours end (HH:MM)
    pub quiet_hours_end: String,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sound_enabled: true,
            vibration_enabled: true,
            badge_enabled: true,
            quiet_hours_enabled: false,
            quiet_hours_start: "22:00".to_string(),
            quiet_hours_end: "08:00".to_string(),
        }
    }
}

/// Notification type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    /// Document updated
    DocumentUpdated { document_id: String, document_title: String },
    
    /// Collaboration request
    CollaborationRequest { from: String, document_id: String },
    
    /// System notification
    System { message: String },
    
    /// Calendar event
    CalendarEvent { event_id: String, event_title: String },
    
    /// Sync completed
    SyncCompleted { documents_count: usize },
    
    /// Sync failed
    SyncFailed { error: String },
    
    /// Backup completed
    BackupCompleted,
    
    /// Backup failed
    BackupFailed { error: String },
}

/// Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID
    pub notification_id: String,
    
    /// Notification type
    pub notification_type: NotificationType,
    
    /// Title
    pub title: String,
    
    /// Body
    pub body: String,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Read status
    pub read: bool,
    
    /// Category
    pub category: Option<String>,
    
    /// Actions
    pub actions: Vec<NotificationAction>,
    
    /// Data
    pub data: HashMap<String, String>,
}

impl Notification {
    /// Create a new notification
    pub fn new(notification_type: NotificationType, title: String, body: String) -> Self {
        Self {
            notification_id: Uuid::new_v4().to_string(),
            notification_type,
            title,
            body,
            timestamp: chrono::Utc::now(),
            read: false,
            category: None,
            actions: Vec::new(),
            data: HashMap::new(),
        }
    }
    
    /// Mark as read
    pub fn mark_as_read(&mut self) {
        self.read = true;
    }
    
    /// Add action
    pub fn add_action(&mut self, action: NotificationAction) {
        self.actions.push(action);
    }
    
    /// Add data
    pub fn add_data(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
}

/// Notification action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationAction {
    /// Action identifier
    pub identifier: String,
    
    /// Action title
    pub title: String,
    
    /// Action type
    pub action_type: ActionType,
}

/// Action type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    /// Open document
    OpenDocument { document_id: String },
    
    /// Open app
    OpenApp { app_type: String },
    
    /// Approve
    Approve,
    
    /// Decline
    Decline,
    
    /// Dismiss
    Dismiss,
    
    /// Custom
    Custom { action: String },
}

/// Notification category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationCategory {
    /// Category identifier
    pub identifier: String,
    
    /// Category name
    pub name: String,
    
    /// Actions
    pub actions: Vec<NotificationAction>,
}

/// Notification manager
pub struct NotificationManager {
    config: NotificationConfig,
    notifications: Arc<RwLock<Vec<Notification>>>,
    categories: Arc<RwLock<HashMap<String, NotificationCategory>>>,
}

impl NotificationManager {
    /// Create a new notification manager
    pub fn new(config: NotificationConfig) -> Self {
        Self {
            config,
            notifications: Arc::new(RwLock::new(Vec::new())),
            categories: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Request notification permission
    pub async fn request_permission(&self) -> MobileResult<bool> {
        // In a real implementation, this would request system permission
        Ok(true)
    }
    
    /// Register for notifications
    pub async fn register(&self) -> MobileResult<()> {
        // In a real implementation, this would register with push notification service
        Ok(())
    }
    
    /// Send notification
    pub async fn send(&self, notification: Notification) -> MobileResult<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        // Check quiet hours
        if self.config.quiet_hours_enabled && self.is_quiet_hours() {
            return Ok(());
        }
        
        let mut notifications = self.notifications.write().await;
        notifications.push(notification);
        
        Ok(())
    }
    
    /// Get all notifications
    pub async fn get_all(&self) -> Vec<Notification> {
        let notifications = self.notifications.read().await;
        notifications.clone()
    }
    
    /// Get unread notifications
    pub async fn get_unread(&self) -> Vec<Notification> {
        let notifications = self.notifications.read().await;
        notifications
            .iter()
            .filter(|n| !n.read)
            .cloned()
            .collect()
    }
    
    /// Get notification by ID
    pub async fn get(&self, notification_id: &str) -> Option<Notification> {
        let notifications = self.notifications.read().await;
        notifications
            .iter()
            .find(|n| n.notification_id == notification_id)
            .cloned()
    }
    
    /// Mark notification as read
    pub async fn mark_as_read(&self, notification_id: &str) -> MobileResult<()> {
        let mut notifications = self.notifications.write().await;
        
        if let Some(notification) = notifications
            .iter_mut()
            .find(|n| n.notification_id == notification_id)
        {
            notification.mark_as_read();
        }
        
        Ok(())
    }
    
    /// Mark all notifications as read
    pub async fn mark_all_as_read(&self) -> MobileResult<()> {
        let mut notifications = self.notifications.write().await;
        
        for notification in notifications.iter_mut() {
            notification.mark_as_read();
        }
        
        Ok(())
    }
    
    /// Delete notification
    pub async fn delete(&self, notification_id: &str) -> MobileResult<()> {
        let mut notifications = self.notifications.write().await;
        notifications.retain(|n| n.notification_id != notification_id);
        Ok(())
    }
    
    /// Clear all notifications
    pub async fn clear_all(&self) -> MobileResult<()> {
        let mut notifications = self.notifications.write().await;
        notifications.clear();
        Ok(())
    }
    
    /// Register category
    pub async fn register_category(&self, category: NotificationCategory) -> MobileResult<()> {
        let mut categories = self.categories.write().await;
        categories.insert(category.identifier.clone(), category);
        Ok(())
    }
    
    /// Get category
    pub async fn get_category(&self, identifier: &str) -> Option<NotificationCategory> {
        let categories = self.categories.read().await;
        categories.get(identifier).cloned()
    }
    
    /// Handle notification action
    pub async fn handle_action(
        &self,
        notification_id: &str,
        action_identifier: &str,
    ) -> MobileResult<()> {
        let notification = self.get(notification_id).await;
        
        if let Some(notification) = notification {
            if let Some(action) = notification
                .actions
                .iter()
                .find(|a| a.identifier == action_identifier)
            {
                // Handle action based on type
                match &action.action_type {
                    ActionType::OpenDocument { document_id: _ } => {
                        // Open document
                    }
                    ActionType::OpenApp { app_type: _ } => {
                        // Open app
                    }
                    ActionType::Approve => {
                        // Approve action
                    }
                    ActionType::Decline => {
                        // Decline action
                    }
                    ActionType::Dismiss => {
                        self.delete(notification_id).await?;
                    }
                    ActionType::Custom { action: _ } => {
                        // Handle custom action
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Check if current time is in quiet hours
    fn is_quiet_hours(&self) -> bool {
        use chrono::Timelike;
        
        let now = chrono::Utc::now();
        let current_hour = now.hour() as u32;
        let current_minute = now.minute() as u32;
        
        let start_parts: Vec<u32> = self
            .config
            .quiet_hours_start
            .split(':')
            .map(|p| p.parse::<u32>().unwrap())
            .collect();
        let end_parts: Vec<u32> = self
            .config
            .quiet_hours_end
            .split(':')
            .map(|p| p.parse::<u32>().unwrap())
            .collect();
        
        let start_minutes = start_parts[0] * 60 + start_parts[1];
        let end_minutes = end_parts[0] * 60 + end_parts[1];
        let current_minutes = current_hour * 60 + current_minute;
        
        if start_minutes < end_minutes {
            current_minutes >= start_minutes && current_minutes < end_minutes
        } else {
            // Quiet hours span midnight
            current_minutes >= start_minutes || current_minutes < end_minutes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_config_default() {
        let config = NotificationConfig::default();
        assert!(config.enabled);
        assert!(config.sound_enabled);
    }

    #[tokio::test]
    async fn test_notification_manager_creation() {
        let config = NotificationConfig::default();
        let manager = NotificationManager::new(config);
        assert_eq!(manager.get_all().await.len(), 0);
    }

    #[tokio::test]
    async fn test_send_notification() {
        let config = NotificationConfig::default();
        let manager = NotificationManager::new(config);
        
        let notification = Notification::new(
            NotificationType::System {
                message: "Test".to_string(),
            },
            "Test Title".to_string(),
            "Test Body".to_string(),
        );
        
        manager.send(notification).await.unwrap();
        assert_eq!(manager.get_all().await.len(), 1);
    }

    #[tokio::test]
    async fn test_mark_as_read() {
        let config = NotificationConfig::default();
        let manager = NotificationManager::new(config);
        
        let mut notification = Notification::new(
            NotificationType::System {
                message: "Test".to_string(),
            },
            "Test Title".to_string(),
            "Test Body".to_string(),
        );
        
        let notification_id = notification.notification_id.clone();
        manager.send(notification).await.unwrap();
        
        manager.mark_as_read(&notification_id).await.unwrap();
        
        let notification = manager.get(&notification_id).await.unwrap();
        assert!(notification.read);
    }

    #[tokio::test]
    async fn test_get_unread() {
        let config = NotificationConfig::default();
        let manager = NotificationManager::new(config);
        
        let notification1 = Notification::new(
            NotificationType::System {
                message: "Test 1".to_string(),
            },
            "Test Title 1".to_string(),
            "Test Body 1".to_string(),
        );
        
        let mut notification2 = Notification::new(
            NotificationType::System {
                message: "Test 2".to_string(),
            },
            "Test Title 2".to_string(),
            "Test Body 2".to_string(),
        );
        
        notification2.mark_as_read();
        
        manager.send(notification1).await.unwrap();
        manager.send(notification2).await.unwrap();
        
        let unread = manager.get_unread().await;
        assert_eq!(unread.len(), 1);
    }
}