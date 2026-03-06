//
//  HomeView.swift
//  VantisMobile
//
//  Home view showing connection status and quick actions
//

import SwiftUI

struct HomeView: View {
    @EnvironmentObject private var tunnelService: SecureTunnelService
    @EnvironmentObject private var biometricService: BiometricAuthService
    
    @State private var showingConnectSheet = false
    @State private var isAuthenticating = false
    
    var body: some View {
        NavigationView {
            ScrollView {
                VStack(spacing: 20) {
                    // Connection Status Card
                    ConnectionStatusCard()
                    
                    // Quick Actions
                    QuickActionsSection()
                    
                    // Recent Documents
                    RecentDocumentsSection()
                    
                    // Notifications Preview
                    NotificationsPreviewSection()
                }
                .padding()
            }
            .navigationTitle("Vantis Mobile")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button {
                        showingConnectSheet = true
                    } label: {
                        Image(systemName: "antenna.radiowaves.left.and.right")
                    }
                }
            }
            .sheet(isPresented: $showingConnectSheet) {
                ConnectView()
            }
        }
    }
}

// MARK: - Connection Status Card

struct ConnectionStatusCard: View {
    @EnvironmentObject private var tunnelService: SecureTunnelService
    
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Image(systemName: statusIcon)
                    .font(.title)
                    .foregroundColor(statusColor)
                
                VStack(alignment: .leading) {
                    Text("Connection Status")
                        .font(.headline)
                    Text(statusText)
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }
                
                Spacer()
                
                Circle()
                    .fill(statusColor)
                    .frame(width: 12, height: 12)
            }
            
            if tunnelService.connectionStatus == .connected {
                Divider()
                
                HStack {
                    VStack(alignment: .leading) {
                        Text("Duration")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        Text(tunnelService.connectionInfo?.formattedDuration ?? "N/A")
                            .font(.subheadline)
                    }
                    
                    Spacer()
                    
                    VStack(alignment: .trailing) {
                        Text("Latency")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        Text(tunnelService.connectionInfo?.formattedLatency ?? "N/A")
                            .font(.subheadline)
                    }
                }
            }
        }
        .padding()
        .background(Color(.systemGray6))
        .cornerRadius(12)
    }
    
    private var statusIcon: String {
        switch tunnelService.connectionStatus {
        case .connected:
            return "checkmark.circle.fill"
        case .connecting:
            return "antenna.radiowaves.left.and.right"
        case .reconnecting:
            return "arrow.clockwise"
        case .disconnected:
            return "xmark.circle.fill"
        }
    }
    
    private var statusColor: Color {
        switch tunnelService.connectionStatus {
        case .connected:
            return .green
        case .connecting, .reconnecting:
            return .orange
        case .disconnected:
            return .red
        }
    }
    
    private var statusText: String {
        switch tunnelService.connectionStatus {
        case .connected:
            return "Connected to VantisOffice"
        case .connecting:
            return "Connecting..."
        case .reconnecting:
            return "Reconnecting..."
        case .disconnected:
            return "Not connected"
        }
    }
}

// MARK: - Quick Actions Section

struct QuickActionsSection: View {
    var body: some View {
        VStack(alignment: .leading) spacing: 12) {
            Text("Quick Actions")
                .font(.headline)
            
            LazyVGrid(columns: [
                GridItem(.flexible()),
                GridItem(.flexible())
            ], spacing: 16) {
                QuickActionButton(
                    title: "New Document",
                    icon: "doc.badge.plus",
                    color: .blue
                ) {
                    // TODO: Create new document
                }
                
                QuickActionButton(
                    title: "Scan QR",
                    icon: "qrcode.viewfinder",
                    color: .green
                ) {
                    // TODO: Scan QR code for connection
                }
                
                QuickActionButton(
                    title: "Sync Now",
                    icon: "arrow.clockwise",
                    color: .orange
                ) {
                    // TODO: Force sync
                }
                
                QuickActionButton(
                    title: "Browse Files",
                    icon: "folder",
                    color: .purple
                ) {
                    // TODO: Browse files
                }
            }
        }
    }
}

struct QuickActionButton: View {
    let title: String
    let icon: String
    let color: Color
    let action: () -> Void
    
    var body: some View {
        Button(action: action) {
            VStack(spacing: 8) {
                Image(systemName: icon)
                    .font(.title2)
                    .foregroundColor(color)
                
                Text(title)
                    .font(.caption)
                    .foregroundColor(.primary)
            }
            .frame(maxWidth: .infinity)
            .padding()
            .background(Color(.systemGray6))
            .cornerRadius(12)
        }
    }
}

// MARK: - Recent Documents Section

struct RecentDocumentsSection: View {
    // TODO: Add documents state
    
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Recent Documents")
                    .font(.headline)
                
                Spacer()
                
                NavigationLink("See All") {
                    DocumentsView()
                }
                .font(.subheadline)
            }
            
            if true { // TODO: Check if documents exist
                VStack(spacing: 8) {
                    ForEach(0..<3) { _ in
                        DocumentRowPlaceholder()
                    }
                }
            } else {
                ContentUnavailableView(
                    "No Documents",
                    systemImage: "doc.text",
                    description: Text("Connect to VantisOffice to access your documents")
                )
            }
        }
    }
}

struct DocumentRowPlaceholder: View {
    var body: some View {
        HStack {
            Image(systemName: "doc.text.fill")
                .font(.title2)
                .foregroundColor(.blue)
                .frame(width: 40, height: 40)
                .background(Color.blue.opacity(0.1))
                .cornerRadius(8)
            
            VStack(alignment: .leading) {
                Text("Document Title")
                    .font(.subheadline)
                    .fontWeight(.medium)
                Text("Modified just now")
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
            
            Image(systemName: "chevron.right")
                .foregroundColor(.secondary)
        }
        .padding(.vertical, 8)
    }
}

// MARK: - Notifications Preview Section

struct NotificationsPreviewSection: View {
    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack {
                Text("Notifications")
                    .font(.headline)
                
                Spacer()
                
                NavigationLink("See All") {
                    NotificationsView()
                }
                .font(.subheadline)
            }
            
            Text("No new notifications")
                .font(.subheadline)
                .foregroundColor(.secondary)
                .frame(maxWidth: .infinity)
                .padding()
                .background(Color(.systemGray6))
                .cornerRadius(8)
        }
    }
}

#Preview {
    HomeView()
        .environmentObject(SecureTunnelService())
        .environmentObject(BiometricAuthService())
}