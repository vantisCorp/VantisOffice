package com.vantiscorp.vantismobile.ui.screens

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.vantiscorp.vantismobile.model.VantisNotification
import com.vantiscorp.vantismobile.model.NotificationPriority
import com.vantiscorp.vantismobile.model.NotificationType

/**
 * Notifications screen displaying all notifications with grouping
 */
@Composable
fun NotificationsScreen(modifier: Modifier = Modifier) {
    var showUnreadOnly by remember { mutableStateOf(false) }
    
    // TODO: Replace with actual data from ViewModel
    var notifications by remember { mutableStateOf<List<VantisNotification>>(emptyList()) }
    
    val filteredNotifications = if (showUnreadOnly) {
        notifications.filter { !it.isRead }
    } else {
        notifications
    }
    
    val unreadCount = notifications.count { !it.isRead }
    
    Column(
        modifier = modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        // Header
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = "Notifications",
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Bold
            )
            
            if (unreadCount > 0) {
                Badge {
                    Text("$unreadCount")
                }
            }
        }
        
        // Filter options
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            FilterChip(
                selected = showUnreadOnly,
                onClick = { showUnreadOnly = !showUnreadOnly },
                label = { Text("Unread only") },
                leadingIcon = if (showUnreadOnly) {
                    { Icon(Icons.Default.Check, contentDescription = null, modifier = Modifier.size(16.dp)) }
                } else null
            )
            
            if (unreadCount > 0) {
                TextButton(onClick = { /* TODO: Mark all as read */ }) {
                    Text("Mark all as read")
                }
            }
        }
        
        Spacer(modifier = Modifier.height(8.dp))
        
        // Notifications list
        if (filteredNotifications.isEmpty()) {
            EmptyNotificationsState()
        } else {
            LazyColumn(
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(filteredNotifications) { notification ->
                    NotificationCard(notification = notification)
                }
            }
        }
    }
}

@Composable
fun NotificationCard(notification: VantisNotification) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = if (notification.isRead) {
                MaterialTheme.colorScheme.surface
            } else {
                MaterialTheme.colorScheme.secondaryContainer
            }
        ),
        onClick = { /* TODO: Handle notification click */ }
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalArrangement = Arrangement.spacedBy(12.dp),
            verticalAlignment = Alignment.Start
        ) {
            // Notification icon
            Surface(
                modifier = Modifier.size(40.dp),
                color = when (notification.priority) {
                    NotificationPriority.URGENT -> MaterialTheme.colorScheme.error
                    NotificationPriority.HIGH -> MaterialTheme.colorScheme.tertiary
                    NotificationPriority.NORMAL -> MaterialTheme.colorScheme.primary
                    NotificationPriority.LOW -> MaterialTheme.colorScheme.secondary
                },
                shape = MaterialTheme.shapes.medium
            ) {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = when (notification.type) {
                            NotificationType.DOCUMENT_SHARED -> Icons.Default.Share
                            NotificationType.COMMENT_ADDED -> Icons.Default.Comment
                            NotificationType.MENTION -> Icons.Default.At
                            NotificationType.TASK_ASSIGNED -> Icons.Default.Clipboard
                            NotificationType.SYNC_COMPLETE -> Icons.Default.CheckCircle
                            NotificationType.ERROR -> Icons.Default.Error
                            NotificationType.SYSTEM -> Icons.Default.Info
                        },
                        contentDescription = null,
                        tint = MaterialTheme.colorScheme.onPrimaryContainer
                    )
                }
            }
            
            // Notification content
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = notification.title,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = if (notification.isRead) FontWeight.Normal else FontWeight.Bold
                )
                Text(
                    text = notification.message,
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                Spacer(modifier = Modifier.height(4.dp))
                Text(
                    text = notification.getRelativeTime(),
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
            
            // Read indicator
            if (!notification.isRead) {
                Badge(
                    modifier = Modifier.align(Alignment.Top)
                ) {
                    Box(
                        modifier = Modifier.size(8.dp),
                        contentAlignment = Alignment.Center
                    )
                }
            }
        }
    }
}

@Composable
fun EmptyNotificationsState() {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surfaceVariant
        )
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(32.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            Icon(
                imageVector = Icons.Default.NotificationsNone,
                contentDescription = null,
                modifier = Modifier.size(64.dp),
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
            Spacer(modifier = Modifier.height(16.dp))
            Text(
                text = "No Notifications",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold
            )
            Text(
                text = "You're all caught up!",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}