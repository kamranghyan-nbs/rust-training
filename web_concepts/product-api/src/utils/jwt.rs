use crate::{error::AppError, models::Claims};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn create_jwt(user_id: &str, username: &str, secret: &str, expiration: i64) -> Result<String, AppError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| AppError::InternalServerError)?
        .as_secs();

    let claims = Claims {
        sub: user_id.to_owned(),
        username: username.to_owned(),
        exp: (now + expiration as u64) as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(AppError::from)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(AppError::from)
}


#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;
    use mockall::predicate::*;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    // Step 1: Trait to abstract time (so we can mock it)
    #[automock]
    trait TimeProvider {
        fn now(&self) -> SystemTime;
    }

    struct RealTime;

    impl TimeProvider for RealTime {
        fn now(&self) -> SystemTime {
            SystemTime::now()
        }
    }

    // Step 2: Helper function to create a JWT using mocked time
    fn create_jwt_with_time(
        time_provider: &dyn TimeProvider,
        user_id: &str,
        username: &str,
        secret: &str,
        expiration: i64,
    ) -> Result<String, AppError> {
        let now = time_provider
            .now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppError::InternalServerError)?
            .as_secs();

        let claims = Claims {
            sub: user_id.to_owned(),
            username: username.to_owned(),
            exp: (now + expiration as u64) as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(AppError::from)
    }

    #[test]
    fn test_create_and_verify_jwt() {
        let secret = "my-secret";
        let user_id = "123";
        let username = "testuser";

        // Mock time to a fixed point
        let mut mock_time = MockTimeProvider::new();
        mock_time.expect_now()
            .return_const(UNIX_EPOCH + Duration::from_secs(1_000_000));

        let token = create_jwt_with_time(&mock_time, user_id, username, secret, 3600).unwrap();

        // Verify token
        let claims = verify_jwt(&token, secret).unwrap();
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.username, username);
        assert_eq!(claims.exp, 1_000_000 + 3600);
    }

    #[test]
    fn test_verify_jwt_invalid_signature() {
        let secret = "secret1";
        let bad_secret = "wrong-secret";
        let token = create_jwt("123", "user", secret, 3600).unwrap();

        let result = verify_jwt(&token, bad_secret);
        assert!(result.is_err());
        if let Err(AppError::JwtError(_)) = result {
            // expected
        } else {
            panic!("Expected JwtError for invalid signature");
        }
    }

    #[test]
    fn test_create_jwt_with_time_error() {
        // Simulate SystemTime before UNIX_EPOCH to force error
        let mut mock_time = MockTimeProvider::new();
        mock_time.expect_now()
            .return_const(UNIX_EPOCH - Duration::from_secs(1));

        let result = create_jwt_with_time(&mock_time, "id", "user", "secret", 3600);
        assert!(matches!(result, Err(AppError::InternalServerError)));
    }

    #[test]
    fn test_verify_expired_jwt() {
        let secret = "secret";
        let user_id = "123";
        let username = "user";

        // Create a token that expired 10 seconds ago
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let claims = Claims {
            sub: user_id.into(),
            username: username.into(),
            exp: (now - 10) as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap();

        let result = verify_jwt(&token, secret);
        assert!(matches!(result, Err(AppError::JwtError(_))));
    }
}
