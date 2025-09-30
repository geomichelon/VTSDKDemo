# iOS Demo

SwiftUI sample app that consumes the SDK via FFI using the `VTSDK.xcframework` in `dist/` and the header `ffi/include/vt_sdk.h`.

## How to open and run

1) Generate the XCFramework: `dist/VTSDK.xcframework`.
2) Open `ios-demo/VTSDKDemo.xcodeproj` in Xcode.
3) Select your `Team` under the `VTSDKDemo` target (Signing & Capabilities).
4) Select a simulator or device and build/run.

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
