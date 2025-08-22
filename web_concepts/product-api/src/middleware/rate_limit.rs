use crate::{error::AppError, models::Claims};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

#[derive(Clone)]
pub struct RateLimiter {
    // IP-based rate limiting
    ip_requests: Arc<Mutex<HashMap<IpAddr, Vec<Instant>>>>,
    // User-based rate limiting  
    user_requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    // Configuration
    max_requests_per_minute: u32,
    max_requests_per_user_per_minute: u32,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests_per_minute: u32, max_requests_per_user_per_minute: u32) -> Self {
        Self {
            ip_requests: Arc::new(Mutex::new(HashMap::new())),
            user_requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests_per_minute,
            max_requests_per_user_per_minute,
            window_duration: Duration::from_secs(60),
        }
    }

    pub fn check_ip_rate_limit(&self, ip: IpAddr) -> bool {
        let mut requests = self.ip_requests.lock().unwrap();
        let now = Instant::now();

        // Clean old requests
        let cutoff = now - self.window_duration;
        
        let ip_requests = requests.entry(ip).or_insert_with(Vec::new);
        ip_requests.retain(|&time| time > cutoff);

        // Check if under limit
        if ip_requests.len() < self.max_requests_per_minute as usize {
            ip_requests.push(now);
            true
        } else {
            false
        }
    }

    pub fn check_user_rate_limit(&self, user_id: &str) -> bool {
        let mut requests = self.user_requests.lock().unwrap();
        let now = Instant::now();

        // Clean old requests
        let cutoff = now - self.window_duration;
        
        let user_requests = requests.entry(user_id.to_string()).or_insert_with(Vec::new);
        user_requests.retain(|&time| time > cutoff);

        // Check if under limit
        if user_requests.len() < self.max_requests_per_user_per_minute as usize {
            user_requests.push(now);
            true
        } else {
            false
        }
    }

    pub fn get_ip_remaining_requests(&self, ip: IpAddr) -> u32 {
        let requests = self.ip_requests.lock().unwrap();
        let now = Instant::now();
        let cutoff = now - self.window_duration;

        if let Some(ip_requests) = requests.get(&ip) {
            let recent_requests = ip_requests.iter().filter(|&&time| time > cutoff).count();
            self.max_requests_per_minute.saturating_sub(recent_requests as u32)
        } else {
            self.max_requests_per_minute
        }
    }

    pub fn get_user_remaining_requests(&self, user_id: &str) -> u32 {
        let requests = self.user_requests.lock().unwrap();
        let now = Instant::now();
        let cutoff = now - self.window_duration;

        if let Some(user_requests) = requests.get(user_id) {
            let recent_requests = user_requests.iter().filter(|&&time| time > cutoff).count();
            self.max_requests_per_user_per_minute.saturating_sub(recent_requests as u32)
        } else {
            self.max_requests_per_user_per_minute
        }
    }
}

// Middleware for IP-based rate limiting (for public endpoints)
pub async fn ip_rate_limit_middleware(
    State(rate_limiter): State<RateLimiter>,
    mut request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Extract IP address
    let ip = extract_ip_from_request(&request);

    if !rate_limiter.check_ip_rate_limit(ip) {
        let remaining = rate_limiter.get_ip_remaining_requests(ip);
        
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            [
                ("X-RateLimit-Limit", rate_limiter.max_requests_per_minute.to_string()),
                ("X-RateLimit-Remaining", remaining.to_string()),
                ("X-RateLimit-Reset", "60".to_string()),
            ],
            "Rate limit exceeded. Too many requests from this IP.",
        ).into_response());
    }

    let remaining = rate_limiter.get_ip_remaining_requests(ip);
    let mut response = next.run(request).await;
    
    // Add rate limit headers
    response.headers_mut().insert(
        "X-RateLimit-Limit",
        rate_limiter.max_requests_per_minute.to_string().parse().unwrap(),
    );
    response.headers_mut().insert(
        "X-RateLimit-Remaining",
        remaining.to_string().parse().unwrap(),
    );

    Ok(response)
}

// Middleware for user-based rate limiting (for authenticated endpoints)
pub async fn user_rate_limit_middleware(
    State(rate_limiter): State<RateLimiter>,
    mut request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Extract user from request extensions (set by auth middleware)
    let claims = request.extensions().get::<Claims>().cloned();

    if let Some(claims) = claims {
        if !rate_limiter.check_user_rate_limit(&claims.sub) {
            let remaining = rate_limiter.get_user_remaining_requests(&claims.sub);
            
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                [
                    ("X-RateLimit-Limit", rate_limiter.max_requests_per_user_per_minute.to_string()),
                    ("X-RateLimit-Remaining", remaining.to_string()),
                    ("X-RateLimit-Reset", "60".to_string()),
                ],
                "Rate limit exceeded. Too many requests for this user.",
            ).into_response());
        }

        let remaining = rate_limiter.get_user_remaining_requests(&claims.sub);
        let mut response = next.run(request).await;
        
        // Add rate limit headers
        response.headers_mut().insert(
            "X-RateLimit-Limit",
            rate_limiter.max_requests_per_user_per_minute.to_string().parse().unwrap(),
        );
        response.headers_mut().insert(
            "X-RateLimit-Remaining",
            remaining.to_string().parse().unwrap(),
        );

        Ok(response)
    } else {
        // No user info, fall back to IP-based rate limiting
        ip_rate_limit_middleware(State(rate_limiter), request, next).await
    }
}

fn extract_ip_from_request(request: &Request) -> IpAddr {
    // Try to get IP from X-Forwarded-For header (if behind proxy)
    if let Some(forwarded_for) = request.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            if let Some(ip_str) = forwarded_str.split(',').next() {
                if let Ok(ip) = ip_str.trim().parse::<IpAddr>() {
                    return ip;
                }
            }
        }
    }

    // Try to get IP from X-Real-IP header
    if let Some(real_ip) = request.headers().get("X-Real-IP") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                return ip;
            }
        }
    }

    // Fallback to a default IP if we can't determine the real IP
    // In a real application, you might want to extract this from the connection info
    "127.0.0.1".parse().unwrap()
}