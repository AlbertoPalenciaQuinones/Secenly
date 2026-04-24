/*use rasn::{AsnType, Encode, Decode};
use chrono::{Datelike, Timelike};

#[derive(AsnType, Encode, Decode, Debug)]
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Date {
    pub fn new(day: u8, month: u8, year: u16, hour: u8, minute: u8, second: u8) -> Date {
        Date { day, month, year, hour, minute, second }
    }

    pub fn get_actual_date() -> Date {
        let now = chrono::Utc::now();
        Date {
            day: now.day() as u8,
            month: now.month() as u8,
            year: now.year() as u16,
            hour: now.hour() as u8,
            minute: now.minute() as u8,
            second: now.second() as u8,
        }
    }

    pub fn cast_date() {

    }
}*/