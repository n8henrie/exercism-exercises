// `/ ```text
// `n every year that is evenly divisible by 4
// ` except every year that is evenly divisible by 100
// `   unless the year is also evenly divisible by 400
// ```
pub fn is_leap_year(year: i32) -> bool {
    match year {
        _ if year % 400 == 0 => true,
        _ if year % 100 == 0 => false,
        _ if year % 4 == 0 => true,
        _ => false,
        }
}
