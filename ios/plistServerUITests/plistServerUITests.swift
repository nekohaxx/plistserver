
import XCTest

final class plistServerUITests: XCTestCase {

    override func setUpWithError() throws {
  
        continueAfterFailure = false

    
    }

    override func tearDownWithError() throws {
       
    }

    @MainActor
    func testExample() throws {
       
        let app = XCUIApplication()
        app.launch()

        
    }

    @MainActor
    func testLaunchPerformance() throws {
        
        measure(metrics: [XCTApplicationLaunchMetric()]) {
            XCUIApplication().launch()
        }
    }
}
