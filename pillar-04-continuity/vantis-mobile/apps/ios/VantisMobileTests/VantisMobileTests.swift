import XCTest
@testable import VantisMobile

final class VantisMobileTests: XCTestCase {
    
    // MARK: - Device Tests
    
    func testDeviceInitialization() {
        let device = DeviceInfo(
            id: UUID(),
            name: "Test Device",
            type: .iPhone,
            osVersion: "17.0",
            appVersion: "1.0.0",
            lastSeen: Date(),
            isTrusted: true
        )
        
        XCTAssertEqual(device.name, "Test Device")
        XCTAssertEqual(device.type, .iPhone)
        XCTAssertTrue(device.isTrusted)
    }
    
    func testDeviceTypeDetection() {
        // iPhone type
        let iphoneType = DeviceType.currentDeviceType()
        XCTAssertTrue([.iPhone, .iPad, .mac].contains(iphoneType))
    }
    
    // MARK: - Document Tests
    
    func testDocumentInitialization() {
        let doc = DocumentMetadata(
            id: UUID(),
            name: "Test Document.pdf",
            type: .pdf,
            size: 1024,
            path: "/documents/test.pdf",
            createdAt: Date(),
            modifiedAt: Date(),
            author: "Test Author",
            tags: ["test", "document"]
        )
        
        XCTAssertEqual(doc.name, "Test Document.pdf")
        XCTAssertEqual(doc.type, .pdf)
        XCTAssertEqual(doc.size, 1024)
        XCTAssertEqual(doc.tags.count, 2)
    }
    
    func testDocumentTypeFromString() {
        XCTAssertEqual(DocumentType.fromString("pdf"), .pdf)
        XCTAssertEqual(DocumentType.fromString("docx"), .word)
        XCTAssertEqual(DocumentType.fromString("xlsx"), .spreadsheet)
        XCTAssertEqual(DocumentType.fromString("pptx"), .presentation)
        XCTAssertEqual(DocumentType.fromString("unknown"), .other)
    }
    
    func testDocumentFormattedSize() {
        let doc1 = DocumentMetadata(
            id: UUID(),
            name: "test",
            type: .pdf,
            size: 500,
            path: "",
            createdAt: Date(),
            modifiedAt: Date()
        )
        XCTAssertEqual(doc1.formattedSize, "500 B")
        
        let doc2 = DocumentMetadata(
            id: UUID(),
            name: "test",
            type: .pdf,
            size: 1536,
            path: "",
            createdAt: Date(),
            modifiedAt: Date()
        )
        XCTAssertEqual(doc2.formattedSize, "1.5 KB")
        
        let doc3 = DocumentMetadata(
            id: UUID(),
            name: "test",
            type: .pdf,
            size: 1572864,
            path: "",
            createdAt: Date(),
            modifiedAt: Date()
        )
        XCTAssertEqual(doc3.formattedSize, "1.5 MB")
    }
    
    // MARK: - Notification Tests
    
    func testNotificationInitialization() {
        let notification = VantisNotification(
            id: UUID(),
            type: .documentShared,
            title: "Document Shared",
            message: "A document was shared with you",
            timestamp: Date(),
            isRead: false,
            priority: .high,
            actionURL: URL(string: "vantis://documents/123")
        )
        
        XCTAssertEqual(notification.type, .documentShared)
        XCTAssertEqual(notification.title, "Document Shared")
        XCTAssertFalse(notification.isRead)
        XCTAssertEqual(notification.priority, .high)
    }
    
    func testNotificationTypeDescriptions() {
        XCTAssertEqual(NotificationType.documentShared.description, "Document Shared")
        XCTAssertEqual(NotificationType.commentAdded.description, "Comment Added")
        XCTAssertEqual(NotificationType.mention.description, "Mention")
        XCTAssertEqual(NotificationType.taskAssigned.description, "Task Assigned")
    }
    
    func testNotificationPriorityIcons() {
        XCTAssertEqual(NotificationPriority.low.icon, "info.circle")
        XCTAssertEqual(NotificationPriority.normal.icon, "bell")
        XCTAssertEqual(NotificationPriority.high.icon, "exclamationmark.triangle")
        XCTAssertEqual(NotificationPriority.urgent.icon, "exclamationmark.3")
    }
    
    // MARK: - Connection Tests
    
    func testConnectionInfoInitialization() {
        let connection = ConnectionInfo(
            id: UUID(),
            serverName: "VantisOffice Desktop",
            serverAddress: "192.168.1.100:8765",
            status: .connected,
            connectedAt: Date(),
            deviceId: UUID()
        )
        
        XCTAssertEqual(connection.serverName, "VantisOffice Desktop")
        XCTAssertEqual(connection.status, .connected)
        XCTAssertNotNil(connection.connectedAt)
    }
    
    func testConnectionStatusDescriptions() {
        XCTAssertEqual(ConnectionStatus.disconnected.description, "Disconnected")
        XCTAssertEqual(ConnectionStatus.connecting.description, "Connecting...")
        XCTAssertEqual(ConnectionStatus.connected.description, "Connected")
        XCTAssertEqual(ConnectionStatus.error.description, "Error")
    }
    
    func testConnectionDuration() {
        let connectedDate = Date().addingTimeInterval(-3600) // 1 hour ago
        let connection = ConnectionInfo(
            id: UUID(),
            serverName: "Test",
            serverAddress: "localhost:8765",
            status: .connected,
            connectedAt: connectedDate,
            deviceId: UUID()
        )
        
        let duration = connection.connectionDuration
        XCTAssertGreaterThan(duration, 3599)
        XCTAssertLessThan(duration, 3601)
    }
    
    // MARK: - Tunnel Config Tests
    
    func testTunnelConfigInitialization() {
        let config = TunnelConfig(
            serverURL: URL(string: "wss://localhost:8765")!,
            deviceId: UUID(),
            deviceName: "Test Device"
        )
        
        XCTAssertNotNil(config.serverURL)
        XCTAssertEqual(config.deviceName, "Test Device")
        XCTAssertFalse(config.autoReconnect) // default value
        XCTAssertNotNil(config.reconnectInterval)
    }
    
    // MARK: - Protocol Message Tests
    
    func testProtocolMessageEncoding() throws {
        let message = ProtocolMessage.ping(id: UUID())
        let data = try JSONEncoder().encode(message)
        XCTAssertGreaterThan(data.count, 0)
        
        let decoder = JSONDecoder()
        let decoded = try decoder.decode(ProtocolMessage.self, from: data)
        
        switch decoded {
        case .ping(let id):
            XCTAssertNotNil(id)
        default:
            XCTFail("Expected ping message")
        }
    }
    
    func testProtocolMessageTypes() {
        let ping = ProtocolMessage.ping(id: UUID())
        let pong = ProtocolMessage.pong(id: UUID())
        let syncRequest = ProtocolMessage.syncRequest(lastSyncTimestamp: Date())
        let syncResponse = ProtocolMessage.syncResponse(documents: [], syncTimestamp: Date())
        let command = ProtocolMessage.command(commandId: UUID(), commandType: "open", payload: nil)
        let notification = ProtocolMessage.notification(title: "Test", body: "Test notification", type: "info")
        let error = ProtocolMessage.error(code: "ERR001", message: "Test error")
        
        // All should be created successfully
        XCTAssertNotNil(ping)
        XCTAssertNotNil(pong)
        XCTAssertNotNil(syncRequest)
        XCTAssertNotNil(syncResponse)
        XCTAssertNotNil(command)
        XCTAssertNotNil(notification)
        XCTAssertNotNil(error)
    }
}