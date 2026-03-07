package com.vantis.pqc.example

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.selection.SelectionContainer
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.vantis.pqc.example.service.*
import com.vantis.pqc.example.ui.theme.VantisPQCTheme
import com.vantis.pqc.example.viewmodel.*

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            VantisPQCTheme {
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    VantisPQCApp()
                }
            }
        }
    }
}

@Composable
fun VantisPQCApp() {
    val pqcService = remember { VantisPQCService() }
    
    // Initialize service
    LaunchedEffect(Unit) {
        pqcService.initialize()
    }
    
    var selectedTab by remember { mutableIntStateOf(0) }
    val tabs = listOf("Key Exchange", "Signatures", "Encryption")
    
    Column(modifier = Modifier.fillMaxSize()) {
        // Tab Row
        TabRow(selectedTabIndex = selectedTab) {
            tabs.forEachIndexed { index, title ->
                Tab(
                    selected = selectedTab == index,
                    onClick = { selectedTab = index },
                    text = { Text(title) }
                )
            }
        }
        
        // Content
        when (selectedTab) {
            0 -> KeyExchangeScreen(pqcService)
            1 -> SignatureScreen(pqcService)
            2 -> EncryptionScreen(pqcService)
        }
    }
}

// MARK: - Key Exchange Screen

@Composable
fun KeyExchangeScreen(service: VantisPQCService) {
    val viewModel: KeyExchangeViewModel = viewModel { KeyExchangeViewModel(service) }
    val state by viewModel.state.collectAsState()
    
    Column(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        // Security Level Selector
        Card(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Security Level",
                    style = MaterialTheme.typography.titleMedium
                )
                Spacer(modifier = Modifier.height(8.dp))
                KyberSecurityLevel.values().forEach { level ->
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        RadioButton(
                            selected = state.selectedLevel == level,
                            onClick = { viewModel.selectLevel(level) }
                        )
                        Text(
                            text = "${level.displayName} (${level.nistLevel})",
                            modifier = Modifier.padding(start = 8.dp)
                        )
                    }
                }
            }
        }
        
        // Key Generation
        Card(modifier = Modifier.fillMaxWidth()) {
            Column(
                modifier = Modifier.padding(16.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp)
            ) {
                Text(
                    text = "Key Generation",
                    style = MaterialTheme.typography.titleMedium
                )
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Button(
                        onClick = { viewModel.generateAliceKeyPair() },
                        modifier = Modifier.weight(1f),
                        colors = ButtonDefaults.buttonColors(
                            containerColor = MaterialTheme.colorScheme.primary
                        )
                    ) {
                        Text("Alice's Keys")
                    }
                    Button(
                        onClick = { viewModel.generateBobKeyPair() },
                        modifier = Modifier.weight(1f),
                        colors = ButtonDefaults.buttonColors(
                            containerColor = MaterialTheme.colorScheme.secondary
                        )
                    ) {
                        Text("Bob's Keys")
                    }
                }
            }
        }
        
        // Generated Keys Status
        if (state.aliceKeyPair != null || state.bobKeyPair != null) {
            Card(modifier = Modifier.fillMaxWidth()) {
                Column(modifier = Modifier.padding(16.dp)) {
                    Text(
                        text = "Generated Keys",
                        style = MaterialTheme.typography.titleMedium
                    )
                    Spacer(modifier = Modifier.height(8.dp))
                    state.aliceKeyPair?.let {
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            Icon(
                                imageVector = androidx.compose.material.icons.Icons.Default.CheckCircle,
                                contentDescription = null,
                                tint = MaterialTheme.colorScheme.primary
                            )
                            Text(
                                text = " Alice: ${it.publicKey.size} bytes public key",
                                modifier = Modifier.padding(start = 4.dp)
                            )
                        }
                    }
                    state.bobKeyPair?.let {
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            Icon(
                                imageVector = androidx.compose.material.icons.Icons.Default.CheckCircle,
                                contentDescription = null,
                                tint = MaterialTheme.colorScheme.secondary
                            )
                            Text(
                                text = " Bob: ${it.publicKey.size} bytes public key",
                                modifier = Modifier.padding(start = 4.dp)
                            )
                        }
                    }
                }
            }
        }
        
        // Encapsulation
        if (state.bobKeyPair != null) {
            Card(modifier = Modifier.fillMaxWidth()) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Text(
                        text = "Key Encapsulation",
                        style = MaterialTheme.typography.titleMedium
                    )
                    Button(
                        onClick = { viewModel.encapsulate() },
                        modifier = Modifier.fillMaxWidth(),
                        colors = ButtonDefaults.buttonColors(
                            containerColor = Color(0xFFFF9800)
                        )
                    ) {
                        Text("Encapsulate to Bob")
                    }
                    state.encapsulationResult?.let { enc ->
                        SelectionContainer {
                            Text(
                                text = "Shared Secret: ${enc.sharedSecret.toHexPrefix(16)}...",
                                style = MaterialTheme.typography.bodySmall,
                                fontFamily = FontFamily.Monospace
                            )
                        }
                        Text(
                            text = "Ciphertext: ${enc.ciphertext.size} bytes",
                            style = MaterialTheme.typography.bodySmall
                        )
                    }
                }
            }
        }
        
        // Decapsulation
        if (state.encapsulationResult != null && state.bobKeyPair != null) {
            Card(modifier = Modifier.fillMaxWidth()) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Text(
                        text = "Key Decapsulation",
                        style = MaterialTheme.typography.titleMedium
                    )
                    Button(
                        onClick = { viewModel.decapsulate() },
                        modifier = Modifier.fillMaxWidth(),
                        colors = ButtonDefaults.buttonColors(
                            containerColor = Color(0xFF9C27B0)
                        )
                    ) {
                        Text("Decapsulate as Bob")
                    }
                    state.decapsulatedSecret?.let { secret ->
                        SelectionContainer {
                            Text(
                                text = "Decapsulated: ${secret.toHexPrefix(16)}...",
                                style = MaterialTheme.typography.bodySmall,
                                fontFamily = FontFamily.Monospace
                            )
                        }
                        
                        // Show if secrets match
                        state.encapsulationResult?.let { enc ->
                            Row(verticalAlignment = Alignment.CenterVertically) {
                                if (enc.sharedSecret.contentEquals(secret)) {
                                    Icon(
                                        imageVector = androidx.compose.material.icons.Icons.Default.CheckCircle,
                                        contentDescription = null,
                                        tint = Color(0xFF4CAF50)
                                    )
                                    Text(
                                        text = " Secrets match!",
                                        color = Color(0xFF4CAF50)
                                    )
                                } else {
                                    Icon(
                                        imageVector = androidx.compose.material.icons.Icons.Default.Close,
                                        contentDescription = null,
                                        tint = Color(0xFFF44336)
                                    )
                                    Text(
                                        text = " Secrets differ (placeholder behavior)",
                                        color = Color(0xFFF44336)
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Error Message
        state.error?.let { error ->
            Text(
                text = error,
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall
            )
        }
        
        // Loading Indicator
        if (state.isLoading) {
            Box(
                modifier = Modifier.fillMaxWidth(),
                contentAlignment = Alignment.Center
            ) {
                CircularProgressIndicator()
            }
        }
    }
}

// MARK: - Signature Screen

@Composable
fun SignatureScreen(service: VantisPQCService) {
    val viewModel: SignatureViewModel = viewModel { SignatureViewModel(service) }
    val state by viewModel.state.collectAsState()
    
    Column(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        // Security Level Selector
        Card(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Security Level",
                    style = MaterialTheme.typography.titleMedium
                )
                Spacer(modifier = Modifier.height(8.dp))
                DilithiumSecurityLevel.values().forEach { level ->
                    Row(
                        modifier = Modifier.fillMaxWidth(),
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        RadioButton(
                            selected = state.selectedLevel == level,
                            onClick = { viewModel.selectLevel(level) }
                        )
                        Text(
                            text = "${level.displayName} (${level.nistLevel})",
                            modifier = Modifier.padding(start = 8.dp)
                        )
                    }
                }
            }
        }
        
        // Key Generation
        Card(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Signing Key",
                    style = MaterialTheme.typography.titleMedium
                )
                Spacer(modifier = Modifier.height(8.dp))
                Button(
                    onClick = { viewModel.generateKeyPair() },
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(
                        containerColor = if (state.keyPair != null) 
                            Color(0xFF4CAF50) 
                        else 
                            MaterialTheme.colorScheme.primary
                    )
                ) {
                    Text(if (state.keyPair != null) "Key Generated" else "Generate Dilithium Key Pair")
                }
                state.keyPair?.let { key ->
                    Text(
                        text = "Public Key: ${key.publicKey.size} bytes\nPrivate Key: ${key.privateKey.size} bytes",
                        style = MaterialTheme.typography.bodySmall
                    )
                }
            }
        }
        
        // Message Input
        Card(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Message to Sign",
                    style = MaterialTheme.typography.titleMedium
                )
                Spacer(modifier = Modifier.height(8.dp))
                OutlinedTextField(
                    value = state.message,
                    onValueChange = { viewModel.updateMessage(it) },
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(100.dp),
                    placeholder = { Text("Enter message to sign") }
                )
            }
        }
        
        // Sign & Verify
        if (state.keyPair != null) {
            Card(modifier = Modifier.fillMaxWidth()) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Text(
                        text = "Sign & Verify",
                        style = MaterialTheme.typography.titleMedium
                    )
                    Button(
                        onClick = { viewModel.sign() },
                        modifier = Modifier.fillMaxWidth(),
                        colors = ButtonDefaults.buttonColors(
                            containerColor = Color(0xFFFF9800)
                        )
                    ) {
                        Text("Sign Message")
                    }
                    state.signature?.let { sig ->
                        Text(
                            text = "Signature: ${sig.size} bytes",
                            style = MaterialTheme.typography.bodySmall
                        )
                        SelectionContainer {
                            Text(
                                text = "${sig.toHexPrefix(32)}...",
                                style = MaterialTheme.typography.bodySmall,
                                fontFamily = FontFamily.Monospace
                            )
                        }
                    }
                    
                    if (state.signature != null) {
                        Button(
                            onClick = { viewModel.verify() },
                            modifier = Modifier.fillMaxWidth(),
                            colors = ButtonDefaults.buttonColors(
                                containerColor = Color(0xFF9C27B0)
                            )
                        ) {
                            Text("Verify Signature")
                        }
                        
                        state.isVerified?.let { verified ->
                            Row(verticalAlignment = Alignment.CenterVertically) {
                                Icon(
                                    imageVector = if (verified) 
                                        androidx.compose.material.icons.Icons.Default.CheckCircle 
                                    else 
                                        androidx.compose.material.icons.Icons.Default.Close,
                                    contentDescription = null,
                                    tint = if (verified) Color(0xFF4CAF50) else Color(0xFFF44336)
                                )
                                Text(
                                    text = if (verified) " Signature Valid" else " Signature Invalid",
                                    color = if (verified) Color(0xFF4CAF50) else Color(0xFFF44336)
                                )
                            }
                        }
                    }
                }
            }
        }
        
        // Error Message
        state.error?.let { error ->
            Text(
                text = error,
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall
            )
        }
        
        // Loading Indicator
        if (state.isLoading) {
            Box(
                modifier = Modifier.fillMaxWidth(),
                contentAlignment = Alignment.Center
            ) {
                CircularProgressIndicator()
            }
        }
    }
}

// MARK: - Encryption Screen

@Composable
fun EncryptionScreen(service: VantisPQCService) {
    val viewModel: EncryptionViewModel = viewModel { EncryptionViewModel(service) }
    val state by viewModel.state.collectAsState()
    
    Column(
        modifier = Modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp)
    ) {
        // Key Generation
        Card(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Encryption Key",
                    style = MaterialTheme.typography.titleMedium
                )
                Spacer(modifier = Modifier.height(8.dp))
                Button(
                    onClick = { viewModel.generateKey() },
                    modifier = Modifier.fillMaxWidth(),
                    colors = ButtonDefaults.buttonColors(
                        containerColor = if (state.encryptionKey != null) 
                            Color(0xFF4CAF50) 
                        else 
                            MaterialTheme.colorScheme.primary
                    )
                ) {
                    Text(if (state.encryptionKey != null) "Key Generated" else "Generate Encryption Key")
                }
                state.encryptionKey?.let { key ->
                    SelectionContainer {
                        Text(
                            text = "Key: ${key.toHexPrefix(16)}...",
                            style = MaterialTheme.typography.bodySmall,
                            fontFamily = FontFamily.Monospace
                        )
                    }
                }
            }
        }
        
        // Plaintext Input
        Card(modifier = Modifier.fillMaxWidth()) {
            Column(modifier = Modifier.padding(16.dp)) {
                Text(
                    text = "Plaintext",
                    style = MaterialTheme.typography.titleMedium
                )
                Spacer(modifier = Modifier.height(8.dp))
                OutlinedTextField(
                    value = state.plaintext,
                    onValueChange = { viewModel.updatePlaintext(it) },
                    modifier = Modifier
                        .fillMaxWidth()
                        .height(100.dp),
                    placeholder = { Text("Enter text to encrypt") }
                )
            }
        }
        
        // Encryption
        if (state.encryptionKey != null) {
            Card(modifier = Modifier.fillMaxWidth()) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(8.dp)
                ) {
                    Text(
                        text = "Encryption",
                        style = MaterialTheme.typography.titleMedium
                    )
                    Button(
                        onClick = { viewModel.encrypt() },
                        modifier = Modifier.fillMaxWidth(),
                        colors = ButtonDefaults.buttonColors(
                            containerColor = Color(0xFFFF9800)
                        )
                    ) {
                        Text("Encrypt")
                    }
                    state.encryptedData?.let { data ->
                        Text(
                            text = "Encrypted: ${data.size} bytes (includes nonce + auth tag)",
                            style = MaterialTheme.typography.bodySmall
                        )
                    }
                }
            }
            
            // Decryption
            if (state.encryptedData != null) {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(8.dp)
                    ) {
                        Text(
                            text = "Decryption",
                            style = MaterialTheme.typography.titleMedium
                        )
                        Button(
                            onClick = { viewModel.decrypt() },
                            modifier = Modifier.fillMaxWidth(),
                            colors = ButtonDefaults.buttonColors(
                                containerColor = Color(0xFF9C27B0)
                            )
                        ) {
                            Text("Decrypt")
                        }
                        state.decryptedText?.let { text ->
                            Text(
                                text = "Decrypted:",
                                style = MaterialTheme.typography.bodySmall
                            )
                            SelectionContainer {
                                Text(
                                    text = text,
                                    style = MaterialTheme.typography.bodyMedium
                                )
                            }
                            
                            Row(verticalAlignment = Alignment.CenterVertically) {
                                Icon(
                                    imageVector = if (text == state.plaintext) 
                                        androidx.compose.material.icons.Icons.Default.CheckCircle 
                                    else 
                                        androidx.compose.material.icons.Icons.Default.Close,
                                    contentDescription = null,
                                    tint = if (text == state.plaintext) Color(0xFF4CAF50) else Color(0xFFF44336)
                                )
                                Text(
                                    text = if (text == state.plaintext) " Matches original!" else " Does not match",
                                    color = if (text == state.plaintext) Color(0xFF4CAF50) else Color(0xFFF44336)
                                )
                            }
                        }
                    }
                }
            }
        }
        
        // Error Message
        state.error?.let { error ->
            Text(
                text = error,
                color = MaterialTheme.colorScheme.error,
                style = MaterialTheme.typography.bodySmall
            )
        }
        
        // Loading Indicator
        if (state.isLoading) {
            Box(
                modifier = Modifier.fillMaxWidth(),
                contentAlignment = Alignment.Center
            ) {
                CircularProgressIndicator()
            }
        }
    }
}