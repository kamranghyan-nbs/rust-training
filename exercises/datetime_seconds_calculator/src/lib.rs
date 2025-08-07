use chrono::{DateTime, NaiveDateTime, Utc, Local};

pub fn seconds_from_now_naive(datetime: &NaiveDateTime) -> i64 {
    let now = Utc::now().naive_utc();
    datetime.signed_duration_since(now).num_seconds()
}

pub fn seconds_from_now_utc(datetime: &DateTime<Utc>) -> i64 {
    let now = Utc::now();
    datetime.signed_duration_since(now).num_seconds()
}

pub fn seconds_from_now_local(datetime: &DateTime<Local>) -> i64 {
    let now = Local::now();
    datetime.signed_duration_since(now).num_seconds()
}

pub fn seconds_from_now<Tz: chrono::TimeZone>(datetime: &DateTime<Tz>) -> i64 {
    let now_utc = Utc::now();
    let datetime_utc = datetime.with_timezone(&Utc);
    datetime_utc.signed_duration_since(now_utc).num_seconds()
}

pub fn format_time_difference(seconds: i64) -> String {
    let abs_seconds = seconds.abs();
    let direction = if seconds >= 0 { "in the future" } else { "in the past" };
    
    if abs_seconds < 60 {
        format!("{} second{} {}", abs_seconds, if abs_seconds == 1 { "" } else { "s" }, direction)
    } else if abs_seconds < 3600 {
        let minutes = abs_seconds / 60;
        let remaining_seconds = abs_seconds % 60;
        if remaining_seconds == 0 {
            format!("{} minute{} {}", minutes, if minutes == 1 { "" } else { "s" }, direction)
        } else {
            format!("{} minute{}, {} second{} {}", 
                   minutes, if minutes == 1 { "" } else { "s" },
                   remaining_seconds, if remaining_seconds == 1 { "" } else { "s" },
                   direction)
        }
    } else if abs_seconds < 86400 {
        let hours = abs_seconds / 3600;
        let remaining_minutes = (abs_seconds % 3600) / 60;
        if remaining_minutes == 0 {
            format!("{} hour{} {}", hours, if hours == 1 { "" } else { "s" }, direction)
        } else {
            format!("{} hour{}, {} minute{} {}", 
                   hours, if hours == 1 { "" } else { "s" },
                   remaining_minutes, if remaining_minutes == 1 { "" } else { "s" },
                   direction)
        }
    } else {
        let days = abs_seconds / 86400;
        let remaining_hours = (abs_seconds % 86400) / 3600;
        if remaining_hours == 0 {
            format!("{} day{} {}", days, if days == 1 { "" } else { "s" }, direction)
        } else {
            format!("{} day{}, {} hour{} {}", 
                   days, if days == 1 { "" } else { "s" },
                   remaining_hours, if remaining_hours == 1 { "" } else { "s" },
                   direction)
        }
    }
}

pub fn absolute_seconds_from_now<Tz: chrono::TimeZone>(datetime: &DateTime<Tz>) -> u64 {
    seconds_from_now(datetime).unsigned_abs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};

    #[test]
    fn test_seconds_from_now_utc_future() {
        let future_dt = Utc::now() + Duration::hours(2);
        let seconds = seconds_from_now_utc(&future_dt);
        
        // Should be approximately 7200 seconds (2 hours)
        assert!(seconds > 7000 && seconds < 7300);
    }

    #[test]
    fn test_seconds_from_now_utc_past() {
        let past_dt = Utc::now() - Duration::minutes(30);
        let seconds = seconds_from_now_utc(&past_dt);
        
        // Should be approximately -1800 seconds (30 minutes ago)
        assert!(seconds < -1700 && seconds > -1900);
    }

    #[test]
    fn test_seconds_from_now_naive() {
        let future_naive = (Utc::now() + Duration::hours(1)).naive_utc();
        let seconds = seconds_from_now_naive(&future_naive);
        
        // Should be approximately 3600 seconds (1 hour)
        assert!(seconds > 3500 && seconds < 3700);
    }

    #[test]
    fn test_seconds_from_now_local() {
        let future_local = Local::now() + Duration::minutes(15);
        let seconds = seconds_from_now_local(&future_local);
        
        // Should be approximately 900 seconds (15 minutes)
        assert!(seconds > 800 && seconds < 1000);
    }

    #[test]
    fn test_generic_seconds_from_now() {
        let utc_dt = Utc::now() + Duration::minutes(10);
        let local_dt = Local::now() - Duration::minutes(5);
        
        let utc_seconds = seconds_from_now(&utc_dt);
        let local_seconds = seconds_from_now(&local_dt);
        
        // Future should be positive, past should be negative
        assert!(utc_seconds > 0);
        assert!(local_seconds < 0);
    }

    #[test]
    fn test_absolute_seconds() {
        let past_dt = Utc::now() - Duration::hours(1);
        let future_dt = Utc::now() + Duration::hours(1);
        
        let past_abs = absolute_seconds_from_now(&past_dt);
        let future_abs = absolute_seconds_from_now(&future_dt);
        
        // Both should be positive and approximately 3600
        assert!(past_abs > 3500 && past_abs < 3700);
        assert!(future_abs > 3500 && future_abs < 3700);
    }

    #[test]
    fn test_format_time_difference() {
        let one_minute = format_time_difference(60);
        let one_hour = format_time_difference(3600);
        let negative_time = format_time_difference(-1800);
        
        assert!(one_minute.contains("1 minute"));
        assert!(one_minute.contains("future"));
        assert!(one_hour.contains("1 hour"));
        assert!(negative_time.contains("past"));
    }
}