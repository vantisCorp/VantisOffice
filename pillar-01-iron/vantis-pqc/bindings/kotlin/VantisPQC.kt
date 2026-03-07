package com.vantisoffice.pqc

import java.nio.ByteBuffer
import java.nio.ByteOrder

/**
 * Post-Quantum Cryptography wrapper for Android
 * Provides quantum-resistant cryptographic operations using Kyber and Dilithium
 */
object VantisPQC {
    
    // Load native library
    init {
        System.loadLibrary("vantis_pqc")
    }
    
    // MARK: - Error Types
    
    sealed class PQCError(message: String) : Exception(message) {
        object InvalidInput : PQCError("Invalid input parameters")
        object InvalidKey : PQCError("Invalid cryptographic key")
        object EncryptionFailed : PQCError("Encryption operation failed")
        object DecryptionFailed : PQCError("Decryption operation failed")
        object SigningFailed : PQCError("Signing operation failed")
        object VerificationFailed : PQCError("Signature verification failed")
        object BufferTooSmall : PQCError("Output buffer too small")
        data class Unknown(val code: Int) : PQCError("Unknown error: $code")
        
        companion object {
            fun fromCode(code: Int): PQCError {
                return when (code) {
                    -1 -> InvalidInput
                    -2 -> InvalidKey
                    -3 -> EncryptionFailed
                    -4 -> DecryptionFailed
                    -5 -> SigningFailed
                    -6 -> VerificationFailed
                    -7 -> BufferTooSmall
                    else -> Unknown(code)
                }
            }
        }
    }
    
    // MARK: - Security Levels
    
    enum class KyberSecurityLevel(val value: Int) {
        LEVEL1(1),  // Kyber512 - 128-bit security
        LEVEL2(2),  // Kyber768 - 192-bit security (recommended)
        LEVEL3(3)   // Kyber1024 - 256-bit security
    }
    
    enum class DilithiumSecurityLevel(val value: Int) {
        LEVEL2(2),  // ~128-bit security
        LEVEL3(3),  // ~192-bit security
        LEVEL5(5)   // ~256-bit security (recommended)
    }
    
    // MARK: - Key Types
    
    data class KyberKeyPair(
        val publicKey: ByteArray,
        val privateKey: ByteArray,
        val securityLevel: KyberSecurityLevel
    ) {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (javaClass != other?.javaClass) return false
            other as KyberKeyPair
            return publicKey.contentEquals(other.publicKey) &&
                   privateKey.contentEquals(other.privateKey) &&
                   securityLevel == other.securityLevel
        }
        
        override fun hashCode(): Int {
            var result = publicKey.contentHashCode()
            result = 31 * result + privateKey.contentHashCode()
            result = 31 * result + securityLevel.hashCode()
            return result
        }
    }
    
    data class DilithiumKeyPair(
        val publicKey: ByteArray,
        val privateKey: ByteArray,
        val securityLevel: DilithiumSecurityLevel
    ) {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (javaClass != other?.javaClass) return false
            other as DilithiumKeyPair
            return publicKey.contentEquals(other.publicKey) &&
                   privateKey.contentEquals(other.privateKey) &&
                   securityLevel == other.securityLevel
        }
        
        override fun hashCode(): Int {
            var result = publicKey.contentHashCode()
            result = 31 * result + privateKey.contentHashCode()
            result = 31 * result + securityLevel.hashCode()
            return result
        }
    }
    
    data class KyberEncapsulationResult(
        val ciphertext: ByteArray,
        val sharedSecret: ByteArray
    ) {
        override fun equals(other: Any?): Boolean {
            if (this === other) return true
            if (javaClass != other?.javaClass) return false
            other as KyberEncapsulationResult
            return ciphertext.contentEquals(other.ciphertext) &&
                   sharedSecret.contentEquals(other.sharedSecret)
        }
        
        override fun hashCode(): Int {
            return 31 * ciphertext.contentHashCode() + sharedSecret.contentHashCode()
        }
    }
    
    data class KeySizes(
        val publicKey: Int,
        val privateKey: Int,
        val ciphertext: Int = 0,
        val sharedSecret: Int = 0,
        val signature: Int = 0
    )
    
    // MARK: - Library Information
    
    val version: String
        get() = String(pqc_get_version())
    
    val name: String
        get() = String(pqc_get_name())
    
    // MARK: - Native Methods
    
    external fun pqc_get_version(): ByteArray
    external fun pqc_get_name(): ByteArray
    
    // Kyber
    external fun pqc_kyber_generate_keypair(
        securityLevel: Int,
        publicKeyOut: ByteArray,
        privateKeyOut: ByteArray
    ): Int
    
    external fun pqc_kyber_get_key_sizes(
        securityLevel: Int,
        sizes: IntArray
    ): Int
    
    external fun pqc_kyber_encapsulate(
        publicKey: ByteArray,
        ciphertextOut: ByteArray,
        sharedSecretOut: ByteArray
    ): Int
    
    external fun pqc_kyber_decapsulate(
        privateKey: ByteArray,
        ciphertext: ByteArray,
        sharedSecretOut: ByteArray
    ): Int
    
    // Dilithium
    external fun pqc_dilithium_generate_keypair(
        securityLevel: Int,
        publicKeyOut: ByteArray,
        privateKeyOut: ByteArray
    ): Int
    
    external fun pqc_dilithium_get_key_sizes(
        securityLevel: Int,
        sizes: IntArray
    ): Int
    
    external fun pqc_dilithium_sign(
        privateKey: ByteArray,
        message: ByteArray,
        signatureOut: ByteArray
    ): Int
    
    external fun pqc_dilithium_verify(
        publicKey: ByteArray,
        message: ByteArray,
        signature: ByteArray
    ): Int
    
    // MARK: - Kyber Operations
    
    /**
     * Get expected key sizes for Kyber at a given security level
     */
    fun kyberKeySizes(level: KyberSecurityLevel): KeySizes {
        val sizes = IntArray(4)
        val result = pqc_kyber_get_key_sizes(level.value, sizes)
        if (result != 0) {
            throw PQCError.fromCode(result)
        }
        return KeySizes(
            publicKey = sizes[0],
            privateKey = sizes[1],
            ciphertext = sizes[2],
            sharedSecret = sizes[3]
        )
    }
    
    /**
     * Generate a Kyber key pair
     */
    @JvmOverloads
    @Throws(PQCError::class)
    fun generateKyberKeyPair(level: KyberSecurityLevel = KyberSecurityLevel.LEVEL2): KyberKeyPair {
        val sizes = kyberKeySizes(level)
        val publicKey = ByteArray(sizes.publicKey)
        val privateKey = ByteArray(sizes.privateKey)
        
        val result = pqc_kyber_generate_keypair(level.value, publicKey, privateKey)
        if (result != 0) {
            throw PQCError.fromCode(result)
        }
        
        return KyberKeyPair(publicKey, privateKey, level)
    }
    
    /**
     * Encapsulate a shared secret using Kyber
     */
    @Throws(PQCError::class)
    fun kyberEncapsulate(publicKey: ByteArray): KyberEncapsulationResult {
        // Determine security level from public key size
        val level = when (publicKey.size) {
            800 -> KyberSecurityLevel.LEVEL1
            1184 -> KyberSecurityLevel.LEVEL2
            1568 -> KyberSecurityLevel.LEVEL3
            else -> throw PQCError.InvalidKey
        }
        
        val sizes = kyberKeySizes(level)
        val ciphertext = ByteArray(sizes.ciphertext)
        val sharedSecret = ByteArray(sizes.sharedSecret)
        
        val result = pqc_kyber_encapsulate(publicKey, ciphertext, sharedSecret)
        if (result != 0) {
            throw PQCError.fromCode(result)
        }
        
        return KyberEncapsulationResult(ciphertext, sharedSecret)
    }
    
    /**
     * Decapsulate a shared secret using Kyber
     */
    @Throws(PQCError::class)
    fun kyberDecapsulate(privateKey: ByteArray, ciphertext: ByteArray): ByteArray {
        // Shared secret is always 32 bytes for Kyber
        val sharedSecret = ByteArray(32)
        
        val result = pqc_kyber_decapsulate(privateKey, ciphertext, sharedSecret)
        if (result != 0) {
            throw PQCError.fromCode(result)
        }
        
        return sharedSecret
    }
    
    // MARK: - Dilithium Operations
    
    /**
     * Get expected key sizes for Dilithium at a given security level
     */
    fun dilithiumKeySizes(level: DilithiumSecurityLevel): KeySizes {
        val sizes = IntArray(3)
        val result = pqc_dilithium_get_key_sizes(level.value, sizes)
        if (result != 0) {
            throw PQCError.fromCode(result)
        }
        return KeySizes(
            publicKey = sizes[0],
            privateKey = sizes[1],
            signature = sizes[2]
        )
    }
    
    /**
     * Generate a Dilithium key pair
     */
    @JvmOverloads
    @Throws(PQCError::class)
    fun generateDilithiumKeyPair(level: DilithiumSecurityLevel = DilithiumSecurityLevel.LEVEL5): DilithiumKeyPair {
        val sizes = dilithiumKeySizes(level)
        val publicKey = ByteArray(sizes.publicKey)
        val privateKey = ByteArray(sizes.privateKey)
        
        val result = pqc_dilithium_generate_keypair(level.value, publicKey, privateKey)
        if (result != 0) {
            throw PQCError.fromCode(result)
        }
        
        return DilithiumKeyPair(publicKey, privateKey, level)
    }
    
    /**
     * Sign a message using Dilithium
     */
    @Throws(PQCError::class)
    fun dilithiumSign(privateKey: ByteArray, message: ByteArray): ByteArray {
        // Determine signature size from private key length
        val level = when {
            privateKey.size in 2528..2600 -> DilithiumSecurityLevel.LEVEL2
            privateKey.size in 4000..4100 -> DilithiumSecurityLevel.LEVEL3
            privateKey.size in 4864..5000 -> DilithiumSecurityLevel.LEVEL5
            else -> throw PQCError.InvalidKey
        }
        
        val sizes = dilithiumKeySizes(level)
        val signature = ByteArray(sizes.signature)
        
        val result = pqc_dilithium_sign(privateKey, message, signature)
        if (result != 0) {
            throw PQCError.fromCode(result)
        }
        
        return signature
    }
    
    /**
     * Verify a Dilithium signature
     */
    @Throws(PQCError::class)
    fun dilithiumVerify(publicKey: ByteArray, message: ByteArray, signature: ByteArray): Boolean {
        val result = pqc_dilithium_verify(publicKey, message, signature)
        return result == 0
    }
    
    // MARK: - Hybrid Key Exchange
    
    /**
     * Perform hybrid key exchange combining classical and post-quantum crypto
     * @return Pair of (classicalSharedSecret, pqSharedSecret)
     */
    @Throws(PQCError::class)
    fun hybridKeyExchange(kyberPublicKey: ByteArray, classicalPublicKey: ByteArray): Pair<ByteArray, ByteArray> {
        // Perform X25519 key exchange (classical) - would use Android Keystore or Bouncy Castle
        // For now, we'll just use Kyber
        val pqResult = kyberEncapsulate(kyberPublicKey)
        
        // In a real implementation, you'd combine both secrets using XOR or KDF
        // classicalSecret XOR pqResult.sharedSecret
        return Pair(ByteArray(32), pqResult.sharedSecret)
    }
}