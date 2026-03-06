// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "VantisMobile",
    platforms: [
        .iOS(.v16)
    ],
    products: [
        .library(
            name: "VantisMobile",
            targets: ["VantisMobile"]
        ),
    ],
    dependencies: [
        // Dependencies can be added here when needed
        // .package(url: "https://github.com/username/repo.git", from: "1.0.0"),
    ],
    targets: [
        .target(
            name: "VantisMobile",
            dependencies: [],
            path: ".",
            exclude: ["Package.swift", "README.md"],
            sources: [
                "App.swift",
                "Models/Device.swift",
                "Models/Document.swift",
                "Models/Notification.swift",
                "Models/Connection.swift",
                "Services/SecureTunnelService.swift",
                "Services/BiometricAuthService.swift",
                "Views/ContentView.swift",
                "Views/HomeView.swift",
                "Views/DocumentsView.swift",
                "Views/NotificationsView.swift",
                "Views/SettingsView.swift",
                "Views/ConnectView.swift"
            ],
            resources: [
                .process("Assets.xcassets"),
                .process("Info.plist")
            ]
        ),
        .testTarget(
            name: "VantisMobileTests",
            dependencies: ["VantisMobile"]
        ),
    ]
)