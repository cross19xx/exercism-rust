use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn format_hours(hrs: i32) -> i32 {
        match hrs < 0 {
            true => (24 - (hrs.abs() % 24)) % 24,
            false => hrs % 24,
        }
    }

    /// Formats the minutes and returns a tuple - the number of minutes,
    /// and hours to add to the hours hand
    fn format_minutes(mins: i32) -> (i32, i16) {
        match mins < 0 {
            true => {
                let remainder = ((mins - 59) / 60) as i16;
                let mins_left = (60 - (mins.abs() % 60)) % 60;

                (mins_left, remainder)
            }
            false => {
                let remainder = (mins / 60) as i16;
                (mins.abs() % 60, remainder)
            }
        }
    }

    pub fn new(hrs: i32, mins: i32) -> Self {
        let (minutes, hours_addition) = Clock::format_minutes(mins);
        let hours = Clock::format_hours(i32::from(hours_addition) + hrs);

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, mins: i32) -> Self {
        let (minutes, hours_addition) = Clock::format_minutes(self.minutes + mins);
        let hours = Clock::format_hours(i32::from(hours_addition) + self.hours);

        Clock::new(hours, minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{:02}:{:02}", self.hours, self.minutes)
    }
}
