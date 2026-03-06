package com.vantiscorp.vantismobile.model

import java.util.UUID

/**
 * Connection status enumeration
 */
enum class ConnectionStatus {
    DISCONNECTED,
    CONNECTING,
    CONNECTED,
    AUTHENTICATING,
    AUTHENTICATED,
    ERROR;

    val description: String
        get() = when (this) {
            DISCONNECTED -> "Disconnected"
            CONNECTING -> "Connecting..."
            CONNECTED -> "Connected"
            AUTHENTICATING -> "Authenticating..."
            AUTHENTICATED -> "Authenticated"
            ERROR -> "Error"
        }

    val color: Long
        get() = when (this) {
            DISCONNECTED -> 0xFF9E9E9E
            CONNECTING -> 0xFFFFC107
            CONNECTED -> 0xFF4CAF50
            AUTHENTICATING -> 0xFF2196F3
            AUTHENTICATED -> 0xFF00BCD4
            ERROR -> 0xFFF44336
        }
}

/**
 * Connection information data class
 */
data class ConnectionInfo(
    val id: UUID,
    val serverName: String,
    val serverAddress: String,
    val status: ConnectionStatus = ConnectionStatus.DISCONNECTED,
    val connectedAt: Long? = null,
    val deviceId: UUID,
    val errorMessage: String? = null,
    val latency: Long? = null,
    val bytesReceived: Long = 0,
    val bytesSent: Long = 0
) {
    /**
     * Check if currently connected
     */
    fun isConnected(): Boolean = status == ConnectionStatus.CONNECTED || 
            status == ConnectionStatus.AUTHENTICATED

    /**
     * Get connection duration in seconds
     */
    fun getConnectionDuration(): Long? {
        val connected = connectedAt ?: return null
        return (System.currentTimeMillis() - connected) / 1000
    }

    /**
     * Format connection duration for display
     */
    fun formattedDuration(): String {
        val duration = getConnectionDuration() ?: return "--:--"
        val hours = duration / 3600
        val minutes = (duration % 3600) / 60
        val seconds = duration % 60
        return when {
            hours > 0 -> String.format("%d:%02d:%02d", hours, minutes, seconds)
            else -> String.format("%02d:%02d", minutes, seconds)
        }
    }

    /**
     * Format latency for display
     */
    fun formattedLatency(): String {
        val lat = latency ?: return "--"
        return "${lat}ms"
    }

    /**
     * Check connection quality based on latency
     */
    fun getConnectionQuality(): ConnectionQuality {
        val lat = latency ?: return ConnectionQuality.UNKNOWN
        return when {
            lat < 50 -> ConnectionQuality.EXCELLENT
            lat < 100 -> ConnectionQuality.GOOD
            lat < 200 -> ConnectionQuality.FAIR
            lat < 500 -> ConnectionQuality.POOR
            else -> ConnectionQuality.BAD
        }
    }
}

/**
 * Connection quality indicator
 */
enum class ConnectionQuality {
    EXCELLENT,
    GOOD,
    FAIR,
    POOR,
    BAD,
    UNKNOWN;

    val bars: Int
        get() = when (this) {
            EXCELLENT -> 4
            GOOD -> 3
            FAIR -> 2
            POOR -> 1
            BAD -> 0
            UNKNOWN -> 0
        }

    val description: String
        get() = when (this) {
            EXCELLENT -> "Excellent"
            GOOD -> "Good"
            FAIR -> "Fair"
            POOR -> "Poor"
            BAD -> "Bad"
            UNKNOWN -> "Unknown"
        }
}

/**
 * Server discovery result
 */
data class DiscoveredServer(
    val name: String,
    val address: String,
    val port: Int,
    val requiresAuth: Boolean = true,
    val version: String? = null
) {
    val fullAddress: String
        get() = "$address:$port"
}

/**
 * Tunnel configuration
 */
data class TunnelConfig(
    val serverUrl: String,
    val deviceId: UUID,
    val deviceName: String,
    val autoReconnect: Boolean = true,
    val reconnectInterval: Long = 5000,
    val connectionTimeout: Long = 30000,
    val encryptionEnabled: Boolean = true
)