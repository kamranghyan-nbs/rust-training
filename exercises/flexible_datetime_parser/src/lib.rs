use chrono::{DateTime, NaiveDateTime, Utc, Local, ParseError, TimeZone};

/// Common datetime formats to try when parsing
const DATETIME_FORMATS: &[&str] = &[
    // ISO-like formats
    "%Y-%m-%d %H:%M:%S",           // 2014-07-01 14:43:00
    "%Y-%m-%dT%H:%M:%S",           // 2014-07-01T14:43:00
    "%Y-%m-%dT%H:%M:%SZ",          // 2014-07-01T14:43:00Z
    "%Y-%m-%dT%H:%M:%S%z",         // 2014-07-01T14:43:00+0000
    
    // Alternative separators
    "%Y/%m/%d %H:%M:%S",           // 2014/07/01 14:43:00
    "%Y.%m.%d %H:%M:%S",           // 2014.07.01 14:43:00
    
    // US format
    "%m/%d/%Y %H:%M:%S",           // 07/01/2014 14:43:00
    "%m-%d-%Y %H:%M:%S",           // 07-01-2014 14:43:00
    
    // Date only formats
    "%Y-%m-%d",                    // 2014-07-01
    "%Y/%m/%d",                    // 2014/07/01
    "%m/%d/%Y",                    // 07/01/2014
    
    // RFC formats
    "%a, %d %b %Y %H:%M:%S",       // Tue, 01 Jul 2014 14:43:00
    "%d %b %Y %H:%M:%S",           // 01 Jul 2014 14:43:00
    
    // Without seconds
    "%Y-%m-%d %H:%M",              // 2014-07-01 14:43
    "%Y-%m-%dT%H:%M",              // 2014-07-01T14:43
];

pub fn parse_flexible_datetime(datetime_str: &str) -> Result<NaiveDateTime, String> {
    let trimmed = datetime_str.trim();
    
    if trimmed.is_empty() {
        return Err("Input string is empty".to_string());
    }
    
    // Try each format until one works
    for format in DATETIME_FORMATS {
        if let Ok(dt) = NaiveDateTime::parse_from_str(trimmed, format) {
            return Ok(dt);
        }
    }
    
    // If none worked, try parsing as RFC3339 (handles timezones)
    if let Ok(dt_with_tz) = DateTime::parse_from_rfc3339(trimmed) {
        return Ok(dt_with_tz.naive_utc());
    }
    
    // Try parsing as RFC2822
    if let Ok(dt_with_tz) = DateTime::parse_from_rfc2822(trimmed) {
        return Ok(dt_with_tz.naive_utc());
    }
    
    Err(format!("Unable to parse '{}' - format not recognized", trimmed))
}

/// Parses datetime string and returns UTC DateTime
pub fn parse_to_utc(datetime_str: &str) -> Result<DateTime<Utc>, String> {
    let naive_dt = parse_flexible_datetime(datetime_str)?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive_dt, Utc))
}

/// Parses datetime string and returns Local DateTime
pub fn parse_to_local(datetime_str: &str) -> Result<DateTime<Local>, String> {
    let naive_dt = parse_flexible_datetime(datetime_str)?;
    Ok(DateTime::<Local>::from_naive_utc_and_offset(
        naive_dt, 
        Local.offset_from_utc_datetime(&naive_dt)
    ))
}

/// Checks if a string can be parsed as datetime
pub fn can_parse_datetime(datetime_str: &str) -> bool {
    parse_flexible_datetime(datetime_str).is_ok()
}