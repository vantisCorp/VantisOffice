// FFI bindings for Post-Quantum Cryptography
// Provides C-compatible interface for mobile platforms

use crate::error::PQCError;
use crate::kyber::{KyberSecurityLevel, encapsulate, decapsulate, generate_keypair};
use crate::dilithium::{DilithiumSecurityLevel, sign, verify, generate_keypair as dilithium_keypair};
use libc::{c_int, c_uchar, size_t};
use std::slice;

/// FFI result codes
#[repr(C)]
pub enum FFIResult {
    Success = 0,
    ErrorInvalidInput = -1,
    ErrorInvalidKey = -2,
    ErrorEncryption = -3,
    ErrorDecryption = -4,
    ErrorSigning = -5,
    ErrorVerification = -6,
    ErrorBufferTooSmall = -7,
}

impl From<Result<(), PQCError>> for FFIResult {
    fn from(result: Result<(), PQCError>) -> Self {
        match result {
            Ok(()) => FFIResult::Success,
            Err(PQCError::InvalidPublicKey) | Err(PQCError::InvalidPrivateKey) => FFIResult::ErrorInvalidKey,
            Err(PQCError::EncapsulationFailed(_)) => FFIResult::ErrorEncryption,
            Err(PQCError::DecapsulationFailed(_)) => FFIResult::ErrorDecryption,
            _ => FFIResult::ErrorInvalidInput,
        }
    }
}

// ============================================================================
// Kyber KEM FFI Functions
// ============================================================================

/// Generate a Kyber key pair
/// 
/// # Arguments
/// * `security_level` - 1 for Kyber512, 2 for Kyber768, 3 for Kyber1024
/// * `public_key_out` - Buffer for public key output
/// * `public_key_len` - Size of public key buffer (input), actual size (output)
/// * `private_key_out` - Buffer for private key output
/// * `private_key_len` - Size of private key buffer (input), actual size (output)
#[no_mangle]
pub extern "C" fn pqc_kyber_generate_keypair(
    security_level: c_int,
    public_key_out: *mut c_uchar,
    public_key_len: *mut size_t,
    private_key_out: *mut c_uchar,
    private_key_len: *mut size_t,
) -> c_int {
    // Validate inputs
    if public_key_out.is_null() || private_key_out.is_null() {
        return FFIResult::ErrorInvalidInput as c_int;
    }

    // Convert security level
    let level = match security_level {
        1 => KyberSecurityLevel::Level1,
        2 => KyberSecurityLevel::Level2,
        3 => KyberSecurityLevel::Level3,
        _ => return FFIResult::ErrorInvalidInput as c_int,
    };

    // Generate key pair
    let keypair = match generate_keypair(level) {
        Ok(kp) => kp,
        Err(_) => return FFIResult::ErrorInvalidKey as c_int,
    };

    // Check buffer sizes
    unsafe {
        if *public_key_len < keypair.public_key.len() || *private_key_len < keypair.private_key.len() {
            *public_key_len = keypair.public_key.len();
            *private_key_len = keypair.private_key.len();
            return FFIResult::ErrorBufferTooSmall as c_int;
        }

        // Copy data
        std::ptr::copy_nonoverlapping(
            keypair.public_key.as_ptr(),
            public_key_out,
            keypair.public_key.len(),
        );
        std::ptr::copy_nonoverlapping(
            keypair.private_key.as_ptr(),
            private_key_out,
            keypair.private_key.len(),
        );

        *public_key_len = keypair.public_key.len();
        *private_key_len = keypair.private_key.len();
    }

    FFIResult::Success as c_int
}

/// Get expected Kyber key sizes for a security level
#[no_mangle]
pub extern "C" fn pqc_kyber_get_key_sizes(
    security_level: c_int,
    public_key_size: *mut size_t,
    private_key_size: *mut size_t,
    ciphertext_size: *mut size_t,
    shared_secret_size: *mut size_t,
) -> c_int {
    if public_key_size.is_null() || private_key_size.is_null() || ciphertext_size.is_null() || shared_secret_size.is_null() {
        return FFIResult::ErrorInvalidInput as c_int;
    }

    let level = match security_level {
        1 => KyberSecurityLevel::Level1,
        2 => KyberSecurityLevel::Level2,
        3 => KyberSecurityLevel::Level3,
        _ => return FFIResult::ErrorInvalidInput as c_int,
    };

    unsafe {
        *public_key_size = level.key_size();
        *private_key_size = match level {
            KyberSecurityLevel::Level1 => 1632,
            KyberSecurityLevel::Level2 => 2400,
            KyberSecurityLevel::Level3 => 3168,
        };
        *ciphertext_size = level.ciphertext_size();
        *shared_secret_size = level.shared_secret_size();
    }

    FFIResult::Success as c_int
}

/// Kyber encapsulation
#[no_mangle]
pub extern "C" fn pqc_kyber_encapsulate(
    public_key: *const c_uchar,
    public_key_len: size_t,
    ciphertext_out: *mut c_uchar,
    ciphertext_len: *mut size_t,
    shared_secret_out: *mut c_uchar,
    shared_secret_len: *mut size_t,
) -> c_int {
    // Validate inputs
    if public_key.is_null() || ciphertext_out.is_null() || shared_secret_out.is_null() {
        return FFIResult::ErrorInvalidInput as c_int;
    }

    // Get public key slice
    let public_key_slice = unsafe { slice::from_raw_parts(public_key, public_key_len) };

    // Perform encapsulation
    let (shared_secret, ciphertext) = match encapsulate(public_key_slice) {
        Ok(result) => result,
        Err(_) => return FFIResult::ErrorEncryption as c_int,
    };

    // Check buffer sizes and copy results
    unsafe {
        if *ciphertext_len < ciphertext.data.len() {
            *ciphertext_len = ciphertext.data.len();
            return FFIResult::ErrorBufferTooSmall as c_int;
        }
        if *shared_secret_len < shared_secret.len() {
            *shared_secret_len = shared_secret.len();
            return FFIResult::ErrorBufferTooSmall as c_int;
        }

        std::ptr::copy_nonoverlapping(
            ciphertext.data.as_ptr(),
            ciphertext_out,
            ciphertext.data.len(),
        );
        std::ptr::copy_nonoverlapping(
            shared_secret.as_ptr(),
            shared_secret_out,
            shared_secret.len(),
        );

        *ciphertext_len = ciphertext.data.len();
        *shared_secret_len = shared_secret.len();
    }

    FFIResult::Success as c_int
}

/// Kyber decapsulation
#[no_mangle]
pub extern "C" fn pqc_kyber_decapsulate(
    private_key: *const c_uchar,
    private_key_len: size_t,
    ciphertext: *const c_uchar,
    ciphertext_len: size_t,
    shared_secret_out: *mut c_uchar,
    shared_secret_len: *mut size_t,
) -> c_int {
    // Validate inputs
    if private_key.is_null() || ciphertext.is_null() || shared_secret_out.is_null() {
        return FFIResult::ErrorInvalidInput as c_int;
    }

    // Get slices
    let private_key_slice = unsafe { slice::from_raw_parts(private_key, private_key_len) };
    let ciphertext_slice = unsafe { slice::from_raw_parts(ciphertext, ciphertext_len) };

    // Perform decapsulation
    let shared_secret = match decapsulate(private_key_slice, ciphertext_slice) {
        Ok(result) => result,
        Err(_) => return FFIResult::ErrorDecryption as c_int,
    };

    // Check buffer size and copy result
    unsafe {
        if *shared_secret_len < shared_secret.len() {
            *shared_secret_len = shared_secret.len();
            return FFIResult::ErrorBufferTooSmall as c_int;
        }

        std::ptr::copy_nonoverlapping(
            shared_secret.as_ptr(),
            shared_secret_out,
            shared_secret.len(),
        );

        *shared_secret_len = shared_secret.len();
    }

    FFIResult::Success as c_int
}

// ============================================================================
// Dilithium Signature FFI Functions
// ============================================================================

/// Generate a Dilithium key pair
#[no_mangle]
pub extern "C" fn pqc_dilithium_generate_keypair(
    security_level: c_int,
    public_key_out: *mut c_uchar,
    public_key_len: *mut size_t,
    private_key_out: *mut c_uchar,
    private_key_len: *mut size_t,
) -> c_int {
    if public_key_out.is_null() || private_key_out.is_null() {
        return FFIResult::ErrorInvalidInput as c_int;
    }

    let level = match security_level {
        2 => DilithiumSecurityLevel::Level2,
        3 => DilithiumSecurityLevel::Level3,
        5 => DilithiumSecurityLevel::Level5,
        _ => return FFIResult::ErrorInvalidInput as c_int,
    };

    let keypair = match dilithium_keypair(level) {
        Ok(kp) => kp,
        Err(_) => return FFIResult::ErrorInvalidKey as c_int,
    };

    unsafe {
        if *public_key_len < keypair.public_key.len() || *private_key_len < keypair.private_key.len() {
            *public_key_len = keypair.public_key.len();
            *private_key_len = keypair.private_key.len();
            return FFIResult::ErrorBufferTooSmall as c_int;
        }

        std::ptr::copy_nonoverlapping(
            keypair.public_key.as_ptr(),
            public_key_out,
            keypair.public_key.len(),
        );
        std::ptr::copy_nonoverlapping(
            keypair.private_key.as_ptr(),
            private_key_out,
            keypair.private_key.len(),
        );

        *public_key_len = keypair.public_key.len();
        *private_key_len = keypair.private_key.len();
    }

    FFIResult::Success as c_int
}

/// Get expected Dilithium key sizes
#[no_mangle]
pub extern "C" fn pqc_dilithium_get_key_sizes(
    security_level: c_int,
    public_key_size: *mut size_t,
    private_key_size: *mut size_t,
    signature_size: *mut size_t,
) -> c_int {
    if public_key_size.is_null() || private_key_size.is_null() || signature_size.is_null() {
        return FFIResult::ErrorInvalidInput as c_int;
    }

    let level = match security_level {
        2 => DilithiumSecurityLevel::Level2,
        3 => DilithiumSecurityLevel::Level3,
        5 => DilithiumSecurityLevel::Level5,
        _ => return FFIResult::ErrorInvalidInput as c_int,
    };

    unsafe {
        *public_key_size = level.public_key_size();
        *private_key_size = level.private_key_size();
        *signature_size = level.signature_size();
    }

    FFIResult::Success as c_int
}

/// Dilithium signing
#[no_mangle]
pub extern "C" fn pqc_dilithium_sign(
    private_key: *const c_uchar,
    private_key_len: size_t,
    message: *const c_uchar,
    message_len: size_t,
    signature_out: *mut c_uchar,
    signature_len: *mut size_t,
) -> c_int {
    if private_key.is_null() || message.is_null() || signature_out.is_null() {
        return FFIResult::ErrorInvalidInput as c_int;
    }

    let private_key_slice = unsafe { slice::from_raw_parts(private_key, private_key_len) };
    let message_slice = unsafe { slice::from_raw_parts(message, message_len) };

    let signature = match sign(private_key_slice, message_slice) {
        Ok(sig) => sig,
        Err(_) => return FFIResult::ErrorSigning as c_int,
    };

    unsafe {
        if *signature_len < signature.data.len() {
            *signature_len = signature.data.len();
            return FFIResult::ErrorBufferTooSmall as c_int;
        }

        std::ptr::copy_nonoverlapping(
            signature.data.as_ptr(),
            signature_out,
            signature.data.len(),
        );

        *signature_len = signature.data.len();
    }

    FFIResult::Success as c_int
}

/// Dilithium verification
#[no_mangle]
pub extern "C" fn pqc_dilithium_verify(
    public_key: *const c_uchar,
    public_key_len: size_t,
    message: *const c_uchar,
    message_len: size_t,
    signature: *const c_uchar,
    signature_len: size_t,
) -> c_int {
    if public_key.is_null() || message.is_null() || signature.is_null() {
        return FFIResult::ErrorInvalidInput as c_int;
    }

    let public_key_slice = unsafe { slice::from_raw_parts(public_key, public_key_len) };
    let message_slice = unsafe { slice::from_raw_parts(message, message_len) };
    let signature_slice = unsafe { slice::from_raw_parts(signature, signature_len) };

    match verify(public_key_slice, message_slice, signature_slice) {
        Ok(true) => FFIResult::Success as c_int,
        Ok(false) => FFIResult::ErrorVerification as c_int,
        Err(_) => FFIResult::ErrorVerification as c_int,
    }
}

// ============================================================================
// Version and Utility Functions
// ============================================================================

/// Get library version
#[no_mangle]
pub extern "C" fn pqc_get_version() -> *const c_uchar {
    static VERSION: &[u8] = b"0.1.0\0";
    VERSION.as_ptr() as *const c_uchar
}

/// Get library name
#[no_mangle]
pub extern "C" fn pqc_get_name() -> *const c_uchar {
    static NAME: &[u8] = b"vantis-pqc\0";
    NAME.as_ptr() as *const c_uchar
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{c_int, CStr};

    #[test]
    fn test_ffi_result_codes() {
        assert_eq!(FFIResult::Success as c_int, 0);
        assert_eq!(FFIResult::ErrorInvalidInput as c_int, -1);
        assert_eq!(FFIResult::ErrorInvalidKey as c_int, -2);
    }

    #[test]
    fn test_version() {
        let version_ptr = pqc_get_version();
        let version = unsafe { CStr::from_ptr(version_ptr as *const i8) };
        assert!(version.to_str().unwrap().starts_with("0.1"));
    }

    #[test]
    fn test_kyber_key_sizes() {
        let mut pk_size: size_t = 0;
        let mut sk_size: size_t = 0;
        let mut ct_size: size_t = 0;
        let mut ss_size: size_t = 0;

        let result = pqc_kyber_get_key_sizes(
            2, // Kyber768
            &mut pk_size,
            &mut sk_size,
            &mut ct_size,
            &mut ss_size,
        );

        assert_eq!(result, FFIResult::Success as c_int);
        assert_eq!(pk_size, 1184);
        assert_eq!(ct_size, 1088);
        assert_eq!(ss_size, 32);
    }
}