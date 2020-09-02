func toLimit(_ limit: Int, inMultiples: [Int]) -> Int {
    if limit < 2 {
        return 0
    }
    var multiples = Set<Int>()
    for mult in inMultiples where mult > 0 {
        for i in stride(from:mult, to:limit, by: mult) {
            multiples.update(with: i)
        }
    }
    return multiples.reduce(0, +)
}
