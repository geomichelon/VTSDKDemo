// swift-tools-version:5.7
import PackageDescription

// NOTE:
// - The URL and checksum below are placeholders.
// - The CI workflow .github/workflows/spm-release.yml updates them on release.

let package = Package(
    name: "VTSDK",
    platforms: [
        .iOS(.v13)
    ],
    products: [
        .library(name: "VTSDK", targets: ["VTSDK"]) 
    ],
    targets: [
        .binaryTarget(
            name: "VTSDK",
            url: "https://github.com/geomichelon/VTSDKDemo/releases/download/v0.0.0/VTSDK.xcframework.zip",
            checksum: "CHECKSUM_PLACEHOLDER"
        )
    ]
)

