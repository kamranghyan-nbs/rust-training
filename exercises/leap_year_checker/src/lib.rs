use chrono::{DateTime, NaiveDateTime, Datelike};

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

pub fn is_leap_year_from_datetime<Tz: chrono::TimeZone>(datetime: &DateTime<Tz>) -> bool {
    is_leap_year(datetime.year())
}

pub fn is_leap_year_from_naive_datetime(datetime: &NaiveDateTime) -> bool {
    is_leap_year(datetime.year())
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use chrono::{TimeZone, Utc, Local};

    #[test]
    fn test_basic_leap_years() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2020));
        assert!(is_leap_year(2016));
        assert!(is_leap_year(2012));
    }

    #[test]
    fn test_basic_non_leap_years() {
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(2022));
        assert!(!is_leap_year(2021));
        assert!(!is_leap_year(2019));
    }

    #[test]
    fn test_century_years() {
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(1800));
        assert!(!is_leap_year(1700));
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn test_datetime_functions() {
        let leap_dt = Utc.with_ymd_and_hms(2024, 2, 29, 12, 0, 0).unwrap();
        let non_leap_dt = Local.with_ymd_and_hms(2023, 3, 15, 10, 30, 0).unwrap();
        
        assert!(is_leap_year_from_datetime(&leap_dt));
        assert!(!is_leap_year_from_datetime(&non_leap_dt));
    }

    #[test]
    fn test_naive_datetine_function() {
        let leap_dt = NaiveDateTime::parse_from_str("2000-06-15 14:30:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let non_leap_dt = NaiveDateTime::parse_from_str("1900-12-25 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        
        assert!(is_leap_year_from_naive_datetime(&leap_dt));
        assert!(!is_leap_year_from_naive_datetime(&non_leap_dt));
    }
}


