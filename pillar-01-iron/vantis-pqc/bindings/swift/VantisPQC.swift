import Foundation
import CryptoKit

/// Post-Quantum Cryptography wrapper for iOS
/// Provides quantum-resistant cryptographic operations using Kyber and Dilithium
public class VantisPQC {
    
    // MARK: - Error Types
    
    public enum PQCError: Error, LocalizedError {
        case invalidInput
        case invalidKey
        case encryptionFailed
        case decryptionFailed
        case signingFailed
        case verificationFailed
        case bufferTooSmall
        case unknown(Int32)
        
        public var errorDescription: String? {
            switch self {
            case .invalidInput: return "Invalid input parameters"
            case .invalidKey: return "Invalid cryptographic key"
            case .encryptionFailed: return "Encryption operation failed"
            case .decryptionFailed: return "Decryption operation failed"
            case .signingFailed: return "Signing operation failed"
            case .verificationFailed: return "Signature verification failed"
            case .bufferTooSmall: return "Output buffer too small"
            case .unknown(let code): return "Unknown error: \(code)"
            }
        }
        
        init(from code: Int32) {
            switch code {
            case -1: self = .invalidInput
            case -2: self = .invalidKey
            case -3: self = .encryptionFailed
            case -4: self = .decryptionFailed
            case -5: self = .signingFailed
            case -6: self = .verificationFailed
            case -7: self = .bufferTooSmall
            default: self = .unknown(code)
            }
        }
    }
    
    // MARK: - Security Levels
    
    public enum KyberSecurityLevel: Int {
        case level1 = 1  // Kyber512 - 128-bit security
        case level2 = 2  // Kyber768 - 192-bit security (recommended)
        case level3 = 3  // Kyber1024 - 256-bit security
    }
    
    public enum DilithiumSecurityLevel: Int {
        case level2 = 2  // ~128-bit security
        case level3 = 3  // ~192-bit security
        case level5 = 5  // ~256-bit security (recommended)
    }
    
    // MARK: - Key Types
    
    public struct KyberKeyPair {
        public let publicKey: Data
        public let privateKey: Data
        public let securityLevel: KyberSecurityLevel
    }
    
    public struct DilithiumKeyPair {
        public let publicKey: Data
        public let privateKey: Data
        public let securityLevel: DilithiumSecurityLevel
    }
    
    public struct KyberEncapsulationResult {
        public let ciphertext: Data
        public let sharedSecret: Data
    }
    
    // MARK: - Library Information
    
    public static var version: String {
        let versionPtr = pqc_get_version()
        return String(cString: versionPtr)
    }
    
    public static var name: String {
        let namePtr = pqc_get_name()
        return String(cString: namePtr)
    }
    
    // MARK: - Kyber Operations
    
    /// Get expected key sizes for Kyber at a given security level
    public static func kyberKeySizes(for level: KyberSecurityLevel) -> (publicKey: Int, privateKey: Int, ciphertext: Int, sharedSecret: Int) {
        var pkSize: Int = 0
        var skSize: Int = 0
        var ctSize: Int = 0
        var ssSize: Int = 0
        
        _ = pqc_kyber_get_key_sizes(
            Int32(level.rawValue),
            &pkSize,
            &skSize,
            &ctSize,
            &ssSize
        )
        
        return (pkSize, skSize, ctSize, ssSize)
    }
    
    /// Generate a Kyber key pair
    public static func generateKyberKeyPair(level: KyberSecurityLevel = .level2) throws -> KyberKeyPair {
        // Get expected sizes
        let sizes = kyberKeySizes(for: level)
        
        // Allocate buffers
        var publicKey = Data(count: sizes.publicKey)
        var privateKey = Data(count: sizes.privateKey)
        var pkLen = sizes.publicKey
        var skLen = sizes.privateKey
        
        // Generate key pair
        let result = publicKey.withUnsafeMutableBytes { pkPtr in
            privateKey.withUnsafeMutableBytes { skPtr in
                pqc_kyber_generate_keypair(
                    Int32(level.rawValue),
                    pkPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                    &pkLen,
                    skPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                    &skLen
                )
            }
        }
        
        guard result == 0 else {
            throw PQCError(from: result)
        }
        
        return KyberKeyPair(
            publicKey: publicKey,
            privateKey: privateKey,
            securityLevel: level
        )
    }
    
    /// Encapsulate a shared secret using Kyber
    public static func kyberEncapsulate(publicKey: Data) throws -> KyberEncapsulationResult {
        // Determine security level from public key size
        let level: KyberSecurityLevel
        switch publicKey.count {
        case 800: level = .level1
        case 1184: level = .level2
        case 1568: level = .level3
        default: throw PQCError.invalidKey
        }
        
        let sizes = kyberKeySizes(for: level)
        var ciphertext = Data(count: sizes.ciphertext)
        var sharedSecret = Data(count: sizes.sharedSecret)
        var ctLen = sizes.ciphertext
        var ssLen = sizes.sharedSecret
        
        let result = publicKey.withUnsafeBytes { pkPtr in
            ciphertext.withUnsafeMutableBytes { ctPtr in
                sharedSecret.withUnsafeMutableBytes { ssPtr in
                    pqc_kyber_encapsulate(
                        pkPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        pkPtr.count,
                        ctPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        &ctLen,
                        ssPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        &ssLen
                    )
                }
            }
        }
        
        guard result == 0 else {
            throw PQCError(from: result)
        }
        
        return KyberEncapsulationResult(
            ciphertext: ciphertext,
            sharedSecret: sharedSecret
        )
    }
    
    /// Decapsulate a shared secret using Kyber
    public static func kyberDecapsulate(privateKey: Data, ciphertext: Data) throws -> Data {
        // Determine shared secret size (always 32 bytes for Kyber)
        var sharedSecret = Data(count: 32)
        var ssLen = 32
        
        let result = privateKey.withUnsafeBytes { skPtr in
            ciphertext.withUnsafeBytes { ctPtr in
                sharedSecret.withUnsafeMutableBytes { ssPtr in
                    pqc_kyber_decapsulate(
                        skPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        skPtr.count,
                        ctPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        ctPtr.count,
                        ssPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        &ssLen
                    )
                }
            }
        }
        
        guard result == 0 else {
            throw PQCError(from: result)
        }
        
        return sharedSecret
    }
    
    // MARK: - Dilithium Operations
    
    /// Get expected key sizes for Dilithium at a given security level
    public static func dilithiumKeySizes(for level: DilithiumSecurityLevel) -> (publicKey: Int, privateKey: Int, signature: Int) {
        var pkSize: Int = 0
        var skSize: Int = 0
        var sigSize: Int = 0
        
        _ = pqc_dilithium_get_key_sizes(
            Int32(level.rawValue),
            &pkSize,
            &skSize,
            &sigSize
        )
        
        return (pkSize, skSize, sigSize)
    }
    
    /// Generate a Dilithium key pair
    public static func generateDilithiumKeyPair(level: DilithiumSecurityLevel = .level5) throws -> DilithiumKeyPair {
        let sizes = dilithiumKeySizes(for: level)
        
        var publicKey = Data(count: sizes.publicKey)
        var privateKey = Data(count: sizes.privateKey)
        var pkLen = sizes.publicKey
        var skLen = sizes.privateKey
        
        let result = publicKey.withUnsafeMutableBytes { pkPtr in
            privateKey.withUnsafeMutableBytes { skPtr in
                pqc_dilithium_generate_keypair(
                    Int32(level.rawValue),
                    pkPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                    &pkLen,
                    skPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                    &skLen
                )
            }
        }
        
        guard result == 0 else {
            throw PQCError(from: result)
        }
        
        return DilithiumKeyPair(
            publicKey: publicKey,
            privateKey: privateKey,
            securityLevel: level
        )
    }
    
    /// Sign a message using Dilithium
    public static func dilithiumSign(privateKey: Data, message: Data) throws -> Data {
        // Determine signature size from private key length
        let level: DilithiumSecurityLevel
        let skLen = privateKey.count
        if skLen >= 2528 && skLen <= 2600 {
            level = .level2
        } else if skLen >= 4000 && skLen <= 4100 {
            level = .level3
        } else if skLen >= 4864 && skLen <= 5000 {
            level = .level5
        } else {
            throw PQCError.invalidKey
        }
        
        let sizes = dilithiumKeySizes(for: level)
        var signature = Data(count: sizes.signature)
        var sigLen = sizes.signature
        
        let result = privateKey.withUnsafeBytes { skPtr in
            message.withUnsafeBytes { msgPtr in
                signature.withUnsafeMutableBytes { sigPtr in
                    pqc_dilithium_sign(
                        skPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        skPtr.count,
                        msgPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        msgPtr.count,
                        sigPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        &sigLen
                    )
                }
            }
        }
        
        guard result == 0 else {
            throw PQCError(from: result)
        }
        
        return signature
    }
    
    /// Verify a Dilithium signature
    public static func dilithiumVerify(publicKey: Data, message: Data, signature: Data) throws -> Bool {
        let result = publicKey.withUnsafeBytes { pkPtr in
            message.withUnsafeBytes { msgPtr in
                signature.withUnsafeBytes { sigPtr in
                    pqc_dilithium_verify(
                        pkPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        pkPtr.count,
                        msgPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        msgPtr.count,
                        sigPtr.baseAddress?.assumingMemoryBound(to: UInt8.self),
                        sigPtr.count
                    )
                }
            }
        }
        
        return result == 0
    }
}

// MARK: - Hybrid Key Exchange

extension VantisPQC {
    /// Perform hybrid key exchange combining classical and post-quantum crypto
    /// Returns (classicalSharedSecret, pqSharedSecret) for XOR combination
    public static func hybridKeyExchange(
        kyberPublicKey: Data,
        classicalPublicKey: Data
    ) throws -> (classicalSecret: Data, pqSecret: Data) {
        // Perform X25519 key exchange (classical)
        let classicalSecret: Data
        if #available(iOS 13.0, *) {
            // Use CryptoKit for X25519
            let privateKey = P256.KeyAgreement.PrivateKey()
            let sharedSecret = try privateKey.sharedSecretFromKeyAgreement(
                with: P256.KeyAgreement.PublicKey(rawRepresentation: classicalPublicKey)
            )
            classicalSecret = sharedSecret.withUnsafeBytes { Data($0) }
        } else {
            // Fallback for older iOS versions
            throw PQCError.encryptionFailed
        }
        
        // Perform Kyber key exchange (post-quantum)
        let pqResult = try kyberEncapsulate(publicKey: kyberPublicKey)
        
        return (classicalSecret, pqResult.sharedSecret)
    }
}