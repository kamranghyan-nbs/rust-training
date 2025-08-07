use flexible_datetime_parser::{parse_flexible_datetime, parse_to_utc, parse_to_local, can_parse_datetime};
use chrono::Datelike;

fn main() {
    println!("=== Flexible DateTime Parser ===\n");

    // Test various formats
    let test_strings = vec![
        "2014-07-01 14:43:00",           // Standard format
        "2014-07-01T14:43:00",           // ISO format
        "2014/07/01 14:43:00",           // Forward slashes
        "07/01/2014 14:43:00",           // US format
        "2014-07-01",                    // Date only
        "2014-07-01T14:43:00Z",          // With timezone Z
        "2014-07-01T14:43:00+05:00",     // With timezone offset
        "01 Jul 2014 14:43:00",          // Text month
        "Tue, 01 Jul 2014 14:43:00 GMT", // RFC format
        "invalid string",                // Should fail
    ];

    println!("Testing different datetime formats:\n");

    for (i, test_str) in test_strings.iter().enumerate() {
        println!("{}. Input: '{}'", i + 1, test_str);
        
        match parse_flexible_datetime(test_str) {
            Ok(dt) => {
                println!("   ✓ Parsed successfully!");
                println!("   └─ Result: {} (Year: {}, Month: {}, Day: {})", 
                        dt, dt.year(), dt.month(), dt.day());
                
                // Show in different timezone interpretations
                if let Ok(utc_dt) = parse_to_utc(test_str) {
                    println!("   └─ As UTC: {}", utc_dt);
                }
            }
            Err(e) => {
                println!("   ✗ Failed: {}", e);
            }
        }
        println!();
    }

    // Demonstrate validation
    println!("=== Format Validation ===\n");
    
    let validation_tests = vec![
        "2014-07-01 14:43:00",
        "2014/07/01 14:43:00", 
        "not a date",
        "2014-13-01 14:43:00", // Invalid month
    ];

    for test in &validation_tests {
        let is_valid = can_parse_datetime(test);
        let status = if is_valid { "✓ Valid" } else { "✗ Invalid" };
        println!("{}: '{}'", status, test);
    }
}