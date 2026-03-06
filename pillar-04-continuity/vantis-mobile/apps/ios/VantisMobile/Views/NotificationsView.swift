//
//  NotificationsView.swift
//  VantisMobile
//
//  Notifications list view
//

import SwiftUI

struct NotificationsView: View {
    @State private var notifications: [VantisNotification] = []
    @State private var showingFilterSheet = false
    @State private var selectedFilter: NotificationPriority?
    
    var body: some View {
        NavigationView {
            Group {
                if filteredNotifications.isEmpty {
                    emptyView
                } else {
                    listContent
                }
            }
            .navigationTitle("Notifications")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button {
                        showingFilterSheet = true
                    } label: {
                        Image(systemName: "line.3.horizontal.decrease.circle")
                    }
                }
            }
            .sheet(isPresented: $showingFilterSheet) {
                NotificationFilterSheet(
                    selectedFilter: $selectedFilter,
                    onClearFilter: { selectedFilter = nil },
                    onMarkAllRead: { markAllAsRead() }
                )
            }
            .onAppear {
                loadNotifications()
            }
        }
    }
    
    private var filteredNotifications: [VantisNotification] {
        if let filter = selectedFilter {
            return notifications.filter { $0.priority >= filter }
        }
        return notifications
    }
    
    private var listContent: some View {
        List {
            ForEach(groupedNotifications.keys.sorted(by: >), id: \.self) { date in
                Section(header: Text(dateSectionTitle(for: date))) {
                    ForEach(groupedNotifications[date] ?? []) { notification in
                        NotificationRow(notification: notification)
                            .onTapGesture {
                                // TODO: Handle notification tap
                            }
                    }
                    .onDelete { indexSet in
                        deleteNotifications(at: indexSet, in: date)
                    }
                }
            }
        }
        .listStyle(.insetGrouped)
    }
    
    private var emptyView: some View {
        ContentUnavailableView(
            "No Notifications",
            systemImage: "bell.slash",
            description: Text("You're all caught up!")
        )
    }
    
    private var groupedNotifications: [Date: [VantisNotification]] {
        Dictionary(grouping: notifications) { notification in
            Calendar.current.startOfDay(for: notification.timestamp)
        }
    }
    
    private func dateSectionTitle(for date: Date) -> String {
        let calendar = Calendar.current
        let today = calendar.startOfDay(for: Date())
        let yesterday = calendar.date(byAdding: .day, value: -1, to: today)!
        
        if date == today {
            return "Today"
        } else if date == yesterday {
            return "Yesterday"
        } else {
            let formatter = DateFormatter()
            formatter.dateStyle = .medium
            return formatter.string(from: date)
        }
    }
    
    // MARK: - Actions
    
    private func loadNotifications() {
        // TODO: Load notifications from service
    }
    
    private func deleteNotifications(at offsets: IndexSet, in date: Date) {
        // TODO: Delete notifications
    }
    
    private func markAllAsRead() {
        notifications.indices.forEach { notifications[$0].read = true }
    }
}

// MARK: - Notification Row

struct NotificationRow: View {
    let notification: VantisNotification
    
    var body: some View {
        HStack(alignment: .top, spacing: 12) {
            // Icon
            Image(systemName: notification.iconName)
                .font(.title2)
                .foregroundColor(.white)
                .frame(width: 40, height: 40)
                .background(priorityColor)
                .cornerRadius(10)
            
            // Content
            VStack(alignment: .leading, spacing: 4) {
                Text(notification.title)
                    .font(.headline)
                    .lineLimit(1)
                
                Text(notification.body)
                    .font(.subheadline)
                    .foregroundColor(.secondary)
                    .lineLimit(2)
                
                Text(formatDate(notification.timestamp))
                    .font(.caption)
                    .foregroundColor(.secondary)
            }
            
            Spacer()
            
            // Read indicator
            if !notification.read {
                Circle()
                    .fill(priorityColor)
                    .frame(width: 8, height: 8)
            }
        }
        .padding(.vertical, 4)
        .opacity(notification.read ? 0.6 : 1.0)
    }
    
    private var priorityColor: Color {
        switch notification.priority {
        case .urgent:
            return .red
        case .high:
            return .orange
        case .normal:
            return .blue
        case .low:
            return .gray
        }
    }
    
    private func formatDate(_ date: Date) -> String {
        let formatter = RelativeDateTimeFormatter()
        formatter.unitsStyle = .short
        return formatter.localizedString(for: date, relativeTo: Date())
    }
}

// MARK: - Notification Filter Sheet

struct NotificationFilterSheet: View {
    @Binding var selectedFilter: NotificationPriority?
    let onClearFilter: () -> Void
    let onMarkAllRead: () -> Void
    
    @Environment(\.dismiss) private var dismiss
    
    var body: some View {
        NavigationView {
            List {
                Section(header: Text("Filter by Priority")) {
                    ForEach([NotificationPriority.urgent, .high, .normal, .low], id: \.self) { priority in
                        Button {
                            selectedFilter = priority
                            dismiss()
                        } label: {
                            HStack {
                                Text(priority.displayName)
                                if selectedFilter == priority {
                                    Spacer()
                                    Image(systemName: "checkmark")
                                        .foregroundColor(.blue)
                                }
                            }
                        }
                    }
                }
                
                Section {
                    Button {
                        onClearFilter()
                        dismiss()
                    } label: {
                        Text("Clear Filter")
                    }
                    
                    Button {
                        onMarkAllRead()
                        dismiss()
                    } label: {
                        Text("Mark All as Read")
                    }
                }
            }
            .navigationTitle("Filter")
            .navigationBarTitleDisplayMode(.inline)
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        dismiss()
                    }
                }
            }
        }
    }
}

// MARK: - Notification Priority Extension

extension NotificationPriority {
    var displayName: String {
        switch self {
        case .urgent:
            return "Urgent"
        case .high:
            return "High"
        case .normal:
            return "Normal"
        case .low:
            return "Low"
        }
    }
}

#Preview {
    NotificationsView()
}