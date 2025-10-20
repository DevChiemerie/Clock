use std::fmt;

#[derive(Debug, PartialEq, Eq)]

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes).rem_euclid(24 * 60);

        Self {
            hours: total_minutes / 60,
            minutes: total_minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Return a new Clock with minutes added (pure, no mutation)
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn difference_in_minutes(&self, other: &Clock) -> i32 {
        let self_total = self.hours * 60 + self.minutes;
        let other_total = other.hours * 60 + other.minutes;
        (self_total - other_total).rem_euclid(24 * 60)
    }

    pub fn add_hours(&self, hours: i32) -> Self {
        Clock::new(self.hours + hours, self.minutes)
    }

    pub fn from_minutes(minutes: i32) -> Self {
        Clock::new(0, minutes)
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

    let c1 = Clock::new(23, 30).add_minutes(90);
    println!("Clock 1: {}", c1);

    let c2 = Clock::from_minutes(150);
    println!("Clock 2: {}", c2);

    let c3 = Clock::new(10, 0).add_hours(5);
    println!("Clock 3: {}", c3);

    let diff = c1.difference_in_minutes(&c3);
    println!(
        "Difference in minutes between Clock 1 and Clock 3: {}",
        diff
    );
}
