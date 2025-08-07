use datetime_parser::{
    parse_to_naive_datetime,
    parse_to_utc_datetime,
    parse_to_local_datetime,
    is_valid_datetime_format,
    parse_multiple_datetimes,
    DateTimeParseError,
};
use chrono::{Datelike, Timelike, Utc, Local};

#[test]
fn test_complete_parsing_workflow() {
    // Test the complete workflow from string to datetime object
    let input = "2014-07-01 14:43:00";
    
    let naive_result = parse_to_naive_datetime(input).unwrap();
    let utc_result = parse_to_utc_datetime(input).unwrap();
    let local_result = parse_to_local_datetime(input).unwrap();
    
    // All should have same date/time values
    assert_eq!(naive_result.year(), 2014);
    assert_eq!(naive_result.month(), 7);
    assert_eq!(naive_result.day(), 1);
    assert_eq!(naive_result.hour(), 14);
    assert_eq!(naive_result.minute(), 43);
    assert_eq!(naive_result.second(), 0);
    
    assert_eq!(utc_result.year(), 2014);
    assert_eq!(local_result.year(), 2014);
}

#[test]
fn test_edge_cases() {
    // Test various edge cases
    let test_cases = vec![
        ("2000-01-01 00:00:00", true),  // Millennium
        ("2000-12-31 23:59:59", true),  // End of millennium
        ("2024-02-29 12:00:00", true),  // Leap year date
        ("2023-02-29 12:00:00", false), // Invalid leap year date
        ("2014-00-01 14:43:00", false), // Invalid month
        ("2014-01-00 14:43:00", false), // Invalid day
        ("2014-01-01 25:00:00", false), // Invalid hour
        ("2014-01-01 14:60:00", false), // Invalid minute
        ("2014-01-01 14:43:60", false), // Invalid second
    ];
    
    for (input, should_succeed) in test_cases {
        let result = parse_to_naive_datetime(input);
        assert_eq!(result.is_ok(), should_succeed, 
                  "Failed for input: {}", input);
    }
}

#[test]
fn test_error_types() {
    // Test different error conditions
    let empty_result = parse_to_naive_datetime("");
    assert!(matches!(empty_result.unwrap_err(), DateTimeParseError::EmptyInput));
    
    let invalid_format_result = parse_to_naive_datetime("not a date");
    assert!(matches!(invalid_format_result.unwrap_err(), DateTimeParseError::InvalidFormat(_)));
}

#[test]
fn test_timezone_consistency() {
    // Test that timezone parsing maintains consistency
    let input = "2014-07-01 14:43:00";
    
    let naive = parse_to_naive_datetime(input).unwrap();
    let utc = parse_to_utc_datetime(input).unwrap();
    let local = parse_to_local_datetime(input).unwrap();
    
    // Check that the underlying datetime values are consistent
    assert_eq!(naive.year(), utc.year());
    assert_eq!(naive.year(), local.year());
    assert_eq!(naive.month(), utc.month());
    assert_eq!(naive.month(), local.month());
    assert_eq!(naive.day(), utc.day());
    assert_eq!(naive.day(), local.day());
}

#[test]
fn test_batch_processing_integration() {
    // Test processing multiple datetime strings
    let mixed_inputs = vec![
        "2014-07-01 14:43:00",  // Valid
        "2020-12-25 09:30:15",  // Valid
        "invalid",              // Invalid format
        "2021-02-29 12:00:00",  // Invalid date (not leap year)
        "2024-01-01 00:00:00",  // Valid
    ];
    
    let (successful, failed) = parse_multiple_datetimes(mixed_inputs.iter().map(|s| *s));
    
    assert_eq!(successful.len(), 3);
    assert_eq!(failed.len(), 2);
    
    // Check that successful dates are correct
    assert_eq!(successful[0].year(), 2014);
    assert_eq!(successful[1].year(), 2020);
    assert_eq!(successful[2].year(), 2024);
}

#[test]
fn test_validation_accuracy() {
    // Test that validation matches actual parsing results
    let test_cases = vec![
        "2014-07-01 14:43:00",
        "invalid",
        "2020-12-25 09:30:15",
        "",
        "2021-13-01 14:43:00",
    ];
    
    for case in test_cases {
        let is_valid = is_valid_datetime_format(case);
        let can_parse = parse_to_naive_datetime(case).is_ok();
        
        assert_eq!(is_valid, can_parse, 
                  "Validation mismatch for: {}", case);
    }
}

#[test]
fn test_real_world_scenarios() {
    // Test realistic datetime strings that might come from logs, databases, etc.
    let real_world_cases = vec![
        "2023-01-15 08:30:00",  // Morning time
        "2023-06-21 12:00:00",  // Noon
        "2023-12-31 23:59:59",  // End of year
        "2024-02-29 16:45:30",  // Leap year
        "2000-01-01 00:00:00",  // Y2K
    ];
    
    for case in real_world_cases {
        let result = parse_to_naive_datetime(case);
        assert!(result.is_ok(), "Failed to parse real-world case: {}", case);
        
        let dt = result.unwrap();
        
        // Verify basic constraints
        assert!(dt.month() >= 1 && dt.month() <= 12);
        assert!(dt.day() >= 1 && dt.day() <= 31);
        assert!(dt.hour() < 24);
        assert!(dt.minute() < 60);
        assert!(dt.second() < 60);
    }
}

#[test]
fn test_performance_benchmark() {
    // Test parsing performance for many strings
    let test_string = "2014-07-01 14:43:00";
    let start = std::time::Instant::now();
    
    for _ in 0..1000 {
        let _ = parse_to_naive_datetime(test_string).unwrap();
    }
    
    let duration = start.elapsed();
    
    // Should parse 1000 strings quickly (under 100ms)
    assert!(duration.as_millis() < 100, 
           "Performance test failed: took {:?}", duration);
}