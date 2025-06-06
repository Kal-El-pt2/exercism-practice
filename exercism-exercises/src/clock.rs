pub struct Clock{
    hrs: i32,
    mins: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{
            hrs: hours,
            mins: minutes,
        }
    }

    pub fn add_minutes(&self, mins: i32) -> Self {
        let total_mins = self.hrs*60 + self.mins + mins;
        let mut hours = (total_mins/60)/24;
        let mut minutes = total_mins%60;

        if (minutes < 0) {
            minutes += 60;
            hours -= 1;
        }
        if (hours < 0) {
            hours = (hours+24)%24;
        }

        Clock {
            hrs: hours,
            mins:minutes
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}",self.hours,self.mins)
    }
}
