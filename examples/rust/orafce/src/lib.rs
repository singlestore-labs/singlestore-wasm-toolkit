use chrono::{
    DateTime, Datelike, Days, Duration, Local, Months, NaiveDate, NaiveDateTime, Timelike, Weekday,
};

wit_bindgen_rust::export!("orafce.wit");

struct Orafce;

const DEFAULT_FMT: &str = "%Y-%m-%d";
const DEFAULT_TIME_FMT: &str = "%Y-%m-%d %H:%M:%S";
const DEFAULT_TZ_FORMAT: &str = "%Y-%m-%d %H:%M:%S%z";
const MDY_TIME_FMT: &str = "%m/%d/%y %H:%M:%S";
const NLS_DATE_FMT: &str = "%y-%b%d %H:%M:%S";

pub fn days_of_month(y: i32, m: u32) -> i64 {
    let start_dt = NaiveDate::from_ymd_opt(y, m, 1).unwrap();
    let year_next_month = y + ((m == 12) as i32);
    let next_month = m % 12 + 1;
    NaiveDate::from_ymd_opt(year_next_month, next_month, 1)
        .unwrap()
        .signed_duration_since(start_dt)
        .num_days()
}

pub fn j2day(naive_date: NaiveDate) -> u32 {
    naive_date.weekday().number_from_sunday()
}

pub fn j2datetime(naive_datetime: NaiveDateTime) -> u32 {
    naive_datetime.weekday().number_from_sunday()
}

pub fn add_days(naive_date: NaiveDate, num_days: i32) -> NaiveDate {
    if num_days >= 0 {
        naive_date
            .checked_add_days(Days::new(num_days as u64))
            .unwrap()
    } else {
        naive_date
            .checked_sub_days(Days::new((-num_days) as u64))
            .unwrap()
    }
}

pub fn add_days_from_dt(naive_datetime: NaiveDateTime, num_days: i32) -> NaiveDateTime {
    if num_days >= 0 {
        naive_datetime
            .checked_add_days(Days::new(num_days as u64))
            .unwrap()
    } else {
        naive_datetime
            .checked_sub_days(Days::new((-num_days) as u64))
            .unwrap()
    }
}

pub fn mod_days(duration: Duration) -> Duration {
    Duration::days(duration.num_days() % 7)
}

pub fn iso_year(y: i32, m: u32, d: u32) -> NaiveDate {
    let mut result = NaiveDate::from_ymd_opt(y, 1, 1).unwrap();
    let day = NaiveDate::from_ymd_opt(y, m, d).unwrap();
    let mut off = 4 - j2day(result) as i32;
    let mut mon: i32 = 4;
    if off >= 0 {
        mon = -3;
    }
    add_days(result, off);
    if result > day {
        result = NaiveDate::from_ymd_opt(y - 1, 1, 1).unwrap();
        off = 4 - j2day(result) as i32;
        add_days(result, off + mon);
    }
    if (day.signed_duration_since(result) / 7)
        .checked_add(&Duration::days(1))
        .unwrap()
        > Duration::days(52)
    {
        let result2 = NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap();
        off = 4 - j2day(result2) as i32;
        add_days(result2, off + mon);

        if day >= result2 {
            return result2;
        }
    }
    return result;
}

fn months_between_general(timestamp1: String, timestamp2: String, to_fmt: &str) -> f32 {
    let dt1 = NaiveDateTime::parse_from_str(&timestamp1, to_fmt).unwrap();
    let dt2 = NaiveDateTime::parse_from_str(&timestamp2, to_fmt).unwrap();
    let d1 = dt1.day() as i32;
    let y1 = dt1.year() as i32;
    let m1 = dt1.month() as i32;
    let d2 = dt2.day() as i32;
    let y2 = dt2.year() as i32;
    let m2 = dt2.month() as i32;
    let mut result: f32 = (((y1 - y2) * 12 + (m1 - m2) + (d1 - d2)) as f32) / 31.0;
    if i64::from(d1) == days_of_month(y1, m1 as u32)
        && i64::from(d2) == days_of_month(y2, m2 as u32)
    {
        result = ((y1 - y2) * 12 + (m1 - m2)) as f32;
    }
    result
}

fn to_date_general(timestamp: String, from_fmt: &str, to_fmt: &str) -> String {
    DateTime::parse_from_str(&timestamp, from_fmt)
        .unwrap()
        .format(to_fmt)
        .to_string()
}

impl crate::orafce::Orafce for Orafce {
    fn add_months(date: String, num_months: u32) -> String {
        let dt = NaiveDate::parse_from_str(&date, DEFAULT_FMT).unwrap();
        dt.checked_add_months(Months::new(num_months))
            .unwrap()
            .to_string()
    }

    fn last_day(date: String) -> String {
        let dt = NaiveDate::parse_from_str(&date, DEFAULT_FMT).unwrap();
        let y = dt.year();
        NaiveDate::from_ymd_opt(y, dt.month() + 1, 1)
            .unwrap_or(NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap())
            .pred_opt()
            .unwrap()
            .format(DEFAULT_FMT)
            .to_string()
    }

    /*
     * day_of_week: 1..7 and interpreted as Sunday..Saturday.
     */
    fn next_day_by_index(date: String, day_of_week: i32) -> String {
        let dt = NaiveDate::parse_from_str(&date, DEFAULT_FMT).unwrap();

        /* cur_idx returns 0..6 as Sun..Sat */
        let cur_idx = j2day(dt);

        let off = day_of_week - 1 - cur_idx as i32;
        if off <= 0 {
            add_days(dt, off + 7);
        } else {
            add_days(dt, off);
        }
        dt.format(DEFAULT_FMT).to_string()
    }

    fn next_day(date: String, day_of_week: String) -> String {
        let idx = day_of_week.parse::<Weekday>().unwrap().number_from_sunday() as i32;
        Self::next_day_by_index(date, idx)
    }

    fn months_between(date1: String, date2: String) -> f32 {
        months_between_general(date1, date2, DEFAULT_FMT)
    }

    fn ora_date_trunc(date: String, format: String) -> String {
        let dt = NaiveDate::parse_from_str(&date, "YYYY-MM-DD").unwrap();
        let y = dt.year();
        let m = dt.month();
        let d = dt.day();
        let result = match format.as_str() {
            // fmt_CC
            "Cc" | "Scc" => NaiveDate::from_ymd_opt(y, 1, 1).unwrap(),
            // fmt_YYYY
            "Y" | "Yy" | "Yyy" | "Yyyy" | "Year" | "Syyyy" | "syear" => {
                NaiveDate::from_ymd_opt(y, 1, 1).unwrap()
            }
            // fmt_IYYY
            "I" | "Iy" | "Iyy" | "Iyyy" => iso_year(y, m, d),
            // fmt_MON
            "Month" | "Mon" | "Mm" | "Rm" => NaiveDate::from_ymd_opt(y, m, 1).unwrap(),
            // fmt_WW
            "Ww" => {
                dt - mod_days(dt.signed_duration_since(NaiveDate::from_ymd_opt(y, 1, 1).unwrap()))
            }
            // fmt_IW
            "Iw" => dt - mod_days(dt.signed_duration_since(iso_year(y, m, d))),
            // fmt_W
            "W" => {
                dt - mod_days(dt.signed_duration_since(NaiveDate::from_ymd_opt(y, m, 1).unwrap()))
            }
            // fmt_DAY
            "Day" | "Dy" | "D" => add_days(dt, -(j2day(dt) as i32)),
            // fmt_Q
            "Q" => NaiveDate::from_ymd_opt(y, ((m - 1) / 3) * 3 + 1, 1).unwrap(),
            _ => dt,
        };
        result.format(DEFAULT_FMT).to_string()
    }

    fn ora_date_round(date: String, format: String) -> String {
        let dt = NaiveDate::parse_from_str(&date, "YYYY-MM-DD").unwrap();
        let y = dt.year();
        let m = dt.month();
        let d = dt.day();
        let result = match format.as_str() {
            // fmt_CC
            "Cc" | "Scc" => {
                if y > 0 {
                    let mut off = 101;
                    if dt < NaiveDate::from_ymd_opt((y / 100) * 100 + 50, 1, 1).unwrap() {
                        off = 1;
                    }
                    NaiveDate::from_ymd_opt((y / 100) * 100 + off, 1, 1).unwrap()
                } else {
                    let mut off = 1;
                    if dt < NaiveDate::from_ymd_opt((y / 100) * 100 - 50 + 1, 1, 1).unwrap() {
                        off = -99
                    }
                    NaiveDate::from_ymd_opt((y / 100) * 100 + off, 1, 1).unwrap()
                }
            }
            // fmt_YYYY
            "Y" | "Yy" | "Yyy" | "Yyyy" | "Year" | "Syyyy" | "syear" => {
                let mut off = 1;
                if dt < NaiveDate::from_ymd_opt(y, 7, 1).unwrap() {
                    off = 0;
                }
                NaiveDate::from_ymd_opt(y + off, 1, 1).unwrap()
            }
            // fmt_IYYY
            "I" | "Iy" | "Iyy" | "Iyyy" => {
                if dt < NaiveDate::from_ymd_opt(y, 7, 1).unwrap() {
                    iso_year(y, m, d)
                } else {
                    let iy1 = iso_year(y + 1, 1, 8);
                    let mut tmp = iy1;

                    if (dt - NaiveDate::from_ymd_opt(y, 1, 1).unwrap()) / 7 + Duration::days(1)
                        >= Duration::days(52)
                    {
                        let overl = NaiveDate::from_ymd_opt(y + 2, 1, 1).unwrap()
                            - NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap()
                            == Duration::days(366);
                        let is_saturday = j2day(dt) == 6;

                        let iy2 = iso_year(y + 2, 1, 8);
                        let day1 = NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap();
                        /* exception saturdays */
                        if iy1 >= day1 && dt >= day1 - Duration::days(2) && is_saturday {
                            if overl {
                                tmp = iy2;
                            }
                        }
                        /* iso year stars in last year and dt >= iso year */
                        else if iy1 <= day1 && dt >= iy1 - Duration::days(3) {
                            let cmp = match iy1 < day1 {
                                true => iy1,
                                false => iy1 - Duration::days(1),
                            };
                            let d2 = j2day(day1);
                            /* some exceptions */
                            if (dt >= cmp - Duration::days(2)) && (!(d2 == 3 && overl)) {
                                /* if year don't starts in thursdt */
                                if (d2 < 4 && j2day(dt) != 5 && !is_saturday)
                                    || (d2 == 2 && is_saturday && overl)
                                {
                                    tmp = iy2;
                                }
                            }
                        }
                    }
                    tmp
                }
            }
            // fmt_MON
            "Month" | "Mon" | "Mm" | "Rm" => {
                let off = match dt < NaiveDate::from_ymd_opt(y, m, 16).unwrap() {
                    true => 0,
                    false => 1,
                };
                NaiveDate::from_ymd_opt(y, m + off, 1).unwrap()
            }
            // fmt_WW
            "Ww" => {
                let z = mod_days(dt - NaiveDate::from_ymd_opt(y, 1, 1).unwrap());
                match z < Duration::days(4) {
                    true => dt - z,
                    false => dt - z - Duration::days(7),
                }
            }
            // fmt_IW
            "Iw" => {
                let z1 = mod_days(dt - iso_year(y, m, d));
                let mut tmp1 = match z1 < Duration::days(4) {
                    true => dt - z1,
                    false => dt - z1 - Duration::days(7),
                };
                if (dt - NaiveDate::from_ymd_opt(y, 1, 1).unwrap()) / 7 + Duration::days(1)
                    >= Duration::days(52)
                {
                    /* only for last iso week */
                    let isoyear = iso_year(y + 1, 1, 8);
                    if isoyear > NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap() - Duration::days(1) {
                        if dt > isoyear - Duration::days(7) {
                            let _d = j2day(dt);
                            if _d == 0 || _d > 4 {
                                tmp1 -= Duration::days(7);
                            }
                        }
                    }
                }
                tmp1
            }
            // fmt_W
            "W" => {
                let z2 = mod_days(dt - NaiveDate::from_ymd_opt(y, m, 1).unwrap());
                match z2 < Duration::days(4) {
                    true => dt - z2,
                    false => dt - z2 - Duration::days(7),
                }
            }
            // fmt_DAY
            "Day" | "Dy" | "D" => {
                let z3 = j2day(dt);
                if y > 0 {
                    match z3 < 4 {
                        true => dt - Duration::days(z3 as i64),
                        false => dt - Duration::days(z3 as i64) - Duration::days(7),
                    }
                } else {
                    let off = 5 - match z3 > 0 {
                        true => match z3 > 1 {
                            true => z3,
                            false => z3 + 7,
                        },
                        false => 7,
                    };
                    dt + Duration::days(off as i64)
                }
            }
            // fmt_Q
            "Q" => {
                let off = match dt < NaiveDate::from_ymd_opt(y, ((m - 1) / 3) * 3 + 2, 16).unwrap()
                {
                    true => 1,
                    false => 4,
                };
                let new_m = ((m - 1) / 3) * 3 + off;
                NaiveDate::from_ymd_opt(y, new_m, 1).unwrap()
            }
            _ => dt,
        };
        result.format(DEFAULT_FMT).to_string()
    }

    fn to_date(tz_date: String) -> String {
        // to_date('2014-05-19 17:23:53+5:30') -> 2014-05-19 17:23:53
        to_date_general(tz_date, DEFAULT_TZ_FORMAT, DEFAULT_TIME_FMT)
    }

    fn oracle_add_months(timestamp: String, num_months: u32) -> String {
        let dt = DateTime::parse_from_str(&timestamp, DEFAULT_TIME_FMT).unwrap();
        dt.checked_add_months(Months::new(num_months))
            .unwrap()
            .format(DEFAULT_TIME_FMT)
            .to_string()
    }

    fn oracle_last_day(timestamp: String) -> String {
        let dt = DateTime::parse_from_str(&timestamp, DEFAULT_TIME_FMT).unwrap();
        let y = dt.year();
        NaiveDate::from_ymd_opt(y, dt.month() + 1, 1)
            .unwrap_or(NaiveDate::from_ymd_opt(y + 1, 1, 1).unwrap())
            .pred_opt()
            .unwrap()
            .and_hms_opt(dt.hour(), dt.minute(), dt.second())
            .unwrap()
            .format(DEFAULT_TIME_FMT)
            .to_string()
    }

    fn oracle_next_day_by_index(timestamp: String, day_of_week: i32) -> String {
        let dt = NaiveDateTime::parse_from_str(&timestamp, DEFAULT_TIME_FMT).unwrap();

        /* cur_idx returns 0..6 as Sun..Sat */
        let cur_idx = j2datetime(dt);

        let off = day_of_week - 1 - cur_idx as i32;
        if off <= 0 {
            add_days_from_dt(dt, off + 7);
        } else {
            add_days_from_dt(dt, off);
        }
        dt.format(DEFAULT_TIME_FMT).to_string()
    }

    fn oracle_next_day(timestamp: String, day_of_week: String) -> String {
        let idx = day_of_week.parse::<Weekday>().unwrap().number_from_sunday() as i32;
        Self::next_day_by_index(timestamp, idx)
    }

    fn oracle_months_between(timestamp1: String, timestamp2: String) -> f32 {
        months_between_general(timestamp1, timestamp2, DEFAULT_TIME_FMT)
    }

    fn oracle_to_date(timestamp: String) -> String {
        to_date_general(timestamp, MDY_TIME_FMT, DEFAULT_TIME_FMT)
    }

    fn oracle_custom_to_date(timestamp: String, from_fmt: String) -> String {
        to_date_general(timestamp, &from_fmt, DEFAULT_TIME_FMT)
    }

    fn oracle_sysdate() -> String {
        Local::now().format(DEFAULT_TIME_FMT).to_string()
    }

    fn oracle_to_char(timestamp: String) -> String {
        to_date_general(timestamp, DEFAULT_TIME_FMT, NLS_DATE_FMT)
    }

    fn oracle_add_days(timestamp: String, num_days: i32) -> String {
        add_days_from_dt(
            NaiveDateTime::parse_from_str(&timestamp, DEFAULT_TIME_FMT).unwrap(),
            num_days,
        )
        .format(DEFAULT_TIME_FMT)
        .to_string()
    }

    fn oracle_sub_days(timestamp: String, num_days: i32) -> String {
        add_days_from_dt(
            NaiveDateTime::parse_from_str(&timestamp, DEFAULT_TIME_FMT).unwrap(),
            -num_days,
        )
        .format(DEFAULT_TIME_FMT)
        .to_string()
    }

    fn oracle_difference_between_days(timestamp1: String, timestamp2: String) -> f64 {
        let seconds_difference = NaiveDateTime::parse_from_str(&timestamp1, DEFAULT_TIME_FMT)
            .unwrap()
            .signed_duration_since(
                NaiveDateTime::parse_from_str(&timestamp2, DEFAULT_TIME_FMT).unwrap(),
            )
            .num_seconds() as f64;
        seconds_difference / 3600.0 / 24.0
    }
}
