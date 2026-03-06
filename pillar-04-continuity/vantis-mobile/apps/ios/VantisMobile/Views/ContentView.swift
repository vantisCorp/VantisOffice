//
//  ContentView.swift
//  VantisMobile
//
//  Main content view for Vantis Mobile app
//

import SwiftUI

struct ContentView: View {
    @StateObject private var tunnelService = SecureTunnelService()
    @StateObject private var biometricService = BiometricAuthService()
    @State private var showingSettings = false
    @State private var showingDocuments = false
    @State private var showingNotifications = false
    @State private var selectedTab = 0
    
    var body: some View {
        TabView(selection: $selectedTab) {
            HomeView()
                .tabItem {
                    Label("Home", systemImage: "house.fill")
                }
                .tag(0)
            
            DocumentsView()
                .tabItem {
                    Label("Documents", systemImage: "doc.text.fill")
                }
                .tag(1)
            
            NotificationsView()
                .tabItem {
                    Label("Notifications", systemImage: "bell.fill")
                }
                .badge(notificationCount)
                .tag(2)
            
            SettingsView()
                .tabItem {
                    Label("Settings", systemImage: "gearshape.fill")
                }
                .tag(3)
        }
        .accentColor(.blue)
        .environmentObject(tunnelService)
        .environmentObject(biometricService)
    }
    
    private var notificationCount: Int {
        // TODO: Get actual notification count
        return 0
    }
}

#Preview {
    ContentView()
}