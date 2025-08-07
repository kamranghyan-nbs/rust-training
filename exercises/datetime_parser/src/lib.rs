use chrono::{DateTime, NaiveDateTime, Utc, Local, TimeZone, ParseError};

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug, Clone, PartialEq)]
pub enum DateTimeParseError {
    /// The input string format is invalid
    InvalidFormat(String),
    /// The datetime values are out of range (e.g., month 13)
    InvalidDateTime(String),
    /// The input string is empty
    EmptyInput,
}

impl std::fmt::Display for DateTimeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateTimeParseError::InvalidFormat(msg) => {
                write!(f, "Invalid datetime format: {}", msg)
            }
            DateTimeParseError::InvalidDateTime(msg) => {
                write!(f, "Invalid datetime values: {}", msg)
            }
            DateTimeParseError::EmptyInput => {
                write!(f, "Input string cannot be empty")
            }
        }
    }
}


impl std::error::Error for DateTimeParseError {}

impl From<ParseError> for DateTimeParseError {
    fn from(err: ParseError) -> Self {
        DateTimeParseError::InvalidFormat(err.to_string())
    }
}

pub fn parse_to_naive_datetime(datetime_str: &str) -> Result<NaiveDateTime, DateTimeParseError> {
    if datetime_str.trim().is_empty() {
        return Err(DateTimeParseError::EmptyInput);
    }

    NaiveDateTime::parse_from_str(datetime_str.trim(), DATETIME_FORMAT)
        .map_err(DateTimeParseError::from)
}

pub fn parse_to_utc_datetime(datetime_str: &str) -> Result<DateTime<Utc>, DateTimeParseError> {
    let naive_dt = parse_to_naive_datetime(datetime_str)?;
    Ok(Utc.from_utc_datetime(&naive_dt))
}

pub fn parse_to_local_datetime(datetime_str: &str) -> Result<DateTime<Local>, DateTimeParseError> {
    let naive_dt = parse_to_naive_datetime(datetime_str)?;
    Ok(Local.from_local_datetime(&naive_dt).single()
        .ok_or_else(|| DateTimeParseError::InvalidDateTime(
            "Ambiguous local time".to_string()
        ))?)
}

pub fn is_valid_datetime_format(datetime_str: &str) -> bool {
    parse_to_naive_datetime(datetime_str).is_ok()
}


// pub fn parse_multiple_datetimes<I>(datetime_strings: I) -> (Vec<NaiveDateTime>, Vec<String>)
// where
//     I: Iterator<Item = &str>,
// {
//     let mut successful = Vec::new();
//     let mut failed = Vec::new();

//     for datetime_str in datetime_strings {
//         match parse_to_naive_datetime(datetime_str) {
//             Ok(dt) => successful.push(dt),
//             Err(_) => failed.push(datetime_str.to_string()),
//         }
//     }

//     (successful, failed)
// }



// Unit tests
#[cfg(test)]
mod unit_tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_parse_to_naive_datetime_success() {
        let result = parse_to_naive_datetime("2014-07-01 14:43:00").unwrap();
        
        assert_eq!(result.year(), 2014);
        assert_eq!(result.month(), 7);
        assert_eq!(result.day(), 1);
        assert_eq!(result.hour(), 14);
        assert_eq!(result.minute(), 43);
        assert_eq!(result.second(), 0);
    }

    #[test]
    fn test_parse_to_naive_datetime_with_whitespace() {
        let result = parse_to_naive_datetime("  2014-07-01 14:43:00  ").unwrap();
        assert_eq!(result.year(), 2014);
    }

    #[test]
    fn test_parse_to_naive_datetime_invalid_format() {
        let result = parse_to_naive_datetime("2014-07-01");
        assert!(result.is_err());
        
        let result = parse_to_naive_datetime("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_to_naive_datetime_empty_input() {
        let result = parse_to_naive_datetime("");
        assert_eq!(result.unwrap_err(), DateTimeParseError::EmptyInput);
    }

    #[test]
    fn test_parse_to_utc_datetime() {
        let result = parse_to_utc_datetime("2014-07-01 14:43:00").unwrap();
        assert_eq!(result.year(), 2014);
        assert_eq!(result.timezone(), Utc);
    }

    #[test]
    fn test_parse_to_local_datetime() {
        let result = parse_to_local_datetime("2014-07-01 14:43:00").unwrap();
        assert_eq!(result.year(), 2014);
    }

    #[test]
    fn test_is_valid_datetime_format() {
        assert!(is_valid_datetime_format("2014-07-01 14:43:00"));
        assert!(is_valid_datetime_format("2020-12-25 09:30:15"));
        
        assert!(!is_valid_datetime_format("2014-07-01"));
        assert!(!is_valid_datetime_format("invalid"));
        assert!(!is_valid_datetime_format(""));
        assert!(!is_valid_datetime_format("2014-13-01 14:43:00")); // Invalid month
    }

    #[test]
    fn test_parse_multiple_datetimes() {
        let strings = vec![
            "2014-07-01 14:43:00",
            "invalid",
            "2020-12-25 09:30:00",
            "2014-13-01 14:43:00", // Invalid month
        ];
        
        let (success, failures) = parse_multiple_datetimes(strings.iter().map(|s| *s));
        
        assert_eq!(success.len(), 2);
        assert_eq!(failures.len(), 2);
        assert!(failures.contains(&"invalid".to_string()));
    }
}


