use datetime_formatter::{
    format_naive_datetime,
    format_utc_datetime,
    format_local_datetime,
    format_datetime,
    format_current_utc,
    format_current_local,
    format_multiple,
    TARGET_FORMAT,
};
use chrono::{DateTime, NaiveDateTime, Utc, Local, TimeZone};

fn main() {
    println!("=== DateTime to String Formatter ===");
    println!("Target format: {}\n", TARGET_FORMAT);

    // Example 1: Format different datetime types
    demonstrate_basic_formatting();

    // Example 2: Format current time
    demonstrate_current_time();

    // Example 3: Generic formatter
    demonstrate_generic_formatter();

    // Example 4: Batch formatting
    demonstrate_batch_formatting();

    // Example 5: Real-world scenarios
    demonstrate_real_world_usage();
}

fn demonstrate_basic_formatting() {
    println!("1. Basic DateTime Formatting:");
    
    // Create sample datetime objects
    let naive_dt = NaiveDateTime::parse_from_str("2014-07-01T14:43:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let utc_dt = Utc.ymd(2014, 7, 1).and_hms(14, 43, 0);
    let local_dt = Local.ymd(2014, 7, 1).and_hms(14, 43, 0);
    
    println!("   NaiveDateTime -> '{}'", format_naive_datetime(&naive_dt));
    println!("   UTC DateTime  -> '{}'", format_utc_datetime(&utc_dt));
    println!("   Local DateTime-> '{}'", format_local_datetime(&local_dt));
    println!();
}

fn demonstrate_current_time() {
    println!("2. Current Time Formatting:");
    
    let utc_now = format_current_utc();
    let local_now = format_current_local();
    
    println!("   Current UTC:   '{}'", utc_now);
    println!("   Current Local: '{}'", local_now);
    println!();
}

fn demonstrate_generic_formatter() {
    println!("3. Generic Formatter:");
    
    let utc_dt = Utc::now();
    let local_dt = Local::now();
    
    // Same function works for both timezone types
    println!("   UTC (generic):   '{}'", format_datetime(&utc_dt));
    println!("   Local (generic): '{}'", format_datetime(&local_dt));
    println!();
}

fn demonstrate_batch_formatting() {
    println!("4. Batch Formatting:");
    
    // Create multiple datetime objects
    let datetimes = vec![
        NaiveDateTime::parse_from_str("2014-07-01 14:43:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        NaiveDateTime::parse_from_str("2020-12-25 09:30:15", "%Y-%m-%d %H:%M:%S").unwrap(),
        NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    ];
    
    let datetime_refs: Vec<&NaiveDateTime> = datetimes.iter().collect();
    let formatted_list = format_multiple(datetime_refs.into_iter());
    
    println!("   Formatted {} datetimes:", formatted_list.len());
    for (i, formatted) in formatted_list.iter().enumerate() {
        println!("   {}. '{}'", i + 1, formatted);
    }
    println!();
}

fn demonstrate_real_world_usage() {
    println!("5. Real-world Usage Examples:");
    
    // Simulate log timestamp formatting
    println!("   Log Entry Timestamps:");
    let events = vec![
        ("User login", Utc::now()),
        ("File uploaded", Utc::now()),
        ("Session expired", Utc::now()),
    ];
    
    for (event, timestamp) in events {
        let formatted_time = format_utc_datetime(&timestamp);
        println!("   [{} - {}] Event occurred", formatted_time, event);
    }
    
    println!();
    
    // Simulate database record formatting
    println!("   Database Record Formatting:");
    let records = vec![
        Utc.ymd(2024, 1, 15).and_hms(10, 30, 0),
        Utc.ymd(2024, 2, 28).and_hms(16, 45, 30),
        Utc.ymd(2024, 3, 10).and_hms(8, 15, 45),
    ];
    
    for (i, record_time) in records.iter().enumerate() {
        println!("   Record {}: created_at = '{}'", 
                i + 1, 
                format_datetime(record_time));
    }
    
    println!();
    
    // Show format consistency
    println!("   Format Consistency Check:");
    let same_moment_different_tz = Utc.ymd(2014, 7, 1).and_hms(14, 43, 0);
    let as_local = same_moment_different_tz.with_timezone(&Local);
    
    println!("   Same moment in UTC:   '{}'", format_datetime(&same_moment_different_tz));
    println!("   Same moment in Local: '{}'", format_datetime(&as_local));
    println!("   (Note: Times may differ due to timezone conversion)");
}
