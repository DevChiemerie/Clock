use std::fmt;

#[derive(Debug, PartialEq, Eq)]

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let h = (total_minutes / 60).rem_euclid(24);
        let m = total_minutes.rem_euclid(60);

        Self {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Return a new Clock with minutes added (pure, no mutation)
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn main() {
    let clock = Clock::new(20, 60).add_minutes(45);

    println!("Clock time: {}", clock);
}
