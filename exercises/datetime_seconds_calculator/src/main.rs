use datetime_seconds_calculator::{
    seconds_from_now_naive,
    seconds_from_now_utc,
    seconds_from_now_local,
    seconds_from_now,
    format_time_difference,
    absolute_seconds_from_now,
};
use chrono::{DateTime, NaiveDateTime, Utc, Local, Duration, TimeZone};

fn main() {
    println!("=== DateTime Seconds Calculator ===\n");

    // Example 1: Different datetime types
    demonstrate_basic_calculations();

    // Example 2: Past vs Future
    demonstrate_past_vs_future();

    // Example 3: Human-readable formatting
    demonstrate_formatting();

    // Example 4: Real-world scenarios
    demonstrate_real_world_usage();
}

fn demonstrate_basic_calculations() {
    println!("1. Basic Calculations with Different Types:");
    
    // Create sample datetimes
    let naive_dt = NaiveDateTime::parse_from_str("2030-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let utc_dt = Utc::now() + Duration::hours(6);
    let local_dt = Local::now() - Duration::minutes(45);
    
    println!("   NaiveDateTime (2030-01-01): {} seconds", seconds_from_now_naive(&naive_dt));
    println!("   UTC (6 hours future):       {} seconds", seconds_from_now_utc(&utc_dt));
    println!("   Local (45 min past):        {} seconds", seconds_from_now_local(&local_dt));
    
    // Generic function
    println!("   Generic UTC:                {} seconds", seconds_from_now(&utc_dt));
    println!("   Generic Local:              {} seconds", seconds_from_now(&local_dt));
    println!();
}

fn demonstrate_past_vs_future() {
    println!("2. Past vs Future Calculations:");
    
    let test_times = vec![
        ("1 hour ago", Utc::now() - Duration::hours(1)),
        ("30 minutes ago", Utc::now() - Duration::minutes(30)),
        ("Right now", Utc::now()),
        ("15 minutes from now", Utc::now() + Duration::minutes(15)),
        ("2 hours from now", Utc::now() + Duration::hours(2)),
    ];
    
    for (description, dt) in test_times {
        let seconds = seconds_from_now_utc(&dt);
        let sign = if seconds >= 0 { "+" } else { "" };
        println!("   {:<20} -> {:>8}{} seconds", description, sign, seconds);
    }
    println!();
}

fn demonstrate_formatting() {
    println!("3. Human-readable Time Differences:");
    
    let test_durations = vec![
        Duration::seconds(45),
        Duration::minutes(5),
        Duration::minutes(90),  // 1.5 hours
        Duration::hours(3),
        Duration::days(2) + Duration::hours(5),
        Duration::seconds(-30),  // Past
        Duration::hours(-4),     // Past
    ];
    
    for duration in test_durations {
        let future_dt = Utc::now() + duration;
        let seconds = seconds_from_now_utc(&future_dt);
        let formatted = format_time_difference(seconds);
        println!("   {} -> {}", seconds, formatted);
    }
    println!();
}

fn demonstrate_real_world_usage() {
    println!("4. Real-world Usage Examples:");
    
    // Example: Event scheduling
    println!("   Event Scheduling:");
    let events = vec![
        ("Meeting start", Utc::now() + Duration::minutes(30)),
        ("Lunch break", Utc::now() + Duration::hours(2)),
        ("Project deadline", Utc::now() + Duration::days(5)),
    ];
    
    for (event_name, event_time) in events {
        let seconds = seconds_from_now_utc(&event_time);
        let readable = format_time_difference(seconds);
        println!("     {}: {} ({})", event_name, readable, seconds);
    }
    
    println!();
    
    // Example: Log analysis
    println!("   Log Entry Analysis:");
    let log_entries = vec![
        ("System start", Utc::now() - Duration::hours(8)),
        ("Last backup", Utc::now() - Duration::minutes(15)),
        ("Error occurred", Utc::now() - Duration::seconds(30)),
    ];
    
    for (log_type, log_time) in log_entries {
        let seconds = seconds_from_now_utc(&log_time);
        let abs_seconds = absolute_seconds_from_now(&log_time);
        println!("     {}: {} seconds ago (absolute: {})", log_type, seconds.abs(), abs_seconds);
    }
    
    println!();
    
    // Example: Timer/countdown
    println!("   Countdown Timer:");
    let target_time = Utc::now() + Duration::minutes(10) + Duration::seconds(30);
    let countdown_seconds = seconds_from_now_utc(&target_time);
    
    println!("     Target time: {}", target_time.format("%H:%M:%S"));
    println!("     Time remaining: {} seconds", countdown_seconds);
    println!("     Readable: {}", format_time_difference(countdown_seconds));
    
    // Simulate countdown
    println!("     Countdown simulation:");
    for i in (0..=3).rev() {
        let sim_time = Utc::now() + Duration::seconds(i);
        let sim_seconds = seconds_from_now_utc(&sim_time);
        println!("       T-{} seconds: {}", i, sim_seconds);
    }
}