# Android Package (AAR) Consumption

This document explains how to consume the Android AAR published from this repository, and alternatives for local development.

## Coordinates (GitHub Packages)

- Group: `com.geomichelon`
- Artifact: `vt-sdk-android`
- Version: matches Git tags (e.g., `v0.2.0`) — use without the leading `v` if desired by your version scheme.

Example dependency:

```groovy
dependencies {
  implementation("com.geomichelon:vt-sdk-android:0.2.0")
}
```

## Gradle setup for GitHub Packages

Add GitHub Packages as a Maven repo with credentials. In `settings.gradle` or `build.gradle` (root):

```groovy
dependencyResolutionManagement {
  repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
  repositories {
    google()
    mavenCentral()
    maven {
      name = "GitHubPackages"
      url = uri("https://maven.pkg.github.com/geomichelon/VTSDKDemo")
      credentials {
        username = System.getenv("GITHUB_ACTOR") ?: project.findProperty("gpr.user")
        password = System.getenv("GITHUB_TOKEN") ?: project.findProperty("gpr.key")
      }
    }
  }
}
```

In `~/.gradle/gradle.properties` you can set:

```
gpr.user=YOUR_GITHUB_USERNAME
gpr.key=YOUR_GITHUB_TOKEN
```

The `GITHUB_TOKEN` should have `read:packages` scope.

## What the AAR includes

- JNI shim (`libvtsdk_shim.so`) compiled for `arm64-v8a`, `armeabi-v7a`, `x86_64`.
- Rust FFI (`libvt_sdk_ffi.so`) for the same ABIs (built in CI and packaged into `jniLibs`).
- Kotlin wrapper `com.geomichelon.vtsdk.VtSdkFFI` with `vtCompareImages(...)` returning JSON.

## Usage in code

```kotlin
import com.geomichelon.vtsdk.VtSdkFFI

val json = VtSdkFFI.vtCompareImages(
  baseline = "/path/to/baseline.png",
  input = "/path/to/input.png",
  minSim = 95,
  noise = 20,
  excludedJson = "[]",
  metaJson = "{\"testName\":\"Android-Compare\"}"
)
```

Parse the JSON to extract `obtainedSimilarity`, `status`, and `resultImageRef`.

## Local development alternative

- Build Rust `.so` locally and drop them under `app/src/main/jniLibs/<ABI>/` (as done in the sample app) and use the sample JNI shim.
- Or reference a locally built AAR in `libs/` and add `implementation(files("libs/vt-sdk-android-release.aar"))`.

## CI publishing

- Workflow `.github/workflows/android-aar.yml` builds Rust `.so`, assembles the AAR, and on tags publishes to GitHub Packages.

