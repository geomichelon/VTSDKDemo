# Guide: Installing and Using in UI Tests (iOS and Android)

This guide explains how to build the SDK libraries (Rust FFI) and integrate them into iOS (Xcode/XCUITest) and Android (Instrumentation/Espresso) projects, with practical examples.

## Overview

- Artifacts produced by the `ffi` crate:
  - iOS: static library (`libvt_sdk_ffi.a`) packaged as `dist/VTSDK.xcframework`.
  - Android: shared libraries (`.so`) per ABI (`arm64-v8a`, `armeabi-v7a`, `x86_64`).
- Public C header: `ffi/include/vt_sdk.h`.
- Exposed functions return JSON as `const char*`; free with `vt_free_string`:
  - `vt_compare_images(...)`
  - `vt_flex_search(...)`
  - `vt_flex_locate(...)`

## Building the binaries

Prereqs: `rustup`, iOS/Android toolchains, Xcode (iOS) and Android NDK (Android).

### iOS (device + simulator)

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
cargo build -p vt-sdk-ffi --release --target aarch64-apple-ios
cargo build -p vt-sdk-ffi --release --target aarch64-apple-ios-sim
cargo build -p vt-sdk-ffi --release --target x86_64-apple-ios
# Package XCFramework (device + simulator fat lib)
lipo -create -output dist/libvt_sdk_ffi_sim.a \
  target/aarch64-apple-ios-sim/release/libvt_sdk_ffi.a \
  target/x86_64-apple-ios/release/libvt_sdk_ffi.a
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libvt_sdk_ffi.a -headers ffi/include \
  -library dist/libvt_sdk_ffi_sim.a -headers ffi/include \
  -output dist/VTSDK.xcframework
```

Outputs:

- `dist/VTSDK.xcframework`
- `ffi/include/vt_sdk.h`

### Android (multiple ABIs)

Set NDK path (e.g., `~/Library/Android/sdk/ndk/25.2.9519653`) in `ANDROID_NDK_HOME`.

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
cargo build -p vt-sdk-ffi --release --target aarch64-linux-android   # arm64-v8a
cargo build -p vt-sdk-ffi --release --target armv7-linux-androideabi # armeabi-v7a
cargo build -p vt-sdk-ffi --release --target x86_64-linux-android    # x86_64
```

Outputs:

- `target/<triple>/release/libvt_sdk_ffi.so`

Note (mock mode): add `--no-default-features --features mock` to build mock libraries.

---

## iOS integration (Xcode + XCUITest)

Two common approaches:

1) Drive via UI (recommended in this sample): UI tests tap buttons and validate the JSON shown.
2) Call the FFI directly from the UI tests target (via bridging header) for direct validation.

### Steps (UI-driven)

1. Use the sample project `ios-demo/VTSDKDemo.xcodeproj` and scheme `VTSDKDemo-UI`.
2. Add a UI Testing target if needed: Xcode → File → New → Target… → iOS → UI Testing Bundle → `VTSDKDemoUITests`.
3. Add `ios-demo/VTSDKDemoUITests/VTSDKDemoUITests.swift` to the test target.
4. Select a simulator and run the `VTSDKDemo-UI` scheme on the UI Tests destination.

The test opens the app, taps “Prepare sample images” and “vt_compare_images”, and waits until the JSON shows `obtainedSimilarity`.

---

## Android integration (Instrumentation/Espresso)

The API is C-based; call it via JNI. A simple approach is a thin JNI shim in C/C++ delegating to `vt_compare_images`.

### Steps

1. Copy the generated `.so` files into your app:
   - `app/src/main/jniLibs/arm64-v8a/libvt_sdk_ffi.so`
   - `app/src/main/jniLibs/armeabi-v7a/libvt_sdk_ffi.so`
   - `app/src/main/jniLibs/x86_64/libvt_sdk_ffi.so`

2. Optionally add the header for reference: copy `ffi/include/vt_sdk.h` to `app/src/main/cpp/include/`.

3. Create a JNI shim (e.g., `app/src/main/cpp/vtsdk_jni.cpp`):

```cpp
#include <jni.h>
#include <string>
#include "vt_sdk.h" // adjust includePath in CMake

extern "C" JNIEXPORT jstring JNICALL
Java_com_example_vtsdk_VtSdkFFI_vtCompareImages(
        JNIEnv* env, jclass,
        jstring jbaseline, jstring jinput, jint jminSim, jint jnoise,
        jstring jexcluded, jstring jmeta) {
    const char* b = env->GetStringUTFChars(jbaseline, nullptr);
    const char* i = env->GetStringUTFChars(jinput, nullptr);
    const char* ex = env->GetStringUTFChars(jexcluded, nullptr);
    const char* me = env->GetStringUTFChars(jmeta, nullptr);

    const char* out = vt_compare_images(b, i, (int32_t)jminSim, (int32_t)jnoise, ex, me);

    env->ReleaseStringUTFChars(jbaseline, b);
    env->ReleaseStringUTFChars(jinput, i);
    env->ReleaseStringUTFChars(jexcluded, ex);
    env->ReleaseStringUTFChars(jmeta, me);

    jstring result = env->NewStringUTF(out ? out : "{}");
    vt_free_string(out);
    return result;
}
```

4. Enable CMake in `app/build.gradle`:

```groovy
android {
  defaultConfig { externalNativeBuild { cmake { cppFlags "-std=c++17" } } }
  externalNativeBuild { cmake { path file("CMakeLists.txt") } }
  // Ensure jniLibs ABI directories are packaged
}
```

5. Expose a Kotlin API for tests:

```kotlin
package com.example.vtsdk

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

6. Use it in Instrumented Tests (Espresso):

```kotlin
@RunWith(AndroidJUnit4::class)
class UiTests {
    @Test fun compareExample() {
        val json = VtSdkFFI.vtCompareImages(
            "baseline.png", "input.png", 50, 20, "[]", "{\"testName\":\"UI-Compare\"}")
        assert(json.isNotEmpty())
    }
}
```

Notes:

- The JNI shim builds per-ABI and delegates to the Rust `.so`, packaged in `jniLibs`.
- You may also compile the shim as part of the app module.

---

## Real vs. mock mode

- `real` (default): uses the `core` implementation.
- `mock`: returns simulated responses; useful to integrate pipeline/UI before real logic.
- Build iOS/Android libraries with the desired features.

Examples:

```bash
# iOS mock
cargo build -p vt-sdk-ffi --release --no-default-features --features mock --target aarch64-apple-ios

# Android mock (arm64)
cargo build -p vt-sdk-ffi --release --no-default-features --features mock --target aarch64-linux-android
```

---

## Best practices and troubleshooting

- Always free strings returned by the SDK with `vt_free_string` after copying.
- Ensure images/URLs used in tests are accessible from the app/test sandbox.
- Xcode Preview requires Debug scheme (`-Onone`).
- On Android, make sure app/test ABIs match included `.so` files.
- iOS linking issues: ensure XCFramework is in “Link Binary With Libraries” for the correct target and Header Search Paths include `ffi/include`.
- Android `UnsatisfiedLinkError`: verify `System.loadLibrary("vtsdk_shim")` and `.so` presence in `jniLibs/<ABI>/`.

---

## Where to look in this repo

- C header: `ffi/include/vt_sdk.h`
- Example XCFramework: `dist/VTSDK.xcframework`
- iOS sample app: `ios-demo/VTSDKDemo.xcodeproj`
- UI Tests example: `ios-demo/VTSDKDemoUITests/VTSDKDemoUITests.swift`
