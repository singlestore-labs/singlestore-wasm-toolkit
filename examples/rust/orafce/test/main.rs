use chrono::{
    DateTime, Datelike, Days, Duration, Local, Months, NaiveDate, NaiveDateTime, Timelike, Weekday,
};

const DEFAULT_FMT: &str = "%Y-%m-%d";
const DEFAULT_TIME_FMT: &str = "%Y-%m-%d %H:%M:%S";
const DEFAULT_TZ_FORMAT: &str = "%Y-%m-%d %H:%M:%S%z";
const MDY_TIME_FMT: &str = "%m/%d/%y %H:%M:%S";
// ISO 8601 format: %Y-%m-%dT%H:%M:%S%.f%:z
const ISO8601_FMT: &str = "%+";

fn main() {
    let iso_test_1 = "1999-10-03T23:59:59.123456789-0700";
    let iso_dt = DateTime::parse_from_str(iso_test_1, ISO8601_FMT).unwrap();
    let utc_dt = iso_dt.naive_utc();
    let local_dt = iso_dt.naive_local();
    println!("{} {} {}", iso_dt.timezone(), utc_dt, local_dt);
}
