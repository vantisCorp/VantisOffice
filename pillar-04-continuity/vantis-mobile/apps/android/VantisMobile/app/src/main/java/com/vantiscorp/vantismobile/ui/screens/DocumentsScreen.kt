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
import com.vantiscorp.vantismobile.model.DocumentMetadata
import com.vantiscorp.vantismobile.model.DocumentType
import com.vantiscorp.vantismobile.model.DocumentFilter
import com.vantiscorp.vantismobile.model.DocumentSortOrder

/**
 * Documents screen displaying document list with search and filters
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DocumentsScreen(modifier: Modifier = Modifier) {
    var searchQuery by remember { mutableStateOf("") }
    var selectedType by remember { mutableStateOf<DocumentType?>(null) }
    var sortOrder by remember { mutableStateOf(DocumentSortOrder.DATE_DESC) }
    var showFilterSheet by remember { mutableStateOf(false) }
    
    // TODO: Replace with actual data from ViewModel
    var documents by remember { mutableStateOf<List<DocumentMetadata>>(emptyList()) }
    
    Column(
        modifier = modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp)
    ) {
        // Header with search
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically
        ) {
            Text(
                text = "Documents",
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Bold
            )
            
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                IconButton(onClick = { /* TODO: Toggle view */ }) {
                    Icon(Icons.Default.GridView, contentDescription = "Grid view")
                }
                IconButton(onClick = { showFilterSheet = true }) {
                    Icon(Icons.Default.FilterList, contentDescription = "Filter")
                }
            }
        }
        
        // Search bar
        OutlinedTextField(
            value = searchQuery,
            onValueChange = { searchQuery = it },
            modifier = Modifier.fillMaxWidth(),
            placeholder = { Text("Search documents...") },
            leadingIcon = { Icon(Icons.Default.Search, contentDescription = null) },
            trailingIcon = {
                if (searchQuery.isNotEmpty()) {
                    IconButton(onClick = { searchQuery = "" }) {
                        Icon(Icons.Default.Clear, contentDescription = "Clear")
                    }
                }
            },
            singleLine = true
        )
        
        // Document type filter chips
        DocumentTypeFilterChips(
            selectedType = selectedType,
            onTypeSelected = { selectedType = it }
        )
        
        // Documents list
        if (documents.isEmpty()) {
            EmptyDocumentsState()
        } else {
            LazyColumn(
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                items(documents) { document ->
                    DocumentCard(document = document)
                }
            }
        }
    }
    
    // Filter bottom sheet
    if (showFilterSheet) {
        FilterBottomSheet(
            onDismiss = { showFilterSheet = false },
            onApply = { /* TODO: Apply filters */ }
        )
    }
}

@Composable
fun DocumentTypeFilterChips(
    selectedType: DocumentType?,
    onTypeSelected: (DocumentType?) -> Unit
) {
    val types = listOf(
        null to "All",
        DocumentType.PDF to "PDF",
        DocumentType.WORD to "Word",
        DocumentType.SPREADSHEET to "Spreadsheet",
        DocumentType.IMAGE to "Images"
    )
    
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        types.forEach { (type, label) ->
            FilterChip(
                selected = selectedType == type,
                onClick = { onTypeSelected(type) },
                label = { Text(label) },
                leadingIcon = if (selectedType == type) {
                    { Icon(Icons.Default.Check, contentDescription = null, modifier = Modifier.size(16.dp)) }
                } else null
            )
        }
    }
}

@Composable
fun DocumentCard(document: DocumentMetadata) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        onClick = { /* TODO: Open document */ }
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(16.dp),
            horizontalArrangement = Arrangement.spacedBy(12.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            // Document icon
            Surface(
                modifier = Modifier.size(48.dp),
                color = MaterialTheme.colorScheme.primaryContainer,
                shape = MaterialTheme.shapes.medium
            ) {
                Box(
                    modifier = Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = when (document.type) {
                            DocumentType.PDF -> Icons.Default.PictureAsPdf
                            DocumentType.WORD -> Icons.Default.Description
                            DocumentType.SPREADSHEET -> Icons.Default.TableChart
                            DocumentType.PRESENTATION -> Icons.Default.Slideshow
                            DocumentType.IMAGE -> Icons.Default.Image
                            else -> Icons.Default.InsertDriveFile
                        },
                        contentDescription = null,
                        tint = MaterialTheme.colorScheme.onPrimaryContainer
                    )
                }
            }
            
            // Document info
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = document.name,
                    style = MaterialTheme.typography.bodyLarge,
                    fontWeight = FontWeight.Medium
                )
                Row(
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Text(
                        text = document.formattedSize(),
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant
                    )
                    if (document.author != null) {
                        Text(
                            text = "•",
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                        Text(
                            text = document.author,
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                }
            }
            
            // Actions
            IconButton(onClick = { /* TODO: More options */ }) {
                Icon(Icons.Default.MoreVert, contentDescription = "More options")
            }
        }
    }
}

@Composable
fun EmptyDocumentsState() {
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
                imageVector = Icons.Default.FolderOff,
                contentDescription = null,
                modifier = Modifier.size(64.dp),
                tint = MaterialTheme.colorScheme.onSurfaceVariant
            )
            Spacer(modifier = Modifier.height(16.dp))
            Text(
                text = "No Documents Found",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.Bold
            )
            Text(
                text = "Connect to VantisOffice to sync your documents",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
            Spacer(modifier = Modifier.height(16.dp))
            Button(onClick = { /* TODO: Navigate to connect */ }) {
                Icon(Icons.Default.Add, contentDescription = null)
                Spacer(modifier = Modifier.width(8.dp))
                Text("Connect to Desktop")
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun FilterBottomSheet(
    onDismiss: () -> Unit,
    onApply: () -> Unit
) {
    // TODO: Implement filter options
    ModalBottomSheet(onDismissRequest = onDismiss) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            Text(
                text = "Filter Documents",
                style = MaterialTheme.typography.titleLarge,
                fontWeight = FontWeight.Bold
            )
            
            // Sort order
            Text("Sort by:", style = MaterialTheme.typography.labelLarge)
            // TODO: Add sort options
            
            Spacer(modifier = Modifier.height(16.dp))
            
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                OutlinedButton(onClick = onDismiss, modifier = Modifier.weight(1f)) {
                    Text("Reset")
                }
                Button(onClick = onApply, modifier = Modifier.weight(1f)) {
                    Text("Apply")
                }
            }
        }
    }
}