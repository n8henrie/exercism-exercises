enum TranscriptionError: Error {
    case invalidNucleotide(message: String)
}

struct Nucleotide {
    var nt: String
    init(_ nt: String) {
        self.nt = nt
    }
    func complementOfDNA() throws -> String {
        var result: String = ""
        for x in nt {
            switch x {
            case "G":
                result += "C"
            case "C":
                result +=  "G"
            case "T":
                result +=  "A"
            case "A":
                result +=  "U"
            default:
                throw TranscriptionError.invalidNucleotide(message: "\(x) is not a valid Nucleotide")
            }
        }
        return result
    }
}
