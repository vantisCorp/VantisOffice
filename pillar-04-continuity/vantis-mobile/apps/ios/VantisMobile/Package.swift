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
        .library(
            name: "VantisMobileFFI",
            targets: ["VantisMobileFFI"]
        ),
    ],
    dependencies: [
        // Dependencies can be added here when needed
    ],
    targets: [
        // Main app target
        .target(
            name: "VantisMobile",
            dependencies: ["VantisMobileFFI"],
            path: ".",
            exclude: ["Package.swift", "README.md", "Sources"],
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
            ],
            linkerSettings: [
                .linkedLibrary("vantis_mobile"),
            ]
        ),
        // FFI wrapper target
        .target(
            name: "VantisMobileFFI",
            dependencies: [],
            path: "Sources",
            publicHeadersPath: ".",
            cSettings: [
                .headerSearchPath("."),
                .define("VANTIS_MOBILE_IMPORT", to: "1")
            ],
            linkerSettings: [
                .linkedLibrary("vantis_mobile"),
            ]
        ),
        .testTarget(
            name: "VantisMobileTests",
            dependencies: ["VantisMobile", "VantisMobileFFI"]
        ),
    ]
)