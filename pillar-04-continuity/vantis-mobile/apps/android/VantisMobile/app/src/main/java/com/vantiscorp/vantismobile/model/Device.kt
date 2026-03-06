package com.vantiscorp.vantismobile.model

import java.util.UUID

/**
 * Device type enumeration for supported platforms
 */
enum class DeviceType {
    SMARTPHONE,
    TABLET,
    DESKTOP,
    LAPTOP,
    UNKNOWN;

    companion object {
        fun fromString(value: String): DeviceType = when (value.lowercase()) {
            "smartphone", "phone", "iphone", "android" -> SMARTPHONE
            "tablet", "ipad" -> TABLET
            "desktop", "pc", "mac" -> DESKTOP
            "laptop", "macbook" -> LAPTOP
            else -> UNKNOWN
        }
    }
}

/**
 * Device information data class
 */
data class DeviceInfo(
    val id: UUID,
    val name: String,
    val type: DeviceType,
    val osVersion: String,
    val appVersion: String,
    val lastSeen: Long = System.currentTimeMillis(),
    val isTrusted: Boolean = false,
    val ipAddress: String? = null,
    val macAddress: String? = null
) {
    companion object {
        /**
         * Get current device information
         */
        fun currentDevice(context: android.content.Context): DeviceInfo {
            val packageManager = context.packageManager
            val packageInfo = packageManager.getPackageInfo(context.packageName, 0)
            
            return DeviceInfo(
                id = UUID.randomUUID(),
                name = android.os.Build.MODEL,
                type = if (android.content.res.Configuration.ORIENTATION_PORTRAIT == 
                    context.resources.configuration.orientation) DeviceType.SMARTPHONE else DeviceType.TABLET,
                osVersion = "Android ${android.os.Build.VERSION.RELEASE}",
                appVersion = packageInfo.versionName ?: "1.0.0",
                lastSeen = System.currentTimeMillis(),
                isTrusted = true
            )
        }
    }
    
    /**
     * Check if device is online (seen within last 5 minutes)
     */
    fun isOnline(): Boolean {
        val fiveMinutesAgo = System.currentTimeMillis() - (5 * 60 * 1000)
        return lastSeen > fiveMinutesAgo
    }
    
    /**
     * Format device type for display
     */
    fun formattedType(): String = when (type) {
        DeviceType.SMARTPHONE -> "Smartphone"
        DeviceType.TABLET -> "Tablet"
        DeviceType.DESKTOP -> "Desktop"
        DeviceType.LAPTOP -> "Laptop"
        DeviceType.UNKNOWN -> "Unknown"
    }
}

/**
 * Device pairing state
 */
enum class PairingState {
    NOT_PAIRED,
    PAIRING_IN_PROGRESS,
    PAIRED,
    PAIRING_FAILED
}

/**
 * Paired device with additional pairing information
 */
data class PairedDevice(
    val deviceInfo: DeviceInfo,
    val pairingState: PairingState,
    val pairingDate: Long? = null,
    val lastSyncDate: Long? = null
) {
    /**
     * Check if device needs sync
     */
    fun needsSync(): Boolean {
        val lastSync = lastSyncDate ?: return true
        val oneHourAgo = System.currentTimeMillis() - (60 * 60 * 1000)
        return lastSync < oneHourAgo
    }
}