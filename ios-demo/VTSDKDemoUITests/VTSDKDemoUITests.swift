import XCTest

final class VTSDKDemoUITests: XCTestCase {

    func testCompareFlowShowsJson() {
        let app = XCUIApplication()
        app.launch()

        let prepare = app.buttons["prepare-button"]
        if prepare.waitForExistence(timeout: 5) { prepare.tap() }

        let compare = app.buttons["compare-button"]
        XCTAssertTrue(compare.waitForExistence(timeout: 5))
        compare.tap()

        let output = app.otherElements["json-output"]
        XCTAssertTrue(output.waitForExistence(timeout: 5))

        // Aguarda até conter uma chave esperada no JSON
        let predicate = NSPredicate(format: "label CONTAINS[c] %@", "obtainedSimilarity")
        expectation(for: predicate, evaluatedWith: output, handler: nil)
        waitForExpectations(timeout: 5)
    }
}

