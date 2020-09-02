struct Bob {
    static func hey(_ input: String) -> String {
        switch input {
        case _ where input.allSatisfy(\.isWhitespace):
            return "Fine. Be that way!"
        case _ where input == input.uppercased() && input.contains(where: \.isLetter):
            return "Whoa, chill out!"            
        case _ where input.hasSuffix("?"):
            return "Sure."
        default:
            return "Whatever."
        }
    }
}
