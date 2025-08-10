#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Clock(i32); // minutes since midnight

impl Clock {
    const DAY_MINUTES: i32 = 24 * 60; // MINUTOS POR UN DIA

    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total = hours * 60 + minutes; // TOTAL DEl TIEMPO (EN MINUTOS)
        // Total =
        total = ((total % Self::DAY_MINUTES) + Self::DAY_MINUTES) % Self::DAY_MINUTES; // 1440
        Clock(total)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.0 + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.0 / 60, self.0 % 60)
    }
}

fn main() {
    // Create a clock at 10:00
    let clock1 = Clock::new(10, 0);
    println!("Clock1: {}", clock1);

    // Create a clock with overflowed minutes
    let clock2 = Clock::new(23, 90); // should wrap to 00:30
    println!("Clock2 (overflow): {}", clock2);

    // Add minutes to a clock
    let clock3 = clock1.add_minutes(125); // 10:00 + 2h 5m => 12:05
    println!("Clock3 (added minutes): {}", clock3);

    // Add negative minutes
    let clock4 = clock1.add_minutes(-70); // 10:00 - 1h 10m => 08:50
    println!("Clock4 (negative minutes): {}", clock4);

    // Check equality
    let clock5 = Clock::new(12, 5);
    println!("Clock3 == Clock5 ? {}", clock3 == clock5);
}

// cargo run --bin clock
