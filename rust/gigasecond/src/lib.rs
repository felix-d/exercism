extern crate chrono;
use chrono::*;

pub fn after(start_date: DateTime<UTC>) -> DateTime<UTC> {
    let base: i64 = 10;
    let duration = Duration::seconds(base.pow(9));
    start_date + duration
}


