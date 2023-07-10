use chrono::{DateTime, Datelike, Days, Duration, DurationRound, Months, Timelike};

wit_bindgen_rust::export!("orafce.wit");

struct Orafce;

//const DEFAULT_FMT: &str = "%Y_%m_%d";
//const DEFAULT_TIME_FMT: &str = "%Y_%m_%d %H:%M:%S";
//const DEFAULT_TZ_FORMAT: &str = "%Y_%m_%d %H:%M:%S%z";
//const MDY_TIME_FMT: &str = "%m/%d/%y %H:%M:%S";
//const NLS_DATE_FMT: &str = "%y_%b%d %H:%M:%S";

impl crate::orafce::Orafce for Orafce {
    // Retrieves the Date without an associated timezone
    fn tsz_from_fmt_get_date(ts: String, fmt: String) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .date_naive()
            .to_string()
    }

    // Retrieves a time component.
    fn tsz_from_fmt_get_time(ts: String, fmt: String) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .time()
            .to_string()
    }

    // Returns the number of non-leap (milli/micro/nano)seconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”).
    fn tsz_from_fmt_timestamp(ts: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt).unwrap().timestamp()
    }

    // Note that this does reduce the number of years that can be represented from ~584 Billion to ~584 Million.
    fn tsz_from_fmt_timestamp_millis(ts: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .timestamp_millis()
    }
    fn tsz_from_fmt_timestamp_micros(ts: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .timestamp_micros()
    }
    fn tsz_from_fmt_timestamp_nanos(ts: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .timestamp_nanos()
    }

    // Returns the number of milli/micro/nano seconds since the last second boundary
    // warning: in event of a leap second, this may exceed 999
    fn tsz_from_fmt_timestamp_subsec_millis(ts: String, fmt: String) -> u32 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .timestamp_subsec_millis()
    }
    fn tsz_from_fmt_timestamp_subsec_micros(ts: String, fmt: String) -> u32 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .timestamp_subsec_micros()
    }
    fn tsz_from_fmt_timestamp_subsec_nanos(ts: String, fmt: String) -> u32 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .timestamp_subsec_nanos()
    }

    // Adds given months/weeks/days/hours/minutes/(milli, micro, nano)seconds to the current date and time.
    // Error when it will result in overflow, or if the local time is not valid on the newly calculated date.
    fn tsz_from_fmt_add_months(ts: String, fmt: String, months: i32) -> String {
        if months >= 0 {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_add_months(Months::new(months as u32))
                .unwrap()
                .to_string()
        } else {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_sub_months(Months::new(-months as u32))
                .unwrap()
                .to_string()
        }
    }
    fn tsz_from_fmt_add_weeks(ts: String, fmt: String, weeks: i64) -> String {
        if weeks >= 0 {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_add_signed(Duration::weeks(weeks))
                .unwrap()
                .to_string()
        } else {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_sub_signed(Duration::weeks(-weeks))
                .unwrap()
                .to_string()
        }
    }
    fn tsz_from_fmt_add_days(ts: String, fmt: String, days: i64) -> String {
        if days >= 0 {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_add_days(Days::new(days as u64))
                .unwrap()
                .to_string()
        } else {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_sub_days(Days::new(-days as u64))
                .unwrap()
                .to_string()
        }
    }
    fn tsz_from_fmt_add_hours(ts: String, fmt: String, hours: i64) -> String {
        if hours >= 0 {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_add_signed(Duration::hours(hours))
                .unwrap()
                .to_string()
        } else {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_sub_signed(Duration::hours(-hours))
                .unwrap()
                .to_string()
        }
    }
    fn tsz_from_fmt_add_minutes(ts: String, fmt: String, minutes: i64) -> String {
        if minutes >= 0 {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_add_signed(Duration::minutes(minutes))
                .unwrap()
                .to_string()
        } else {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_sub_signed(Duration::minutes(-minutes))
                .unwrap()
                .to_string()
        }
    }
    fn tsz_from_fmt_add_seconds(ts: String, fmt: String, seconds: i64) -> String {
        if seconds >= 0 {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_add_signed(Duration::seconds(seconds))
                .unwrap()
                .to_string()
        } else {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_sub_signed(Duration::seconds(-seconds))
                .unwrap()
                .to_string()
        }
    }
    fn tsz_from_fmt_add_milliseconds(ts: String, fmt: String, milliseconds: i64) -> String {
        if milliseconds >= 0 {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_add_signed(Duration::milliseconds(milliseconds))
                .unwrap()
                .to_string()
        } else {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_sub_signed(Duration::milliseconds(-milliseconds))
                .unwrap()
                .to_string()
        }
    }
    fn tsz_from_fmt_add_microseconds(ts: String, fmt: String, microseconds: i64) -> String {
        if microseconds >= 0 {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_add_signed(Duration::microseconds(microseconds))
                .unwrap()
                .to_string()
        } else {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_sub_signed(Duration::microseconds(-microseconds))
                .unwrap()
                .to_string()
        }
    }
    fn tsz_from_fmt_add_nanoseconds(ts: String, fmt: String, nanoseconds: i64) -> String {
        if nanoseconds >= 0 {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_add_signed(Duration::nanoseconds(nanoseconds))
                .unwrap()
                .to_string()
        } else {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .checked_sub_signed(Duration::nanoseconds(-nanoseconds))
                .unwrap()
                .to_string()
        }
    }
    fn tsz_from_fmt_years_since(ts: String, origints: String, fmt: String) -> u32 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .years_since(DateTime::parse_from_str(&origints, &fmt).unwrap())
            .unwrap()
    }

    fn tsz_from_fmt_weeks_since(ts: String, origints: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .signed_duration_since(DateTime::parse_from_str(&origints, &fmt).unwrap())
            .num_weeks()
    }
    fn tsz_from_fmt_days_since(ts: String, origints: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .signed_duration_since(DateTime::parse_from_str(&origints, &fmt).unwrap())
            .num_days()
    }
    fn tsz_from_fmt_hours_since(ts: String, origints: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .signed_duration_since(DateTime::parse_from_str(&origints, &fmt).unwrap())
            .num_hours()
    }
    fn tsz_from_fmt_minutes_since(ts: String, origints: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .signed_duration_since(DateTime::parse_from_str(&origints, &fmt).unwrap())
            .num_minutes()
    }
    fn tsz_from_fmt_seconds_since(ts: String, origints: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .signed_duration_since(DateTime::parse_from_str(&origints, &fmt).unwrap())
            .num_seconds()
    }
    fn tsz_from_fmt_milliseconds_since(ts: String, origints: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .signed_duration_since(DateTime::parse_from_str(&origints, &fmt).unwrap())
            .num_milliseconds()
    }
    fn tsz_from_fmt_microseconds_since(ts: String, origints: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .signed_duration_since(DateTime::parse_from_str(&origints, &fmt).unwrap())
            .num_microseconds()
            .unwrap()
    }
    fn tsz_from_fmt_nanoseconds_since(ts: String, origints: String, fmt: String) -> i64 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .signed_duration_since(DateTime::parse_from_str(&origints, &fmt).unwrap())
            .num_nanoseconds()
            .unwrap()
    }
    fn tsz_from_fmt_to_utc(ts: String, fmt: String) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .naive_utc()
            .to_string()
    }
    fn tsz_from_fmt_year(ts: String, fmt: String) -> i32 {
        DateTime::parse_from_str(&ts, &fmt).unwrap().year()
    }
    fn tsz_from_fmt_month(ts: String, fmt: String) -> u32 {
        DateTime::parse_from_str(&ts, &fmt).unwrap().month()
    }
    fn tsz_from_fmt_day(ts: String, fmt: String) -> u32 {
        DateTime::parse_from_str(&ts, &fmt).unwrap().day()
    }
    // Returns the day of year starting from 1 (max 366)
    fn tsz_from_fmt_ordinal(ts: String, fmt: String) -> u32 {
        DateTime::parse_from_str(&ts, &fmt).unwrap().ordinal()
    }
    // returns day-of-week starting from Monday = 1
    fn tsz_from_fmt_weekday(ts: String, fmt: String) -> u32 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .weekday()
            .number_from_monday()
    }
    // Returns the ISO week number starting from 1.
    // The return value ranges from 1 to 53. (The last week of year differs by years.)
    fn tsz_from_fmt_isoweek(ts: String, fmt: String) -> u32 {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .iso_week()
            .week()
    }

    // Makes a new value with the year / month / day / ordinal / hour / minute / (nano)second number changed
    fn tsz_from_fmt_change_year(ts: String, fmt: String, year: i32) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .with_year(year)
            .unwrap()
            .to_string()
    }

    fn tsz_from_fmt_change_month(ts: String, fmt: String, month: u32) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .with_month(month)
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_change_day(ts: String, fmt: String, day: u32) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .with_day(day)
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_change_ordinal(ts: String, fmt: String, ordinal: u32) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .with_ordinal(ordinal)
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_change_hour(ts: String, fmt: String, hour: u32) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .with_hour(hour)
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_change_minute(ts: String, fmt: String, minute: u32) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .with_minute(minute)
            .unwrap()
            .to_string()
    }
    // As with the second method, the input range is restricted to 0 through 59.
    fn tsz_from_fmt_change_second(ts: String, fmt: String, second: u32) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .with_second(second)
            .unwrap()
            .to_string()
    }

    // As with the nanosecond method, the input range can exceed 1,000,000,000 for leap seconds.
    fn tsz_from_fmt_change_nanosecond(ts: String, fmt: String, nanosecond: u32) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .with_nanosecond(nanosecond)
            .unwrap()
            .to_string()
    }

    // Return a copy rounded by (week / day / hour / minute / (milli, micro) second)
    fn tsz_from_fmt_round_by_week(ts: String, fmt: String, week: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_round(Duration::weeks(week))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_round_by_day(ts: String, fmt: String, day: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_round(Duration::days(day))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_round_by_hour(ts: String, fmt: String, hour: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_round(Duration::hours(hour))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_round_by_minute(ts: String, fmt: String, minute: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_round(Duration::minutes(minute))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_round_by_second(ts: String, fmt: String, second: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_round(Duration::seconds(second))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_round_by_millisecond(ts: String, fmt: String, milliseconds: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_round(Duration::milliseconds(milliseconds))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_round_by_microsecond(ts: String, fmt: String, microseconds: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_round(Duration::microseconds(microseconds))
            .unwrap()
            .to_string()
    }

    // Return a copy trunced by (week / day / hour / minute / (milli, micro) second)
    fn tsz_from_fmt_trunc_by_week(ts: String, fmt: String, week: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_trunc(Duration::weeks(week))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_trunc_by_day(ts: String, fmt: String, day: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_trunc(Duration::days(day))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_trunc_by_hour(ts: String, fmt: String, hour: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_trunc(Duration::hours(hour))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_trunc_by_minute(ts: String, fmt: String, minute: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_trunc(Duration::minutes(minute))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_trunc_by_second(ts: String, fmt: String, second: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_trunc(Duration::seconds(second))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_trunc_by_millisecond(ts: String, fmt: String, milliseconds: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_trunc(Duration::milliseconds(milliseconds))
            .unwrap()
            .to_string()
    }
    fn tsz_from_fmt_trunc_by_microsecond(ts: String, fmt: String, microseconds: i64) -> String {
        DateTime::parse_from_str(&ts, &fmt)
            .unwrap()
            .duration_trunc(Duration::microseconds(microseconds))
            .unwrap()
            .to_string()
    }
}
