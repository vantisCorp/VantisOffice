package com.vantiscorp.vantismobile.service

import android.content.Context
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity
import kotlinx.coroutines.suspendCancellableCoroutine
import java.util.concurrent.Executor
import kotlin.coroutines.resume

/**
 * Biometric Authentication Service for Android
 * Supports fingerprint and face authentication
 */
class BiometricAuthService(private val context: Context) {
    
    private val executor: Executor = ContextCompat.getMainExecutor(context)
    private val biometricManager = BiometricManager.from(context)
    
    /**
     * Check if biometric authentication is available
     */
    fun isBiometricAvailable(): Boolean {
        val canAuthenticate = biometricManager.canAuthenticate(
            BiometricManager.Authenticators.BIOMETRIC_STRONG or 
            BiometricManager.Authenticators.DEVICE_CREDENTIAL
        )
        return canAuthenticate == BiometricManager.BIOMETRIC_SUCCESS
    }
    
    /**
     * Get the availability status of biometric authentication
     */
    fun getBiometricStatus(): BiometricStatus {
        val canAuthenticate = biometricManager.canAuthenticate(
            BiometricManager.Authenticators.BIOMETRIC_STRONG or 
            BiometricManager.Authenticators.DEVICE_CREDENTIAL
        )
        
        return when (canAuthenticate) {
            BiometricManager.BIOMETRIC_SUCCESS -> BiometricStatus.AVAILABLE
            BiometricManager.BIOMETRIC_ERROR_NO_HARDWARE -> BiometricStatus.NO_HARDWARE
            BiometricManager.BIOMETRIC_ERROR_HW_UNAVAILABLE -> BiometricStatus.HW_UNAVAILABLE
            BiometricManager.BIOMETRIC_ERROR_NONE_ENROLLED -> BiometricStatus.NOT_ENROLLED
            BiometricManager.BIOMETRIC_ERROR_SECURITY_UPDATE_REQUIRED -> BiometricStatus.SECURITY_UPDATE_REQUIRED
            BiometricManager.BIOMETRIC_ERROR_UNSUPPORTED -> BiometricStatus.UNSUPPORTED
            BiometricManager.BIOMETRIC_STATUS_UNKNOWN -> BiometricStatus.UNKNOWN
            else -> BiometricStatus.UNKNOWN
        }
    }
    
    /**
     * Check if fingerprint is available
     */
    fun isFingerprintAvailable(): Boolean {
        val canAuthenticate = biometricManager.canAuthenticate(BiometricManager.Authenticators.BIOMETRIC_STRONG)
        return canAuthenticate == BiometricManager.BIOMETRIC_SUCCESS
    }
    
    /**
     * Check if face authentication is available
     */
    fun isFaceAvailable(): Boolean {
        // Android doesn't provide a direct API to check for face specifically
        // We can check if any biometric is available
        return isBiometricAvailable()
    }
    
    /**
     * Prompt user for biometric authentication
     */
    suspend fun authenticate(
        activity: FragmentActivity,
        title: String = "Authenticate",
        subtitle: String = "Please authenticate to continue",
        description: String = "Use your fingerprint or face to verify your identity",
        negativeButtonText: String = "Cancel"
    ): BiometricResult = suspendCancellableCoroutine { continuation ->
        
        val promptInfo = BiometricPrompt.PromptInfo.Builder()
            .setTitle(title)
            .setSubtitle(subtitle)
            .setDescription(description)
            .setNegativeButtonText(negativeButtonText)
            .setAllowedAuthenticators(
                BiometricManager.Authenticators.BIOMETRIC_STRONG or 
                BiometricManager.Authenticators.DEVICE_CREDENTIAL
            )
            .build()
        
        val biometricPrompt = BiometricPrompt(activity, executor, 
            object : BiometricPrompt.AuthenticationCallback() {
                override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                    continuation.resume(BiometricResult.Success)
                }
                
                override fun onAuthenticationFailed() {
                    // This is called when the authentication attempt fails but user can try again
                    // Don't resume here, wait for user to either succeed or cancel
                }
                
                override fun onAuthenticationError(errorCode: Int, errString: CharSequence) {
                    when (errorCode) {
                        BiometricPrompt.ERROR_USER_CANCELED,
                        BiometricPrompt.ERROR_NEGATIVE_BUTTON -> {
                            continuation.resume(BiometricResult.Cancelled)
                        }
                        else -> {
                            continuation.resume(BiometricResult.Failed(errString.toString()))
                        }
                    }
                }
            })
        
        biometricPrompt.authenticate(promptInfo)
        
        continuation.invokeOnCancellation {
            // Clean up if coroutine is cancelled
        }
    }
    
    /**
     * Check if device credential fallback is available
     */
    fun isDeviceCredentialAvailable(): Boolean {
        val canAuthenticate = biometricManager.canAuthenticate(BiometricManager.Authenticators.DEVICE_CREDENTIAL)
        return canAuthenticate == BiometricManager.BIOMETRIC_SUCCESS
    }
}

/**
 * Biometric authentication result
 */
sealed class BiometricResult {
    object Success : BiometricResult()
    object Cancelled : BiometricResult()
    data class Failed(val error: String) : BiometricResult()
}

/**
 * Biometric availability status
 */
enum class BiometricStatus {
    AVAILABLE,
    NO_HARDWARE,
    HW_UNAVAILABLE,
    NOT_ENROLLED,
    SECURITY_UPDATE_REQUIRED,
    UNSUPPORTED,
    UNKNOWN
}

/**
 * Biometric authentication type
 */
enum class BiometricType {
    FINGERPRINT,
    FACE,
    IRIS,
    DEVICE_CREDENTIAL,
    UNKNOWN
}

/**
 * Authentication configuration
 */
data class BiometricConfig(
    val title: String = "Authenticate",
    val subtitle: String = "Please authenticate to continue",
    val description: String = "Use your fingerprint or face to verify your identity",
    val negativeButtonText: String = "Cancel",
    val allowDeviceCredential: Boolean = true,
    val requireStrongBiometric: Boolean = true
)