use unix_timestamp_converter::{
    timestamp_to_naive_datetime,
    timestamp_to_utc,
    timestamp_to_local,
    timestamp_f64_to_naive_datetime,
    timestamp_f64_to_utc,
    get_current_timestamp,
    current_timestamp_to_datetime,
    is_valid_timestamp,
    convert_multiple_timestamps,
    timestamp_to_formatted_string,
};

use chrono::Timelike;

fn main() {
    println!("=== Unix Timestamp to DateTime Converter ===\n");

    // Example 1: Basic conversions
    demonstrate_basic_conversions();

    // Example 2: Fractional seconds
    demonstrate_fractional_seconds();

    // Example 3: Current timestamp
    demonstrate_current_timestamp();

    // Example 4: Batch processing
    demonstrate_batch_processing();

    // Example 5: Real-world examples
    demonstrate_real_world_examples();

    // Example 6: Validation and formatting
    demonstrate_validation_and_formatting();
}

fn demonstrate_basic_conversions() {
    println!("1. Basic Timestamp Conversions:");
    
    let test_timestamp = 1404230580; // 2014-07-01 14:43:00 UTC
    println!("   Unix timestamp: {}", test_timestamp);
    
    if let Some(naive_dt) = timestamp_to_naive_datetime(test_timestamp) {
        println!("   NaiveDateTime:  {}", naive_dt);
    }
    
    if let Some(utc_dt) = timestamp_to_utc(test_timestamp) {
        println!("   UTC DateTime:   {}", utc_dt);
    }
    
    if let Some(local_dt) = timestamp_to_local(test_timestamp) {
        println!("   Local DateTime: {}", local_dt);
        println!("   Local offset:   {}", local_dt.offset());
    }
    println!();
}

fn demonstrate_fractional_seconds() {
    println!("2. Fractional Seconds Support:");
    
    let fractional_timestamp = 1404230580.75; // .75 = 750 milliseconds
    println!("   Fractional timestamp: {}", fractional_timestamp);
    
    if let Some(naive_dt) = timestamp_f64_to_naive_datetime(fractional_timestamp) {
        println!("   NaiveDateTime: {}", naive_dt);
        println!("   Nanoseconds:   {}", naive_dt.nanosecond());
    }
    
    if let Some(utc_dt) = timestamp_f64_to_utc(fractional_timestamp) {
        println!("   UTC DateTime:  {}", utc_dt.format("%Y-%m-%d %H:%M:%S%.3f"));
    }
    println!();
}

fn demonstrate_current_timestamp() {
    println!("3. Current Timestamp:");
    
    let current_ts = get_current_timestamp();
    let current_dt = current_timestamp_to_datetime();
    
    println!("   Current timestamp:  {}", current_ts);
    println!("   Current datetime:   {}", current_dt);
    
    // Convert back to verify
    if let Some(converted) = timestamp_to_utc(current_ts) {
        println!("   Roundtrip check:    {}", converted);
        println!("   Match:              {}", (converted.timestamp() - current_ts).abs() <= 1);
    }
    println!();
}

fn demonstrate_batch_processing() {
    println!("4. Batch Processing:");
    
    let timestamps = vec![
        0,           // Unix epoch
        946684800,   // Y2K (2000-01-01)
        1404230580,  // 2014-07-01 14:43:00
        1609459200,  // 2021-01-01 00:00:00
        -1,          // Invalid (before epoch)
    ];
    
    println!("   Converting {} timestamps...", timestamps.len());
    let results = convert_multiple_timestamps(timestamps.iter().cloned());
    
    for (i, (timestamp, result)) in timestamps.iter().zip(results.iter()).enumerate() {
        match result {
            Some(dt) => println!("   {}: {} -> {}", i + 1, timestamp, dt.format("%Y-%m-%d %H:%M:%S")),
            None => println!("   {}: {} -> Invalid timestamp", i + 1, timestamp),
        }
    }
    println!();
}

fn demonstrate_real_world_examples() {
    println!("5. Real-world Examples:");
    
    // Common timestamps you might encounter
    let examples = vec![
        (0, "Unix Epoch"),
        (946684800, "Y2K (Millennium)"),
        (1234567890, "Famous test timestamp"),
        (1404230580, "Example date"),
        (1609459200, "New Year 2021"),
        (1672531200, "New Year 2023"),
    ];
    
    for (timestamp, description) in examples {
        if let Some(dt) = timestamp_to_utc(timestamp) {
            println!("   {} ({}): {}", 
                   timestamp, 
                   description, 
                   dt.format("%A, %B %d, %Y at %H:%M:%S UTC"));
        }
    }
    println!();
}

fn demonstrate_validation_and_formatting() {
    println!("6. Validation and Custom Formatting:");
    
    let test_timestamps = vec![
        -1,           // Invalid - before epoch
        0,            // Valid - epoch
        1404230580,   // Valid - normal date
        2147483647,   // Valid - but near limit
        3000000000,   // Invalid - too far in future
    ];
    
    println!("   Timestamp Validation:");
    for timestamp in &test_timestamps {
        let is_valid = is_valid_timestamp(*timestamp);
        let status = if is_valid { "✓ Valid" } else { "✗ Invalid" };
        println!("   {} -> {}", status, timestamp);
    }
    
    println!("\n   Custom Formatting:");
    let sample_timestamp = 1404230580;
    let formats = vec![
        ("%Y-%m-%d %H:%M:%S", "Standard"),
        ("%Y-%m-%dT%H:%M:%SZ", "ISO 8601"),
        ("%A, %B %d, %Y", "Friendly date"),
        ("%H:%M:%S", "Time only"),
        ("%Y%m%d", "Compact date"),
    ];
    
    for (format, description) in formats {
        if let Some(formatted) = timestamp_to_formatted_string(sample_timestamp, format) {
            println!("   {:<15}: '{}'", description, formatted);
        }
    }
}