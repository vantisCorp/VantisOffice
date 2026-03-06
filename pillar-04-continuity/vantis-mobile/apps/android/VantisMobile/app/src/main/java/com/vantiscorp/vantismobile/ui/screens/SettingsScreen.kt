package com.vantiscorp.vantismobile.ui.screens

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp

/**
 * Settings screen for app configuration
 */
@Composable
fun SettingsScreen(modifier: Modifier = Modifier) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        Text(
            text = "Settings",
            style = MaterialTheme.typography.headlineMedium,
            fontWeight = FontWeight.Bold
        )
        
        // Connection Settings
        SettingsSection(title = "Connection") {
            SettingsItem(
                icon = Icons.Default.CloudSync,
                title = "Sync Settings",
                subtitle = "Configure sync frequency and behavior",
                onClick = { /* TODO */ }
            )
            SettingsItem(
                icon = Icons.Default.Wifi,
                title = "Network Settings",
                subtitle = "Configure network and discovery",
                onClick = { /* TODO */ }
            )
            SettingsItem(
                icon = Icons.Default.Link,
                title = "Connected Devices",
                subtitle = "Manage paired devices",
                onClick = { /* TODO */ }
            )
        }
        
        // Security Settings
        SettingsSection(title = "Security") {
            SettingsToggleItem(
                icon = Icons.Default.Fingerprint,
                title = "Biometric Authentication",
                subtitle = "Use fingerprint or face to unlock",
                checked = true,
                onCheckedChange = { /* TODO */ }
            )
            SettingsItem(
                icon = Icons.Default.Lock,
                title = "Change Passcode",
                subtitle = "Update your security passcode",
                onClick = { /* TODO */ }
            )
            SettingsToggleItem(
                icon = Icons.Default.Visibility,
                title = "Hide Sensitive Content",
                subtitle = "Blur previews in notifications",
                checked = false,
                onCheckedChange = { /* TODO */ }
            )
        }
        
        // Notification Settings
        SettingsSection(title = "Notifications") {
            SettingsToggleItem(
                icon = Icons.Default.Notifications,
                title = "Push Notifications",
                subtitle = "Receive notifications from VantisOffice",
                checked = true,
                onCheckedChange = { /* TODO */ }
            )
            SettingsItem(
                icon = Icons.Default.NotificationsActive,
                title = "Notification Preferences",
                subtitle = "Configure which notifications to receive",
                onClick = { /* TODO */ }
            )
            SettingsToggleItem(
                icon = Icons.Default.VolumeUp,
                title = "Sound",
                subtitle = "Play sound for notifications",
                checked = true,
                onCheckedChange = { /* TODO */ }
            )
        }
        
        // Appearance Settings
        SettingsSection(title = "Appearance") {
            SettingsItem(
                icon = Icons.Default.Palette,
                title = "Theme",
                subtitle = "Dark mode",
                onClick = { /* TODO */ }
            )
            SettingsItem(
                icon = Icons.Default.Translate,
                title = "Language",
                subtitle = "English",
                onClick = { /* TODO */ }
            )
        }
        
        // Data & Storage
        SettingsSection(title = "Data & Storage") {
            SettingsItem(
                icon = Icons.Default.Storage,
                title = "Storage",
                subtitle = "0 MB used",
                onClick = { /* TODO */ }
            )
            SettingsItem(
                icon = Icons.Default.Cached,
                title = "Clear Cache",
                subtitle = "Free up storage space",
                onClick = { /* TODO */ }
            )
            SettingsItem(
                icon = Icons.Default.Delete,
                title = "Clear All Data",
                subtitle = "Reset the app to initial state",
                onClick = { /* TODO */ }
            )
        }
        
        // About
        SettingsSection(title = "About") {
            SettingsItem(
                icon = Icons.Default.Info,
                title = "Version",
                subtitle = "1.0.0",
                showChevron = false
            )
            SettingsItem(
                icon = Icons.Default.PrivacyTip,
                title = "Privacy Policy",
                subtitle = "View our privacy policy",
                onClick = { /* TODO */ }
            )
            SettingsItem(
                icon = Icons.Default.Description,
                title = "Terms of Service",
                subtitle = "View our terms of service",
                onClick = { /* TODO */ }
            )
            SettingsItem(
                icon = Icons.Default.Help,
                title = "Help & Support",
                subtitle = "Get help and report issues",
                onClick = { /* TODO */ }
            )
        }
        
        // Sign Out
        Spacer(modifier = Modifier.height(8.dp))
        OutlinedButton(
            onClick = { /* TODO: Sign out */ },
            modifier = Modifier.fillMaxWidth(),
            colors = ButtonDefaults.outlinedButtonColors(
                contentColor = MaterialTheme.colorScheme.error
            )
        ) {
            Icon(Icons.Default.Logout, contentDescription = null)
            Spacer(modifier = Modifier.width(8.dp))
            Text("Sign Out")
        }
        
        Spacer(modifier = Modifier.height(16.dp))
    }
}

@Composable
fun SettingsSection(
    title: String,
    content: @Composable ColumnScope.() -> Unit
) {
    Column {
        Text(
            text = title.uppercase(),
            style = MaterialTheme.typography.labelMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Spacer(modifier = Modifier.height(8.dp))
        Card(
            modifier = Modifier.fillMaxWidth(),
            colors = CardDefaults.cardColors(
                containerColor = MaterialTheme.colorScheme.surfaceVariant
            )
        ) {
            Column {
                content()
            }
        }
    }
}

@Composable
fun SettingsItem(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    title: String,
    subtitle: String,
    showChevron: Boolean = true,
    onClick: (() -> Unit)? = null
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .then(if (onClick != null) Modifier.clickable { onClick() } else Modifier)
            .padding(16.dp),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            imageVector = icon,
            contentDescription = null,
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Column(modifier = Modifier.weight(1f)) {
            Text(text = title, style = MaterialTheme.typography.bodyLarge)
            Text(
                text = subtitle,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
        if (showChevron) {
            Icon(
                imageVector = Icons.Default.ChevronRight,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}

@Composable
fun SettingsToggleItem(
    icon: androidx.compose.ui.graphics.vector.ImageVector,
    title: String,
    subtitle: String,
    checked: Boolean,
    onCheckedChange: (Boolean) -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable { onCheckedChange(!checked) }
            .padding(16.dp),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Icon(
            imageVector = icon,
            contentDescription = null,
            tint = MaterialTheme.colorScheme.onSurfaceVariant
        )
        Column(modifier = Modifier.weight(1f)) {
            Text(text = title, style = MaterialTheme.typography.bodyLarge)
            Text(
                text = subtitle,
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
        Switch(
            checked = checked,
            onCheckedChange = onCheckedChange
        )
    }
}