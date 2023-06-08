pub use chrono::Weekday as wd;
use chrono::NaiveDate;
use chrono::Datelike;


// this function will return the middle day of the year passed as argument or an error
pub fn middle_day(year: i32) -> Option<wd> {
    if NaiveDate::from_ymd_opt(year, 2, 29).is_some() { // is some is true if the date is valid
        return None;
    } else {
        // find the week day of the 2nd of July of the year passed as argument
        return Some(NaiveDate::from_ymd_opt(year, 7, 2)?.weekday());
    }
    
}