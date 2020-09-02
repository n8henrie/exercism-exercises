import Foundation

fileprivate extension TimeZone {
  static let utc = TimeZone(secondsFromGMT: 0)
}

fileprivate extension TimeInterval {
  static let oneBillion = 1_000_000_000.0
}

struct Gigasecond {
    let description: String

    private static let formatter: DateFormatter = {
        let df = DateFormatter()
        df.dateFormat = "yyyy-MM-dd'T'HH:mm:ss"
        df.timeZone = TimeZone.utc
        return df
    }()
    
    init?(from dateString: String) {
        guard let date = Gigasecond.formatter.date(from: dateString)
          else {
            return nil
        }
        self.description = Gigasecond.formatter.string(from: date.addingTimeInterval(.oneBillion))
    }
}
