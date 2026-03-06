//
//  Document.swift
//  VantisMobile
//
//  Document models for mobile document access
//

import Foundation

/// Document type enumeration
enum DocumentType: String, Codable {
    case writer
    case flow
    case canvas
    case grid
    case file
}

/// Document metadata model
struct DocumentMetadata: Codable, Identifiable, Hashable {
    let id: UUID
    let documentType: DocumentType
    let title: String
    let ownerId: UUID
    let createdAt: Date
    let modifiedAt: Date
    let size: Int64
    var cached: Bool
    var lastSync: Date?
    
    init(documentType: DocumentType, title: String, ownerId: UUID) {
        self.id = UUID()
        self.documentType = documentType
        self.title = title
        self.ownerId = ownerId
        self.createdAt = Date()
        self.modifiedAt = Date()
        self.size = 0
        self.cached = false
        self.lastSync = nil
    }
    
    /// Format file size for display
    var formattedSize: String {
        let formatter = ByteCountFormatter()
        formatter.allowedUnits = [.useBytes, .useKB, .useMB, .useGB]
        formatter.countStyle = .file
        return formatter.string(fromByteCount: size)
    }
    
    /// Document icon name based on type
    var iconName: String {
        switch documentType {
        case .writer:
            return "doc.text"
        case .flow:
            return "flowchart"
        case .canvas:
            return "paintbrush"
        case .grid:
            return "tablecells"
        case .file:
            return "doc"
        }
    }
    
    /// Document type color
    var typeColor: String {
        switch documentType {
        case .writer:
            return "blue"
        case .flow:
            return "purple"
        case .canvas:
            return "orange"
        case .grid:
            return "green"
        case .file:
            return "gray"
        }
    }
}

/// Document content wrapper
struct DocumentContent: Codable {
    let documentId: UUID
    let content: Data
    let checksum: String
    let timestamp: Date
}