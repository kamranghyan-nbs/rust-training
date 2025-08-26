#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::str::FromStr;

    // Import your rate limiter from the main crate
    // Note: This assumes your main crate is named "product_api"
    // Adjust the import path based on your actual crate name
    use product_api::middleware::rate_limit::RateLimiter;

    #[test]
    fn test_ip_rate_limiting() {
        let rate_limiter = RateLimiter::new(5, 10); // 5 requests per minute per IP
        let test_ip = IpAddr::from_str("192.168.1.1").unwrap();

        // First 5 requests should succeed
        for i in 0..5 {
            assert!(
                rate_limiter.check_ip_rate_limit(test_ip),
                "Request {} should have been allowed",
                i + 1
            );
        }

        // 6th request should be rate limited
        assert!(
            !rate_limiter.check_ip_rate_limit(test_ip),
            "6th request should have been rate limited"
        );

        // Check remaining requests
        assert_eq!(rate_limiter.get_ip_remaining_requests(test_ip), 0);
    }

    #[test]
    fn test_user_rate_limiting() {
        let rate_limiter = RateLimiter::new(5, 3); // 3 requests per minute per user
        let test_user = "user123";

        // First 3 requests should succeed
        for i in 0..3 {
            assert!(
                rate_limiter.check_user_rate_limit(test_user),
                "Request {} should have been allowed",
                i + 1
            );
        }

        // 4th request should be rate limited
        assert!(
            !rate_limiter.check_user_rate_limit(test_user),
            "4th request should have been rate limited"
        );

        // Check remaining requests
        assert_eq!(rate_limiter.get_user_remaining_requests(test_user), 0);
    }

    #[test]
    fn test_different_ips_separate_limits() {
        let rate_limiter = RateLimiter::new(2, 5); // 2 requests per minute per IP
        let ip1 = IpAddr::from_str("192.168.1.1").unwrap();
        let ip2 = IpAddr::from_str("192.168.1.2").unwrap();

        // Use up limit for IP1
        assert!(rate_limiter.check_ip_rate_limit(ip1));
        assert!(rate_limiter.check_ip_rate_limit(ip1));
        assert!(!rate_limiter.check_ip_rate_limit(ip1)); // Should be rate limited

        // IP2 should still have full limit available
        assert!(rate_limiter.check_ip_rate_limit(ip2));
        assert!(rate_limiter.check_ip_rate_limit(ip2));
        assert!(!rate_limiter.check_ip_rate_limit(ip2)); // Should be rate limited
    }

    #[test]
    fn test_different_users_separate_limits() {
        let rate_limiter = RateLimiter::new(5, 2); // 2 requests per minute per user
        let user1 = "user1";
        let user2 = "user2";

        // Use up limit for user1
        assert!(rate_limiter.check_user_rate_limit(user1));
        assert!(rate_limiter.check_user_rate_limit(user1));
        assert!(!rate_limiter.check_user_rate_limit(user1)); // Should be rate limited

        // user2 should still have full limit available
        assert!(rate_limiter.check_user_rate_limit(user2));
        assert!(rate_limiter.check_user_rate_limit(user2));
        assert!(!rate_limiter.check_user_rate_limit(user2)); // Should be rate limited
    }
}
