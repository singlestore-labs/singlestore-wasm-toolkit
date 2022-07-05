wit_bindgen_rust::export!("dates.wit");
struct Dates;

use chrono::{NaiveDate, TimeZone, Datelike, Duration, Utc, Weekday};

impl dates::Dates for Dates {
    fn next_saturday(in_date: String) -> String {
        next_saturday(in_date)
    }
}

fn next_saturday(in_date: String) -> String {
    let start_naive  = NaiveDate::parse_from_str(&in_date, "%Y-%m-%d").expect("Parse failed");
    let start_utc = Utc.from_utc_date(&start_naive);
    let mut current = start_utc;
    loop {
        current = current + Duration::days(1);
        if current.weekday() == Weekday::Sat {
            return current.format("%Y-%m-%d").to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known_dates() {
        assert_eq!(next_saturday("2022-06-04".to_string()), "2022-06-11".to_string());
        assert_eq!(next_saturday("2022-06-05".to_string()), "2022-06-11".to_string());
        assert_eq!(next_saturday("2022-06-06".to_string()), "2022-06-11".to_string());
        assert_eq!(next_saturday("2022-06-07".to_string()), "2022-06-11".to_string());
        assert_eq!(next_saturday("2022-06-08".to_string()), "2022-06-11".to_string());
        assert_eq!(next_saturday("2022-06-09".to_string()), "2022-06-11".to_string());
        assert_eq!(next_saturday("2022-06-10".to_string()), "2022-06-11".to_string());
    }
}
