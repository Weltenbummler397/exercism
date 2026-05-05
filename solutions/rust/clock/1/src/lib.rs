#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let normalized_minutes = ((total_minutes % 1440) + 1440) % 1440;
        let new_hours = normalized_minutes / 60;
        let new_minutes = normalized_minutes % 60;
        Clock { hours: new_hours, minutes: new_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.hours * 60 + self.minutes + minutes;
        let normalized_minutes = ((total_minutes % 1440) + 1440) % 1440;
        let new_hours = normalized_minutes / 60;
        let new_minutes = normalized_minutes % 60;
        Clock::new(new_hours, new_minutes)
    }
}
impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
