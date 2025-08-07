use datetime_parser::{
    parse_to_naive_datetime,
    parse_to_utc_datetime,
    parse_to_local_datetime,
    is_valid_datetime_format,
    DateTimeParseError,
    DATETIME_FORMAT,
};
use chrono::{Datelike, Timelike};

fn main() {
    println!("=== DateTime String Parser Examples ===\n");
    println!("Expected format: {}\n", DATETIME_FORMAT);

    // Example 1: Basic parsing
    demonstrate_basic_parsing();

    // Example 2: Different timezone interpretations
    demonstrate_timezone_parsing();

    // Example 3: Error handling
    demonstrate_error_handling();

    // Example 4: Validation
    demonstrate_validation();

    // Example 5: Interactive example
    demonstrate_interactive_usage();
}

fn demonstrate_basic_parsing() {
    println!("1. Basic DateTime Parsing:");
    
    let datetime_str = "2014-07-01 14:43:00";
    
    match parse_to_naive_datetime(datetime_str) {
        Ok(dt) => {
            println!("   Input: {}", datetime_str);
            println!("   Parsed NaiveDateTime:");
            println!("     Year: {}", dt.year());
            println!("     Month: {}", dt.month());
            println!("     Day: {}", dt.day());
            println!("     Hour: {}", dt.hour());
            println!("     Minute: {}", dt.minute());
            println!("     Second: {}", dt.second());
            println!("     Formatted: {}", dt.format("%A, %B %d, %Y at %I:%M %p"));
        }
        Err(e) => println!("   Error: {}", e),
    }
    println!();
}

fn demonstrate_timezone_parsing() {
    println!("2. Timezone Interpretations:");
    
    let datetime_str = "2014-07-01 14:43:00";
    
    // Parse as naive (no timezone)
    if let Ok(naive_dt) = parse_to_naive_datetime(datetime_str) {
        println!("   Naive: {}", naive_dt);
    }
    
    // Parse as UTC
    if let Ok(utc_dt) = parse_to_utc_datetime(datetime_str) {
        println!("   UTC: {}", utc_dt);
        println!("   UTC ISO format: {}", utc_dt.to_rfc3339());
    }
    
    // Parse as Local
    if let Ok(local_dt) = parse_to_local_datetime(datetime_str) {
        println!("   Local: {}", local_dt);
        println!("   Local timezone: {}", local_dt.format("%Y-%m-%d %H:%M:%S %Z"));
    }
    println!();
}

fn demonstrate_error_handling() {
    println!("3. Error Handling Examples:");
    
    let test_cases = [
        "2014-07-01 14:43:00",  // Valid
        "2014-07-01",           // Missing time
        "invalid string",       // Invalid format
        "",                     // Empty
        "2014-13-01 14:43:00",  // Invalid month
        "2014-07-32 14:43:00",  // Invalid day
    ];
    
    for case in &test_cases {
        match parse_to_naive_datetime(case) {
            Ok(dt) => println!("   ✓ '{}' -> {}", case, dt),
            Err(e) => println!("   ✗ '{}' -> {}", case, e),
        }
    }
    println!();
}

fn demonstrate_validation() {
    println!("4. Format Validation:");
    
    let test_strings = [
        "2014-07-01 14:43:00",
        "2020-12-25 09:30:15",
        "invalid",
        "2014-07-01",
        "",
    ];
    
    for test_str in &test_strings {
        let is_valid = is_valid_datetime_format(test_str);
        let status = if is_valid { "✓ Valid" } else { "✗ Invalid" };
        println!("   {} -> '{}'", status, test_str);
    }
    println!();
}


fn demonstrate_interactive_usage() {
    println!("6. Interactive Usage Example:");
    println!("   (This would typically read from user input)");
    
    // Simulate user input
    let user_inputs = [
        "2024-03-15 10:30:00",
        "2023-12-25 16:00:00",
    ];
    
    for input in &user_inputs {
        println!("   User input: '{}'", input);
        
        match parse_to_naive_datetime(input) {
            Ok(dt) => {
                println!("     Parsed successfully!");
                println!("     Date: {}", dt.date());
                println!("     Time: {}", dt.time());
                println!("     Day of week: {}", dt.weekday());
                
                // Example calculations
                let days_since_epoch = dt.and_utc().timestamp() / (24 * 60 * 60);
                println!("     Days since Unix epoch: {}", days_since_epoch);
            }
            Err(e) => {
                println!("     Error: {}", e);
                println!("     Please use format: {}", DATETIME_FORMAT);
            }
        }
        println!();
    }
}