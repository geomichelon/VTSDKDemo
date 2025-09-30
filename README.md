# VT-SDK

[![iOS XCFramework](https://github.com/geomichelon/VTSDKDemo/actions/workflows/build-ios-xcframework.yml/badge.svg?branch=main)](https://github.com/geomichelon/VTSDKDemo/actions/workflows/build-ios-xcframework.yml)
[![Android .so](https://github.com/geomichelon/VTSDKDemo/actions/workflows/build-android-so.yml/badge.svg?branch=main)](https://github.com/geomichelon/VTSDKDemo/actions/workflows/build-android-so.yml)
[![Android Sample](https://github.com/geomichelon/VTSDKDemo/actions/workflows/build-android-sample.yml/badge.svg?branch=main)](https://github.com/geomichelon/VTSDKDemo/actions/workflows/build-android-sample.yml)
[![macOS Tests](https://github.com/geomichelon/VTSDKDemo/actions/workflows/macos-tests.yml/badge.svg?branch=main)](https://github.com/geomichelon/VTSDKDemo/actions/workflows/macos-tests.yml)
[![Coverage (core)](https://github.com/geomichelon/VTSDKDemo/actions/workflows/coverage.yml/badge.svg?branch=main)](https://github.com/geomichelon/VTSDKDemo/actions/workflows/coverage.yml)
[![SPM](https://img.shields.io/badge/SPM-supported-orange.svg)](docs/SPM.md)

Rust multi-platform SDK for Visual Testing, with FFI bindings for iOS (Swift) and Android (Kotlin/Java).

---

## UI Testing Integration Guide

Step-by-step instructions for integrating with Xcode/XCUITest and Android Instrumentation/Espresso:

- docs/GUIA_UI_TESTES.md

---

## Workspace Layout

- Workspace: root `Cargo.toml` declares `core`, `mock`, and `ffi` members.
- Core: `core/` contains the real implementation (e.g., compare).
- Mock: `mock/` contains a mock implementation for integration/testing.
- FFI: `ffi/` exposes a C ABI and builds static/shared libraries.

## Features/Modes

- `real` (default): uses `vt-sdk-core`.
- `mock`: uses `vt-sdk-mock`.
- Mutually exclusive; at least one must be enabled (default enables `real`).

## Desktop Build

- Real: `cargo build -p vt-sdk-ffi --release`
- Mock: `cargo build -p vt-sdk-ffi --release --no-default-features --features mock`

Typical outputs (macOS/Linux):
- `target/release/libvt_sdk_ffi.dylib` or `.so` (dynamic)
- `target/release/libvt_sdk_ffi.a` (static)

## Android

1) Install Android NDK/toolchains and desired targets (`aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`).
2) Build FFI (real):
   - `cargo build -p vt-sdk-ffi --release --target aarch64-linux-android`
3) For mock: add `--no-default-features --features mock`.
4) Link `libvt_sdk_ffi.so` into the app and load via `System.loadLibrary("vt_sdk_ffi")`.

### Android Package (AAR)

Prefer using the published AAR from GitHub Packages over copying `.so` manually.

- Coordinates: `com.geomichelon:vt-sdk-android:<version>` (version follows repo tags).
- Setup the GitHub Packages repository and credentials in Gradle, then add the dependency.
- Kotlin wrapper entry point: `com.geomichelon.vtsdk.VtSdkFFI.vtCompareImages(...)` returns JSON.

See docs/ANDROID_PACKAGE.md for complete setup and code examples.

## iOS

The `ffi` crate builds `staticlib` and `cdylib`. For iOS, use the static (`.a`). Common targets:

- Device: `aarch64-apple-ios`
- Simulator: `aarch64-apple-ios-sim` and `x86_64-apple-ios` (Intel simulators)

Example (adjust to your toolchain):

- `rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios`
- Real: `cargo build -p vt-sdk-ffi --release --target aarch64-apple-ios`
- Mock: `cargo build -p vt-sdk-ffi --release --no-default-features --features mock --target aarch64-apple-ios`

To produce an XCFramework, combine device + simulator outputs.

## Swift Package Manager (Binary)

Preferred integration for iOS apps is via SPM binaryTarget:

- The CI publishes `VTSDK.xcframework.zip` on tagged releases (vX.Y.Z) and updates `Package.swift` with the correct URL and checksum.
- In Xcode: File → Add Packages… → enter this repo URL → pick a release tag.
- Xcode will download the XCFramework and link it; headers are included inside the XCFramework.

Local development option:

- You can drag `dist/VTSDK.xcframework` into your project or use a path‑based binary target during local dev.
- See docs/SPM.md for details and the automated release flow.

## FFI API

Exposed functions (C ABI), see header `ffi/include/vt_sdk.h`:

- `const char* vt_compare_images(const char* baseline_url, const char* input_url, int32_t min_similarity, int32_t noise_filter, const char* excluded_areas_json, const char* meta_json);`
- `const char* vt_flex_search(const char* parent_url, const char* child_url, const char* meta_json);`
- `const char* vt_flex_locate(const char* container_url, const char* main_url, const char* relative_url, const char* meta_json);`
- `void vt_free_string(const char* ptr);` (free strings returned by the functions)

Usage rules:
- Pointers must be valid C strings (NUL-terminated). Always free the returned string with `vt_free_string` after copying it.

## Recommended Usage

- Baselines and storage
  - Keep baselines versioned (e.g., by app version/platform/theme/locale) in your repo or a storage bucket.
  - Resolve a local path for the baseline at test time (copy to tmp if needed).
- Thresholds and status
  - Provide `min_similarity` to get a `status` field (`Passed`/`Failed`). Start with 95–99 depending on tolerance, adjust per-screen.
- Excluded areas
  - Use `excluded_areas_json` to mask dynamic regions (time, ads, counters) and reduce flaky diffs.
- Diff output
  - Read `resultImageRef` to attach the diff to test reports (XCTest attachments, Android Instrumented tests logs/artifacts).
- CI integration
  - iOS: build XCFramework in CI and ship to consumers; Android: ship `.so` per ABI.
  - Run tests + coverage (core has a 90% gate) to keep quality high.
- Performance
  - The core normalizes to 256×256 grayscale for a robust/fast metric. For very large inputs, pre-scale images to reduce I/O.


## iOS binding (Swift)

- Link the static library (prefer via XCFramework) and include the header `vt_sdk.h` in a bridging header.

```swift
@_silgen_name("vt_compare_images")
func vt_compare_images(
  _ baseline: UnsafePointer<CChar>!,
  _ input: UnsafePointer<CChar>!,
  _ minSimilarity: Int32,
  _ noiseFilter: Int32,
  _ excludedAreasJson: UnsafePointer<CChar>!,
  _ metaJson: UnsafePointer<CChar>!
) -> UnsafePointer<CChar>!

@_silgen_name("vt_free_string")
func vt_free_string(_ ptr: UnsafePointer<CChar>!)

func compareJSON(baseline: String, input: String) -> String {
  var out = "{}"
  baseline.withCString { b in
    input.withCString { i in
      "[]".withCString { ex in
        "{}".withCString { me in
          if let res = vt_compare_images(b, i, 95, 20, ex, me) {
            out = String(cString: res)
            vt_free_string(res)
          }
        }
      }
    }
  }
  return out
}
```

## Android binding (Kotlin/Java)

- Load the library and call through a small JNI shim that returns a JSON `String`:

```kotlin
object VtSdkFFI {
    init { System.loadLibrary("vtsdk_shim") }
    external fun vtCompareImages(
        baseline: String,
        input: String,
        minSim: Int,
        noise: Int,
        excludedJson: String,
        metaJson: String
    ): String
}
```

Implement a JNI method in C/C++ that converts `jstring` to `const char*` and delegates to `vt_compare_images`, then returns a `jstring` with the JSON (see `android-demo/app/src/main/cpp/vtsdk_jni.cpp`).

---

## Contributing

- Getting started
  - Install Rust (stable) via `rustup`.
  - Optional: iOS toolchains (`aarch64-apple-ios`, `aarch64-apple-ios-sim`, `x86_64-apple-ios`) and Android NDK targets if working on FFI builds.
  - Build + test: `cargo build` and `cargo test`.
- Code style & quality
  - Run `cargo fmt` and `cargo clippy` locally; CI runs clippy on save via rust-analyzer (VS Code settings included).
  - Add tests for changes in `core` — CI enforces >= 90% line coverage (cargo-llvm-cov).
- PR guidelines
  - Prefer small, focused PRs with clear descriptions and rationale.
  - Include tests and docs updates (README or docs/). Keep FFI header changes (`ffi/include/vt_sdk.h`) in sync with Rust.
  - Ensure CI passes: iOS XCFramework build, Android .so build, coverage, and sample app builds.
- Reporting issues
  - Use GitHub Issues with reproducible steps, platform info, and sample images if applicable.
- Feature requests
  - Describe the use case and API expectations. We can discuss metrics (SSIM/PSNR), additional outputs (heatmaps), or new bindings.


---

## Notes

- iOS: production apps usually link static libraries; avoid custom `.dylib` on iOS.
- If offline, IDE extensions might not suggest crate versions; stick to structure/keys.
- The core now implements pixel-wise similarity (not a stub); further metrics (SSIM/PSNR) can be added as needed.
