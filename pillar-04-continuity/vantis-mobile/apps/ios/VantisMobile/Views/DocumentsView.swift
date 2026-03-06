//
//  DocumentsView.swift
//  VantisMobile
//
//  Documents list view
//

import SwiftUI

struct DocumentsView: View {
    @State private var searchText = ""
    @State private var selectedFilter: DocumentType?
    @State private var showingSortOptions = false
    @State private var sortOption: SortOption = .modifiedDate
    
    enum SortOption: String, CaseIterable {
        case modifiedDate = "Modified Date"
        case createdDate = "Created Date"
        case name = "Name"
        case size = "Size"
    }
    
    var body: some View {
        NavigationView {
            VStack {
                // Search Bar
                SearchBar(text: $searchText)
                    .padding(.horizontal)
                
                // Filter Chips
                ScrollView(.horizontal, showsIndicators: false) {
                    HStack {
                        FilterChip(title: "All", isSelected: selectedFilter == nil) {
                            selectedFilter = nil
                        }
                        
                        ForEach(DocumentType.allCases, id: \.self) { type in
                            FilterChip(title: type.displayName, isSelected: selectedFilter == type) {
                                selectedFilter = type
                            }
                        }
                    }
                    .padding(.horizontal)
                }
                
                // Documents List
                List {
                    ForEach(filteredDocuments) { document in
                        DocumentRow(document: document)
                            .onTapGesture {
                                // TODO: Open document
                            }
                    }
                }
                .listStyle(.plain)
            }
            .navigationTitle("Documents")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Menu {
                        Button {
                            showingSortOptions = true
                        } label: {
                            Label("Sort", systemImage: "arrow.up.arrow.down")
                        }
                        
                        Button {
                            // TODO: Create new document
                        } label: {
                            Label("New Document", systemImage: "plus")
                        }
                    } label: {
                        Image(systemName: "ellipsis.circle")
                    }
                }
            }
            .confirmationDialog("Sort by", isPresented: $showingSortOptions) {
                ForEach(SortOption.allCases, id: \.self) { option in
                    Button(option.rawValue) {
                        sortOption = option
                    }
                }
            }
        }
    }
    
    private var filteredDocuments: [DocumentMetadata] {
        // TODO: Implement actual filtering
        return []
    }
}

// MARK: - Search Bar

struct SearchBar: View {
    @Binding var text: String
    
    var body: some View {
        HStack {
            Image(systemName: "magnifyingglass")
                .foregroundColor(.secondary)
            
            TextField("Search documents...", text: $text)
                .textFieldStyle(.plain)
            
            if !text.isEmpty {
                Button {
                    text = ""
                } label: {
                    Image(systemName: "xmark.circle.fill")
                        .foregroundColor(.secondary)
                }
            }
        }
        .padding(8)
        .background(Color(.systemGray6))
        .cornerRadius(8)
    }
}

// MARK: - Filter Chip

struct FilterChip: View {
    let title: String
    let isSelected: Bool
    let action: () -> Void
    
    var body: some View {
        Button(action: action) {
            Text(title)
                .font(.subheadline)
                .padding(.horizontal, 16)
                .padding(.vertical, 8)
                .background(isSelected ? Color.blue : Color(.systemGray6))
                .foregroundColor(isSelected ? .white : .primary)
                .cornerRadius(20)
        }
    }
}

// MARK: - Document Row

struct DocumentRow: View {
    let document: DocumentMetadata
    
    var body: some View {
        HStack(spacing: 12) {
            // Document Icon
            Image(systemName: document.iconName)
                .font(.title2)
                .foregroundColor(.white)
                .frame(width: 44, height: 44)
                .background(documentTypeColor)
                .cornerRadius(10)
            
            // Document Info
            VStack(alignment: .leading, spacing: 4) {
                Text(document.title)
                    .font(.headline)
                    .lineLimit(1)
                
                HStack {
                    Text(document.documentType.displayName)
                        .font(.caption)
                        .foregroundColor(.secondary)
                    
                    Text("•")
                        .foregroundColor(.secondary)
                    
                    Text(document.formattedSize)
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
            }
            
            Spacer()
            
            // Cached indicator
            if document.cached {
                Image(systemName: "checkmark.icloud")
                    .foregroundColor(.green)
            }
        }
        .padding(.vertical, 4)
    }
    
    private var documentTypeColor: Color {
        switch document.documentType {
        case .writer:
            return .blue
        case .flow:
            return .purple
        case .canvas:
            return .orange
        case .grid:
            return .green
        case .file:
            return .gray
        }
    }
}

// MARK: - Document Type Extension

extension DocumentType {
    var displayName: String {
        switch self {
        case .writer:
            return "Writer"
        case .flow:
            return "Flow"
        case .canvas:
            return "Canvas"
        case .grid:
            return "Grid"
        case .file:
            return "File"
        }
    }
}

#Preview {
    DocumentsView()
}