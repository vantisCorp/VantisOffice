//
//  Connection.swift
//  VantisMobile
//
//  Connection models for secure tunnel
//

import Foundation

/// Connection status enumeration
enum ConnectionStatus: String, Codable {
    case disconnected
    case connecting
    case connected
    case reconnecting
}

/// Connection information model
struct ConnectionInfo: Codable, Identifiable, Hashable {
    let id: UUID
    let status: ConnectionStatus
    let device: DeviceInfo
    let connectedAt: Date?
    let latencyMs: UInt64?
    let lastPing: Date?
    
    init(device: DeviceInfo) {
        self.id = UUID()
        self.status = .disconnected
        self.device = device
        self.connectedAt = nil
        self.latencyMs = nil
        self.lastPing = nil
    }
    
    /// Set connected status
    mutating func setConnected() {
        self.status = .connected
        self.connectedAt = Date()
    }
    
    /// Set disconnected
    mutating func setDisconnected() {
        self.status = .disconnected
        self.connectedAt = nil
        self.latencyMs = nil
        self.lastPing = nil
    }
    
    /// Update latency
    mutating func updateLatency(_ latencyMs: UInt64) {
        self.latencyMs = latencyMs
        self.lastPing = Date()
    }
    
    /// Connection duration
    var connectionDuration: TimeInterval? {
        guard let connectedAt = connectedAt else { return nil }
        return Date().timeIntervalSince(connectedAt)
    }
    
    /// Formatted connection duration
    var formattedDuration: String {
        guard let duration = connectionDuration else { return "Not connected" }
        
        if duration < 60 {
            return "\(Int(duration))s"
        } else if duration < 3600 {
            return "\(Int(duration / 60))m \(Int(duration % 60))s"
        } else {
            let hours = Int(duration / 3600)
            let minutes = Int((duration.truncatingRemainder(dividingBy: 3600)) / 60)
            return "\(hours)h \(minutes)m"
        }
    }
    
    /// Latency formatted as string
    var formattedLatency: String {
        guard let latency = latencyMs else { return "N/A" }
        return "\(latency)ms"
    }
}