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

    // ========================================================================
    // Cross-Platform FFI Integration Tests
    // ========================================================================

    #[test]
    fn test_ffi_kyber_full_roundtrip() {
        // Test all three Kyber security levels
        for level in [1, 2, 3].iter() {
            // Get key sizes
            let mut pk_size: size_t = 0;
            let mut sk_size: size_t = 0;
            let mut ct_size: size_t = 0;
            let mut ss_size: size_t = 0;

            let result = pqc_kyber_get_key_sizes(
                *level,
                &mut pk_size,
                &mut sk_size,
                &mut ct_size,
                &mut ss_size,
            );
            assert_eq!(result, FFIResult::Success as c_int);

            // Allocate buffers
            let mut public_key = vec![0u8; pk_size];
            let mut private_key = vec![0u8; sk_size];
            let mut actual_pk_len = pk_size;
            let mut actual_sk_len = sk_size;

            // Generate keypair
            let result = pqc_kyber_generate_keypair(
                *level,
                public_key.as_mut_ptr(),
                &mut actual_pk_len,
                private_key.as_mut_ptr(),
                &mut actual_sk_len,
            );
            assert_eq!(result, FFIResult::Success as c_int);
            assert_eq!(actual_pk_len, pk_size);
            assert_eq!(actual_sk_len, sk_size);

            // Encapsulate
            let mut ciphertext = vec![0u8; ct_size];
            let mut shared_secret1 = vec![0u8; ss_size];
            let mut actual_ct_len = ct_size;
            let mut actual_ss_len = ss_size;

            let result = pqc_kyber_encapsulate(
                public_key.as_ptr(),
                pk_size,
                ciphertext.as_mut_ptr(),
                &mut actual_ct_len,
                shared_secret1.as_mut_ptr(),
                &mut actual_ss_len,
            );
            assert_eq!(result, FFIResult::Success as c_int);
            assert_eq!(actual_ct_len, ct_size);
            assert_eq!(actual_ss_len, ss_size);

            // Decapsulate
            let mut shared_secret2 = vec![0u8; ss_size];
            let mut actual_ss2_len = ss_size;

            let result = pqc_kyber_decapsulate(
                private_key.as_ptr(),
                sk_size,
                ciphertext.as_ptr(),
                ct_size,
                shared_secret2.as_mut_ptr(),
                &mut actual_ss2_len,
            );
            assert_eq!(result, FFIResult::Success as c_int);
            assert_eq!(actual_ss2_len, ss_size);

            // Verify shared secrets match
            assert_eq!(shared_secret1, shared_secret2);
        }
    }

    #[test]
    fn test_ffi_dilithium_full_roundtrip() {
        // Test all three Dilithium security levels
        for level in [2, 3, 5].iter() {
            // Get key sizes
            let mut pk_size: size_t = 0;
            let mut sk_size: size_t = 0;
            let mut sig_size: size_t = 0;

            let result = pqc_dilithium_get_key_sizes(
                *level,
                &mut pk_size,
                &mut sk_size,
                &mut sig_size,
            );
            assert_eq!(result, FFIResult::Success as c_int);

            // Allocate buffers
            let mut public_key = vec![0u8; pk_size];
            let mut private_key = vec![0u8; sk_size];
            let mut actual_pk_len = pk_size;
            let mut actual_sk_len = sk_size;

            // Generate keypair
            let result = pqc_dilithium_generate_keypair(
                *level,
                public_key.as_mut_ptr(),
                &mut actual_pk_len,
                private_key.as_mut_ptr(),
                &mut actual_sk_len,
            );
            assert_eq!(result, FFIResult::Success as c_int);
            assert_eq!(actual_pk_len, pk_size);
            assert_eq!(actual_sk_len, sk_size);

            // Sign a message - use a buffer large enough for any signature
            // Dilithium signatures can vary slightly in size
            let message = b"Test message for Dilithium signing";
            let mut signature = vec![0u8; 5000]; // Large buffer for any signature
            let mut actual_sig_len: size_t = 5000;

            let result = pqc_dilithium_sign(
                private_key.as_ptr(),
                sk_size,
                message.as_ptr(),
                message.len(),
                signature.as_mut_ptr(),
                &mut actual_sig_len,
            );
            assert_eq!(result, FFIResult::Success as c_int, "Sign failed for level {} with sig_len {}, expected {} max", level, actual_sig_len, sig_size);
            assert!(actual_sig_len <= 5000);

            // Truncate signature to actual size
            signature.truncate(actual_sig_len);

            // Verify signature
            let result = pqc_dilithium_verify(
                public_key.as_ptr(),
                pk_size,
                message.as_ptr(),
                message.len(),
                signature.as_ptr(),
                actual_sig_len,
            );
            assert_eq!(result, FFIResult::Success as c_int);
        }
    }

    #[test]
    fn test_ffi_kyber_buffer_too_small() {
        let mut pk_size: size_t = 0;
        let mut sk_size: size_t = 0;
        let mut ct_size: size_t = 0;
        let mut ss_size: size_t = 0;

        pqc_kyber_get_key_sizes(2, &mut pk_size, &mut sk_size, &mut ct_size, &mut ss_size);

        // Try with too small buffers
        let mut public_key = vec![0u8; 10]; // Too small
        let mut private_key = vec![0u8; 10]; // Too small
        let mut actual_pk_len = 10usize;
        let mut actual_sk_len = 10usize;

        let result = pqc_kyber_generate_keypair(
            2,
            public_key.as_mut_ptr(),
            &mut actual_pk_len,
            private_key.as_mut_ptr(),
            &mut actual_sk_len,
        );

        assert_eq!(result, FFIResult::ErrorBufferTooSmall as c_int);
        // Should return required sizes
        assert_eq!(actual_pk_len, pk_size);
        assert_eq!(actual_sk_len, sk_size);
    }

    #[test]
    fn test_ffi_kyber_null_pointers() {
        let mut len: size_t = 100;

        // Null public key
        let result = pqc_kyber_generate_keypair(
            2,
            std::ptr::null_mut(),
            &mut len,
            vec![0u8; 100].as_mut_ptr(),
            &mut len,
        );
        assert_eq!(result, FFIResult::ErrorInvalidInput as c_int);

        // Null private key
        let result = pqc_kyber_generate_keypair(
            2,
            vec![0u8; 100].as_mut_ptr(),
            &mut len,
            std::ptr::null_mut(),
            &mut len,
        );
        assert_eq!(result, FFIResult::ErrorInvalidInput as c_int);
    }

    #[test]
    fn test_ffi_dilithium_verify_wrong_signature() {
        let mut pk_size: size_t = 0;
        let mut sk_size: size_t = 0;
        let mut sig_size: size_t = 0;

        pqc_dilithium_get_key_sizes(3, &mut pk_size, &mut sk_size, &mut sig_size);

        // Generate keypair
        let mut public_key = vec![0u8; pk_size];
        let mut private_key = vec![0u8; sk_size];
        let mut actual_pk_len = pk_size;
        let mut actual_sk_len = sk_size;

        pqc_dilithium_generate_keypair(
            3,
            public_key.as_mut_ptr(),
            &mut actual_pk_len,
            private_key.as_mut_ptr(),
            &mut actual_sk_len,
        );

        let message = b"Original message";
        let wrong_message = b"Different message";

        // Sign original message
        let mut signature = vec![0u8; sig_size * 2];
        let mut actual_sig_len = sig_size * 2;

        pqc_dilithium_sign(
            private_key.as_ptr(),
            sk_size,
            message.as_ptr(),
            message.len(),
            signature.as_mut_ptr(),
            &mut actual_sig_len,
        );

        // Verify with wrong message
        let result = pqc_dilithium_verify(
            public_key.as_ptr(),
            pk_size,
            wrong_message.as_ptr(),
            wrong_message.len(),
            signature.as_ptr(),
            actual_sig_len,
        );

        assert_eq!(result, FFIResult::ErrorVerification as c_int);
    }

    #[test]
    fn test_ffi_invalid_security_levels() {
        // Invalid Kyber security level
        let result = pqc_kyber_get_key_sizes(99, &mut 0, &mut 0, &mut 0, &mut 0);
        assert_eq!(result, FFIResult::ErrorInvalidInput as c_int);

        // Invalid Dilithium security level
        let result = pqc_dilithium_get_key_sizes(99, &mut 0, &mut 0, &mut 0);
        assert_eq!(result, FFIResult::ErrorInvalidInput as c_int);
    }

    #[test]
    fn test_ffi_cross_platform_consistency() {
        // This test verifies that the FFI produces consistent results
        // across multiple calls, simulating cross-platform behavior

        // Generate keypair
        let mut pk_size: size_t = 1184;
        let mut sk_size: size_t = 2400;
        let mut public_key = vec![0u8; pk_size];
        let mut private_key = vec![0u8; sk_size];

        pqc_kyber_generate_keypair(
            2,
            public_key.as_mut_ptr(),
            &mut pk_size,
            private_key.as_mut_ptr(),
            &mut sk_size,
        );

        // Multiple encapsulations should produce different ciphertexts
        // but decapsulation should always work
        for _ in 0..5 {
            let mut ct_size: size_t = 1088;
            let mut ss_size: size_t = 32;
            let mut ciphertext = vec![0u8; ct_size];
            let mut shared_secret1 = vec![0u8; ss_size];

            pqc_kyber_encapsulate(
                public_key.as_ptr(),
                pk_size,
                ciphertext.as_mut_ptr(),
                &mut ct_size,
                shared_secret1.as_mut_ptr(),
                &mut ss_size,
            );

            let mut shared_secret2 = vec![0u8; 32];
            let mut ss2_len: size_t = 32;

            pqc_kyber_decapsulate(
                private_key.as_ptr(),
                sk_size,
                ciphertext.as_ptr(),
                ct_size,
                shared_secret2.as_mut_ptr(),
                &mut ss2_len,
            );

            assert_eq!(shared_secret1, shared_secret2);
        }
    }
}