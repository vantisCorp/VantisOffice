/**
 * VantisMobile - C Header for FFI Bindings
 *
 * This header provides C-compatible interface for the vantis-mobile Rust library.
 * Use this for iOS Swift integration via C interoperability.
 */

#ifndef VANTIS_MOBILE_H
#define VANTIS_MOBILE_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================================
 * Types
 * ============================================================================ */

/** Opaque handle for encryption key pair */
typedef struct VantisKeyPair VantisKeyPair;

/** Opaque handle for encryptor */
typedef struct VantisEncryptor VantisEncryptor;

/** Opaque handle for device info */
typedef struct VantisDeviceInfo VantisDeviceInfo;

/** Error codes */
typedef enum {
    VANTIS_SUCCESS = 0,
    VANTIS_NULL_POINTER = 1,
    VANTIS_INVALID_UTF8 = 2,
    VANTIS_INVALID_DATA = 3,
    VANTIS_ENCRYPTION_ERROR = 4,
    VANTIS_DECRYPTION_ERROR = 5,
    VANTIS_CONNECTION_ERROR = 6,
    VANTIS_INVALID_STATE = 7,
    VANTIS_OUT_OF_MEMORY = 8,
    VANTIS_UNKNOWN_ERROR = 99
} VantisErrorCode;

/** Result structure for operations */
typedef struct {
    int code;
    char* message;
} VantisResult;

/* ============================================================================
 * Library Lifecycle
 * ============================================================================ */

/**
 * Initialize the library.
 * Call once at application startup.
 * @return 0 on success
 */
int vantis_initialize(void);

/**
 * Cleanup the library.
 * Call once at application shutdown.
 */
void vantis_cleanup(void);

/**
 * Get library version.
 * @param out Buffer to write version string
 * @param out_len Buffer length
 * @return VantisResult with status
 */
VantisResult vantis_version(char* out, unsigned int out_len);

/* ============================================================================
 * Memory Management
 * ============================================================================ */

/**
 * Free a string allocated by the library.
 * @param s String to free
 */
void vantis_free_string(char* s);

/**
 * Free a byte buffer allocated by the library.
 * @param ptr Buffer pointer
 * @param len Buffer length
 */
void vantis_free_buffer(unsigned char* ptr, unsigned int len);

/* ============================================================================
 * Key Pair Operations
 * ============================================================================ */

/**
 * Generate a new encryption key pair.
 * @return Opaque handle to key pair, or NULL on error
 */
VantisKeyPair* vantis_keypair_generate(void);

/**
 * Free a key pair.
 * @param keypair Key pair to free
 */
void vantis_keypair_free(VantisKeyPair* keypair);

/**
 * Get public key as base64 encoded string.
 * @param keypair Key pair handle
 * @param out Buffer to write base64 string
 * @param out_len Buffer length
 * @return VantisResult with status
 */
VantisResult vantis_keypair_public_key_base64(const VantisKeyPair* keypair, char* out, unsigned int out_len);

/* ============================================================================
 * Encryption Operations
 * ============================================================================ */

/**
 * Create an encryptor from a base64-encoded shared secret.
 * @param shared_secret_base64 Base64 encoded encryption key (32 bytes when decoded)
 * @return Opaque handle to encryptor, or NULL on error
 */
VantisEncryptor* vantis_encryptor_create(const char* shared_secret_base64);

/**
 * Free an encryptor.
 * @param encryptor Encryptor to free
 */
void vantis_encryptor_free(VantisEncryptor* encryptor);

/**
 * Encrypt data and return JSON-encoded encrypted message.
 * The output is a JSON string containing nonce, ciphertext, and tag (all base64 encoded).
 * @param encryptor Encryptor handle
 * @param plaintext Plaintext data
 * @param plaintext_len Plaintext length
 * @param json_out Output buffer for JSON encrypted message
 * @param json_out_len Pointer to output length (set to buffer size, updated with actual length)
 * @return 0 on success, -1 on error, -2 if buffer too small
 */
int vantis_encrypt(
    VantisEncryptor* encryptor,
    const unsigned char* plaintext,
    unsigned int plaintext_len,
    char* json_out,
    unsigned int* json_out_len
);

/**
 * Decrypt a JSON-encoded encrypted message.
 * @param encryptor Encryptor handle
 * @param json_encrypted JSON encrypted message (from vantis_encrypt)
 * @param plaintext_out Output buffer for plaintext
 * @param plaintext_len_out Pointer to output length (set to buffer size, updated with actual length)
 * @return 0 on success, -1 on error, -2 if buffer too small
 */
int vantis_decrypt(
    VantisEncryptor* encryptor,
    const char* json_encrypted,
    unsigned char* plaintext_out,
    unsigned int* plaintext_len_out
);

/* ============================================================================
 * Device Info Operations
 * ============================================================================ */

/**
 * Device types
 */
typedef enum {
    VANTIS_DEVICE_IOS = 0,
    VANTIS_DEVICE_ANDROID = 1,
    VANTIS_DEVICE_DESKTOP = 2,
    VANTIS_DEVICE_LAPTOP = 3,
    VANTIS_DEVICE_TABLET = 4
} VantisDeviceType;

/**
 * Create device info.
 * @param name Device name
 * @param device_type Device type enum value (VantisDeviceType)
 * @param os_version OS version string
 * @param app_version App version string
 * @return Opaque handle to device info, or NULL on error
 */
VantisDeviceInfo* vantis_device_info_create(
    const char* name,
    int device_type,
    const char* os_version,
    const char* app_version
);

/**
 * Free device info.
 * @param info Device info to free
 */
void vantis_device_info_free(VantisDeviceInfo* info);

/**
 * Get device info as JSON.
 * @param info Device info handle
 * @param out Buffer to write JSON
 * @param out_len Buffer length
 * @return VantisResult with status
 */
VantisResult vantis_device_info_to_json(const VantisDeviceInfo* info, char* out, unsigned int out_len);

/* ============================================================================
 * Protocol Message Operations
 * ============================================================================ */

/**
 * Create a ping message.
 * @param out Buffer to write JSON message
 * @param out_len Buffer length
 * @return VantisResult with status
 */
VantisResult vantis_message_ping(char* out, unsigned int out_len);

/**
 * Create a sync request message.
 * @param last_sync_timestamp Last sync timestamp (milliseconds since epoch)
 * @param out Buffer to write JSON message
 * @param out_len Buffer length
 * @return VantisResult with status
 */
VantisResult vantis_message_sync_request(unsigned long last_sync_timestamp, char* out, unsigned int out_len);

/**
 * Create a notification message.
 * @param title Notification title
 * @param body Notification body
 * @param notification_type Notification type string (can be NULL for "info")
 * @param priority Priority (0=low, 1=normal, 2=high, 3=urgent)
 * @param out Buffer to write JSON message
 * @param out_len Buffer length
 * @return VantisResult with status
 */
VantisResult vantis_message_notification(
    const char* title,
    const char* body,
    const char* notification_type,
    int priority,
    char* out,
    unsigned int out_len
);

#ifdef __cplusplus
}
#endif

#endif /* VANTIS_MOBILE_H */