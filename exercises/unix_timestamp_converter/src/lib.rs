use chrono::{DateTime, NaiveDateTime, Utc, Local, TimeZone};

pub fn timestamp_to_naive_datetime(timestamp: i64) -> Option<NaiveDateTime> {
    NaiveDateTime::from_timestamp_opt(timestamp, 0)
}

pub fn timestamp_to_utc(timestamp: i64) -> Option<DateTime<Utc>> {
    Utc.timestamp_opt(timestamp, 0).single()
}

pub fn timestamp_to_local(timestamp: i64) -> Option<DateTime<Local>> {
    Local.timestamp_opt(timestamp, 0).single()
}

pub fn timestamp_f64_to_naive_datetime(timestamp: f64) -> Option<NaiveDateTime> {
    let secs = timestamp.floor() as i64;
    let nanos = ((timestamp.fract()) * 1_000_000_000.0) as u32;
    NaiveDateTime::from_timestamp_opt(secs, nanos)
}

pub fn timestamp_f64_to_utc(timestamp: f64) -> Option<DateTime<Utc>> {
    let secs = timestamp.floor() as i64;
    let nanos = ((timestamp.fract()) * 1_000_000_000.0) as u32;
    Utc.timestamp_opt(secs, nanos).single()
}

pub fn current_timestamp_to_datetime() -> DateTime<Utc> {
    Utc::now()
}

pub fn get_current_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn is_valid_timestamp(timestamp: i64) -> bool {
    // Reasonable range: 1970-01-01 to ~2038-01-19 (32-bit signed int limit)
    timestamp >= 0 && timestamp <= 2_147_483_647
}

pub fn convert_multiple_timestamps<I>(timestamps: I) -> Vec<Option<DateTime<Utc>>>
where
    I: Iterator<Item = i64>,
{
    timestamps.map(timestamp_to_utc).collect()
}


pub fn timestamp_to_formatted_string(timestamp: i64, format: &str) -> Option<String> {
    timestamp_to_utc(timestamp).map(|dt| dt.format(format).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_timestamp_to_naive_datetime() {
        // 1404230580 = 2014-07-01 14:43:00 UTC
        let dt = timestamp_to_naive_datetime(1404230580).unwrap();
        
        assert_eq!(dt.year(), 2014);
        assert_eq!(dt.month(), 7);
        assert_eq!(dt.day(), 1);
        assert_eq!(dt.hour(), 14);
        assert_eq!(dt.minute(), 43);
        assert_eq!(dt.second(), 0);
    }

    #[test]
    fn test_timestamp_to_utc() {
        let dt = timestamp_to_utc(1404230580).unwrap();
        
        assert_eq!(dt.year(), 2014);
        assert_eq!(dt.month(), 7);
        assert_eq!(dt.day(), 1);
        assert_eq!(dt.timezone(), Utc);
    }

    #[test]
    fn test_timestamp_to_local() {
        let dt = timestamp_to_local(1404230580).unwrap();
        
        assert_eq!(dt.year(), 2014);
        // Note: exact time may differ based on local timezone
    }

    #[test]
    fn test_timestamp_f64_to_naive() {
        let dt = timestamp_f64_to_naive_datetime(1404230580.5).unwrap();
        
        assert_eq!(dt.year(), 2014);
        assert_eq!(dt.nanosecond(), 500_000_000); // 0.5 seconds = 500ms
    }

    #[test]
    fn test_invalid_timestamps() {
        assert!(timestamp_to_naive_datetime(-1).is_none());
        assert!(timestamp_to_utc(3_000_000_000).is_none());
        assert!(timestamp_f64_to_naive_datetime(-1.0).is_none());
    }

    #[test]
    fn test_is_valid_timestamp() {
        assert!(is_valid_timestamp(0));              // Unix epoch
        assert!(is_valid_timestamp(1404230580));     // Valid date
        assert!(is_valid_timestamp(2_147_483_647));  // 2038 problem limit
        
        assert!(!is_valid_timestamp(-1));           // Before epoch
        assert!(!is_valid_timestamp(3_000_000_000)); // Too far in future
    }

    #[test]
    fn test_current_timestamp() {
        let now = current_timestamp_to_datetime();
        let timestamp = get_current_timestamp();
        
        // Should be recent
        assert!(timestamp > 1_600_000_000); // After 2020
        assert_eq!(now.timezone(), Utc);
    }

    #[test]
    fn test_convert_multiple_timestamps() {
        let timestamps = vec![1404230580, 1609459200, -1]; // Valid, Valid, Invalid
        let results = convert_multiple_timestamps(timestamps.iter().cloned());
        
        assert_eq!(results.len(), 3);
        assert!(results[0].is_some());
        assert!(results[1].is_some());
        assert!(results[2].is_none());
    }

    #[test]
    fn test_timestamp_to_formatted_string() {
        let formatted = timestamp_to_formatted_string(1404230580, "%Y-%m-%d %H:%M:%S").unwrap();
        assert_eq!(formatted, "2014-07-01 14:43:00");
        
        let iso_formatted = timestamp_to_formatted_string(1404230580, "%Y-%m-%dT%H:%M:%SZ").unwrap();
        assert_eq!(iso_formatted, "2014-07-01T14:43:00Z");
    }

    #[test]
    fn test_known_timestamps() {
        // Test some well-known timestamps
        let epoch = timestamp_to_utc(0).unwrap();
        assert_eq!(epoch.year(), 1970);
        assert_eq!(epoch.month(), 1);
        assert_eq!(epoch.day(), 1);

        // Y2K
        let y2k = timestamp_to_utc(946684800).unwrap(); // 2000-01-01 00:00:00
        assert_eq!(y2k.year(), 2000);
        assert_eq!(y2k.month(), 1);
        assert_eq!(y2k.day(), 1);
    }
}