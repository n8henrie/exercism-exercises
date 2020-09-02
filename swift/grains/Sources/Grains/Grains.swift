struct Grains {
    static var total = UInt64.max
    
    enum GrainsError: Error {
        case inputTooLow(message: String)
        case inputTooHigh(message: String)
    }
    
    static func square(_ num: Int) throws -> UInt64 {
        switch num {
        case Int.min...0:
            throw GrainsError.inputTooLow(message: "Input[\(num)] invalid. Input should be between 1 and 64 (inclusive)")
        case 65...Int.max:
            throw GrainsError.inputTooHigh(message: "Input[\(num)] invalid. Input should be between 1 and 64 (inclusive)")
        default:
            return (1..<num).reduce(1) { (first, _) in first * 2 }
        }
    }
}
