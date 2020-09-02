use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let min_div = minutes / 60;
        let min_mod = minutes % 60;
        let h_mod = (hours + min_div) % 24 - ((min_mod < 0) as i32);
        Clock {
            hours: Self::mod_euc(h_mod, 24),
            minutes: Self::mod_euc(min_mod, 60),
        }
    }

    fn mod_euc(lhs: i32, rhs: i32) -> i32 {
        let r = lhs % rhs;
        if r < 0 {
            if rhs < 0 {
                r - rhs
            } else {
                r + rhs
            }
        } else {
            r
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
