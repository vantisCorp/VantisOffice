//
//  App.swift
//  VantisMobile
//
//  Main application entry point
//

import SwiftUI

@main
struct VantisMobileApp: App {
    @StateObject private var tunnelService = SecureTunnelService()
    @StateObject private var biometricService = BiometricAuthService()
    @StateObject private var notificationService = NotificationService()
    
    init() {
        // Configure app appearance
        configureAppearance()
    }
    
    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(tunnelService)
                .environmentObject(biometricService)
                .environmentObject(notificationService)
                .onAppear {
                    setupNotifications()
                }
        }
    }
    
    // MARK: - Setup Methods
    
    private func configureAppearance() {
        // Configure navigation bar appearance
        let appearance = UINavigationBarAppearance()
        appearance.configureWithOpaqueBackground()
        appearance.backgroundColor = .systemBackground
        
        UINavigationBar.appearance().standardAppearance = appearance
        UINavigationBar.appearance().scrollEdgeAppearance = appearance
        UINavigationBar.appearance().compactAppearance = appearance
        
        // Configure tab bar appearance
        let tabBarAppearance = UITabBarAppearance()
        tabBarAppearance.configureWithOpaqueBackground()
        tabBarAppearance.backgroundColor = .systemBackground
        
        UITabBar.appearance().standardAppearance = tabBarAppearance
        if #available(iOS 15.0, *) {
            UITabBar.appearance().scrollEdgeAppearance = tabBarAppearance
        }
    }
    
    private func setupNotifications() {
        notificationService.requestAuthorization { granted in
            print("Notification authorization granted: \(granted)")
        }
    }
}

// MARK: - Notification Service

class NotificationService: ObservableObject {
    private let notificationCenter = UNUserNotificationCenter.current()
    
    func requestAuthorization(completion: @escaping (Bool) -> Void) {
        notificationCenter.requestAuthorization(options: [.alert, .sound, .badge]) { granted, error in
            completion(granted)
        }
    }
    
    func send(notification: VantisNotification) {
        let content = notification.toUNNotificationContent()
        let request = UNNotificationRequest(
            identifier: notification.id.uuidString,
            content: content,
            trigger: nil
        )
        
        notificationCenter.add(request) { error in
            if let error = error {
                print("Failed to send notification: \(error)")
            }
        }
    }
    
    func removeAllPending() {
        notificationCenter.removeAllPendingNotificationRequests()
    }
    
    func removeAllDelivered() {
        notificationCenter.removeAllDeliveredNotifications()
    }
}