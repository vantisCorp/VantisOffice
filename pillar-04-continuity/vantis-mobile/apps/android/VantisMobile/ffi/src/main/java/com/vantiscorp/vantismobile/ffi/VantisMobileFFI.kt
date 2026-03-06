package com.vantiscorp.vantismobile.ffi

/**
 * Kotlin JNI wrapper for VantisMobile FFI bindings
 * Provides a safe, idiomatic Kotlin interface to the Rust library
 */
object VantisMobileFFI {
    
    init {
        System.loadLibrary("vantis_mobile")
        initialize()
    }
    
    // MARK: - Native Methods
    
    /**
     * Initialize the library.
     * Called automatically when the class is loaded.
     */
    private external fun initialize()
    
    /**
     * Cleanup the library.
     * Should be called when the application is terminating.
     */
    external fun cleanup()
    
    /**
     * Get the library version.
     * @return Version string
     */
    external fun getVersion(): String
    
    // MARK: - KeyPair
    
    /**
     * Generate a new encryption key pair.
     * @return Native handle to the key pair (must be freed with keypairFree)
     */
    external fun keypairGenerate(): Long
    
    /**
     * Free a key pair.
     * @param handle Native handle from keypairGenerate()
     */
    external fun keypairFree(handle: Long)
    
    /**
     * Get public key as base64 encoded string.
     * @param handle Native handle from keypairGenerate()
     * @return Base64 encoded public key
     */
    external fun keypairPublicKeyBase64(handle: Long): String
    
    // MARK: - Encryptor
    
    /**
     * Create an encryptor from a base64-encoded shared secret.
     * @param sharedSecretBase64 Base64 encoded encryption key (32 bytes when decoded)
     * @return Native handle to the encryptor (must be freed with encryptorFree)
     */
    external fun encryptorCreate(sharedSecretBase64: String): Long
    
    /**
     * Free an encryptor.
     * @param handle Native handle from encryptorCreate()
     */
    external fun encryptorFree(handle: Long)
    
    /**
     * Encrypt data and return JSON-encoded encrypted message.
     * @param handle Native handle from encryptorCreate()
     * @param plaintext Data to encrypt
     * @return JSON string containing encrypted message
     */
    external fun encrypt(handle: Long, plaintext: ByteArray): String?
    
    /**
     * Decrypt a JSON-encoded encrypted message.
     * @param handle Native handle from encryptorCreate()
     * @param jsonEncrypted JSON string from encrypt()
     * @return Decrypted data
     */
    external fun decrypt(handle: Long, jsonEncrypted: String): ByteArray?
    
    // MARK: - Device Info
    
    /**
     * Create device info.
     * @param name Device name
     * @param deviceType Device type (0=iOS, 1=Android, 2=Desktop, 3=Laptop, 4=Tablet)
     * @param osVersion OS version string
     * @param appVersion App version string
     * @return Native handle to device info (must be freed with deviceInfoFree)
     */
    external fun deviceInfoCreate(
        name: String,
        deviceType: Int,
        osVersion: String,
        appVersion: String
    ): Long
    
    /**
     * Free device info.
     * @param handle Native handle from deviceInfoCreate()
     */
    external fun deviceInfoFree(handle: Long)
    
    /**
     * Get device info as JSON.
     * @param handle Native handle from deviceInfoCreate()
     * @return JSON string
     */
    external fun deviceInfoToJson(handle: Long): String?
    
    // MARK: - Protocol Messages
    
    /**
     * Create a ping message.
     * @return JSON message string
     */
    external fun messagePing(): String?
    
    /**
     * Create a sync request message.
     * @param lastSyncTimestamp Last sync timestamp (milliseconds since epoch)
     * @return JSON message string
     */
    external fun messageSyncRequest(lastSyncTimestamp: Long): String?
    
    /**
     * Create a notification message.
     * @param title Notification title
     * @param body Notification body
     * @param type Notification type
     * @param priority Priority (0=low, 1=normal, 2=high, 3=urgent)
     * @return JSON message string
     */
    external fun messageNotification(
        title: String,
        body: String,
        type: String,
        priority: Int
    ): String?
}

// MARK: - Wrapper Classes

/**
 * Wrapper for encryption key pair
 */
class VantisKeyPair {
    private var handle: Long = 0
    
    init {
        handle = VantisMobileFFI.keypairGenerate()
        if (handle == 0L) {
            throw VantisError.InvalidData("Failed to generate key pair")
        }
    }
    
    /**
     * Get public key as base64 encoded string
     */
    val publicKeyBase64: String
        get() = VantisMobileFFI.keypairPublicKeyBase64(handle)
    
    /**
     * Free the key pair resources
     */
    fun free() {
        if (handle != 0L) {
            VantisMobileFFI.keypairFree(handle)
            handle = 0
        }
    }
    
    protected fun finalize() {
        free()
    }
}

/**
 * Wrapper for encryption/decryption
 */
class VantisEncryptor(sharedSecretBase64: String) {
    private var handle: Long = 0
    
    init {
        handle = VantisMobileFFI.encryptorCreate(sharedSecretBase64)
        if (handle == 0L) {
            throw VantisError.InvalidData("Failed to create encryptor")
        }
    }
    
    /**
     * Encrypt data and return JSON-encoded encrypted message
     * @param plaintext Data to encrypt
     * @return JSON string containing encrypted message, or null on error
     */
    fun encrypt(plaintext: ByteArray): String? {
        return VantisMobileFFI.encrypt(handle, plaintext)
    }
    
    /**
     * Decrypt a JSON-encoded encrypted message
     * @param jsonEncrypted JSON string from encrypt()
     * @return Decrypted data, or null on error
     */
    fun decrypt(jsonEncrypted: String): ByteArray? {
        return VantisMobileFFI.decrypt(handle, jsonEncrypted)
    }
    
    /**
     * Free the encryptor resources
     */
    fun free() {
        if (handle != 0L) {
            VantisMobileFFI.encryptorFree(handle)
            handle = 0
        }
    }
    
    protected fun finalize() {
        free()
    }
}

/**
 * Device types
 */
enum class VantisDeviceType(val value: Int) {
    IOS(0),
    ANDROID(1),
    DESKTOP(2),
    LAPTOP(3),
    TABLET(4)
}

/**
 * Wrapper for device info
 */
class VantisDeviceInfo(
    name: String,
    deviceType: VantisDeviceType,
    osVersion: String,
    appVersion: String
) {
    private var handle: Long = 0
    
    init {
        handle = VantisMobileFFI.deviceInfoCreate(
            name,
            deviceType.value,
            osVersion,
            appVersion
        )
        if (handle == 0L) {
            throw VantisError.InvalidData("Failed to create device info")
        }
    }
    
    /**
     * Get device info as JSON
     */
    val json: String?
        get() = VantisMobileFFI.deviceInfoToJson(handle)
    
    /**
     * Free the device info resources
     */
    fun free() {
        if (handle != 0L) {
            VantisMobileFFI.deviceInfoFree(handle)
            handle = 0
        }
    }
    
    protected fun finalize() {
        free()
    }
}

// MARK: - Error Types

sealed class VantisError(message: String) : Exception(message) {
    class NullPointer : VantisError("Null pointer")
    class InvalidUtf8 : VantisError("Invalid UTF-8")
    class InvalidData(message: String) : VantisError(message)
    class EncryptionError(message: String) : VantisError(message)
    class DecryptionError(message: String) : VantisError(message)
    class ConnectionError(message: String) : VantisError(message)
    class InvalidState(message: String) : VantisError(message)
    class OutOfMemory : VantisError("Out of memory")
    class Unknown(message: String) : VantisError(message)
    
    companion object {
        fun fromCode(code: Int, message: String? = null): VantisError {
            return when (code) {
                1 -> NullPointer()
                2 -> InvalidUtf8()
                3 -> InvalidData(message ?: "Invalid data")
                4 -> EncryptionError(message ?: "Encryption error")
                5 -> DecryptionError(message ?: "Decryption error")
                6 -> ConnectionError(message ?: "Connection error")
                7 -> InvalidState(message ?: "Invalid state")
                8 -> OutOfMemory()
                else -> Unknown(message ?: "Unknown error")
            }
        }
    }
}