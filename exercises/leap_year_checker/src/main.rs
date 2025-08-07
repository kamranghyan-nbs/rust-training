use leap_year_checker::{is_leap_year, is_leap_year_from_datetime, is_leap_year_from_naive_datetime};
use chrono::{DateTime, Utc, Local, NaiveDateTime, Datelike};

fn main() {
    println!("=== Leap Year Checker Examples ===\n");

    // Example 1: Current datetime
    demostrate_current_datetime();

    // Example 2: Specific test dates
    demostrate_test_datetime();

    // Example 3: NaiveDateTime usage
    demostrate_naive_datetime();

    // Example 4: Historical examples
    demonstrate_historical_examples();
}

fn demostrate_current_datetime() {
    println!("1. Current DateTime Examples:");

    let utc_now = Utc::now();
    let local_now = Local::now();

    println!(" UTC: {} -> Year {} is leap year: {}",
            utc_now.format("%Y-%m-%d %H:%M:%S"),
            utc_now.year(),
            is_leap_year_from_datetime(&utc_now)
            );
    
    println!(" Local: {} -> Year {} is leap year: {}",
            local_now.format("%Y-%m-%d %H:%M:%S"),
            local_now.year(),
            is_leap_year_from_datetime(&local_now)
            );

    println!();
}

fn demostrate_test_datetime() {
    println!("2. Specific Test Dates:");

    let test_dates = [
        "2024-03-15T10:30:00Z", // Leap year
        "2023-07-20T14:45:00Z", // Not leap year
        "2000-12-31T23:59:59Z", // Leap year (divisible by 400)
        "1900-01-01T00:00:00Z", // Not leap year (divisible by 100 but not 400)
    ];

    for date_str in &test_dates {
        let datetime = date_str.parse::<DateTime<Utc>>().unwrap();
        println!("   {} -> Year {} is leap year: {}", 
                 date_str, 
                 datetime.year(), 
                 is_leap_year_from_datetime(&datetime));
    }
    println!();
}

fn demostrate_naive_datetime() {
    println!("3. NaiveDateTime Examples:");

    let test_cases = [
        "2024-02-29 12:00:00", // Leap year date
        "2023-03-15 08:30:00", // Non-leap year date
    ];

    for date_str in &test_cases {
        let naive_dt = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S").unwrap();
        println!("   {} -> Year {} is leap year: {}", 
                 date_str, 
                 naive_dt.year(), 
                 is_leap_year_from_naive_datetime(&naive_dt));
    }
    println!();
}

fn demonstrate_historical_examples() {
    println!("3. NaiveDateTime Examples:");

    let historical_years = [
        (1896, "Regular leap year"),
        (1900, "Century year, NOT leap (divisible by 100, not 400)"),
        (2000, "Century year, IS leap (divisible by 400)"),
        (2024, "Regular leap year"),
        (2100, "Future century year, NOT leap"),
    ];

    for (year, description) in &historical_years {
        println!(" {} -> {} -> Leap: {}", year, description, is_leap_year(*year));
    }
}