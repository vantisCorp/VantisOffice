//! FFI (Foreign Function Interface) module for mobile platforms
//!
//! This module provides C-compatible bindings for the vantis-mobile Rust library,
//! enabling integration with iOS (Swift) and Android (Kotlin) applications.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong};
use std::ptr;
use std::slice;

use crate::crypto::{Encryptor, EncryptionKey, KeyPair};
use crate::error::MobileError;
use crate::models::{DeviceInfo, DeviceType};

/// Opaque handle for Encryptor
pub struct VantisEncryptor {
    inner: Encryptor,
}

/// Opaque handle for KeyPair
pub struct VantisKeyPair {
    inner: KeyPair,
}

// ============================================================================
// Error Handling
// ============================================================================

/// Error codes for FFI operations
#[repr(C)]
pub enum VantisErrorCode {
    Success = 0,
    NullPointer = 1,
    InvalidUtf8 = 2,
    InvalidData = 3,
    EncryptionError = 4,
    DecryptionError = 5,
    ConnectionError = 6,
    InvalidState = 7,
    OutOfMemory = 8,
    Unknown = 99,
}

impl From<MobileError> for VantisErrorCode {
    fn from(error: MobileError) -> Self {
        match error {
            MobileError::Crypto(_) => VantisErrorCode::EncryptionError,
            MobileError::TunnelConnection(_) => VantisErrorCode::ConnectionError,
            MobileError::Network(_) => VantisErrorCode::ConnectionError,
            MobileError::NotConnected => VantisErrorCode::ConnectionError,
            MobileError::InvalidInput(_) => VantisErrorCode::InvalidData,
            _ => VantisErrorCode::Unknown,
        }
    }
}

/// Result type for FFI operations
#[repr(C)]
pub struct VantisResult {
    pub code: c_int,
    pub message: *mut c_char,
}

impl VantisResult {
    pub fn success() -> Self {
        VantisResult {
            code: VantisErrorCode::Success as c_int,
            message: ptr::null_mut(),
        }
    }

    pub fn error(code: VantisErrorCode, message: &str) -> Self {
        let c_message = CString::new(message).unwrap_or_default();
        VantisResult {
            code: code as c_int,
            message: c_message.into_raw(),
        }
    }
}

// ============================================================================
// String Helpers
// ============================================================================

/// Free a string allocated by Rust
#[no_mangle]
pub extern "C" fn vantis_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/// Free a byte buffer allocated by Rust
#[no_mangle]
pub extern "C" fn vantis_free_buffer(ptr: *mut c_uchar, len: c_uint) {
    if !ptr.is_null() && len > 0 {
        unsafe {
            let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
        }
    }
}

// ============================================================================
// Key Pair Operations
// ============================================================================

/// Generate a new key pair
#[no_mangle]
pub extern "C" fn vantis_keypair_generate() -> *mut VantisKeyPair {
    let keypair = match KeyPair::generate() {
        Ok(kp) => kp,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(VantisKeyPair { inner: keypair }))
}

/// Free a key pair
#[no_mangle]
pub extern "C" fn vantis_keypair_free(keypair: *mut VantisKeyPair) {
    if !keypair.is_null() {
        unsafe {
            let _ = Box::from_raw(keypair);
        }
    }
}

/// Get the public key as base64 string
#[no_mangle]
pub extern "C" fn vantis_keypair_public_key_base64(
    keypair: *const VantisKeyPair,
    out: *mut c_char,
    out_len: c_uint,
) -> VantisResult {
    if keypair.is_null() || out.is_null() {
        return VantisResult::error(VantisErrorCode::NullPointer, "Null pointer");
    }

    unsafe {
        let keypair = &*keypair;
        let base64 = keypair.inner.public_key_base64();

        if base64.len() + 1 > out_len as usize {
            return VantisResult::error(VantisErrorCode::InvalidData, "Buffer too small");
        }

        ptr::copy_nonoverlapping(base64.as_ptr(), out as *mut u8, base64.len());
        *out.add(base64.len()) = 0; // null terminator

        VantisResult::success()
    }
}

// ============================================================================
// Encryption Operations
// ============================================================================

/// Create a new encryptor from a base64-encoded shared secret
#[no_mangle]
pub extern "C" fn vantis_encryptor_create(
    shared_secret_base64: *const c_char,
) -> *mut VantisEncryptor {
    if shared_secret_base64.is_null() {
        return ptr::null_mut();
    }

    let secret_str = unsafe {
        match CStr::from_ptr(shared_secret_base64).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ptr::null_mut(),
        }
    };

    let key = match EncryptionKey::from_base64(&secret_str) {
        Ok(key) => key,
        Err(_) => return ptr::null_mut(),
    };

    let encryptor = match Encryptor::new(&key) {
        Ok(enc) => enc,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(VantisEncryptor { inner: encryptor }))
}

/// Free an encryptor
#[no_mangle]
pub extern "C" fn vantis_encryptor_free(encryptor: *mut VantisEncryptor) {
    if !encryptor.is_null() {
        unsafe {
            let _ = Box::from_raw(encryptor);
        }
    }
}

/// Encrypt data and return JSON-encoded encrypted message
/// Returns 0 on success, -1 on error, -2 if buffer too small
#[no_mangle]
pub extern "C" fn vantis_encrypt(
    encryptor: *mut VantisEncryptor,
    plaintext: *const c_uchar,
    plaintext_len: c_uint,
    json_out: *mut c_char,
    json_out_len: *mut c_uint,
) -> c_int {
    if encryptor.is_null() || plaintext.is_null() || json_out.is_null() || json_out_len.is_null() {
        return -1;
    }

    unsafe {
        let encryptor = &mut *encryptor;
        let data = slice::from_raw_parts(plaintext, plaintext_len as usize);
        let max_len = *json_out_len as usize;

        match encryptor.inner.encrypt(data) {
            Ok(encrypted) => {
                // Serialize the encrypted message to JSON
                match serde_json::to_string(&encrypted) {
                    Ok(json) => {
                        if json.len() + 1 > max_len {
                            return -2; // Buffer too small
                        }
                ptr::copy_nonoverlapping(json.as_ptr(), json_out as *mut u8, json.len());
                        *json_out.add(json.len()) = 0; // null terminator
                        *json_out_len = json.len() as c_uint;
                        0
                    }
                    Err(_) => -1,
                }
            }
            Err(_) => -1,
        }
    }
}

/// Decrypt JSON-encoded encrypted message
/// Returns 0 on success, -1 on error, -2 if buffer too small
#[no_mangle]
pub extern "C" fn vantis_decrypt(
    encryptor: *mut VantisEncryptor,
    json_encrypted: *const c_char,
    plaintext_out: *mut c_uchar,
    plaintext_len_out: *mut c_uint,
) -> c_int {
    if encryptor.is_null() || json_encrypted.is_null() || plaintext_out.is_null() || plaintext_len_out.is_null() {
        return -1;
    }

    unsafe {
        let encryptor = &mut *encryptor;
        let json_str = match CStr::from_ptr(json_encrypted).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        };

        // Deserialize the encrypted message from JSON
        let encrypted: crate::crypto::EncryptedMessage = match serde_json::from_str(json_str) {
            Ok(msg) => msg,
            Err(_) => return -1,
        };

        let max_len = *plaintext_len_out as usize;

        match encryptor.inner.decrypt(&encrypted) {
            Ok(decrypted) => {
                if decrypted.len() > max_len {
                    return -2; // Buffer too small
                }
                ptr::copy_nonoverlapping(decrypted.as_ptr(), plaintext_out, decrypted.len());
                *plaintext_len_out = decrypted.len() as c_uint;
                0
            }
            Err(_) => -1,
        }
    }
}

// ============================================================================
// Device Info Operations
// ============================================================================

/// Opaque device info handle
pub struct VantisDeviceInfo {
    inner: DeviceInfo,
}

/// Create a new device info
#[no_mangle]
pub extern "C" fn vantis_device_info_create(
    name: *const c_char,
    device_type: c_int,
    os_version: *const c_char,
    app_version: *const c_char,
) -> *mut VantisDeviceInfo {
    if name.is_null() || os_version.is_null() || app_version.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        let name = match CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ptr::null_mut(),
        };

        let os_version = match CStr::from_ptr(os_version).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ptr::null_mut(),
        };

        let app_version = match CStr::from_ptr(app_version).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return ptr::null_mut(),
        };

        let device_type = match device_type {
            0 => DeviceType::Ios,       // iOS device
            1 => DeviceType::Android,   // Android device
            2 => DeviceType::Desktop,   // Desktop computer
            3 => DeviceType::Laptop,    // Laptop
            4 => DeviceType::Tablet,    // Tablet
            _ => DeviceType::Ios,       // Default to iOS
        };

        let info = DeviceInfo::new(device_type, name, os_version, app_version);

        Box::into_raw(Box::new(VantisDeviceInfo { inner: info }))
    }
}

/// Free device info
#[no_mangle]
pub extern "C" fn vantis_device_info_free(info: *mut VantisDeviceInfo) {
    if !info.is_null() {
        unsafe {
            let _ = Box::from_raw(info);
        }
    }
}

/// Get device info as JSON
#[no_mangle]
pub extern "C" fn vantis_device_info_to_json(
    info: *const VantisDeviceInfo,
    out: *mut c_char,
    out_len: c_uint,
) -> VantisResult {
    if info.is_null() || out.is_null() {
        return VantisResult::error(VantisErrorCode::NullPointer, "Null pointer");
    }

    unsafe {
        let info = &*info;
        match serde_json::to_string(&info.inner) {
            Ok(json) => {
                if json.len() + 1 > out_len as usize {
                    return VantisResult::error(VantisErrorCode::InvalidData, "Buffer too small");
                }
                ptr::copy_nonoverlapping(json.as_ptr(), out as *mut u8, json.len());
                *out.add(json.len()) = 0;
                VantisResult::success()
            }
            Err(e) => VantisResult::error(VantisErrorCode::InvalidData, &e.to_string()),
        }
    }
}

// ============================================================================
// Protocol Message Operations
// ============================================================================

/// Create a ping message
#[no_mangle]
pub extern "C" fn vantis_message_ping(
    out: *mut c_char,
    out_len: c_uint,
) -> VantisResult {
    if out.is_null() {
        return VantisResult::error(VantisErrorCode::NullPointer, "Null pointer");
    }

    let msg = serde_json::json!({
        "type": "ping",
        "timestamp": chrono::Utc::now().timestamp_millis()
    });

    unsafe {
        match serde_json::to_string(&msg) {
            Ok(json) => {
                if json.len() + 1 > out_len as usize {
                    return VantisResult::error(VantisErrorCode::InvalidData, "Buffer too small");
                }
                ptr::copy_nonoverlapping(json.as_ptr(), out as *mut u8, json.len());
                *out.add(json.len()) = 0;
                VantisResult::success()
            }
            Err(e) => VantisResult::error(VantisErrorCode::InvalidData, &e.to_string()),
        }
    }
}

/// Create a sync request message
#[no_mangle]
pub extern "C" fn vantis_message_sync_request(
    last_sync_timestamp: c_ulong,
    out: *mut c_char,
    out_len: c_uint,
) -> VantisResult {
    if out.is_null() {
        return VantisResult::error(VantisErrorCode::NullPointer, "Null pointer");
    }

    let msg = serde_json::json!({
        "type": "sync_request",
        "last_sync_timestamp": last_sync_timestamp
    });

    unsafe {
        match serde_json::to_string(&msg) {
            Ok(json) => {
                if json.len() + 1 > out_len as usize {
                    return VantisResult::error(VantisErrorCode::InvalidData, "Buffer too small");
                }
                ptr::copy_nonoverlapping(json.as_ptr(), out as *mut u8, json.len());
                *out.add(json.len()) = 0;
                VantisResult::success()
            }
            Err(e) => VantisResult::error(VantisErrorCode::InvalidData, &e.to_string()),
        }
    }
}

/// Create a notification message
#[no_mangle]
pub extern "C" fn vantis_message_notification(
    title: *const c_char,
    body: *const c_char,
    notification_type: *const c_char,
    priority: c_int,
    out: *mut c_char,
    out_len: c_uint,
) -> VantisResult {
    if title.is_null() || body.is_null() || out.is_null() {
        return VantisResult::error(VantisErrorCode::NullPointer, "Null pointer");
    }

    unsafe {
        let title = match CStr::from_ptr(title).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return VantisResult::error(VantisErrorCode::InvalidUtf8, "Invalid title"),
        };

        let body = match CStr::from_ptr(body).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return VantisResult::error(VantisErrorCode::InvalidUtf8, "Invalid body"),
        };

        let ntype = if notification_type.is_null() {
            "info".to_string()
        } else {
            match CStr::from_ptr(notification_type).to_str() {
                Ok(s) => s.to_string(),
                Err(_) => "info".to_string(),
            }
        };

        let priority_str = match priority {
            0 => "low",
            1 => "normal",
            2 => "high",
            3 => "urgent",
            _ => "normal",
        };

        let msg = serde_json::json!({
            "type": "notification",
            "title": title,
            "body": body,
            "notification_type": ntype,
            "priority": priority_str
        });

        match serde_json::to_string(&msg) {
            Ok(json) => {
                if json.len() + 1 > out_len as usize {
                    return VantisResult::error(VantisErrorCode::InvalidData, "Buffer too small");
                }
                ptr::copy_nonoverlapping(json.as_ptr(), out as *mut u8, json.len());
                *out.add(json.len()) = 0;
                VantisResult::success()
            }
            Err(e) => VantisResult::error(VantisErrorCode::InvalidData, &e.to_string()),
        }
    }
}

// ============================================================================
// Library Version
// ============================================================================

/// Get library version
#[no_mangle]
pub extern "C" fn vantis_version(out: *mut c_char, out_len: c_uint) -> VantisResult {
    if out.is_null() {
        return VantisResult::error(VantisErrorCode::NullPointer, "Null pointer");
    }

    let version = env!("CARGO_PKG_VERSION");

    unsafe {
        if version.len() + 1 > out_len as usize {
            return VantisResult::error(VantisErrorCode::InvalidData, "Buffer too small");
        }
        ptr::copy_nonoverlapping(version.as_ptr(), out as *mut u8, version.len());
        *out.add(version.len()) = 0;
        VantisResult::success()
    }
}

// ============================================================================
// iOS Swift Compatibility Layer
// ============================================================================

/// Initialize the library (call once at app startup)
#[no_mangle]
pub extern "C" fn vantis_initialize() -> c_int {
    // Initialize any global state if needed
    0
}

/// Cleanup the library (call once at app shutdown)
#[no_mangle]
pub extern "C" fn vantis_cleanup() {
    // Cleanup any global state if needed
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_keypair_generate() {
        let keypair = vantis_keypair_generate();
        assert!(!keypair.is_null());

        let mut buffer = vec![0i8; 128];
        let result = vantis_keypair_public_key_base64(
            keypair,
            buffer.as_mut_ptr(),
            buffer.len() as c_uint,
        );
        assert_eq!(result.code, 0);

        vantis_keypair_free(keypair);
    }

    #[test]
    fn test_version() {
        let mut buffer = vec![0i8; 32];
        let result = vantis_version(buffer.as_mut_ptr(), buffer.len() as c_uint);
        assert_eq!(result.code, 0);

        let version = unsafe { CStr::from_ptr(buffer.as_ptr()) };
        assert!(!version.to_str().unwrap().is_empty());
    }

    #[test]
    fn test_encrypt_decrypt() {
        // Generate a key pair and derive a shared secret
        let keypair = vantis_keypair_generate();
        assert!(!keypair.is_null());

        // Get the public key
        let mut pub_key_buffer = vec![0i8; 64];
        let result = vantis_keypair_public_key_base64(
            keypair,
            pub_key_buffer.as_mut_ptr(),
            pub_key_buffer.len() as c_uint,
        );
        assert_eq!(result.code, 0);

        // Create a random encryption key for testing
        let key = EncryptionKey::generate().unwrap();
        let key_base64 = key.to_base64();
        let key_cstr = CString::new(key_base64).unwrap();

        let encryptor = vantis_encryptor_create(key_cstr.as_ptr());
        assert!(!encryptor.is_null());

        let plaintext = b"Hello, World!";
        let mut json_buffer = vec![0i8; 1024];
        let mut json_len = json_buffer.len() as c_uint;

        let result = vantis_encrypt(
            encryptor,
            plaintext.as_ptr(),
            plaintext.len() as c_uint,
            json_buffer.as_mut_ptr(),
            &mut json_len,
        );
        assert_eq!(result, 0);

        let mut decrypted = vec![0u8; 1024];
        let mut decrypted_len = decrypted.len() as c_uint;

        let result = vantis_decrypt(
            encryptor,
            json_buffer.as_ptr(),
            decrypted.as_mut_ptr(),
            &mut decrypted_len,
        );
        assert_eq!(result, 0);

        assert_eq!(&decrypted[..decrypted_len as usize], plaintext);

        vantis_encryptor_free(encryptor);
        vantis_keypair_free(keypair);
    }
}