use chrono::{DateTime, Datelike, Duration, Months, NaiveDateTime, Timelike, Utc};
use chrono_tz::Tz;
use itertools::Itertools;

wit_bindgen_rust::export!("tsz.wit");

struct Tsz;

fn contains_tz(fmt: &str) -> bool {
    if fmt.contains("%z")
        || fmt.contains("%Z")
        || fmt == "%+"
        || fmt.contains("%:z")
        || fmt.contains("%::z")
        || fmt.contains("%:::z")
        || fmt.contains("%#z")
    {
        true
    } else {
        false
    }
}

fn year_fmt(ts: &str, fmt: &str) -> u32 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(ts, fmt).unwrap();
        dt.year() as u32
    } else {
        let dt = NaiveDateTime::parse_from_str(ts, fmt).unwrap();
        dt.year() as u32
    }
}

fn month_fmt(ts: &str, fmt: &str) -> u32 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(ts, fmt).unwrap();
        dt.month()
    } else {
        let dt = NaiveDateTime::parse_from_str(ts, fmt).unwrap();
        dt.month()
    }
}

fn weekday_fmt(ts: &str, fmt: &str) -> u32 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(ts, fmt).unwrap();
        dt.weekday().num_days_from_monday()
    } else {
        let dt = NaiveDateTime::parse_from_str(ts, fmt).unwrap();
        dt.weekday().num_days_from_monday()
    }
}

fn day_fmt(ts: &str, fmt: &str) -> u32 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(ts, fmt).unwrap();
        dt.day()
    } else {
        let dt = NaiveDateTime::parse_from_str(ts, fmt).unwrap();
        dt.day()
    }
}

fn hour_fmt(ts: &str, fmt: &str) -> u32 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(ts, fmt).unwrap();
        dt.hour()
    } else {
        let dt = NaiveDateTime::parse_from_str(ts, fmt).unwrap();
        dt.hour()
    }
}

fn minute_fmt(ts: &str, fmt: &str) -> u32 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(ts, fmt).unwrap();
        dt.minute()
    } else {
        let dt = NaiveDateTime::parse_from_str(ts, fmt).unwrap();
        dt.minute()
    }
}

fn millisecond_fmt(ts: &str, fmt: &str) -> u32 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(ts, fmt).unwrap();
        dt.timestamp_subsec_millis()
    } else {
        let dt = NaiveDateTime::parse_from_str(ts, fmt).unwrap();
        dt.timestamp_subsec_millis()
    }
}

fn microsecond_fmt(ts: &str, fmt: &str) -> u32 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(ts, fmt).unwrap();
        dt.timestamp_subsec_micros()
    } else {
        let dt = NaiveDateTime::parse_from_str(ts, fmt).unwrap();
        dt.timestamp_subsec_micros()
    }
}

fn nanosecond_fmt(ts: &str, fmt: &str) -> u32 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(ts, fmt).unwrap();
        dt.nanosecond()
    } else {
        let dt = NaiveDateTime::parse_from_str(ts, fmt).unwrap();
        dt.nanosecond()
    }
}

fn timestampadd_tz_fmt(unit: String, num: i64, ts: String, fmt: String, res_fmt: String) -> String {
    let dt = DateTime::parse_from_str(&ts, &fmt).unwrap();
    let res = if num >= 0 {
        match unit.to_lowercase().as_str() {
            "year" => unimplemented!("year unimplemented"),
            "quarter" => unimplemented!("quarter unimplemented"),
            "month" => dt.checked_add_months(Months::new(num as u32)),
            "week" => dt.checked_add_signed(Duration::weeks(num)),
            "day" => dt.checked_add_signed(Duration::days(num)),
            "hour" => dt.checked_add_signed(Duration::hours(num)),
            "minute" => dt.checked_add_signed(Duration::minutes(num)),
            "second" => dt.checked_add_signed(Duration::seconds(num)),
            "millisecond" => dt.checked_add_signed(Duration::milliseconds(num)),
            "microsecond" => dt.checked_add_signed(Duration::microseconds(num)),
            "nanosecond" => dt.checked_add_signed(Duration::nanoseconds(num)),
            _ => unimplemented!("{} is not implemented!", &unit),
        }
    } else {
        match unit.to_lowercase().as_str() {
            "year" => unimplemented!("year unimplemented"),
            "quarter" => unimplemented!("quarter unimplemented"),
            "month" => dt.checked_sub_months(Months::new(-num as u32)),
            "week" => dt.checked_sub_signed(Duration::weeks(-num)),
            "day" => dt.checked_sub_signed(Duration::days(-num)),
            "hour" => dt.checked_sub_signed(Duration::hours(-num)),
            "minute" => dt.checked_sub_signed(Duration::minutes(-num)),
            "second" => dt.checked_sub_signed(Duration::seconds(-num)),
            "millisecond" => dt.checked_sub_signed(Duration::milliseconds(-num)),
            "microsecond" => dt.checked_sub_signed(Duration::microseconds(-num)),
            "nanosecond" => dt.checked_sub_signed(Duration::nanoseconds(-num)),
            _ => unimplemented!("{} is not implemented!", &unit),
        }
    };
    res.unwrap().format(&res_fmt).to_string()
}

fn timestampadd_naive_fmt(
    unit: String,
    num: i64,
    ts: String,
    fmt: String,
    res_fmt: String,
) -> String {
    let dt = NaiveDateTime::parse_from_str(&ts, &fmt).unwrap();
    let res = if num >= 0 {
        match unit.to_lowercase().as_str() {
            "year" => unimplemented!("year unimplemented"),
            "quarter" => unimplemented!("quarter unimplemented"),
            "month" => dt.checked_add_months(Months::new(num as u32)),
            "week" => dt.checked_add_signed(Duration::weeks(num)),
            "day" => dt.checked_add_signed(Duration::days(num)),
            "hour" => dt.checked_add_signed(Duration::hours(num)),
            "minute" => dt.checked_add_signed(Duration::minutes(num)),
            "second" => dt.checked_add_signed(Duration::seconds(num)),
            "millisecond" => dt.checked_add_signed(Duration::milliseconds(num)),
            "microsecond" => dt.checked_add_signed(Duration::microseconds(num)),
            "nanosecond" => dt.checked_add_signed(Duration::nanoseconds(num)),
            _ => unimplemented!("{} is not implemented!", &unit),
        }
    } else {
        match unit.to_lowercase().as_str() {
            "year" => unimplemented!("year unimplemented"),
            "quarter" => unimplemented!("quarter unimplemented"),
            "month" => dt.checked_sub_months(Months::new(-num as u32)),
            "week" => dt.checked_sub_signed(Duration::weeks(-num)),
            "day" => dt.checked_sub_signed(Duration::days(-num)),
            "hour" => dt.checked_sub_signed(Duration::hours(-num)),
            "minute" => dt.checked_sub_signed(Duration::minutes(-num)),
            "second" => dt.checked_sub_signed(Duration::seconds(-num)),
            "millisecond" => dt.checked_sub_signed(Duration::milliseconds(-num)),
            "microsecond" => dt.checked_sub_signed(Duration::microseconds(-num)),
            "nanosecond" => dt.checked_sub_signed(Duration::nanoseconds(-num)),
            _ => unimplemented!("{} is not implemented!", &unit),
        }
    };
    res.unwrap().format(&res_fmt).to_string()
}

fn unix_timestamp_second_fmt(ts: &str, fmt: &str) -> i64 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(&ts, &fmt).unwrap();
        dt.timestamp()
    } else {
        let dt = NaiveDateTime::parse_from_str(&ts, &fmt).unwrap();
        dt.timestamp()
    }
}

fn unix_timestamp_millis_fmt(ts: &str, fmt: &str) -> i64 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(&ts, &fmt).unwrap();
        dt.timestamp_millis()
    } else {
        let dt = NaiveDateTime::parse_from_str(&ts, &fmt).unwrap();
        dt.timestamp_millis()
    }
}

fn unix_timestamp_micros_fmt(ts: &str, fmt: &str) -> i64 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(&ts, &fmt).unwrap();
        dt.timestamp_micros()
    } else {
        let dt = NaiveDateTime::parse_from_str(&ts, &fmt).unwrap();
        dt.timestamp_micros()
    }
}

fn unix_timestamp_nanos_fmt(ts: &str, fmt: &str) -> i64 {
    if contains_tz(fmt) {
        let dt = DateTime::parse_from_str(&ts, &fmt).unwrap();
        dt.timestamp_nanos()
    } else {
        let dt = NaiveDateTime::parse_from_str(&ts, &fmt).unwrap();
        dt.timestamp_nanos()
    }
}

impl crate::tsz::Tsz for Tsz {
    fn unix_timestamp_fmt(unit: String, ts: String, fmt: String) -> i64 {
        match unit.to_lowercase().as_str() {
            "millisecond" | "milli" => unix_timestamp_millis_fmt(ts.as_str(), fmt.as_str()),
            "microsecond" | "micro" => unix_timestamp_micros_fmt(ts.as_str(), fmt.as_str()),
            "nanosecond" | "nano" => unix_timestamp_nanos_fmt(ts.as_str(), fmt.as_str()),
            _ => unix_timestamp_second_fmt(ts.as_str(), fmt.as_str()),
        }
    }

    // Adds given months/weeks/days/hours/minutes/(milli, micro, nano)seconds to the current date and time.
    // Error when it will result in overflow, or if the local time is not valid on the newly calculated date.
    fn timestampadd_fmt(
        unit: String,
        num: i64,
        ts: String,
        fmt: String,
        res_fmt: String,
    ) -> String {
        let resolved_res_fmt = match res_fmt.as_str() {
            "" => fmt.to_string(),
            _ => res_fmt.to_string(),
        };
        if contains_tz(&fmt) {
            timestampadd_tz_fmt(unit, num, ts, fmt, resolved_res_fmt)
        } else {
            timestampadd_naive_fmt(unit, num, ts, fmt, resolved_res_fmt)
        }
    }

    fn timestampdiff_fmt(
        unit: String,
        ts1: String,
        fmt1: String,
        ts2: String,
        fmt2: String,
    ) -> i64 {
        let dt1 = DateTime::parse_from_str(&ts1, &fmt1).unwrap();
        let dt2 = DateTime::parse_from_str(&ts2, &fmt2).unwrap();

        match unit.to_lowercase().as_str() {
            "year" => dt2.years_since(dt1).unwrap() as i64,
            "quarter" => unimplemented!("quarter"),
            "month" => unimplemented!("month"),
            "week" => dt2.signed_duration_since(dt1).num_weeks(),
            "day" => dt2.signed_duration_since(dt1).num_days(),
            "hour" => dt2.signed_duration_since(dt1).num_hours(),
            "minute" => dt2.signed_duration_since(dt1).num_minutes(),
            "second" => dt2.signed_duration_since(dt1).num_seconds(),
            "millisecond" => dt2.signed_duration_since(dt1).num_milliseconds(),
            "microsecond" => dt2.signed_duration_since(dt1).num_microseconds().unwrap(),
            "nanosecond" => dt2.signed_duration_since(dt1).num_nanoseconds().unwrap(),
            _ => unimplemented!("{} is not implemented!", &unit),
        }
    }

    fn convert_to_utc_fmt(ts: String, fmt: String) -> String {
        if contains_tz(&fmt) {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .naive_utc()
                .to_string()
        } else {
            DateTime::<Utc>::from_utc(NaiveDateTime::parse_from_str(&ts, &fmt).unwrap(), Utc)
                .naive_utc()
                .to_string()
        }
    }

    fn extract_fmt(lunit: String, ts: String, fmt: String) -> u64 {
        let ts_str = ts.as_str();
        let fmt_str = fmt.as_str();
        lunit
            .split("_")
            .map(|unit| {
                match unit.to_lowercase().as_str() {
                    "year" => year_fmt(ts_str, fmt_str),
                    "quarter" => unimplemented!("quarter"),
                    "month" => month_fmt(ts_str, fmt_str),
                    "weekday" => weekday_fmt(ts_str, fmt_str),
                    "day" => day_fmt(ts_str, fmt_str),
                    "hour" => hour_fmt(ts_str, fmt_str),
                    "minute" => minute_fmt(ts_str, fmt_str),
                    "millisecond" => millisecond_fmt(ts_str, fmt_str),
                    "microsecond" => microsecond_fmt(ts_str, fmt_str),
                    "nanosecond" => nanosecond_fmt(ts_str, fmt_str),
                    _ => unimplemented!("{} is not implemented!", &unit),
                }
                .to_string()
            })
            .join("")
            .parse::<u64>()
            .unwrap()
    }

    fn convert_tz_fmt(ts: String, fmt: String, tz_str: String, res_fmt: String) -> String {
        let tz: Tz = tz_str.parse().unwrap();
        let resolved_res_fmt = match res_fmt.as_str() {
            "" => fmt.to_string(),
            _ => res_fmt.to_string(),
        };
        if contains_tz(&fmt) {
            DateTime::parse_from_str(&ts, &fmt)
                .unwrap()
                .with_timezone(&tz)
                .format(&resolved_res_fmt)
                .to_string()
        } else {
            DateTime::<Utc>::from_utc(NaiveDateTime::parse_from_str(&ts, &fmt).unwrap(), Utc)
                .with_timezone(&tz)
                .format(&resolved_res_fmt)
                .to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAIVE_TS: &str = "%Y-%m-%d %H:%M:%S";
    const NAIVE_TS_MILLIS: &str = "%Y-%m-%d %H:%M:%S%.3f";
    const NAIVE_TS_MICROS: &str = "%Y-%m-%d %H:%M:%S%.6f";
    const NAIVE_TS_NANOS: &str = "%Y-%m-%d %H:%M:%S%.9f";
    const TZ_TS: &str = "%Y-%m-%d %H:%M:%S%z";
    const TZ_TS_STR: &str = "%Y-%m-%d %H:%M:%S %Z";
    const TZ_TS_MILLIS: &str = "%Y-%m-%d %H:%M:%S%.3f%z";
    const TZ_TS_MICROS: &str = "%Y-%m-%d %H:%M:%S%.6f%z";
    const TZ_TS_NANOS: &str = "%Y-%m-%d %H:%M:%S%.9f%z";

    #[test]
    fn test_naive_micros() {
        let micro_naive_tsr = "2019-03-25 10:15:21.000423";
        assert_eq!(
            <Tsz as tsz::Tsz>::extract_fmt(
                "microsecond".to_string(),
                micro_naive_tsr.to_string(),
                NAIVE_TS_MICROS.to_string()
            ),
            423
        );
    }
    #[test]
    fn test_tz_micros() {
        let micro_tz_tsr = "2019-03-25 10:15:21.000423+06:00";
        assert_eq!(
            <Tsz as tsz::Tsz>::extract_fmt(
                "microsecond".to_string(),
                micro_tz_tsr.to_string(),
                TZ_TS_MICROS.to_string()
            ),
            423
        );
    }
    #[test]
    fn test_naive_nanos() {
        let nano_naive_tsr = "2019-03-25 10:15:21.000423986";
        let nano_naive_dt = NaiveDateTime::parse_from_str(&nano_naive_tsr, NAIVE_TS_NANOS).unwrap();
        assert_eq!(
            <Tsz as tsz::Tsz>::extract_fmt(
                "nanosecond".to_string(),
                nano_naive_tsr.to_string(),
                NAIVE_TS_NANOS.to_string()
            ),
            nano_naive_dt.timestamp_subsec_nanos() as u64
        );
    }
    #[test]
    fn test_tz_nanos() {
        let nano_tz_tsr = "2019-03-25 10:15:21.000423986-09:00";
        let nano_tz_dt = NaiveDateTime::parse_from_str(&nano_tz_tsr, TZ_TS_NANOS).unwrap();
        assert_eq!(
            <Tsz as tsz::Tsz>::extract_fmt(
                "nanosecond".to_string(),
                nano_tz_tsr.to_string(),
                TZ_TS_NANOS.to_string()
            ),
            nano_tz_dt.timestamp_subsec_nanos() as u64
        );
    }
    #[test]
    fn test_naive_convert_to_utc() {
        let naive_ts_str = "2014-04-18 12:00:00";
        assert_eq!(
            <Tsz as tsz::Tsz>::convert_to_utc_fmt(naive_ts_str.to_string(), NAIVE_TS.to_string()),
            "2014-04-18 12:00:00".to_string()
        );
    }
    #[test]
    fn test_tz_convert_to_utc() {
        let naive_ts_str = "2014-04-18 12:00:00+0500";
        assert_eq!(
            <Tsz as tsz::Tsz>::convert_to_utc_fmt(naive_ts_str.to_string(), TZ_TS.to_string()),
            "2014-04-18 07:00:00".to_string()
        );
    }
    #[test]
    fn test_tz_convert_tz() {
        let tz_ts_str = "2014-04-18 12:00:00+07:00";
        assert_eq!(
            <Tsz as tsz::Tsz>::convert_tz_fmt(
                tz_ts_str.to_string(),
                TZ_TS.to_string(),
                "EST".to_string(),
                TZ_TS.to_string()
            ),
            "2014-04-18 00:00:00-0500".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::convert_tz_fmt(
                tz_ts_str.to_string(),
                TZ_TS.to_string(),
                "EST".to_string(),
                "".to_string()
            ),
            "2014-04-18 00:00:00-0500".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::convert_tz_fmt(
                tz_ts_str.to_string(),
                TZ_TS.to_string(),
                "EST".to_string(),
                TZ_TS_STR.to_string()
            ),
            "2014-04-18 00:00:00 EST".to_string()
        );
    }
    #[test]
    fn test_naive_timestampadd() {
        let naive_ts_str = "2014-04-18 12:00:00";
        let naive_ts_str1 = "2023-07-18 12:00:00";

        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "month".to_string(),
                5,
                naive_ts_str.to_string(),
                NAIVE_TS.to_string(),
                "".to_string()
            ),
            "2014-09-18 12:00:00".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "day".to_string(),
                5,
                naive_ts_str.to_string(),
                NAIVE_TS.to_string(),
                NAIVE_TS.to_string()
            ),
            "2014-04-23 12:00:00".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "week".to_string(),
                2,
                naive_ts_str1.to_string(),
                NAIVE_TS.to_string(),
                NAIVE_TS_MILLIS.to_string()
            ),
            "2023-08-01 12:00:00.000".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "hour".to_string(),
                24,
                naive_ts_str1.to_string(),
                NAIVE_TS.to_string(),
                NAIVE_TS_MILLIS.to_string()
            ),
            "2023-07-19 12:00:00.000".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "minute".to_string(),
                2880,
                naive_ts_str1.to_string(),
                NAIVE_TS.to_string(),
                NAIVE_TS_MILLIS.to_string()
            ),
            "2023-07-20 12:00:00.000".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "second".to_string(),
                172800,
                naive_ts_str1.to_string(),
                NAIVE_TS.to_string(),
                NAIVE_TS_MILLIS.to_string()
            ),
            "2023-07-20 12:00:00.000".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "millisecond".to_string(),
                172800000,
                naive_ts_str1.to_string(),
                NAIVE_TS.to_string(),
                NAIVE_TS_MILLIS.to_string()
            ),
            "2023-07-20 12:00:00.000".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "nanosecond".to_string(),
                172800000000000,
                naive_ts_str1.to_string(),
                NAIVE_TS.to_string(),
                NAIVE_TS_NANOS.to_string()
            ),
            "2023-07-20 12:00:00.000000000".to_string()
        );
    }
    #[test]
    fn test_tz_timestampadd() {
        let tz_ts_str = "2014-04-18 12:00:00-0500";
        let tz_ts_str1 = "2023-07-18 12:00:00-0500";

        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "month".to_string(),
                5,
                tz_ts_str.to_string(),
                TZ_TS.to_string(),
                "".to_string()
            ),
            "2014-09-18 12:00:00-0500".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "day".to_string(),
                5,
                tz_ts_str.to_string(),
                TZ_TS.to_string(),
                TZ_TS.to_string()
            ),
            "2014-04-23 12:00:00-0500".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "week".to_string(),
                2,
                tz_ts_str1.to_string(),
                TZ_TS.to_string(),
                TZ_TS_MILLIS.to_string()
            ),
            "2023-08-01 12:00:00.000-0500".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "hour".to_string(),
                24,
                tz_ts_str1.to_string(),
                TZ_TS.to_string(),
                TZ_TS_MILLIS.to_string()
            ),
            "2023-07-19 12:00:00.000-0500".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "minute".to_string(),
                2880,
                tz_ts_str1.to_string(),
                TZ_TS.to_string(),
                TZ_TS_MILLIS.to_string()
            ),
            "2023-07-20 12:00:00.000-0500".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "second".to_string(),
                172800,
                tz_ts_str1.to_string(),
                TZ_TS.to_string(),
                TZ_TS_MILLIS.to_string()
            ),
            "2023-07-20 12:00:00.000-0500".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "millisecond".to_string(),
                172800000,
                tz_ts_str1.to_string(),
                TZ_TS.to_string(),
                TZ_TS_MILLIS.to_string()
            ),
            "2023-07-20 12:00:00.000-0500".to_string()
        );
        assert_eq!(
            <Tsz as tsz::Tsz>::timestampadd_fmt(
                "nanosecond".to_string(),
                172800000000000,
                tz_ts_str1.to_string(),
                TZ_TS.to_string(),
                TZ_TS_NANOS.to_string()
            ),
            "2023-07-20 12:00:00.000000000-0500".to_string()
        );
    }
}
