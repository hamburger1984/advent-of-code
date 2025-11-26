// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "day",
    platforms: [.macOS(.v13)],
    targets: [
        .executableTarget(
            name: "day",
            path: "Sources"
        ),
        .testTarget(
            name: "SolutionTests",
            dependencies: ["day"],
            path: "Tests"
        )
    ]
)
