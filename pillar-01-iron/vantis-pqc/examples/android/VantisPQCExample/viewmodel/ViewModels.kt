package com.vantis.pqc.example.viewmodel

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.vantis.pqc.example.service.*
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

// MARK: - Key Exchange ViewModel

class KeyExchangeViewModel(
    private val service: VantisPQCService
) : ViewModel() {
    
    private val _state = MutableStateFlow(KeyExchangeState())
    val state: StateFlow<KeyExchangeState> = _state.asStateFlow()
    
    fun selectLevel(level: KyberSecurityLevel) {
        _state.update { it.copy(selectedLevel = level) }
    }
    
    fun generateAliceKeyPair() {
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            
            service.generateKyberKeyPair(_state.value.selectedLevel)
                .onSuccess { keyPair ->
                    _state.update { it.copy(aliceKeyPair = keyPair, isLoading = false) }
                }
                .onFailure { error ->
                    _state.update { it.copy(error = error.message, isLoading = false) }
                }
        }
    }
    
    fun generateBobKeyPair() {
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            
            service.generateKyberKeyPair(_state.value.selectedLevel)
                .onSuccess { keyPair ->
                    _state.update { it.copy(bobKeyPair = keyPair, isLoading = false) }
                }
                .onFailure { error ->
                    _state.update { it.copy(error = error.message, isLoading = false) }
                }
        }
    }
    
    fun encapsulate() {
        val bobKeyPair = _state.value.bobKeyPair ?: return
        val level = _state.value.selectedLevel
        
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            
            service.encapsulate(bobKeyPair.publicKey, level)
                .onSuccess { result ->
                    _state.update { it.copy(encapsulationResult = result, isLoading = false) }
                }
                .onFailure { error ->
                    _state.update { it.copy(error = error.message, isLoading = false) }
                }
        }
    }
    
    fun decapsulate() {
        val bobKeyPair = _state.value.bobKeyPair ?: return
        val encapsulation = _state.value.encapsulationResult ?: return
        val level = _state.value.selectedLevel
        
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            
            service.decapsulate(
                privateKey = bobKeyPair.privateKey,
                ciphertext = encapsulation.ciphertext,
                securityLevel = level
            )
                .onSuccess { secret ->
                    _state.update { it.copy(decapsulatedSecret = secret, isLoading = false) }
                }
                .onFailure { error ->
                    _state.update { it.copy(error = error.message, isLoading = false) }
                }
        }
    }
}

data class KeyExchangeState(
    val selectedLevel: KyberSecurityLevel = KyberSecurityLevel.KYBER_768,
    val aliceKeyPair: KyberKeyPairResult? = null,
    val bobKeyPair: KyberKeyPairResult? = null,
    val encapsulationResult: EncapsulationResult? = null,
    val decapsulatedSecret: ByteArray? = null,
    val isLoading: Boolean = false,
    val error: String? = null
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false
        other as KeyExchangeState
        return selectedLevel == other.selectedLevel &&
               aliceKeyPair == other.aliceKeyPair &&
               bobKeyPair == other.bobKeyPair &&
               encapsulationResult == other.encapsulationResult &&
               decapsulatedSecret?.contentEquals(other.decapsulatedSecret) == true &&
               isLoading == other.isLoading &&
               error == other.error
    }

    override fun hashCode(): Int {
        var result = selectedLevel.hashCode()
        result = 31 * result + (aliceKeyPair?.hashCode() ?: 0)
        result = 31 * result + (bobKeyPair?.hashCode() ?: 0)
        result = 31 * result + (encapsulationResult?.hashCode() ?: 0)
        result = 31 * result + (decapsulatedSecret?.contentHashCode() ?: 0)
        result = 31 * result + isLoading.hashCode()
        result = 31 * result + (error?.hashCode() ?: 0)
        return result
    }
}

// MARK: - Signature ViewModel

class SignatureViewModel(
    private val service: VantisPQCService
) : ViewModel() {
    
    private val _state = MutableStateFlow(SignatureState())
    val state: StateFlow<SignatureState> = _state.asStateFlow()
    
    fun selectLevel(level: DilithiumSecurityLevel) {
        _state.update { it.copy(selectedLevel = level) }
    }
    
    fun generateKeyPair() {
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            
            service.generateDilithiumKeyPair(_state.value.selectedLevel)
                .onSuccess { keyPair ->
                    _state.update { 
                        it.copy(
                            keyPair = keyPair, 
                            signature = null,
                            isVerified = null,
                            isLoading = false
                        ) 
                    }
                }
                .onFailure { error ->
                    _state.update { it.copy(error = error.message, isLoading = false) }
                }
        }
    }
    
    fun updateMessage(message: String) {
        _state.update { it.copy(message = message) }
    }
    
    fun sign() {
        val keyPair = _state.value.keyPair ?: return
        val message = _state.value.message.toByteArray(Charsets.UTF_8)
        val level = _state.value.selectedLevel
        
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            
            service.sign(keyPair.privateKey, message, level)
                .onSuccess { signature ->
                    _state.update { it.copy(signature = signature, isVerified = null, isLoading = false) }
                }
                .onFailure { error ->
                    _state.update { it.copy(error = error.message, isLoading = false) }
                }
        }
    }
    
    fun verify() {
        val keyPair = _state.value.keyPair ?: return
        val signature = _state.value.signature ?: return
        val message = _state.value.message.toByteArray(Charsets.UTF_8)
        val level = _state.value.selectedLevel
        
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            
            service.verify(keyPair.publicKey, message, signature, level)
                .onSuccess { verified ->
                    _state.update { it.copy(isVerified = verified, isLoading = false) }
                }
                .onFailure { error ->
                    _state.update { it.copy(error = error.message, isLoading = false) }
                }
        }
    }
}

data class SignatureState(
    val selectedLevel: DilithiumSecurityLevel = DilithiumSecurityLevel.DILITHIUM_3,
    val keyPair: DilithiumKeyPairResult? = null,
    val message: String = "Hello, Post-Quantum World!",
    val signature: ByteArray? = null,
    val isVerified: Boolean? = null,
    val isLoading: Boolean = false,
    val error: String? = null
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false
        other as SignatureState
        return selectedLevel == other.selectedLevel &&
               keyPair == other.keyPair &&
               message == other.message &&
               signature?.contentEquals(other.signature) == true &&
               isVerified == other.isVerified &&
               isLoading == other.isLoading &&
               error == other.error
    }

    override fun hashCode(): Int {
        var result = selectedLevel.hashCode()
        result = 31 * result + (keyPair?.hashCode() ?: 0)
        result = 31 * result + message.hashCode()
        result = 31 * result + (signature?.contentHashCode() ?: 0)
        result = 31 * result + (isVerified?.hashCode() ?: 0)
        result = 31 * result + isLoading.hashCode()
        result = 31 * result + (error?.hashCode() ?: 0)
        return result
    }
}

// MARK: - Encryption ViewModel

class EncryptionViewModel(
    private val service: VantisPQCService
) : ViewModel() {
    
    private val _state = MutableStateFlow(EncryptionState())
    val state: StateFlow<EncryptionState> = _state.asStateFlow()
    
    fun generateKey() {
        val key = ByteArray(32)
        java.security.SecureRandom().nextBytes(key)
        _state.update { 
            it.copy(
                encryptionKey = key,
                encryptedData = null,
                decryptedText = null
            ) 
        }
    }
    
    fun updatePlaintext(plaintext: String) {
        _state.update { it.copy(plaintext = plaintext) }
    }
    
    fun encrypt() {
        val key = _state.value.encryptionKey ?: return
        val data = _state.value.plaintext.toByteArray(Charsets.UTF_8)
        
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            
            service.encryptStream(data, key)
                .onSuccess { encrypted ->
                    _state.update { 
                        it.copy(encryptedData = encrypted, decryptedText = null, isLoading = false) 
                    }
                }
                .onFailure { error ->
                    _state.update { it.copy(error = error.message, isLoading = false) }
                }
        }
    }
    
    fun decrypt() {
        val key = _state.value.encryptionKey ?: return
        val encrypted = _state.value.encryptedData ?: return
        
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true, error = null) }
            
            service.decryptStream(encrypted, key)
                .onSuccess { decrypted ->
                    val text = String(decrypted, Charsets.UTF_8)
                    _state.update { it.copy(decryptedText = text, isLoading = false) }
                }
                .onFailure { error ->
                    _state.update { it.copy(error = error.message, isLoading = false) }
                }
        }
    }
}

data class EncryptionState(
    val encryptionKey: ByteArray? = null,
    val plaintext: String = "Secret message to encrypt",
    val encryptedData: ByteArray? = null,
    val decryptedText: String? = null,
    val isLoading: Boolean = false,
    val error: String? = null
) {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false
        other as EncryptionState
        return encryptionKey?.contentEquals(other.encryptionKey) == true &&
               plaintext == other.plaintext &&
               encryptedData?.contentEquals(other.encryptedData) == true &&
               decryptedText == other.decryptedText &&
               isLoading == other.isLoading &&
               error == other.error
    }

    override fun hashCode(): Int {
        var result = encryptionKey?.contentHashCode() ?: 0
        result = 31 * result + plaintext.hashCode()
        result = 31 * result + (encryptedData?.contentHashCode() ?: 0)
        result = 31 * result + (decryptedText?.hashCode() ?: 0)
        result = 31 * result + isLoading.hashCode()
        result = 31 * result + (error?.hashCode() ?: 0)
        return result
    }
}