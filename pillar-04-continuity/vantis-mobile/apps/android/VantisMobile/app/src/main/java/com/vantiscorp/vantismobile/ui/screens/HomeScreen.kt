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
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.vantiscorp.vantismobile.model.ConnectionStatus
import com.vantiscorp.vantismobile.model.DocumentMetadata

/**
 * Home screen displaying connection status and quick actions
 */
@Composable
fun HomeScreen(modifier: Modifier = Modifier) {
    var connectionStatus by remember { mutableStateOf(ConnectionStatus.DISCONNECTED) }
    var recentDocuments by remember { mutableStateOf<List<DocumentMetadata>>(emptyList()) }
    
    Column(
        modifier = modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        // Header
        Text(
            text = "VantisMobile",
            style = MaterialTheme.typography.headlineMedium,
            fontWeight = FontWeight.Bold
        )
        
        // Connection Status Card
        ConnectionStatusCard(connectionStatus)
        
        // Quick Actions
        QuickActionsGrid()
        
        // Recent Documents
        RecentDocumentsSection(recentDocuments)
    }
}

@Composable
fun ConnectionStatusCard(status: ConnectionStatus) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = when (status) {
                ConnectionStatus.CONNECTED -> Color(0xFF4CAF50)
                ConnectionStatus.CONNECTING -> Color(0xFFFFC107)
                ConnectionStatus.ERROR -> Color(0xFFF44336)
                else -> MaterialTheme.colorScheme.surfaceVariant
            }
        )
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Column {
                Text(
                    text = "Connection Status",
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.onPrimaryContainer
                )
                Text(
                    text = status.description,
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onPrimaryContainer
                )
            }
            Icon(
                imageVector = when (status) {
                    ConnectionStatus.CONNECTED -> Icons.Default.CheckCircle
                    ConnectionStatus.CONNECTING -> Icons.Default.Sync
                    ConnectionStatus.ERROR -> Icons.Default.Error
                    else -> Icons.Default.CloudOff
                },
                contentDescription = null,
                tint = MaterialTheme.colorScheme.onPrimaryContainer,
                modifier = Modifier.size(32.dp)
            )
        }
    }
}

@Composable
fun QuickActionsGrid() {
    Column {
        Text(
            text = "Quick Actions",
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.Bold
        )
        Spacer(modifier = Modifier.height(8.dp))
        
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            QuickActionCard(
                icon = Icons.Default.Refresh,
                title = "Sync",
                description = "Sync documents",
                onClick = { /* TODO: Implement sync */ },
                modifier = Modifier.weight(1f)
            )
            QuickActionCard(
                icon = Icons.Default.QrCodeScanner,
                title = "Connect",
                description = "Connect to desktop",
                onClick = { /* TODO: Implement connect */ },
                modifier = Modifier.weight(1f)
            )
        }
        
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            QuickActionCard(
                icon = Icons.Default.Share,
                title = "Share",
                description = "Share document",
                onClick = { /* TODO: Implement share */ },
                modifier = Modifier.weight(1f)
            )
            QuickActionCard(
                icon = Icons.Default.Settings,
                title = "Settings",
                description = "App settings",
                onClick = { /* TODO: Navigate to settings */ },
                modifier = Modifier.weight(1f)
            )
        }
    }
}

@Composable
fun QuickActionCard(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    title: String,
    description: String,
    onClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    Card(
        onClick = onClick,
        modifier = modifier.aspectRatio(1f),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surfaceVariant
        )
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(12.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center
        ) {
            Icon(
                imageVector = icon,
                contentDescription = null,
                modifier = Modifier.size(32.dp),
                tint = MaterialTheme.colorScheme.primary
            )
            Spacer(modifier = Modifier.height(8.dp))
            Text(
                text = title,
                style = MaterialTheme.typography.titleSmall,
                fontWeight = FontWeight.Bold
            )
            Text(
                text = description,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}

@Composable
fun RecentDocumentsSection(documents: List<DocumentMetadata>) {
    Column {
        Text(
            text = "Recent Documents",
            style = MaterialTheme.typography.titleMedium,
            fontWeight = FontWeight.Bold
        )
        Spacer(modifier = Modifier.height(8.dp))
        
        if (documents.isEmpty()) {
            Card(
                modifier = Modifier.fillMaxWidth(),
                colors = CardDefaults.cardColors(
                    containerColor = MaterialTheme.colorScheme.surfaceVariant
                )
            ) {
                Box(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(32.dp),
                    contentAlignment = Alignment.Center
                ) {
                    Text(
                        text = "No recent documents",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }
        } else {
            LazyColumn(
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(documents.take(5)) { document ->
                    DocumentListItem(document = document)
                }
            }
        }
    }
}

@Composable
fun DocumentListItem(document: DocumentMetadata) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surface
        )
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Row(
                horizontalArrangement = Arrangement.spacedBy(12.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                Icon(
                    imageVector = Icons.Default.Description,
                    contentDescription = null,
                    tint = MaterialTheme.colorScheme.primary
                )
                Column {
                    Text(
                        text = document.name,
                        style = MaterialTheme.typography.bodyLarge,
                        fontWeight = FontWeight.Medium
                    )
                    Text(
                        text = document.formattedSize(),
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                }
            }
            Icon(
                imageVector = Icons.Default.ChevronRight,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}