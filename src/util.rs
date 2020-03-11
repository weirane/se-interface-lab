use crate::errors::Errors;
use chrono::naive::NaiveDate;
use chrono::Datelike;

pub fn parse_date(date: &str) -> Result<(i32, u32, u32), Errors> {
    date.parse::<NaiveDate>()
        .map(|d| (d.year(), d.month(), d.day()))
        .map_err(|_| Errors::InvalidDate)
}
