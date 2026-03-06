package com.vantiscorp.vantismobile.service

import com.vantiscorp.vantismobile.model.*
import kotlinx.coroutines.*
import kotlinx.coroutines.flow.*
import kotlinx.serialization.*
import kotlinx.serialization.json.*
import org.java_websocket.client.WebSocketClient
import org.java_websocket.handshake.ServerHandshake
import java.net.URI
import java.util.UUID
import java.util.concurrent.ConcurrentLinkedQueue

/**
 * Secure Tunnel Service for WebSocket communication with VantisOffice desktop
 */
class SecureTunnelService {
    private val json = Json { ignoreUnknownKeys = true; encodeDefaults = true }
    private var webSocketClient: WebSocketClient? = null
    private val scope = CoroutineScope(Dispatchers.IO + SupervisorJob())
    
    private val _connectionState = MutableStateFlow<ConnectionStatus>(ConnectionStatus.DISCONNECTED)
    val connectionState: StateFlow<ConnectionStatus> = _connectionState.asStateFlow()
    
    private val _messages = MutableSharedFlow<ProtocolMessage>()
    val messages: SharedFlow<ProtocolMessage> = _messages.asSharedFlow()
    
    private val _errors = MutableSharedFlow<String>()
    val errors: SharedFlow<String> = _errors.asSharedFlow()
    
    private val messageQueue = ConcurrentLinkedQueue<String>()
    private var config: TunnelConfig? = null
    
    /**
     * Connect to VantisOffice server
     */
    suspend fun connect(config: TunnelConfig): Result<Unit> = withContext(Dispatchers.IO) {
        this@SecureTunnelService.config = config
        
        try {
            _connectionState.value = ConnectionStatus.CONNECTING
            
            val uri = URI(config.serverUrl)
            webSocketClient = object : WebSocketClient(uri) {
                override fun onOpen(handshakedata: ServerHandshake?) {
                    _connectionState.value = ConnectionStatus.CONNECTED
                    // Send queued messages
                    while (messageQueue.isNotEmpty()) {
                        send(messageQueue.poll())
                    }
                }
                
                override fun onMessage(message: String?) {
                    message?.let { 
                        scope.launch {
                            try {
                                val protocolMessage = json.decodeFromString<ProtocolMessage>(it)
                                _messages.emit(protocolMessage)
                            } catch (e: Exception) {
                                _errors.emit("Failed to parse message: ${e.message}")
                            }
                        }
                    }
                }
                
                override fun onClose(code: Int, reason: String?, remote: Boolean) {
                    _connectionState.value = ConnectionStatus.DISCONNECTED
                    if (config.autoReconnect) {
                        scope.launch {
                            delay(config.reconnectInterval)
                            connect(config)
                        }
                    }
                }
                
                override fun onError(ex: Exception?) {
                    _connectionState.value = ConnectionStatus.ERROR
                    ex?.let { 
                        scope.launch { _errors.emit(it.message ?: "Unknown error") }
                    }
                }
            }
            
            webSocketClient?.connect()
            Result.success(Unit)
        } catch (e: Exception) {
            _connectionState.value = ConnectionStatus.ERROR
            Result.failure(e)
        }
    }
    
    /**
     * Disconnect from server
     */
    fun disconnect() {
        webSocketClient?.close()
        webSocketClient = null
        _connectionState.value = ConnectionStatus.DISCONNECTED
    }
    
    /**
     * Send message through tunnel
     */
    suspend fun sendMessage(message: ProtocolMessage): Result<Unit> = withContext(Dispatchers.IO) {
        try {
            val messageJson = json.encodeToString(message)
            when (_connectionState.value) {
                ConnectionStatus.CONNECTED, ConnectionStatus.AUTHENTICATED -> {
                    webSocketClient?.send(messageJson)
                    Result.success(Unit)
                }
                else -> {
                    messageQueue.offer(messageJson)
                    Result.failure(Exception("Not connected, message queued"))
                }
            }
        } catch (e: Exception) {
            Result.failure(e)
        }
    }
    
    /**
     * Request document sync
     */
    suspend fun requestSync(lastSyncTimestamp: Long? = null) {
        val message = ProtocolMessage.SyncRequest(
            lastSyncTimestamp = lastSyncTimestamp ?: System.currentTimeMillis()
        )
        sendMessage(message)
    }
    
    /**
     * Execute remote command
     */
    suspend fun executeCommand(commandType: String, payload: Map<String, JsonElement>? = null): Result<UUID> {
        val commandId = UUID.randomUUID()
        val message = ProtocolMessage.Command(
            commandId = commandId,
            commandType = commandType,
            payload = payload?.let { JsonObject(it) }
        )
        return sendMessage(message).map { commandId }
    }
    
    /**
     * Ping server for latency check
     */
    suspend fun ping(): Long {
        val startTime = System.currentTimeMillis()
        sendMessage(ProtocolMessage.Ping(UUID.randomUUID()))
        return System.currentTimeMillis() - startTime
    }
    
    /**
     * Get current connection info
     */
    fun getConnectionInfo(): ConnectionInfo? {
        val currentConfig = config ?: return null
        return ConnectionInfo(
            id = UUID.randomUUID(),
            serverName = currentConfig.deviceName,
            serverAddress = currentConfig.serverUrl,
            status = _connectionState.value,
            deviceId = currentConfig.deviceId
        )
    }
    
    /**
     * Check if connected
     */
    fun isConnected(): Boolean = _connectionState.value == ConnectionStatus.CONNECTED ||
            _connectionState.value == ConnectionStatus.AUTHENTICATED
    
    /**
     * Clean up resources
     */
    fun cleanup() {
        disconnect()
        scope.cancel()
    }
}

/**
 * Protocol message types for WebSocket communication
 */
@Serializable
sealed class ProtocolMessage {
    @Serializable
    @SerialName("ping")
    data class Ping(val id: UUID) : ProtocolMessage()
    
    @Serializable
    @SerialName("pong")
    data class Pong(val id: UUID) : ProtocolMessage()
    
    @Serializable
    @SerialName("sync_request")
    data class SyncRequest(val lastSyncTimestamp: Long) : ProtocolMessage()
    
    @Serializable
    @SerialName("sync_response")
    data class SyncResponse(
        val documents: List<JsonElement>,
        val syncTimestamp: Long
    ) : ProtocolMessage()
    
    @Serializable
    @SerialName("command")
    data class Command(
        val commandId: UUID,
        val commandType: String,
        val payload: JsonObject? = null
    ) : ProtocolMessage()
    
    @Serializable
    @SerialName("command_response")
    data class CommandResponse(
        val commandId: UUID,
        val success: Boolean,
        val result: JsonObject? = null,
        val error: String? = null
    ) : ProtocolMessage()
    
    @Serializable
    @SerialName("notification")
    data class Notification(
        val title: String,
        val body: String,
        val type: String,
        val priority: String = "normal"
    ) : ProtocolMessage()
    
    @Serializable
    @SerialName("error")
    data class Error(val code: String, val message: String) : ProtocolMessage()
    
    @Serializable
    @SerialName("sync_progress")
    data class SyncProgress(
        val progress: Float,
        val currentFile: String? = null,
        val filesRemaining: Int
    ) : ProtocolMessage()
    
    @Serializable
    @SerialName("device_info")
    data class DeviceInfo(
        val deviceId: UUID,
        val deviceName: String,
        val deviceType: String,
        val osVersion: String
    ) : ProtocolMessage()
}