package com.vantiscorp.vantismobile.model

import java.util.UUID

/**
 * Notification type enumeration
 */
enum class NotificationType {
    DOCUMENT_SHARED,
    COMMENT_ADDED,
    MENTION,
    TASK_ASSIGNED,
    SYNC_COMPLETE,
    ERROR,
    SYSTEM;

    val description: String
        get() = when (this) {
            DOCUMENT_SHARED -> "Document Shared"
            COMMENT_ADDED -> "Comment Added"
            MENTION -> "Mention"
            TASK_ASSIGNED -> "Task Assigned"
            SYNC_COMPLETE -> "Sync Complete"
            ERROR -> "Error"
            SYSTEM -> "System"
        }

    val icon: String
        get() = when (this) {
            DOCUMENT_SHARED -> "share"
            COMMENT_ADDED -> "comment"
            MENTION -> "at"
            TASK_ASSIGNED -> "clipboard"
            SYNC_COMPLETE -> "checkmark"
            ERROR -> "error"
            SYSTEM -> "info"
        }
}

/**
 * Notification priority
 */
enum class NotificationPriority {
    LOW,
    NORMAL,
    HIGH,
    URGENT;

    val icon: String
        get() = when (this) {
            LOW -> "info_outline"
            NORMAL -> "notifications"
            HIGH -> "warning"
            URGENT -> "error"
        }

    val color: Long
        get() = when (this) {
            LOW -> 0xFF9E9E9E
            NORMAL -> 0xFF2196F3
            HIGH -> 0xFFFF9800
            URGENT -> 0xFFF44336
        }
}

/**
 * Vantis notification data class
 */
data class VantisNotification(
    val id: UUID,
    val type: NotificationType,
    val title: String,
    val message: String,
    val timestamp: Long = System.currentTimeMillis(),
    val isRead: Boolean = false,
    val priority: NotificationPriority = NotificationPriority.NORMAL,
    val actionUrl: String? = null,
    val metadata: Map<String, String> = emptyMap()
) {
    /**
     * Get relative time string
     */
    fun getRelativeTime(): String {
        val now = System.currentTimeMillis()
        val diff = now - timestamp
        
        val seconds = diff / 1000
        val minutes = seconds / 60
        val hours = minutes / 60
        val days = hours / 24
        
        return when {
            seconds < 60 -> "Just now"
            minutes < 60 -> "$minutes minute${if (minutes > 1) "s" else ""} ago"
            hours < 24 -> "$hours hour${if (hours > 1) "s" else ""} ago"
            days < 7 -> "$days day${if (days > 1) "s" else ""} ago"
            else -> {
                val sdf = java.text.SimpleDateFormat("MMM dd, yyyy", java.util.Locale.getDefault())
                sdf.format(java.util.Date(timestamp))
            }
        }
    }

    /**
     * Group key for notification grouping
     */
    fun getGroupKey(): String {
        val calendar = java.util.Calendar.getInstance()
        calendar.timeInMillis = timestamp
        val today = java.util.Calendar.getInstance()
        
        return when {
            isSameDay(calendar, today) -> "Today"
            isYesterday(calendar) -> "Yesterday"
            isSameWeek(calendar, today) -> "This Week"
            else -> {
                val sdf = java.text.SimpleDateFormat("MMMM yyyy", java.util.Locale.getDefault())
                sdf.format(java.util.Date(timestamp))
            }
        }
    }

    private fun isSameDay(cal1: java.util.Calendar, cal2: java.util.Calendar): Boolean {
        return cal1.get(java.util.Calendar.YEAR) == cal2.get(java.util.Calendar.YEAR) &&
                cal1.get(java.util.Calendar.DAY_OF_YEAR) == cal2.get(java.util.Calendar.DAY_OF_YEAR)
    }

    private fun isYesterday(cal: java.util.Calendar): Boolean {
        val yesterday = java.util.Calendar.getInstance()
        yesterday.add(java.util.Calendar.DAY_OF_YEAR, -1)
        return isSameDay(cal, yesterday)
    }

    private fun isSameWeek(cal1: java.util.Calendar, cal2: java.util.Calendar): Boolean {
        return cal1.get(java.util.Calendar.YEAR) == cal2.get(java.util.Calendar.YEAR) &&
                cal1.get(java.util.Calendar.WEEK_OF_YEAR) == cal2.get(java.util.Calendar.WEEK_OF_YEAR)
    }
}

/**
 * Notification group for UI display
 */
data class NotificationGroup(
    val groupKey: String,
    val notifications: List<VantisNotification>,
    val unreadCount: Int
) {
    val hasUnread: Boolean
        get() = unreadCount > 0
}