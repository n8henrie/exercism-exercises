extension Int {
    func squared() -> Int {
        return self * self
    }
}

class Squares {
    let count: Int
    lazy var squareOfSum: Int = (self.count * (self.count + 1) / 2).squared()
    lazy var sumOfSquares = count * (count + 1) * (2 * count + 1) / 6
    lazy var differenceOfSquares: Int = self.squareOfSum - self.sumOfSquares
    init(_ count: Int) {
        self.count = count
    }
}
