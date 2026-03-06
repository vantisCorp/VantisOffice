//
//  Device.swift
//  VantisMobile
//
//  Core device models for mobile-desktop communication
//

import Foundation

/// Device type enumeration
enum DeviceType: String, Codable {
    case desktop
    case laptop
    case ios = "ios"
    case android
    case tablet
}

/// Device information model
struct DeviceInfo: Codable, Identifiable, Hashable {
    let id: UUID
    let deviceType: DeviceType
    let deviceName: String
    let osVersion: String
    let appVersion: String
    let lastSeen: Date
    
    init(deviceType: DeviceType, deviceName: String, osVersion: String, appVersion: String) {
        self.id = UUID()
        self.deviceType = deviceType
        self.deviceName = deviceName
        self.osVersion = osVersion
        self.appVersion = appVersion
        self.lastSeen = Date()
    }
    
    /// Update last seen timestamp
    mutating func updateLastSeen() {
        self.lastSeen = Date()
    }
    
    /// Create device info for current iOS device
    static func current() -> DeviceInfo {
        let device = UIDevice.current
        let osVersion = "\(device.systemName) \(device.systemVersion)"
        let deviceName = device.name
        
        // Determine device type
        let deviceType: DeviceType
        if UIDevice.current.userInterfaceIdiom == .pad {
            deviceType = .tablet
        } else {
            deviceType = .ios
        }
        
        return DeviceInfo(
            deviceType: deviceType,
            deviceName: deviceName,
            osVersion: osVersion,
            appVersion: "1.0.0"
        )
    }
}