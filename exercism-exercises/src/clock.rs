#[derive(Debug, PartialEq, Eq)]
pub struct Clock{
    hrs: i32,
    mins: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * 60 + minutes).rem_euclid(24 * 60);
        Clock{
            hrs: total_minutes / 60,
            mins: total_minutes % 60,
        }
    }

    pub fn add_minutes(&self, mins: i32) -> Self {
        Clock::new(self.hrs, self.mins + mins)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hrs, self.mins)
    }
}
