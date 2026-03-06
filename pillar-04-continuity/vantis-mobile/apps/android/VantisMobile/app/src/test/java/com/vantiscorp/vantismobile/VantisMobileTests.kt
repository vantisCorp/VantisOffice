package com.vantiscorp.vantismobile

import com.vantiscorp.vantismobile.model.*
import org.junit.Assert.*
import org.junit.Test
import java.util.UUID

/**
 * Unit tests for VantisMobile application
 */
class VantisMobileTests {
    
    // MARK: - Device Tests
    
    @Test
    fun testDeviceTypeFromString() {
        assertEquals(DeviceType.SMARTPHONE, DeviceType.fromString("smartphone"))
        assertEquals(DeviceType.TABLET, DeviceType.fromString("tablet"))
        assertEquals(DeviceType.DESKTOP, DeviceType.fromString("desktop"))
        assertEquals(DeviceType.LAPTOP, DeviceType.fromString("laptop"))
        assertEquals(DeviceType.UNKNOWN, DeviceType.fromString("unknown"))
    }
    
    @Test
    fun testDeviceInfoFormattedType() {
        val device = DeviceInfo(
            id = UUID.randomUUID(),
            name = "Test Device",
            type = DeviceType.SMARTPHONE,
            osVersion = "Android 14",
            appVersion = "1.0.0"
        )
        assertEquals("Smartphone", device.formattedType())
    }
    
    @Test
    fun testDeviceInfoIsOnline() {
        val recentDevice = DeviceInfo(
            id = UUID.randomUUID(),
            name = "Recent Device",
            type = DeviceType.SMARTPHONE,
            osVersion = "Android 14",
            appVersion = "1.0.0",
            lastSeen = System.currentTimeMillis() - 60000 // 1 minute ago
        )
        assertTrue(recentDevice.isOnline())
        
        val oldDevice = DeviceInfo(
            id = UUID.randomUUID(),
            name = "Old Device",
            type = DeviceType.SMARTPHONE,
            osVersion = "Android 14",
            appVersion = "1.0.0",
            lastSeen = System.currentTimeMillis() - 600000 // 10 minutes ago
        )
        assertFalse(oldDevice.isOnline())
    }
    
    // MARK: - Document Tests
    
    @Test
    fun testDocumentTypeFromExtension() {
        assertEquals(DocumentType.PDF, DocumentType.fromExtension("pdf"))
        assertEquals(DocumentType.WORD, DocumentType.fromExtension("docx"))
        assertEquals(DocumentType.SPREADSHEET, DocumentType.fromExtension("xlsx"))
        assertEquals(DocumentType.PRESENTATION, DocumentType.fromExtension("pptx"))
        assertEquals(DocumentType.IMAGE, DocumentType.fromExtension("jpg"))
        assertEquals(DocumentType.OTHER, DocumentType.fromExtension("unknown"))
    }
    
    @Test
    fun testDocumentFormattedSize() {
        val doc1 = DocumentMetadata(
            id = UUID.randomUUID(),
            name = "test",
            type = DocumentType.PDF,
            size = 500,
            path = ""
        )
        assertEquals("500 B", doc1.formattedSize())
        
        val doc2 = DocumentMetadata(
            id = UUID.randomUUID(),
            name = "test",
            type = DocumentType.PDF,
            size = 1536,
            path = ""
        )
        assertEquals("1.5 KB", doc2.formattedSize())
        
        val doc3 = DocumentMetadata(
            id = UUID.randomUUID(),
            name = "test",
            type = DocumentType.PDF,
            size = 1572864,
            path = ""
        )
        assertEquals("1.5 MB", doc3.formattedSize())
    }
    
    @Test
    fun testDocumentIsRecent() {
        val recentDoc = DocumentMetadata(
            id = UUID.randomUUID(),
            name = "Recent",
            type = DocumentType.PDF,
            size = 1024,
            path = "",
            modifiedAt = System.currentTimeMillis() - 86400000 // 1 day ago
        )
        assertTrue(recentDoc.isRecent())
        
        val oldDoc = DocumentMetadata(
            id = UUID.randomUUID(),
            name = "Old",
            type = DocumentType.PDF,
            size = 1024,
            path = "",
            modifiedAt = System.currentTimeMillis() - 864000000 // 10 days ago
        )
        assertFalse(oldDoc.isRecent())
    }
    
    @Test
    fun testDocumentFilterMatches() {
        val doc = DocumentMetadata(
            id = UUID.randomUUID(),
            name = "Test Document.pdf",
            type = DocumentType.PDF,
            size = 1024,
            path = "",
            tags = listOf("test", "important"),
            isFavorite = true
        )
        
        val filter1 = DocumentFilter(type = DocumentType.PDF)
        assertTrue(filter1.matches(doc))
        
        val filter2 = DocumentFilter(type = DocumentType.WORD)
        assertFalse(filter2.matches(doc))
        
        val filter3 = DocumentFilter(tags = listOf("test"))
        assertTrue(filter3.matches(doc))
        
        val filter4 = DocumentFilter(favoritesOnly = true)
        assertTrue(filter4.matches(doc))
    }
    
    // MARK: - Notification Tests
    
    @Test
    fun testNotificationTypeDescription() {
        assertEquals("Document Shared", NotificationType.DOCUMENT_SHARED.description)
        assertEquals("Comment Added", NotificationType.COMMENT_ADDED.description)
        assertEquals("Mention", NotificationType.MENTION.description)
    }
    
    @Test
    fun testNotificationPriorityIcons() {
        assertEquals("info_outline", NotificationPriority.LOW.icon)
        assertEquals("notifications", NotificationPriority.NORMAL.icon)
        assertEquals("warning", NotificationPriority.HIGH.icon)
        assertEquals("error", NotificationPriority.URGENT.icon)
    }
    
    @Test
    fun testNotificationRelativeTime() {
        val notification = VantisNotification(
            id = UUID.randomUUID(),
            type = NotificationType.SYSTEM,
            title = "Test",
            message = "Test notification"
        )
        
        val relativeTime = notification.getRelativeTime()
        assertTrue(relativeTime.isNotEmpty())
    }
    
    // MARK: - Connection Tests
    
    @Test
    fun testConnectionStatusDescription() {
        assertEquals("Disconnected", ConnectionStatus.DISCONNECTED.description)
        assertEquals("Connecting...", ConnectionStatus.CONNECTING.description)
        assertEquals("Connected", ConnectionStatus.CONNECTED.description)
        assertEquals("Error", ConnectionStatus.ERROR.description)
    }
    
    @Test
    fun testConnectionIsConnected() {
        val connected = ConnectionInfo(
            id = UUID.randomUUID(),
            serverName = "Test",
            serverAddress = "localhost:8765",
            status = ConnectionStatus.CONNECTED,
            deviceId = UUID.randomUUID()
        )
        assertTrue(connected.isConnected())
        
        val disconnected = ConnectionInfo(
            id = UUID.randomUUID(),
            serverName = "Test",
            serverAddress = "localhost:8765",
            status = ConnectionStatus.DISCONNECTED,
            deviceId = UUID.randomUUID()
        )
        assertFalse(disconnected.isConnected())
    }
    
    @Test
    fun testConnectionQuality() {
        val excellent = ConnectionInfo(
            id = UUID.randomUUID(),
            serverName = "Test",
            serverAddress = "localhost:8765",
            status = ConnectionStatus.CONNECTED,
            deviceId = UUID.randomUUID(),
            latency = 30
        )
        assertEquals(ConnectionQuality.EXCELLENT, excellent.getConnectionQuality())
        
        val good = ConnectionInfo(
            id = UUID.randomUUID(),
            serverName = "Test",
            serverAddress = "localhost:8765",
            status = ConnectionStatus.CONNECTED,
            deviceId = UUID.randomUUID(),
            latency = 75
        )
        assertEquals(ConnectionQuality.GOOD, good.getConnectionQuality())
    }
    
    // MARK: - Tunnel Config Tests
    
    @Test
    fun testTunnelConfigDefaults() {
        val config = TunnelConfig(
            serverUrl = "wss://localhost:8765",
            deviceId = UUID.randomUUID(),
            deviceName = "Test Device"
        )
        
        assertTrue(config.autoReconnect)
        assertEquals(5000, config.reconnectInterval)
        assertEquals(30000, config.connectionTimeout)
        assertTrue(config.encryptionEnabled)
    }
}