//
//  ConnectView.swift
//  VantisMobile
//
//  View for connecting to VantisOffice desktop
//

import SwiftUI

struct ConnectView: View {
    @Environment(\.dismiss) private var dismiss
    @EnvironmentObject private var tunnelService: SecureTunnelService
    @EnvironmentObject private var biometricService: BiometricAuthService
    
    @State private var serverUrl = ""
    @State private var showingQRScanner = false
    @State private var isConnecting = false
    @State private var errorMessage: String?
    @State private var showingSuccess = false
    
    var body: some View {
        NavigationView {
            Form {
                Section(header: Text("Server URL")) {
                    TextField("wss://your-server.vantis.ai", text: $serverUrl)
                        .textContentType(.URL)
                        .autocapitalization(.none)
                        .autocorrectionDisabled()
                        .keyboardType(.URL)
                }
                
                Section {
                    Button {
                        showingQRScanner = true
                    } label: {
                        Label("Scan QR Code", systemImage: "qrcode.viewfinder")
                    }
                    
                    Button {
                        // TODO: Use last connected server
                        serverUrl = "wss://tunnel.vantis.ai"
                    } label: {
                        Label("Use Default Server", systemImage: "network")
                    }
                }
                
                if let error = errorMessage {
                    Section {
                        Label(error, systemImage: "exclamationmark.triangle")
                            .foregroundColor(.red)
                    }
                }
                
                Section {
                    Button {
                        connect()
                    } label: {
                        if isConnecting {
                            ProgressView()
                                .frame(maxWidth: .infinity)
                        } else {
                            Text("Connect")
                                .frame(maxWidth: .infinity)
                        }
                    }
                    .disabled(serverUrl.isEmpty || isConnecting)
                    .buttonStyle(.borderedProminent)
                }
            }
            .navigationTitle("Connect to Desktop")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
            .sheet(isPresented: $showingQRScanner) {
                QRScannerView(serverUrl: $serverUrl)
            }
            .alert("Connected!", isPresented: $showingSuccess) {
                Button("OK") {
                    dismiss()
                }
            } message: {
                Text("Successfully connected to VantisOffice")
            }
        }
    }
    
    private func connect() {
        guard !serverUrl.isEmpty else { return }
        
        isConnecting = true
        errorMessage = nil
        
        // Authenticate with biometrics first
        biometricService.authenticate(reason: biometricService.authenticationReason(for: "connect to VantisOffice")) { result in
            switch result {
            case .success:
                Task {
                    await performConnection()
                }
            case .failure(let error):
                isConnecting = false
                errorMessage = error.localizedDescription
            }
        }
    }
    
    private func performConnection() async {
        guard let url = URL(string: serverUrl) else {
            await MainActor.run {
                isConnecting = false
                errorMessage = "Invalid server URL"
            }
            return
        }
        
        let config = TunnelConfig(
            serverUrl: url,
            deviceId: UUID(),
            encryptionKey: Data(repeating: 0, count: 32) // TODO: Use proper key
        )
        
        do {
            try await tunnelService.connect(config: config)
            await MainActor.run {
                isConnecting = false
                showingSuccess = true
            }
        } catch {
            await MainActor.run {
                isConnecting = false
                errorMessage = error.localizedDescription
            }
        }
    }
}

// MARK: - QR Scanner View

struct QRScannerView: View {
    @Binding var serverUrl: String
    @Environment(\.dismiss) private var dismiss
    
    var body: some View {
        NavigationView {
            VStack {
                // TODO: Implement actual QR scanner
                ContentUnavailableView(
                    "QR Scanner",
                    systemImage: "qrcode.viewfinder",
                    description: Text("Scan the QR code displayed in VantisOffice desktop to connect")
                )
                
                Spacer()
                
                Button("Manual Entry") {
                    dismiss()
                }
                .buttonStyle(.borderedProminent)
            }
            .padding()
            .navigationTitle("Scan QR Code")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("Cancel") {
                        dismiss()
                    }
                }
            }
        }
    }
}

#Preview {
    ConnectView()
        .environmentObject(SecureTunnelService())
        .environmentObject(BiometricAuthService())
}