import Foundation

/// Swift wrapper for VantisMobile FFI bindings
/// Provides a safe, idiomatic Swift interface to the Rust library
public class VantisMobileFFI {
    
    // MARK: - Singleton
    
    public static let shared = VantisMobileFFI()
    
    private init() {
        vantis_initialize()
    }
    
    deinit {
        vantis_cleanup()
    }
    
    // MARK: - Version
    
    /// Get the library version
    public var version: String {
        var buffer = [CChar](repeating: 0, count: 64)
        let result = vantis_version(&buffer, UInt32(buffer.count))
        guard result.code == 0 else { return "unknown" }
        return String(cString: buffer)
    }
    
    // MARK: - KeyPair
    
    /// Wrapper for encryption key pair
    public class KeyPair {
        private var handle: OpaquePointer?
        
        public init?() {
            guard let ptr = vantis_keypair_generate() else {
                return nil
            }
            self.handle = OpaquePointer(ptr)
        }
        
        deinit {
            if let handle = handle {
                vantis_keypair_free(UnsafeMutableRawPointer(handle))
            }
        }
        
        /// Get public key as base64 encoded string
        public var publicKeyBase64: String {
            var buffer = [CChar](repeating: 0, count: 128)
            let result = vantis_keypair_public_key_base64(
                UnsafeRawPointer(handle),
                &buffer,
                UInt32(buffer.count)
            )
            guard result.code == 0 else { return "" }
            return String(cString: buffer)
        }
    }
    
    // MARK: - Encryptor
    
    /// Wrapper for encryption/decryption
    public class Encryptor {
        private var handle: OpaquePointer?
        
        /// Create an encryptor from a base64-encoded shared secret
        /// - Parameter sharedSecretBase64: Base64 encoded encryption key (32 bytes when decoded)
        public init?(sharedSecretBase64: String) {
            guard let ptr = sharedSecretBase64.withCString({ secretPtr in
                vantis_encryptor_create(secretPtr)
            }) else {
                return nil
            }
            self.handle = OpaquePointer(ptr)
        }
        
        deinit {
            if let handle = handle {
                vantis_encryptor_free(UnsafeMutableRawPointer(handle))
            }
        }
        
        /// Encrypt data and return JSON-encoded encrypted message
        /// - Parameter plaintext: Data to encrypt
        /// - Returns: JSON string containing encrypted message, or nil on error
        public func encrypt(_ plaintext: Data) -> String? {
            var jsonLen = UInt32(plaintext.count + 256) // Extra space for JSON overhead
            var jsonBuffer = [CChar](repeating: 0, count: Int(jsonLen))
            
            let result = plaintext.withUnsafeBytes { plaintextPtr in
                vantis_encrypt(
                    UnsafeMutableRawPointer(handle),
                    plaintextPtr.baseAddress?.assumingMemoryBound(to: CUnsignedChar.self),
                    UInt32(plaintext.count),
                    &jsonBuffer,
                    &jsonLen
                )
            }
            
            guard result == 0 else { return nil }
            return String(cString: jsonBuffer)
        }
        
        /// Decrypt a JSON-encoded encrypted message
        /// - Parameter jsonEncrypted: JSON string from encrypt()
        /// - Returns: Decrypted data, or nil on error
        public func decrypt(_ jsonEncrypted: String) -> Data? {
            var plaintextLen = UInt32(jsonEncrypted.count) // Usually enough
            var plaintext = [UInt8](repeating: 0, count: Int(plaintextLen))
            
            let result = jsonEncrypted.withCString { jsonPtr in
                vantis_decrypt(
                    UnsafeMutableRawPointer(handle),
                    jsonPtr,
                    &plaintext,
                    &plaintextLen
                )
            }
            
            guard result == 0 else { return nil }
            return Data(plaintext[0..<Int(plaintextLen)])
        }
    }
    
    // MARK: - Device Info
    
    /// Device types
    public enum DeviceType: Int {
        case ios = 0
        case android = 1
        case desktop = 2
        case laptop = 3
        case tablet = 4
    }
    
    /// Wrapper for device info
    public class DeviceInfo {
        private var handle: OpaquePointer?
        
        public init?(name: String, deviceType: DeviceType, osVersion: String, appVersion: String) {
            guard let ptr = name.withCString({ namePtr in
                osVersion.withCString { osPtr in
                    appVersion.withCString { appPtr in
                        vantis_device_info_create(
                            namePtr,
                            Int32(deviceType.rawValue),
                            osPtr,
                            appPtr
                        )
                    }
                }
            }) else {
                return nil
            }
            self.handle = OpaquePointer(ptr)
        }
        
        deinit {
            if let handle = handle {
                vantis_device_info_free(UnsafeMutableRawPointer(handle))
            }
        }
        
        /// Get device info as JSON
        public var json: String? {
            var buffer = [CChar](repeating: 0, count: 4096)
            let result = vantis_device_info_to_json(
                UnsafeRawPointer(handle),
                &buffer,
                UInt32(buffer.count)
            )
            guard result.code == 0 else { return nil }
            return String(cString: buffer)
        }
    }
    
    // MARK: - Protocol Messages
    
    /// Create a ping message
    public func createPingMessage() -> String? {
        var buffer = [CChar](repeating: 0, count: 4096)
        let result = vantis_message_ping(&buffer, UInt32(buffer.count))
        guard result.code == 0 else { return nil }
        return String(cString: buffer)
    }
    
    /// Create a sync request message
    public func createSyncRequestMessage(lastSyncTimestamp: UInt64) -> String? {
        var buffer = [CChar](repeating: 0, count: 4096)
        let result = vantis_message_sync_request(
            CUnsignedLong(lastSyncTimestamp),
            &buffer,
            UInt32(buffer.count)
        )
        guard result.code == 0 else { return nil }
        return String(cString: buffer)
    }
    
    /// Create a notification message
    public func createNotificationMessage(
        title: String,
        body: String,
        type: String,
        priority: Int
    ) -> String? {
        var buffer = [CChar](repeating: 0, count: 4096)
        let result = title.withCString { titlePtr in
            body.withCString { bodyPtr in
                type.withCString { typePtr in
                    vantis_message_notification(
                        titlePtr,
                        bodyPtr,
                        typePtr,
                        Int32(priority),
                        &buffer,
                        UInt32(buffer.count)
                    )
                }
            }
        }
        guard result.code == 0 else { return nil }
        return String(cString: buffer)
    }
}

// MARK: - Error Types

public enum VantisError: Error {
    case nullPointer
    case invalidUtf8
    case invalidData
    case encryptionError
    case decryptionError
    case connectionError
    case invalidState
    case outOfMemory
    case unknown(String)
    
    init(code: Int, message: String? = nil) {
        switch code {
        case 1: self = .nullPointer
        case 2: self = .invalidUtf8
        case 3: self = .invalidData
        case 4: self = .encryptionError
        case 5: self = .decryptionError
        case 6: self = .connectionError
        case 7: self = .invalidState
        case 8: self = .outOfMemory
        default: self = .unknown(message ?? "Unknown error")
        }
    }
}