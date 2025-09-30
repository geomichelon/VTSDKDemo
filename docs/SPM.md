# Swift Package Manager (binaryTarget) for VTSDK

This package uses a `binaryTarget` to distribute the Rust-built XCFramework for iOS.

## Two ways to consume

1) Local development

- Use the path-based target in Xcode during development:
  - File → Add Packages… → Add this repository but select “Add Local Package” and point to `dist/VTSDK.xcframework` (Xcode supports local framework references). Alternatively, drag `VTSDK.xcframework` into your project.
- For production, prefer the binaryTarget URL below managed by CI.

2) Binary target (recommended)

- The manifest `Package.swift` declares a binaryTarget named `VTSDK`.
- CI updates the `url` and `checksum` on each release tag.
- In Xcode: File → Add Packages… → enter this repo URL, pick the version (tag) and add the library.

## CI-driven release flow

Workflow: `.github/workflows/spm-release.yml`

- Trigger: manually (workflow_dispatch) with an input `version` (e.g., `0.2.0`).
- Steps:
  1. Build the iOS XCFramework (device + simulator) using Rust and Xcode tools.
  2. Zip it: `dist/VTSDK.xcframework.zip`.
  3. Compute checksum: `swift package compute-checksum dist/VTSDK.xcframework.zip`.
  4. Update `Package.swift` (URL to `.../releases/download/v{version}/VTSDK.xcframework.zip` and the computed checksum).
  5. Commit changes, tag `v{version}`, and push.
  6. Create a GitHub Release and upload the zip as an asset.

Consumers can then add the package at the tag, and SPM will verify the checksum matches.

