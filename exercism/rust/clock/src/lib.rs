use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let temporary = Self { hours, minutes };
        temporary.add_minutes(0)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut current_minutes = 0;
        current_minutes += self.hours * 60;
        current_minutes += self.minutes;
        current_minutes += minutes;
        current_minutes = current_minutes.rem_euclid(60 * 24);
        let new_hours = current_minutes / 60;
        let new_minutes = current_minutes.rem_euclid(60);
        Self {
            hours: new_hours,
            minutes: new_minutes,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}", self.hours, self.minutes))
    }
}
