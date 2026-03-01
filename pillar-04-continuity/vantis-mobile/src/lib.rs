//! Vantis Mobile - Secure mobile companion for VantisOffice
//! 
//! This library provides the core functionality for mobile apps (iOS/Android)
//! to securely connect to and control VantisOffice applications.

pub mod error;
pub mod core;
pub mod tunnel;
pub mod sync;
pub mod notification;
pub mod auth;
pub mod protocol;

pub use error::{MobileError, MobileResult};
pub use core::{
    AppConfig, DeviceInfo, Session,
    Document, DocumentFormat, DocumentMetadata,
};
pub use tunnel::{
    SecureTunnel, TunnelConfig, TunnelState,
    KeyExchange, EncryptionKey,
};
pub use sync::{
    SyncManager, SyncStatus, SyncDirection,
    DocumentSync, ChangeSet, Change, ChangeType, SyncConfig,
};
pub use notification::{
    NotificationManager, Notification, NotificationType,
    NotificationAction, NotificationCategory, NotificationConfig, ActionType,
};
pub use auth::{
    AuthManager, AuthMethod, AuthResult,
    BiometricAuth, PinAuth, AuthConfig,
};
pub use protocol::{
    Message, MessageType, Command, Response,
    ProtocolVersion, ProtocolHandler,
};

/// Vantis Mobile Application
pub struct VantisMobileApp {
    config: AppConfig,
    tunnel: Option<SecureTunnel>,
    sync: Option<SyncManager>,
    notification: Option<NotificationManager>,
    auth: Option<AuthManager>,
}

impl VantisMobileApp {
    /// Create a new Vantis Mobile application instance
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            tunnel: None,
            sync: None,
            notification: None,
            auth: None,
        }
    }

    /// Initialize the application
    pub async fn initialize(&mut self) -> MobileResult<()> {
        // Initialize tunnel
        let tunnel = SecureTunnel::new(self.config.tunnel_config.clone())?;
        self.tunnel = Some(tunnel);

        // Initialize sync manager
        let sync = SyncManager::new(self.config.sync_config.clone());
        self.sync = Some(sync);

        // Initialize notification manager
        let notification = NotificationManager::new(self.config.notification_config.clone());
        self.notification = Some(notification);

        // Initialize auth manager
        let auth = AuthManager::new(self.config.auth_config.clone());
        self.auth = Some(auth);

        Ok(())
    }

    /// Connect to the VantisOffice server
    pub async fn connect(&mut self) -> MobileResult<()> {
        if let Some(tunnel) = &mut self.tunnel {
            tunnel.connect().await?;
        }
        Ok(())
    }

    /// Disconnect from the server
    pub async fn disconnect(&mut self) -> MobileResult<()> {
        if let Some(tunnel) = &mut self.tunnel {
            tunnel.disconnect().await?;
        }
        Ok(())
    }

    /// Get the tunnel instance
    pub fn tunnel(&self) -> Option<&SecureTunnel> {
        self.tunnel.as_ref()
    }

    /// Get the sync manager
    pub fn sync(&self) -> Option<&SyncManager> {
        self.sync.as_ref()
    }

    /// Get the notification manager
    pub fn notification(&self) -> Option<&NotificationManager> {
        self.notification.as_ref()
    }

    /// Get the auth manager
    pub fn auth(&self) -> Option<&AuthManager> {
        self.auth.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let config = AppConfig::default();
        let app = VantisMobileApp::new(config);
        assert!(app.tunnel().is_none());
        assert!(app.sync().is_none());
    }
}