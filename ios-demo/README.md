# iOS Demo

SwiftUI sample app that consumes the SDK via FFI using the `VTSDK.xcframework` in `dist/` and the header `ffi/include/vt_sdk.h`.

## How to open and run

Preferred (via SPM):
- In Xcode, File → Add Packages… → enter the repo URL and choose a tag (vX.Y.Z). CI keeps the URL/checksum updated in `Package.swift` on each release.
- Add the `VTSDK` product to the `VTSDKDemo` target.
- Build/run on a simulator or device.

Local/manual (fallback):
1) Generate the XCFramework locally: `dist/VTSDK.xcframework`.
2) Open `ios-demo/VTSDKDemo.xcodeproj` in Xcode.
3) Add `VTSDK.xcframework` to “Frameworks, Libraries, and Embedded Content” (Do Not Embed for static).
4) Select your `Team` under the `VTSDKDemo` target (Signing & Capabilities) and run.

The "Run FFI" button calls `vt_compare_images` and shows the returned JSON.

Included examples
- The app generates sample images under the temporary directory and executes:
  - `vt_compare_images` (baseline vs. input)
  - `vt_flex_search` (child within parent)
  - `vt_flex_locate` (main and relative within container)
  The JSON responses are rendered in a formatted view.

Visualization and comparison
- You can pick two images from the Photo Library (buttons “Choose Baseline” and “Choose Input”).
- Images are shown side-by-side; tap “Compare selected” to run `vt_compare_images` with those files.
- The returned similarity is highlighted and the full JSON is shown below.

## UI Tests (template)

- Extra test scheme: `ios-demo/VTSDKDemo.xcodeproj/xcshareddata/xcschemes/VTSDKDemo-UI.xcscheme` (Debug)
- Add an iOS UI Testing target named `VTSDKDemoUITests` and include `ios-demo/VTSDKDemoUITests/VTSDKDemoUITests.swift`.
- Run the tests using the `VTSDKDemo-UI` scheme; the test drives the sample UI and validates the JSON output.

Main structure:
- Project: `ios-demo/VTSDKDemo.xcodeproj`
- Sources: `ios-demo/VTSDKDemo/`
- Bridging header: `ios-demo/VTSDKDemo/VTSDKDemo-Bridging-Header.h`
- Assets: `ios-demo/VTSDKDemo/Assets.xcassets`
