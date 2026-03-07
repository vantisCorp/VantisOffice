import SwiftUI

struct ContentView: View {
    @StateObject private var pqcService = VantisPQCService()
    @State private var selectedTab = 0
    
    var body: some View {
        TabView(selection: $selectedTab) {
            KeyExchangeView()
                .tabItem {
                    Label("Key Exchange", systemImage: "key.fill")
                }
                .tag(0)
            
            SignatureView()
                .tabItem {
                    Label("Signatures", systemImage: "signature")
                }
                .tag(1)
            
            EncryptionView()
                .tabItem {
                    Label("Encryption", systemImage: "lock.shield")
                }
                .tag(2)
        }
        .environmentObject(pqcService)
        .navigationTitle("VantisPQC Demo")
    }
}

// MARK: - Key Exchange View

struct KeyExchangeView: View {
    @EnvironmentObject var pqcService: VantisPQCService
    @State private var aliceKeyPair: KyberKeyPairResult?
    @State private var bobKeyPair: KyberKeyPairResult?
    @State private var encapsulationResult: EncapsulationResult?
    @State private var decapsulatedSecret: Data?
    @State private var selectedLevel: KyberSecurityLevel = .kyber768
    @State private var isLoading = false
    @State private var errorMessage: String?
    
    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Security Level Picker
                VStack(alignment: .leading) {
                    Text("Security Level")
                        .font(.headline)
                    Picker("Security Level", selection: $selectedLevel) {
                        ForEach(KyberSecurityLevel.allCases, id: \.self) { level in
                            Text("\(level.displayName) (\(level.nistLevel))")
                                .tag(level)
                        }
                    }
                    .pickerStyle(.segmented)
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(10)
                
                // Key Generation Section
                GroupBox("Key Generation") {
                    VStack(spacing: 15) {
                        Button(action: generateAliceKeyPair) {
                            HStack {
                                Image(systemName: "person.fill")
                                Text("Generate Alice's Key Pair")
                            }
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.blue)
                            .foregroundColor(.white)
                            .cornerRadius(10)
                        }
                        
                        Button(action: generateBobKeyPair) {
                            HStack {
                                Image(systemName: "person.fill")
                                Text("Generate Bob's Key Pair")
                            }
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color.green)
                            .foregroundColor(.white)
                            .cornerRadius(10)
                        }
                    }
                }
                
                // Key Status
                if aliceKeyPair != nil || bobKeyPair != nil {
                    GroupBox("Generated Keys") {
                        VStack(alignment: .leading, spacing: 10) {
                            if let alice = aliceKeyPair {
                                HStack {
                                    Image(systemName: "checkmark.circle.fill")
                                        .foregroundColor(.blue)
                                    Text("Alice: \(alice.publicKey.count) bytes public key")
                                }
                            }
                            if let bob = bobKeyPair {
                                HStack {
                                    Image(systemName: "checkmark.circle.fill")
                                        .foregroundColor(.green)
                                    Text("Bob: \(bob.publicKey.count) bytes public key")
                                }
                            }
                        }
                    }
                }
                
                // Encapsulation Section
                if bobKeyPair != nil {
                    GroupBox("Key Encapsulation") {
                        VStack(spacing: 15) {
                            Button(action: performEncapsulation) {
                                HStack {
                                    Image(systemName: "lock.fill")
                                    Text("Encapsulate to Bob")
                                }
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.orange)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                            }
                            
                            if let enc = encapsulationResult {
                                VStack(alignment: .leading) {
                                    Text("Shared Secret: \(enc.sharedSecret.hexString)")
                                        .font(.system(.caption, design: .monospaced))
                                    Text("Ciphertext: \(enc.ciphertext.count) bytes")
                                        .font(.caption)
                                }
                            }
                        }
                    }
                }
                
                // Decapsulation Section
                if encapsulationResult != nil && bobKeyPair != nil {
                    GroupBox("Key Decapsulation") {
                        VStack(spacing: 15) {
                            Button(action: performDecapsulation) {
                                HStack {
                                    Image(systemName: "lock.open.fill")
                                    Text("Decapsulate as Bob")
                                }
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.purple)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                            }
                            
                            if let secret = decapsulatedSecret {
                                VStack(alignment: .leading) {
                                    Text("Decapsulated Secret: \(secret.hexString)")
                                        .font(.system(.caption, design: .monospaced))
                                    
                                    if let enc = encapsulationResult {
                                        HStack {
                                            if enc.sharedSecret == secret {
                                                Image(systemName: "checkmark.circle.fill")
                                                    .foregroundColor(.green)
                                                Text("Secrets match!")
                                            } else {
                                                Image(systemName: "xmark.circle.fill")
                                                    .foregroundColor(.red)
                                                Text("Secrets differ (placeholder behavior)")
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Error Message
                if let error = errorMessage {
                    Text(error)
                        .foregroundColor(.red)
                        .font(.caption)
                }
            }
            .padding()
        }
        .overlay {
            if isLoading {
                ProgressView("Processing...")
                    .padding()
                    .background(.regularMaterial)
                    .cornerRadius(10)
            }
        }
    }
    
    private func generateAliceKeyPair() {
        Task {
            isLoading = true
            errorMessage = nil
            defer { isLoading = false }
            
            do {
                aliceKeyPair = try await pqcService.generateKyberKeyPair(securityLevel: selectedLevel)
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }
    
    private func generateBobKeyPair() {
        Task {
            isLoading = true
            errorMessage = nil
            defer { isLoading = false }
            
            do {
                bobKeyPair = try await pqcService.generateKyberKeyPair(securityLevel: selectedLevel)
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }
    
    private func performEncapsulation() {
        guard let bob = bobKeyPair else { return }
        
        Task {
            isLoading = true
            errorMessage = nil
            defer { isLoading = false }
            
            do {
                encapsulationResult = try await pqcService.encapsulate(
                    publicKey: bob.publicKey,
                    securityLevel: selectedLevel
                )
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }
    
    private func performDecapsulation() {
        guard let bob = bobKeyPair, let enc = encapsulationResult else { return }
        
        Task {
            isLoading = true
            errorMessage = nil
            defer { isLoading = false }
            
            do {
                decapsulatedSecret = try await pqcService.decapsulate(
                    privateKey: bob.privateKey,
                    ciphertext: enc.ciphertext,
                    securityLevel: selectedLevel
                )
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }
}

// MARK: - Signature View

struct SignatureView: View {
    @EnvironmentObject var pqcService: VantisPQCService
    @State private var keyPair: DilithiumKeyPairResult?
    @State private var messageText = "Hello, Post-Quantum World!"
    @State private var signature: Data?
    @State private var isVerified: Bool?
    @State private var selectedLevel: DilithiumSecurityLevel = .dilithium3
    @State private var isLoading = false
    @State private var errorMessage: String?
    
    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Security Level Picker
                VStack(alignment: .leading) {
                    Text("Security Level")
                        .font(.headline)
                    Picker("Security Level", selection: $selectedLevel) {
                        ForEach(DilithiumSecurityLevel.allCases, id: \.self) { level in
                            Text("\(level.displayName)")
                                .tag(level)
                        }
                    }
                    .pickerStyle(.segmented)
                }
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(10)
                
                // Key Generation
                GroupBox("Signing Key") {
                    Button(action: generateKeyPair) {
                        HStack {
                            Image(systemName: "key.fill")
                            Text("Generate Dilithium Key Pair")
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(keyPair != nil ? Color.green : Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                    }
                    
                    if let key = keyPair {
                        VStack(alignment: .leading) {
                            Text("Public Key: \(key.publicKey.count) bytes")
                            Text("Private Key: \(key.privateKey.count) bytes")
                        }
                        .font(.caption)
                    }
                }
                
                // Message Input
                GroupBox("Message to Sign") {
                    TextEditor(text: $messageText)
                        .frame(height: 100)
                        .border(Color.gray.opacity(0.3))
                }
                
                // Sign & Verify
                if keyPair != nil {
                    GroupBox("Sign & Verify") {
                        VStack(spacing: 15) {
                            Button(action: signMessage) {
                                HStack {
                                    Image(systemName: "signature")
                                    Text("Sign Message")
                                }
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.orange)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                            }
                            
                            if let sig = signature {
                                VStack(alignment: .leading) {
                                    Text("Signature: \(sig.count) bytes")
                                        .font(.caption)
                                    Text(sig.hexString.prefix(64) + "...")
                                        .font(.system(.caption2, design: .monospaced))
                                }
                            }
                            
                            if signature != nil {
                                Button(action: verifySignature) {
                                    HStack {
                                        Image(systemName: "checkmark.shield")
                                        Text("Verify Signature")
                                    }
                                    .frame(maxWidth: .infinity)
                                    .padding()
                                    .background(Color.purple)
                                    .foregroundColor(.white)
                                    .cornerRadius(10)
                                }
                                
                                if let verified = isVerified {
                                    HStack {
                                        Image(systemName: verified ? "checkmark.circle.fill" : "xmark.circle.fill")
                                            .foregroundColor(verified ? .green : .red)
                                        Text(verified ? "Signature Valid" : "Signature Invalid")
                                    }
                                }
                            }
                        }
                    }
                }
                
                if let error = errorMessage {
                    Text(error)
                        .foregroundColor(.red)
                        .font(.caption)
                }
            }
            .padding()
        }
        .overlay {
            if isLoading {
                ProgressView("Processing...")
                    .padding()
                    .background(.regularMaterial)
                    .cornerRadius(10)
            }
        }
    }
    
    private func generateKeyPair() {
        Task {
            isLoading = true
            errorMessage = nil
            defer { isLoading = false }
            
            do {
                keyPair = try await pqcService.generateDilithiumKeyPair(securityLevel: selectedLevel)
                signature = nil
                isVerified = nil
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }
    
    private func signMessage() {
        guard let key = keyPair else { return }
        let message = messageText.data(using: .utf8) ?? Data()
        
        Task {
            isLoading = true
            errorMessage = nil
            defer { isLoading = false }
            
            do {
                signature = try await pqcService.sign(
                    privateKey: key.privateKey,
                    message: message,
                    securityLevel: selectedLevel
                )
                isVerified = nil
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }
    
    private func verifySignature() {
        guard let key = keyPair, let sig = signature else { return }
        let message = messageText.data(using: .utf8) ?? Data()
        
        Task {
            isLoading = true
            errorMessage = nil
            defer { isLoading = false }
            
            do {
                isVerified = try await pqcService.verify(
                    publicKey: key.publicKey,
                    message: message,
                    signature: sig,
                    securityLevel: selectedLevel
                )
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }
}

// MARK: - Encryption View

struct EncryptionView: View {
    @EnvironmentObject var pqcService: VantisPQCService
    @State private var plaintext = "Secret message to encrypt"
    @State private var encryptedData: Data?
    @State private var decryptedText: String?
    @State private var encryptionKey: Data?
    @State private var isLoading = false
    @State private var errorMessage: String?
    
    var body: some View {
        ScrollView {
            VStack(spacing: 20) {
                // Key Generation
                GroupBox("Encryption Key") {
                    Button(action: generateKey) {
                        HStack {
                            Image(systemName: "key.fill")
                            Text(encryptionKey != nil ? "Key Generated" : "Generate Encryption Key")
                        }
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(encryptionKey != nil ? Color.green : Color.blue)
                        .foregroundColor(.white)
                        .cornerRadius(10)
                    }
                    
                    if let key = encryptionKey {
                        Text("Key: \(key.hexString.prefix(32))...")
                            .font(.system(.caption, design: .monospaced))
                    }
                }
                
                // Plaintext Input
                GroupBox("Plaintext") {
                    TextEditor(text: $plaintext)
                        .frame(height: 100)
                        .border(Color.gray.opacity(0.3))
                }
                
                // Encryption
                if encryptionKey != nil {
                    GroupBox("Encryption") {
                        VStack(spacing: 15) {
                            Button(action: encryptData) {
                                HStack {
                                    Image(systemName: "lock.fill")
                                    Text("Encrypt")
                                }
                                .frame(maxWidth: .infinity)
                                .padding()
                                .background(Color.orange)
                                .foregroundColor(.white)
                                .cornerRadius(10)
                            }
                            
                            if let encrypted = encryptedData {
                                VStack(alignment: .leading) {
                                    Text("Encrypted: \(encrypted.count) bytes")
                                        .font(.caption)
                                    Text("(includes nonce + auth tag)")
                                        .font(.caption2)
                                        .foregroundColor(.secondary)
                                }
                            }
                        }
                    }
                    
                    // Decryption
                    if encryptedData != nil {
                        GroupBox("Decryption") {
                            VStack(spacing: 15) {
                                Button(action: decryptData) {
                                    HStack {
                                        Image(systemName: "lock.open.fill")
                                        Text("Decrypt")
                                    }
                                    .frame(maxWidth: .infinity)
                                    .padding()
                                    .background(Color.purple)
                                    .foregroundColor(.white)
                                    .cornerRadius(10)
                                }
                                
                                if let decrypted = decryptedText {
                                    VStack(alignment: .leading) {
                                        Text("Decrypted:")
                                            .font(.headline)
                                        Text(decrypted)
                                    }
                                    
                                    HStack {
                                        if decrypted == plaintext {
                                            Image(systemName: "checkmark.circle.fill")
                                                .foregroundColor(.green)
                                            Text("Matches original!")
                                        } else {
                                            Image(systemName: "xmark.circle.fill")
                                                .foregroundColor(.red)
                                            Text("Does not match")
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                if let error = errorMessage {
                    Text(error)
                        .foregroundColor(.red)
                        .font(.caption)
                }
            }
            .padding()
        }
        .overlay {
            if isLoading {
                ProgressView("Processing...")
                    .padding()
                    .background(.regularMaterial)
                    .cornerRadius(10)
            }
        }
    }
    
    private func generateKey() {
        encryptionKey = try? pqcService.generateRandomBytes(count: 32)
        encryptedData = nil
        decryptedText = nil
    }
    
    private func encryptData() {
        guard let key = encryptionKey else { return }
        let data = plaintext.data(using: .utf8) ?? Data()
        
        Task {
            isLoading = true
            errorMessage = nil
            defer { isLoading = false }
            
            do {
                encryptedData = try await pqcService.encryptStream(data: data, key: key)
                decryptedText = nil
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }
    
    private func decryptData() {
        guard let key = encryptionKey, let encrypted = encryptedData else { return }
        
        Task {
            isLoading = true
            errorMessage = nil
            defer { isLoading = false }
            
            do {
                let decrypted = try await pqcService.decryptStream(encryptedData: encrypted, key: key)
                decryptedText = String(data: decrypted, encoding: .utf8)
            } catch {
                errorMessage = error.localizedDescription
            }
        }
    }
}

// MARK: - Extensions

extension KyberSecurityLevel {
    var displayName: String {
        switch self {
        case .kyber512: return "Kyber-512"
        case .kyber768: return "Kyber-768"
        case .kyber1024: return "Kyber-1024"
        }
    }
}

extension DilithiumSecurityLevel {
    var displayName: String {
        switch self {
        case .dilithium2: return "Dilithium2"
        case .dilithium3: return "Dilithium3"
        case .dilithium5: return "Dilithium5"
        }
    }
}

extension Data {
    var hexString: String {
        map { String(format: "%02hhx", $0) }.joined()
    }
}

#Preview {
    NavigationStack {
        ContentView()
    }
}