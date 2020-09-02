struct Hamming {
    static func compute(_ first: String, against second: String) -> Int? {
        if first.count != second.count {
            return nil
        }
        return zip(first, second).reduce(0) { (acc, zipped) in zipped.0 == zipped.1 ? acc : 1 + acc }
    }
}
