//
//  SettingsView.swift
//  VantisMobile
//
//  Settings view for app configuration
//

import SwiftUI

struct SettingsView: View {
    @EnvironmentObject private var biometricService: BiometricAuthService
    
    @AppStorage("biometricEnabled") private var biometricEnabled = true
    @AppStorage("autoSyncEnabled") private var autoSyncEnabled = true
    @AppStorage("notificationsEnabled") private var notificationsEnabled = true
    @AppStorage("cacheSize") private var cacheSize = "500"
    
    @State private var showingClearCacheAlert = false
    @State private var showingSignOutAlert = false
    
    var body: some View {
        NavigationView {
            List {
                // Connection Settings
                Section(header: Text("Connection")) {
                    NavigationLink {
                        ConnectView()
                    } label: {
                        HStack {
                            Image(systemName: "antenna.radiowaves.left.and.right")
                                .foregroundColor(.blue)
                            Text("Server Settings")
                        }
                    }
                }
                
                // Security Settings
                Section(header: Text("Security")) {
                    Toggle(isOn: $biometricEnabled) {
                        Label {
                            Text(biometricService.biometricTypeName)
                        } icon: {
                            Image(systemName: biometricService.hasFaceID ? "faceid" : "touchid")
                                .foregroundColor(.green)
                        }
                    }
                    .disabled(!biometricService.isAvailable)
                    
                    NavigationLink {
                        ChangePinView()
                    } label: {
                        HStack {
                            Image(systemName: "lock")
                                .foregroundColor(.orange)
                            Text("Change PIN")
                        }
                    }
                }
                
                // Sync Settings
                Section(header: Text("Sync")) {
                    Toggle(isOn: $autoSyncEnabled) {
                        Label("Auto Sync", systemImage: "arrow.clockwise")
                    }
                    
                    Picker("Cache Size", selection: $cacheSize) {
                        Text("100 MB").tag("100")
                        Text("500 MB").tag("500")
                        Text("1 GB").tag("1000")
                        Text("Unlimited").tag("0")
                    }
                    
                    Button {
                        showingClearCacheAlert = true
                    } label: {
                        HStack {
                            Image(systemName: "trash")
                                .foregroundColor(.red)
                            Text("Clear Cache")
                        }
                    }
                }
                
                // Notifications
                Section(header: Text("Notifications")) {
                    Toggle(isOn: $notificationsEnabled) {
                        Label("Enable Notifications", systemImage: "bell")
                    }
                }
                
                // About
                Section(header: Text("About")) {
                    HStack {
                        Text("Version")
                        Spacer()
                        Text("1.0.0")
                            .foregroundColor(.secondary)
                    }
                    
                    NavigationLink {
                        PrivacyPolicyView()
                    } label: {
                        Text("Privacy Policy")
                    }
                    
                    NavigationLink {
                        TermsOfServiceView()
                    } label: {
                        Text("Terms of Service")
                    }
                }
                
                // Sign Out
                Section {
                    Button(role: .destructive) {
                        showingSignOutAlert = true
                    } label: {
                        HStack {
                            Spacer()
                            Text("Sign Out")
                            Spacer()
                        }
                    }
                }
            }
            .navigationTitle("Settings")
            .alert("Clear Cache", isPresented: $showingClearCacheAlert) {
                Button("Cancel", role: .cancel) {}
                Button("Clear", role: .destructive) {
                    // TODO: Clear cache
                }
            } message: {
                Text("This will remove all cached documents. You'll need to re-download them when needed.")
            }
            .alert("Sign Out", isPresented: $showingSignOutAlert) {
                Button("Cancel", role: .cancel) {}
                Button("Sign Out", role: .destructive) {
                    // TODO: Sign out
                }
            } message: {
                Text("Are you sure you want to sign out?")
            }
        }
    }
}

// MARK: - Placeholder Views

struct ChangePinView: View {
    var body: some View {
        Text("Change PIN")
            .navigationTitle("Change PIN")
    }
}

struct PrivacyPolicyView: View {
    var body: some View {
        Text("Privacy Policy")
            .navigationTitle("Privacy Policy")
    }
}

struct TermsOfServiceView: View {
    var body: some View {
        Text("Terms of Service")
            .navigationTitle("Terms of Service")
    }
}

#Preview {
    SettingsView()
        .environmentObject(BiometricAuthService())
}