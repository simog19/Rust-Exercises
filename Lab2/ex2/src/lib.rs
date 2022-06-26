use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Clock{
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {minutes: hours*60 + minutes}.handle_overflow()
    }

    fn handle_overflow(&self) -> Self {
        let new_minutes =
            if self.minutes >= 0 {
                self.minutes % (60*24)
            }
            else {
                self.minutes % (60*24) + 1440
            };

        Clock{minutes: new_minutes}
    }

    //bonus task: overloading +/- operator in order to sum/sub minutes from Clock object, to be used instead of add_minutes
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock{minutes: self.minutes + minutes}.handle_overflow()
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.minutes/60, self.minutes%60)
    }
}

impl Sub for Clock{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self{
            minutes: self.minutes-other.minutes
        }.handle_overflow()
    }
}
impl Add for Clock{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output{
        Self{
            minutes: self.minutes+other.minutes
        }.handle_overflow()
    }
}

impl PartialEq for Clock{
    fn eq(&self, other: &Self) -> bool {
        self.minutes==other.minutes
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.to_string())
    }
}