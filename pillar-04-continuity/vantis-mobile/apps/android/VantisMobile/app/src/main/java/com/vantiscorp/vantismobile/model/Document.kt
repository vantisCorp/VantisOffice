package com.vantiscorp.vantismobile.model

import java.util.UUID
import java.text.DecimalFormat

/**
 * Document type enumeration
 */
enum class DocumentType {
    PDF,
    WORD,
    SPREADSHEET,
    PRESENTATION,
    IMAGE,
    TEXT,
    ARCHIVE,
    AUDIO,
    VIDEO,
    OTHER;

    companion object {
        fun fromExtension(extension: String): DocumentType = when (extension.lowercase()) {
            "pdf" -> PDF
            "doc", "docx", "odt" -> WORD
            "xls", "xlsx", "ods", "csv" -> SPREADSHEET
            "ppt", "pptx", "odp" -> PRESENTATION
            "jpg", "jpeg", "png", "gif", "bmp", "webp" -> IMAGE
            "txt", "md", "rtf" -> TEXT
            "zip", "rar", "7z", "tar", "gz" -> ARCHIVE
            "mp3", "wav", "ogg", "flac" -> AUDIO
            "mp4", "avi", "mkv", "mov", "webm" -> VIDEO
            else -> OTHER
        }
    }
}

/**
 * Document metadata
 */
data class DocumentMetadata(
    val id: UUID,
    val name: String,
    val type: DocumentType,
    val size: Long,
    val path: String,
    val createdAt: Long = System.currentTimeMillis(),
    val modifiedAt: Long = System.currentTimeMillis(),
    val author: String? = null,
    val tags: List<String> = emptyList(),
    val thumbnailUrl: String? = null,
    val isFavorite: Boolean = false,
    val isShared: Boolean = false,
    val sharedWith: List<String> = emptyList(),
    val version: Int = 1
) {
    /**
     * Get file extension from name
     */
    val extension: String
        get() = name.substringAfterLast('.', "")

    /**
     * Format file size for human-readable display
     */
    fun formattedSize(): String {
        if (size < 1024) return "$size B"
        val kb = size / 1024.0
        if (kb < 1024) return "${DecimalFormat("#.#").format(kb)} KB"
        val mb = kb / 1024.0
        if (mb < 1024) return "${DecimalFormat("#.#").format(mb)} MB"
        val gb = mb / 1024.0
        return "${DecimalFormat("#.#").format(gb)} GB"
    }

    /**
     * Get icon for document type
     */
    fun getIcon(): String = when (type) {
        DocumentType.PDF -> "pdf"
        DocumentType.WORD -> "document"
        DocumentType.SPREADSHEET -> "table"
        DocumentType.PRESENTATION -> "slideshow"
        DocumentType.IMAGE -> "image"
        DocumentType.TEXT -> "text"
        DocumentType.ARCHIVE -> "archive"
        DocumentType.AUDIO -> "music"
        DocumentType.VIDEO -> "video"
        DocumentType.OTHER -> "file"
    }

    /**
     * Check if document is recent (modified in last 7 days)
     */
    fun isRecent(): Boolean {
        val sevenDaysAgo = System.currentTimeMillis() - (7 * 24 * 60 * 60 * 1000)
        return modifiedAt > sevenDaysAgo
    }

    /**
     * Check if document is large (over 10 MB)
     */
    fun isLarge(): Boolean = size > 10 * 1024 * 1024
}

/**
 * Document sort order
 */
enum class DocumentSortOrder {
    NAME_ASC,
    NAME_DESC,
    DATE_ASC,
    DATE_DESC,
    SIZE_ASC,
    SIZE_DESC,
    TYPE_ASC
}

/**
 * Document filter options
 */
data class DocumentFilter(
    val type: DocumentType? = null,
    val tags: List<String> = emptyList(),
    val author: String? = null,
    val favoritesOnly: Boolean = false,
    val sharedOnly: Boolean = false,
    val startDate: Long? = null,
    val endDate: Long? = null
) {
    /**
     * Check if filter is active
     */
    fun isActive(): Boolean = type != null || tags.isNotEmpty() || 
            author != null || favoritesOnly || sharedOnly || 
            startDate != null || endDate != null

    /**
     * Apply filter to document
     */
    fun matches(document: DocumentMetadata): Boolean {
        if (type != null && document.type != type) return false
        if (tags.isNotEmpty() && tags.intersect(document.tags.toSet()).isEmpty()) return false
        if (author != null && document.author != author) return false
        if (favoritesOnly && !document.isFavorite) return false
        if (sharedOnly && !document.isShared) return false
        if (startDate != null && document.createdAt < startDate) return false
        if (endDate != null && document.createdAt > endDate) return false
        return true
    }
}