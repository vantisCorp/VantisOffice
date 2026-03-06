//
//  Notification.swift
//  VantisMobile
//
//  Notification models for mobile notifications
//

import Foundation
import UserNotifications

/// Notification type enumeration
enum NotificationType: String, Codable {
    case documentUpdated = "document_updated"
    case collaborationRequest = "collaboration_request"
    case system
    case calendarEvent = "calendar_event"
    case commentAdded = "comment_added"
    case shareRequest = "share_request"
}

/// Notification priority enumeration
enum NotificationPriority: String, Codable, Comparable {
    case low
    case normal
    case high
    case urgent
    
    static func < (lhs: NotificationPriority, rhs: NotificationPriority) -> Bool {
        switch (lhs, rhs) {
        case (.low, .normal), (.low, .high), (.low, .urgent),
             (.normal, .high), (.normal, .urgent),
             (.high, .urgent):
            return true
        default:
            return false
        }
    }
}

/// Notification model
struct VantisNotification: Codable, Identifiable, Hashable {
    let id: UUID
    let notificationType: NotificationType
    let priority: NotificationPriority
    let title: String
    let body: String
    let timestamp: Date
    var read: Bool
    let documentId: UUID?
    let userId: UUID?
    let actionUrl: String?
    
    init(
        notificationType: NotificationType,
        priority: NotificationPriority,
        title: String,
        body: String,
        documentId: UUID? = nil,
        userId: UUID? = nil,
        actionUrl: String? = nil
    ) {
        self.id = UUID()
        self.notificationType = notificationType
        self.priority = priority
        self.title = title
        self.body = body
        self.timestamp = Date()
        self.read = false
        self.documentId = documentId
        self.userId = userId
        self.actionUrl = actionUrl
    }
    
    /// Mark notification as read
    mutating func markRead() {
        self.read = true
    }
    
    /// Convert to UNNotificationContent
    func toUNNotificationContent() -> UNMutableNotificationContent {
        let content = UNMutableNotificationContent()
        content.title = self.title
        content.body = self.body
        content.sound = .default
        content.badge = NSNumber(value: 1)
        content.userInfo = [
            "notificationId": self.id.uuidString,
            "type": self.notificationType.rawValue,
            "documentId": self.documentId?.uuidString ?? ""
        ]
        return content
    }
    
    /// Icon name for notification type
    var iconName: String {
        switch notificationType {
        case .documentUpdated:
            return "doc.badge.arrow.up"
        case .collaborationRequest:
            return "person.badge.plus"
        case .system:
            return "bell"
        case .calendarEvent:
            return "calendar"
        case .commentAdded:
            return "bubble.left"
        case .shareRequest:
            return "square.and.arrow.up"
        }
    }
}