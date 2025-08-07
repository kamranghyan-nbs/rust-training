use chrono::{DateTime, NaiveDateTime, Utc, Local, TimeZone};

/// The target datetime format: "YYYY-MM-DD HH:MM:SS"
pub const TARGET_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn format_naive_datetime(datetime: &NaiveDateTime) -> String {
    datetime.format(TARGET_FORMAT).to_string()
}

pub fn format_utc_datetime(datetime: &DateTime<Utc>) -> String {
    datetime.format(TARGET_FORMAT).to_string()
}

pub fn format_local_datetime(datetime: &DateTime<Local>) -> String {
    datetime.format(TARGET_FORMAT).to_string()
}

pub fn format_datetime<Tz: TimeZone>(datetime: &DateTime<Tz>) -> String 
where
    Tz::Offset: std::fmt::Display,
{
    datetime.format(TARGET_FORMAT).to_string()
}

pub fn format_current_utc() -> String {
    format_utc_datetime(&Utc::now())
}

pub fn format_current_local() -> String {
    format_local_datetime(&Local::now())
}


pub fn format_multiple<'a, I>(datetimes: I) -> Vec<String>
where
    I: Iterator<Item = &'a NaiveDateTime>,
{
    datetimes.map(format_naive_datetime).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_format_naive_datetime() {
        let dt = NaiveDateTime::parse_from_str("2014-07-01T14:43:00", "%Y-%m-%dT%H:%M:%S").unwrap();
        let formatted = format_naive_datetime(&dt);
        assert_eq!(formatted, "2014-07-01 14:43:00");
    }

    #[test]
    fn test_format_utc_datetime() {
        let dt = Utc.ymd(2014, 7, 1).and_hms(14, 43, 0);
        let formatted = format_utc_datetime(&dt);
        assert_eq!(formatted, "2014-07-01 14:43:00");
    }

    #[test]
    fn test_format_local_datetime() {
        let dt = Local.ymd(2014, 7, 1).and_hms(14, 43, 0);
        let formatted = format_local_datetime(&dt);
        assert_eq!(formatted, "2014-07-01 14:43:00");
    }

    #[test]
    fn test_generic_format_datetime() {
        let utc_dt = Utc.ymd(2014, 7, 1).and_hms(14, 43, 0);
        let local_dt = Local.ymd(2014, 7, 1).and_hms(14, 43, 0);
        
        let utc_formatted = format_datetime(&utc_dt);
        let local_formatted = format_datetime(&local_dt);
        
        assert_eq!(utc_formatted, "2014-07-01 14:43:00");
        assert_eq!(local_formatted, "2014-07-01 14:43:00");
    }

    #[test]
    fn test_current_time_formatting() {
        let utc_now = format_current_utc();
        let local_now = format_current_local();
        
        // Should match the expected format (basic validation)
        assert_eq!(utc_now.len(), 19); // "YYYY-MM-DD HH:MM:SS" is 19 chars
        assert_eq!(local_now.len(), 19);
        
        // Should contain expected separators
        assert!(utc_now.contains('-'));
        assert!(utc_now.contains(':'));
        assert!(utc_now.contains(' '));
    }

    #[test]
    fn test_format_multiple() {
        let dt1 = NaiveDateTime::parse_from_str("2014-07-01 14:43:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let dt2 = NaiveDateTime::parse_from_str("2020-12-25 09:30:15", "%Y-%m-%d %H:%M:%S").unwrap();
        
        let datetimes = vec![&dt1, &dt2];
        let formatted = format_multiple(datetimes.into_iter());
        
        assert_eq!(formatted.len(), 2);
        assert_eq!(formatted[0], "2014-07-01 14:43:00");
        assert_eq!(formatted[1], "2020-12-25 09:30:15");
    }

    #[test]
    fn test_edge_cases() {
        // Test edge cases like midnight, end of year, etc.
        let midnight = NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end_of_year = NaiveDateTime::parse_from_str("2024-12-31 23:59:59", "%Y-%m-%d %H:%M:%S").unwrap();
        
        assert_eq!(format_naive_datetime(&midnight), "2024-01-01 00:00:00");
        assert_eq!(format_naive_datetime(&end_of_year), "2024-12-31 23:59:59");
    }
}
