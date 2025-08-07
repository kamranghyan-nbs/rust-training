use leap_year_checker::{is_leap_year, is_leap_year_from_datetime, is_leap_year_from_naive_datetime};
use chrono::{DateTime, Utc, Local, NaiveDateTime, TimeZone};

#[test]
fn test_comprehensive_leap_year_rules() {
    // Test the complete leap year algorithm
    let test_cases = vec![
        // Basic 4-year rule
        (2024, true), (2023, false), (2022, false), (2021, false),
        
        // Century rule (divisible by 100, not by 400)
        (1700, false), (1800, false), (1900, false), (2100, false),
        
        // 400-year rule override
        (1600, true), (2000, true), (2400, true),
        
        // Edge cases
        (4, true), (1, false), (100, false), (400, true),
    ];

    for (year, expected) in test_cases {
        assert_eq!(is_leap_year(year), expected, 
                  "Failed for year {}", year);
    }
}

#[test]
fn test_datetime_integration() {
    // Test with various DateTime types
    
    // UTC DateTime
    let utc_leap = Utc.with_ymd_and_hms(2024, 2, 29, 12, 0, 0).unwrap();
    let utc_non_leap = Utc.with_ymd_and_hms(2023, 3, 1, 10, 0, 0).unwrap();
    
    assert!(is_leap_year_from_datetime(&utc_leap));
    assert!(!is_leap_year_from_datetime(&utc_non_leap));
    
    // Local DateTime
    let local_leap = Local.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    let local_non_leap = Local.with_ymd_and_hms(2021, 12, 31, 23, 59, 59).unwrap();
    
    assert!(is_leap_year_from_datetime(&local_leap));
    assert!(!is_leap_year_from_datetime(&local_non_leap));
}

#[test]
fn test_naive_datetime_integration() {
    let leap_cases = [
        "2024-01-01 00:00:00",
        "2000-12-31 23:59:59",
        "1600-06-15 12:30:00",
    ];
    
    let non_leap_cases = [
        "2023-01-01 00:00:00", 
        "1900-12-31 23:59:59",
        "1700-06-15 12:30:00",
    ];
    
    for case in &leap_cases {
        let dt = NaiveDateTime::parse_from_str(case, "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(is_leap_year_from_naive_datetime(&dt), "Failed for: {}", case);
    }
    
    for case in &non_leap_cases {
        let dt = NaiveDateTime::parse_from_str(case, "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(!is_leap_year_from_naive_datetime(&dt), "Failed for: {}", case);
    }
}

#[test]
fn test_string_parsing_workflow() {
    // Test a complete workflow: parse string -> check leap year
    let test_strings = vec![
        ("2024-03-15T10:30:00Z", true),
        ("2023-07-20T14:45:00Z", false),
        ("2000-12-31T23:59:59Z", true),
        ("1900-01-01T00:00:00Z", false),
    ];

    for (date_str, expected) in test_strings {
        let datetime: DateTime<Utc> = date_str.parse().unwrap();
        let result = is_leap_year_from_datetime(&datetime);
        assert_eq!(result, expected, 
                  "Failed for date string: {}", date_str);
    }
}

#[test]
fn test_performance_benchmark() {
    // Test that the function performs well over many iterations
    let start = std::time::Instant::now();
    
    for year in 1..10000 {
        let _ = is_leap_year(year);
    }
    
    let duration = start.elapsed();
    
    // Should complete very quickly (under 100ms for 10k iterations)
    assert!(duration.as_millis() < 100, 
           "Performance test failed: took {:?}", duration);
}
