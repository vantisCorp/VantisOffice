/**
 * VantisOffice Post-Quantum Cryptography Library
 * C Header for FFI bindings
 * 
 * Provides quantum-resistant cryptographic algorithms:
 * - Kyber KEM (Key Encapsulation Mechanism)
 * - Dilithium Signatures
 */

#ifndef VANTIS_PQC_H
#define VANTIS_PQC_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Result Codes
// ============================================================================

typedef enum {
    PQC_SUCCESS = 0,
    PQC_ERROR_INVALID_INPUT = -1,
    PQC_ERROR_INVALID_KEY = -2,
    PQC_ERROR_ENCRYPTION = -3,
    PQC_ERROR_DECRYPTION = -4,
    PQC_ERROR_SIGNING = -5,
    PQC_ERROR_VERIFICATION = -6,
    PQC_ERROR_BUFFER_TOO_SMALL = -7,
} PQCResult;

// ============================================================================
// Version Information
// ============================================================================

/**
 * Get library version string
 * @return Version string (e.g., "0.1.0")
 */
const uint8_t* pqc_get_version(void);

/**
 * Get library name
 * @return Library name string
 */
const uint8_t* pqc_get_name(void);

// ============================================================================
// Kyber Key Encapsulation Mechanism (KEM)
// ============================================================================

/**
 * Security levels for Kyber:
 * 1 = Kyber512 (128-bit security)
 * 2 = Kyber768 (192-bit security, recommended)
 * 3 = Kyber1024 (256-bit security)
 */

/**
 * Generate a Kyber key pair
 * 
 * @param security_level Security level (1, 2, or 3)
 * @param public_key_out Buffer for public key output
 * @param public_key_len Input: buffer size, Output: actual size
 * @param private_key_out Buffer for private key output
 * @param private_key_len Input: buffer size, Output: actual size
 * @return PQCResult code
 */
int pqc_kyber_generate_keypair(
    int security_level,
    uint8_t* public_key_out,
    size_t* public_key_len,
    uint8_t* private_key_out,
    size_t* private_key_len
);

/**
 * Get expected Kyber key sizes for a security level
 * 
 * @param security_level Security level (1, 2, or 3)
 * @param public_key_size Output: public key size
 * @param private_key_size Output: private key size
 * @param ciphertext_size Output: ciphertext size
 * @param shared_secret_size Output: shared secret size
 * @return PQCResult code
 */
int pqc_kyber_get_key_sizes(
    int security_level,
    size_t* public_key_size,
    size_t* private_key_size,
    size_t* ciphertext_size,
    size_t* shared_secret_size
);

/**
 * Encapsulate a shared secret using Kyber
 * 
 * @param public_key Public key buffer
 * @param public_key_len Public key length
 * @param ciphertext_out Buffer for ciphertext output
 * @param ciphertext_len Input: buffer size, Output: actual size
 * @param shared_secret_out Buffer for shared secret output
 * @param shared_secret_len Input: buffer size, Output: actual size
 * @return PQCResult code
 */
int pqc_kyber_encapsulate(
    const uint8_t* public_key,
    size_t public_key_len,
    uint8_t* ciphertext_out,
    size_t* ciphertext_len,
    uint8_t* shared_secret_out,
    size_t* shared_secret_len
);

/**
 * Decapsulate a shared secret using Kyber
 * 
 * @param private_key Private key buffer
 * @param private_key_len Private key length
 * @param ciphertext Ciphertext buffer
 * @param ciphertext_len Ciphertext length
 * @param shared_secret_out Buffer for shared secret output
 * @param shared_secret_len Input: buffer size, Output: actual size
 * @return PQCResult code
 */
int pqc_kyber_decapsulate(
    const uint8_t* private_key,
    size_t private_key_len,
    const uint8_t* ciphertext,
    size_t ciphertext_len,
    uint8_t* shared_secret_out,
    size_t* shared_secret_len
);

// ============================================================================
// Dilithium Digital Signatures
// ============================================================================

/**
 * Security levels for Dilithium:
 * 2 = Dilithium2 (NIST level 2, ~128-bit)
 * 3 = Dilithium3 (NIST level 3, ~192-bit)
 * 5 = Dilithium5 (NIST level 5, ~256-bit, recommended)
 */

/**
 * Generate a Dilithium key pair
 * 
 * @param security_level Security level (2, 3, or 5)
 * @param public_key_out Buffer for public key output
 * @param public_key_len Input: buffer size, Output: actual size
 * @param private_key_out Buffer for private key output
 * @param private_key_len Input: buffer size, Output: actual size
 * @return PQCResult code
 */
int pqc_dilithium_generate_keypair(
    int security_level,
    uint8_t* public_key_out,
    size_t* public_key_len,
    uint8_t* private_key_out,
    size_t* private_key_len
);

/**
 * Get expected Dilithium key sizes for a security level
 * 
 * @param security_level Security level (2, 3, or 5)
 * @param public_key_size Output: public key size
 * @param private_key_size Output: private key size
 * @param signature_size Output: signature size
 * @return PQCResult code
 */
int pqc_dilithium_get_key_sizes(
    int security_level,
    size_t* public_key_size,
    size_t* private_key_size,
    size_t* signature_size
);

/**
 * Sign a message using Dilithium
 * 
 * @param private_key Private key buffer
 * @param private_key_len Private key length
 * @param message Message buffer to sign
 * @param message_len Message length
 * @param signature_out Buffer for signature output
 * @param signature_len Input: buffer size, Output: actual size
 * @return PQCResult code
 */
int pqc_dilithium_sign(
    const uint8_t* private_key,
    size_t private_key_len,
    const uint8_t* message,
    size_t message_len,
    uint8_t* signature_out,
    size_t* signature_len
);

/**
 * Verify a Dilithium signature
 * 
 * @param public_key Public key buffer
 * @param public_key_len Public key length
 * @param message Original message buffer
 * @param message_len Message length
 * @param signature Signature buffer
 * @param signature_len Signature length
 * @return PQC_SUCCESS if valid, PQC_ERROR_VERIFICATION if invalid
 */
int pqc_dilithium_verify(
    const uint8_t* public_key,
    size_t public_key_len,
    const uint8_t* message,
    size_t message_len,
    const uint8_t* signature,
    size_t signature_len
);

// ============================================================================
// Convenience Macros
// ============================================================================

/** Recommended Kyber security level */
#define PQC_KYBER_RECOMMENDED 2

/** Recommended Dilithium security level */
#define PQC_DILITHIUM_RECOMMENDED 5

/** Check if result is success */
#define PQC_IS_SUCCESS(result) ((result) == PQC_SUCCESS)

/** Check if result is an error */
#define PQC_IS_ERROR(result) ((result) != PQC_SUCCESS)

#ifdef __cplusplus
}
#endif

#endif // VANTIS_PQC_H