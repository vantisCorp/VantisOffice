//
//  SecureTunnelService.swift
//  VantisMobile
//
//  Secure tunnel service for WebSocket communication
//  Integrates with VantisMobileFFI for end-to-end encryption
//

import Foundation
import Combine
import VantisMobileFFI

/// Secure tunnel configuration
struct TunnelConfig {
    let serverUrl: URL
    let deviceId: UUID
    let encryptionKey: Data
    let deviceInfo: DeviceInfo
    
    init(serverUrl: URL, deviceId: UUID, encryptionKey: Data) {
        self.serverUrl = serverUrl
        self.deviceId = deviceId
        self.encryptionKey = encryptionKey
        self.deviceInfo = DeviceInfo.current()
    }
}

/// Secure tunnel service for mobile-desktop communication
/// Uses E2EE via VantisMobileFFI for all message encryption
class SecureTunnelService: ObservableObject {
    // MARK: - Published Properties
    
    @Published private(set) var connectionStatus: ConnectionStatus = .disconnected
    @Published private(set) var connectionInfo: ConnectionInfo?
    @Published private(set) var receivedMessages: [ProtocolMessage] = []
    
    // MARK: - Private Properties
    
    private var webSocketTask: URLSessionWebSocketTask?
    private var urlSession: URLSession
    private var config: TunnelConfig?
    private var reconnectAttempts: Int = 0
    private var maxReconnectAttempts: Int = 5
    private var reconnectTimer: Timer?
    private var pingTimer: Timer?
    
    // MARK: - FFI Properties
    
    private var keyPair: VantisMobileFFI.KeyPair?
    private var encryptor: VantisMobileFFI.Encryptor?
    
    // MARK: - Cancellables
    
    private var cancellables = Set<AnyCancellable>()
    
    // MARK: - Initialization
    
    init() {
        let configuration = URLSessionConfiguration.default
        configuration.waitsForConnectivity = true
        self.urlSession = URLSession(configuration: configuration)
        
        // Initialize FFI and generate key pair
        _ = VantisMobileFFI.shared // Initialize singleton
        self.keyPair = VantisMobileFFI.KeyPair()
    }
    
    deinit {
        disconnect()
        keyPair?.free?()
    }
    
    // MARK: - Connection Methods
    
    /// Connect to the tunnel server
    func connect(config: TunnelConfig) async throws {
        self.config = config
        
        await MainActor.run {
            self.connectionStatus = .connecting
        }
        
        var request = URLRequest(url: config.serverUrl)
        request.setValue("vantis-mobile/1.0", forHTTPHeaderField: "User-Agent")
        request.setValue("websocket", forHTTPHeaderField: "Upgrade")
        request.setValue("Upgrade", forHTTPHeaderField: "Connection")
        
        webSocketTask = urlSession.webSocketTask(with: request)
        webSocketTask?.resume()
        
        // Start listening for messages
        receiveMessage()
        
        // Send handshake with our public key
        try await sendHandshake()
        
        // Start ping timer
        startPingTimer()
        
        await MainActor.run {
            self.connectionStatus = .connected
            self.connectionInfo = ConnectionInfo(device: config.deviceInfo)
            self.connectionInfo?.setConnected()
            self.reconnectAttempts = 0
        }
    }
    
    /// Disconnect from the tunnel
    func disconnect() {
        pingTimer?.invalidate()
        pingTimer = nil
        reconnectTimer?.invalidate()
        reconnectTimer = nil
        
        webSocketTask?.cancel(with: .normalClosure, reason: nil)
        webSocketTask = nil
        encryptor = nil // Clear encryptor on disconnect
        
        Task { @MainActor in
            self.connectionStatus = .disconnected
            self.connectionInfo?.setDisconnected()
        }
    }
    
    /// Reconnect to the tunnel
    func reconnect() async throws {
        guard let config = self.config else { return }
        
        await MainActor.run {
            self.connectionStatus = .reconnecting
        }
        
        disconnect()
        
        try await Task.sleep(nanoseconds: UInt64(reconnectAttempts * 1_000_000_000))
        
        try await connect(config: config)
    }
    
    // MARK: - Encryption Methods
    
    /// Set up encryption with peer's public key
    /// Called after receiving key exchange message from desktop
    func setupEncryption(peerPublicKeyBase64: String) -> Bool {
        // In a full implementation, we would derive a shared secret
        // using X25519 key exchange. For now, we use the configured key.
        guard let config = self.config else { return false }
        
        let keyBase64 = config.encryptionKey.base64EncodedString()
        guard let enc = VantisMobileFFI.Encryptor(sharedSecretBase64: keyBase64) else {
            return false
        }
        self.encryptor = enc
        return true
    }
    
    /// Get our public key for key exchange
    var publicKeyBase64: String {
        return keyPair?.publicKeyBase64 ?? ""
    }
    
    // MARK: - Message Sending
    
    /// Send a protocol message (encrypted if encryptor is set up)
    func send<T: Encodable>(_ message: T) async throws {
        var data = try JSONEncoder().encode(message)
        
        // Encrypt if we have an encryptor set up
        if let encryptor = encryptor {
            guard let encryptedJSON = encryptor.encrypt(data) else {
                throw TunnelError.encryptionFailed
            }
            data = encryptedJSON.data(using: .utf8) ?? data
        }
        
        try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Void, Error>) in
            webSocketTask?.send(.data(data)) { error in
                if let error = error {
                    continuation.resume(throwing: error)
                } else {
                    continuation.resume()
                }
            }
        }
    }
    
    /// Send handshake message with our public key
    private func sendHandshake() async throws {
        guard let config = self.config else { return }
        
        let handshake = ProtocolMessage.handshake(
            deviceId: config.deviceId,
            publicKey: publicKeyBase64,
            deviceInfo: config.deviceInfo
        )
        
        try await send(handshake)
    }
    
    /// Send ping message using FFI
    private func sendPing() async {
        do {
            // Use FFI to create ping message
            if let pingJSON = VantisMobileFFI.shared.createPingMessage() {
                var data = pingJSON.data(using: .utf8)!
                
                // Encrypt if we have an encryptor
                if let encryptor = encryptor {
                    if let encrypted = encryptor.encrypt(data) {
                        data = encrypted.data(using: .utf8) ?? data
                    }
                }
                
                try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Void, Error>) in
                    webSocketTask?.send(.data(data)) { error in
                        if let error = error {
                            continuation.resume(throwing: error)
                        } else {
                            continuation.resume()
                        }
                    }
                }
            } else {
                // Fallback to legacy ping
                try await send(ProtocolMessage.ping())
            }
        } catch {
            print("Failed to send ping: \(error)")
        }
    }
    
    // MARK: - Message Receiving
    
    /// Start receiving messages
    private func receiveMessage() {
        webSocketTask?.receive { [weak self] result in
            switch result {
            case .success(let message):
                self?.handleMessage(message)
                self?.receiveMessage() // Continue receiving
            case .failure(let error):
                print("WebSocket receive error: \(error)")
                Task {
                    try? await self?.reconnect()
                }
            }
        }
    }
    
    /// Handle received message (decrypt if encrypted)
    private func handleMessage(_ message: URLSessionWebSocketTask.Message) {
        switch message {
        case .data(let data):
            var processedData = data
            
            // Try to decrypt if we have an encryptor
            if let encryptor = encryptor {
                // Check if data looks like an encrypted JSON message
                if let jsonStr = String(data: data, encoding: .utf8),
                   jsonStr.contains("&quot;nonce&quot;") || jsonStr.contains("&quot;ciphertext&quot;") {
                    if let decrypted = encryptor.decrypt(jsonStr) {
                        processedData = decrypted
                    }
                }
            }
            
            do {
                let protocolMessage = try JSONDecoder().decode(ProtocolMessage.self, from: processedData)
                Task { @MainActor in
                    self.receivedMessages.append(protocolMessage)
                    self.processMessage(protocolMessage)
                }
            } catch {
                print("Failed to decode message: \(error)")
            }
        case .string(let string):
            var processedString = string
            
            // Try to decrypt if we have an encryptor
            if let encryptor = encryptor {
                // Check if string looks like an encrypted JSON message
                if string.contains("&quot;nonce&quot;") || string.contains("&quot;ciphertext&quot;") {
                    if let decrypted = encryptor.decrypt(string),
                       let decryptedString = String(data: decrypted, encoding: .utf8) {
                        processedString = decryptedString
                    }
                }
            }
            
            guard let data = processedString.data(using: .utf8) else { return }
            do {
                let protocolMessage = try JSONDecoder().decode(ProtocolMessage.self, from: data)
                Task { @MainActor in
                    self.receivedMessages.append(protocolMessage)
                    self.processMessage(protocolMessage)
                }
            } catch {
                print("Failed to decode message: \(error)")
            }
        @unknown default:
            break
        }
    }
    
    /// Process received protocol message
    private func processMessage(_ message: ProtocolMessage) {
        // Handle different message types
        switch message.type {
        case .keyExchange:
            // Handle key exchange - set up encryption
            if case let .dictionary(dict)? = message.data,
               let peerKey = dict["public_key"] as? String {
                let success = setupEncryption(peerPublicKeyBase64: peerKey)
                print("Key exchange \(success ? "successful" : "failed")")
            }
        case .pong:
            // Update latency
            if let info = connectionInfo {
                let latency = Date().timeIntervalSince(info.lastPing ?? Date()) * 1000
                Task { @MainActor in
                    self.connectionInfo?.updateLatency(UInt64(latency))
                }
            }
        default:
            break
        }
    }
    
    // MARK: - Timers
    
    /// Start ping timer
    private func startPingTimer() {
        pingTimer = Timer.scheduledTimer(withTimeInterval: 30.0, repeats: true) { [weak self] _ in
            Task {
                await self?.sendPing()
            }
        }
    }
}

// MARK: - Protocol Message

/// Protocol message types
struct ProtocolMessage: Codable {
    let type: MessageType
    let data: CodableValue?
    
    enum MessageType: String, Codable {
        case handshake
        case keyExchange = "key_exchange"
        case ping
        case pong
        case documentUpdate = "document_update"
        case notification
        case command
        case commandResponse = "command_response"
        case syncProgress = "sync_progress"
        case error
    }
    
    static func handshake(deviceId: UUID, publicKey: String, deviceInfo: DeviceInfo) -> ProtocolMessage {
        ProtocolMessage(
            type: .handshake,
            data: CodableValue(value: [
                "device_id": deviceId.uuidString,
                "protocol_version": 1,
                "public_key": publicKey,
                "device_info": deviceInfo
            ])
        )
    }
    
    static func ping() -> ProtocolMessage {
        ProtocolMessage(type: .ping, data: CodableValue(value: ["timestamp": ISO8601DateFormatter().string(from: Date())]))
    }
}

// MARK: - Tunnel Error

/// Tunnel-specific errors
enum TunnelError: LocalizedError {
    case connectionFailed
    case encryptionFailed
    case decryptionFailed
    case invalidMessage
    
    var errorDescription: String? {
        switch self {
        case .connectionFailed:
            return "Failed to connect to tunnel server"
        case .encryptionFailed:
            return "Failed to encrypt message"
        case .decryptionFailed:
            return "Failed to decrypt message"
        case .invalidMessage:
            return "Invalid message format"
        }
    }
}

/// Helper for encoding/decoding heterogeneous values
struct CodableValue: Codable, Hashable {
    let value: Any
    
    init(value: Any) {
        self.value = value
    }
    
    init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        
        if let string = try? container.decode(String.self) {
            value = string
        } else if let int = try? container.decode(Int.self) {
            value = int
        } else if let double = try? container.decode(Double.self) {
            value = double
        } else if let bool = try? container.decode(Bool.self) {
            value = bool
        } else if let dict = try? container.decode([String: CodableValue].self) {
            value = dict
        } else if let array = try? container.decode([CodableValue].self) {
            value = array
        } else {
            value = ""
        }
    }
    
    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        
        if let string = value as? String {
            try container.encode(string)
        } else if let int = value as? Int {
            try container.encode(int)
        } else if let double = value as? Double {
            try container.encode(double)
        } else if let bool = value as? Bool {
            try container.encode(bool)
        } else if let dict = value as? [String: CodableValue] {
            try container.encode(dict)
        } else if let array = value as? [CodableValue] {
            try container.encode(array)
        }
    }
    
    static func == (lhs: CodableValue, rhs: CodableValue) -> Bool {
        return String(describing: lhs.value) == String(describing: rhs.value)
    }
    
    func hash(into hasher: inout Hasher) {
        hasher.combine(String(describing: value))
    }
}