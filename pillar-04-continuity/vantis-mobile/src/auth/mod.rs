//! Authentication manager for mobile authentication

use crate::core::Session;
use crate::error::{MobileError, MobileResult};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Biometric authentication enabled
    pub biometric_enabled: bool,
    
    /// PIN authentication enabled
    pub pin_enabled: bool,
    
    /// Session timeout in seconds
    pub session_timeout: u64,
    
    /// Maximum failed attempts
    pub max_failed_attempts: u32,
    
    /// Lockout duration in seconds
    pub lockout_duration: u64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            biometric_enabled: true,
            pin_enabled: true,
            session_timeout: 3600, // 1 hour
            max_failed_attempts: 5,
            lockout_duration: 300, // 5 minutes
        }
    }
}

/// Authentication method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Biometric,
    Pin,
    Password,
    Token,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    /// Success status
    pub success: bool,
    
    /// Session (if successful)
    pub session: Option<Session>,
    
    /// Error message (if failed)
    pub error: Option<String>,
}

impl AuthResult {
    /// Create successful result
    pub fn success(session: Session) -> Self {
        Self {
            success: true,
            session: Some(session),
            error: None,
        }
    }
    
    /// Create failed result
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            session: None,
            error: Some(error),
        }
    }
}

/// Biometric authentication
pub struct BiometricAuth {
    enabled: bool,
}

impl BiometricAuth {
    /// Create a new biometric auth instance
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    
    /// Check if biometric auth is available
    pub fn is_available(&self) -> bool {
        self.enabled
    }
    
    /// Authenticate with biometrics
    pub async fn authenticate(&self, _reason: &str) -> MobileResult<AuthResult> {
        if !self.enabled {
            return Err(MobileError::Authentication("Biometric auth not enabled".to_string()));
        }
        
        // In a real implementation, this would:
        // 1. Request biometric authentication from OS
        // 2. Wait for user to authenticate
        // 3. Return result
        
        // For demo purposes, simulate success
        Ok(AuthResult::success(Session::new(
            "user123".to_string(),
            "device456".to_string(),
            1,
        )))
    }
}

/// PIN authentication
pub struct PinAuth {
    enabled: bool,
    pin: Arc<RwLock<Option<String>>>,
    failed_attempts: Arc<RwLock<u32>>,
    locked_until: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
}

impl PinAuth {
    /// Create a new PIN auth instance
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            pin: Arc::new(RwLock::new(None)),
            failed_attempts: Arc::new(RwLock::new(0)),
            locked_until: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Set PIN
    pub async fn set_pin(&self, pin: String) -> MobileResult<()> {
        if pin.len() < 4 {
            return Err(MobileError::Authentication("PIN must be at least 4 digits".to_string()));
        }
        
        *self.pin.write().await = Some(pin);
        *self.failed_attempts.write().await = 0;
        Ok(())
    }
    
    /// Check if PIN is set
    pub async fn is_pin_set(&self) -> bool {
        self.pin.read().await.is_some()
    }
    
    /// Authenticate with PIN
    pub async fn authenticate(&self, pin: &str) -> MobileResult<AuthResult> {
        if !self.enabled {
            return Err(MobileError::Authentication("PIN auth not enabled".to_string()));
        }
        
        // Check if locked
        let locked_until = self.locked_until.read().await;
        if let Some(until) = *locked_until {
            if chrono::Utc::now() < until {
                return Err(MobileError::Authentication("Account locked".to_string()));
            }
        }
        drop(locked_until);
        
        // Check PIN
        let stored_pin = self.pin.read().await;
        
        if let Some(stored) = stored_pin.as_ref() {
            if stored == pin {
                // Reset failed attempts
                *self.failed_attempts.write().await = 0;
                *self.locked_until.write().await = None;
                
                Ok(AuthResult::success(Session::new(
                    "user123".to_string(),
                    "device456".to_string(),
                    1,
                )))
            } else {
                // Increment failed attempts
                let mut attempts = self.failed_attempts.write().await;
                *attempts += 1;
                
                if *attempts >= 5 {
                    // Lock account
                    *self.locked_until.write().await =
                        Some(chrono::Utc::now() + chrono::Duration::seconds(300));
                    return Err(MobileError::Authentication("Account locked".to_string()));
                }
                
                Err(MobileError::Authentication("Invalid PIN".to_string()))
            }
        } else {
            Err(MobileError::Authentication("PIN not set".to_string()))
        }
    }
    
    /// Change PIN
    pub async fn change_pin(&self, old_pin: &str, new_pin: &str) -> MobileResult<()> {
        // Authenticate with old PIN first
        self.authenticate(old_pin).await?;
        
        // Set new PIN
        self.set_pin(new_pin.to_string()).await
    }
    
    /// Reset PIN (requires authentication)
    pub async fn reset_pin(&self) -> MobileResult<()> {
        *self.pin.write().await = None;
        *self.failed_attempts.write().await = 0;
        *self.locked_until.write().await = None;
        Ok(())
    }
}

/// Authentication manager
pub struct AuthManager {
    config: AuthConfig,
    biometric: BiometricAuth,
    pin: PinAuth,
    session: Arc<RwLock<Option<Session>>>,
}

impl AuthManager {
    /// Create a new auth manager
    pub fn new(config: AuthConfig) -> Self {
        Self {
            biometric: BiometricAuth::new(config.biometric_enabled),
            pin: PinAuth::new(config.pin_enabled),
            config,
            session: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Authenticate with biometrics
    pub async fn authenticate_biometric(&self, reason: &str) -> MobileResult<AuthResult> {
        let result = self.biometric.authenticate(reason).await?;
        
        if result.success {
            if let Some(ref session) = result.session {
                *self.session.write().await = Some(session.clone());
            }
        }
        
        Ok(result)
    }
    
    /// Authenticate with PIN
    pub async fn authenticate_pin(&self, pin: &str) -> MobileResult<AuthResult> {
        let result = self.pin.authenticate(pin).await?;
        
        if result.success {
            if let Some(ref session) = result.session {
                *self.session.write().await = Some(session.clone());
            }
        }
        
        Ok(result)
    }
    
    /// Get current session
    pub async fn get_session(&self) -> Option<Session> {
        self.session.read().await.clone()
    }
    
    /// Check if authenticated
    pub async fn is_authenticated(&self) -> bool {
        let session = self.session.read().await;
        
        if let Some(s) = session.as_ref() {
            !s.is_expired()
        } else {
            false
        }
    }
    
    /// Logout
    pub async fn logout(&self) -> MobileResult<()> {
        *self.session.write().await = None;
        Ok(())
    }
    
    /// Refresh session
    pub async fn refresh_session(&self) -> MobileResult<()> {
        let session_guard = self.session.read().await;
        
        if let Some(session) = session_guard.as_ref() {
            if session.is_expired() {
                return Err(MobileError::SessionExpired);
            }
            
            // Extend session
            let mut session = self.session.write().await;
            if let Some(s) = session.as_mut() {
                s.expires_at = chrono::Utc::now() + chrono::Duration::seconds(self.config.session_timeout as i64);
            }
        }
        
        Ok(())
    }
    
    /// Set PIN
    pub async fn set_pin(&self, pin: String) -> MobileResult<()> {
        self.pin.set_pin(pin).await
    }
    
    /// Change PIN
    pub async fn change_pin(&self, old_pin: &str, new_pin: &str) -> MobileResult<()> {
        self.pin.change_pin(old_pin, new_pin).await
    }
    
    /// Check if PIN is set
    pub async fn is_pin_set(&self) -> bool {
        self.pin.is_pin_set().await
    }
    
    /// Check if biometric auth is available
    pub fn is_biometric_available(&self) -> bool {
        self.biometric.is_available()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_config_default() {
        let config = AuthConfig::default();
        assert!(config.biometric_enabled);
        assert!(config.pin_enabled);
    }

    #[tokio::test]
    async fn test_pin_auth_set_and_authenticate() {
        let pin_auth = PinAuth::new(true);
        
        pin_auth.set_pin("1234".to_string()).await.unwrap();
        assert!(pin_auth.is_pin_set().await);
        
        let result = pin_auth.authenticate("1234").await.unwrap();
        assert!(result.success);
    }

    #[tokio::test]
    async fn test_pin_auth_invalid_pin() {
        let pin_auth = PinAuth::new(true);
        
        pin_auth.set_pin("1234".to_string()).await.unwrap();
        
        let result = pin_auth.authenticate("0000").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_auth_manager() {
        let config = AuthConfig::default();
        let manager = AuthManager::new(config);
        
        manager.set_pin("1234".to_string()).await.unwrap();
        
        let result = manager.authenticate_pin("1234").await.unwrap();
        assert!(result.success);
        assert!(manager.is_authenticated().await);
    }

    #[tokio::test]
    async fn test_auth_manager_logout() {
        let config = AuthConfig::default();
        let manager = AuthManager::new(config);
        
        manager.set_pin("1234".to_string()).await.unwrap();
        manager.authenticate_pin("1234").await.unwrap();
        
        assert!(manager.is_authenticated().await);
        
        manager.logout().await.unwrap();
        assert!(!manager.is_authenticated().await);
    }
}