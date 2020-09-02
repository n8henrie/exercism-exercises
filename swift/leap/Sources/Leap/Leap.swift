struct Year {
    let isLeapYear: Bool
    init(calendarYear: Int) {
        if calendarYear.isMultiple(of: 400) {
            self.isLeapYear = true
        } else if calendarYear.isMultiple(of: 100) {
            self.isLeapYear = false
        } else  if calendarYear.isMultiple(of: 4) {
            self.isLeapYear = true
        } else  {
            self.isLeapYear = false
        }
    }
}
