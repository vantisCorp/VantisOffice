package com.vantiscorp.vantismobile.ui

import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.Description
import androidx.compose.material.icons.filled.Notifications
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import com.vantiscorp.vantismobile.ui.screens.*

/**
 * Main application with bottom navigation
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MainApp() {
    var selectedTab by remember { mutableStateOf(0) }
    
    Scaffold(
        bottomBar = {
            NavigationBar {
                NavigationBarItem(
                    icon = { Icon(Icons.Default.Home, contentDescription = null) },
                    label = { Text("Home") },
                    selected = selectedTab == 0,
                    onClick = { selectedTab = 0 }
                )
                NavigationBarItem(
                    icon = { Icon(Icons.Default.Description, contentDescription = null) },
                    label = { Text("Documents") },
                    selected = selectedTab == 1,
                    onClick = { selectedTab = 1 }
                )
                NavigationBarItem(
                    icon = { Icon(Icons.Default.Notifications, contentDescription = null) },
                    label = { Text("Notifications") },
                    selected = selectedTab == 2,
                    onClick = { selectedTab = 2 }
                )
                NavigationBarItem(
                    icon = { Icon(Icons.Default.Settings, contentDescription = null) },
                    label = { Text("Settings") },
                    selected = selectedTab == 3,
                    onClick = { selectedTab = 3 }
                )
            }
        }
    ) { paddingValues ->
        when (selectedTab) {
            0 -> HomeScreen(modifier = Modifier.padding(paddingValues))
            1 -> DocumentsScreen(modifier = Modifier.padding(paddingValues))
            2 -> NotificationsScreen(modifier = Modifier.padding(paddingValues))
            3 -> SettingsScreen(modifier = Modifier.padding(paddingValues))
        }
    }
}