import Foundation
import Network

class IOSClient {
    private let monitor = NWPathMonitor()
    private var currentType: String = "Unknown"
    
    init() {
        monitor.pathUpdateHandler = { path in
            if path.usesInterfaceType(.wifi) {
                self.currentType = "WiFi"
            } else if path.usesInterfaceType(.cellular) {
                self.currentType = "Cellular"
            } else if path.usesInterfaceType(.wiredEthernet) {
                self.currentType = "Ethernet"
            } else {
                self.currentType = "Other"
            }
        }
        let queue = DispatchQueue(label: "NetworkMonitor")
        monitor.start(queue: queue)
    }
    
    func networkType() -> String {
        return currentType
    }
}