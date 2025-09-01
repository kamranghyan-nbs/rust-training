use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use chrono::{Duration, Utc};
use crate_errors::{AppError, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,           // User ID
    pub tenant_id: Uuid,     // Tenant ID for multi-tenancy
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at
    pub iss: String,        // Issuer
}

#[derive(Clone)]
pub struct JwtManager {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    issuer: String,
    access_token_expire_minutes: u64,
    refresh_token_expire_days: u64,
}

impl JwtManager {
    pub fn new(
        secret: &str,
        issuer: String,
        access_token_expire_minutes: u64,
        refresh_token_expire_days: u64,
    ) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_ref()),
            decoding_key: DecodingKey::from_secret(secret.as_ref()),
            issuer,
            access_token_expire_minutes,
            refresh_token_expire_days,
        }
    }

    pub fn create_access_token(&self, claims: &Claims) -> Result<String> {
        let mut claims = claims.clone();
        let now = Utc::now();
        claims.iat = now.timestamp();
        claims.exp = (now + Duration::minutes(self.access_token_expire_minutes as i64)).timestamp();
        claims.iss = self.issuer.clone();

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(AppError::Jwt)
    }

    pub fn create_refresh_token(&self, user_id: Uuid, tenant_id: Uuid) -> Result<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id,
            tenant_id,
            email: String::new(), // Refresh tokens don't need email
            roles: vec![],
            permissions: vec![],
            iat: now.timestamp(),
            exp: (now + Duration::days(self.refresh_token_expire_days as i64)).timestamp(),
            iss: self.issuer.clone(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(AppError::Jwt)
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&self.issuer]);

        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(AppError::Jwt)
    }

    pub fn is_token_expired(&self, claims: &Claims) -> bool {
        let now = Utc::now().timestamp();
        claims.exp < now
    }
}

#[derive(Debug, Clone)]
pub struct PasswordManager;

impl PasswordManager {
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(format!("Invalid password hash: {}", e)))?;
        
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}

#[derive(Debug, Clone)]
pub struct PermissionChecker;

impl PermissionChecker {
    pub fn has_permission(user_permissions: &[String], required_permission: &str) -> bool {
        user_permissions.iter().any(|p| {
            // Support wildcard permissions like "users.*" matching "users.create"
            if p.ends_with('*') {
                let prefix = &p[..p.len() - 1];
                required_permission.starts_with(prefix)
            } else {
                p == required_permission
            }
        })
    }

    pub fn has_any_role(user_roles: &[String], required_roles: &[String]) -> bool {
        let user_roles_set: HashSet<&String> = user_roles.iter().collect();
        let required_roles_set: HashSet<&String> = required_roles.iter().collect();
        !user_roles_set.is_disjoint(&required_roles_set)
    }

    pub fn has_all_roles(user_roles: &[String], required_roles: &[String]) -> bool {
        let user_roles_set: HashSet<&String> = user_roles.iter().collect();
        required_roles.iter().all(|role| user_roles_set.contains(role))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "test_password_123";
        let hash = PasswordManager::hash_password(password).unwrap();
        
        assert!(PasswordManager::verify_password(password, &hash).unwrap());
        assert!(!PasswordManager::verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_jwt_creation_and_verification() {
        let jwt_manager = JwtManager::new(
            "test_secret_key_should_be_long_enough",
            "test_issuer".to_string(),
            60, // 1 hour
            7,  // 7 days
        );

        let claims = Claims {
            sub: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            roles: vec!["admin".to_string()],
            permissions: vec!["users.create".to_string(), "users.read".to_string()],
            exp: 0, // Will be set by create_access_token
            iat: 0, // Will be set by create_access_token
            iss: String::new(), // Will be set by create_access_token
        };

        let token = jwt_manager.create_access_token(&claims).unwrap();
        let verified_claims = jwt_manager.verify_token(&token).unwrap();

        assert_eq!(verified_claims.sub, claims.sub);
        assert_eq!(verified_claims.tenant_id, claims.tenant_id);
        assert_eq!(verified_claims.email, claims.email);
        assert_eq!(verified_claims.roles, claims.roles);
        assert_eq!(verified_claims.permissions, claims.permissions);
    }

    #[test]
    fn test_permission_checking() {
        let user_permissions = vec![
            "users.create".to_string(),
            "users.read".to_string(),
            "products.*".to_string(),
        ];

        assert!(PermissionChecker::has_permission(&user_permissions, "users.create"));
        assert!(PermissionChecker::has_permission(&user_permissions, "users.read"));
        assert!(PermissionChecker::has_permission(&user_permissions, "products.create"));
        assert!(PermissionChecker::has_permission(&user_permissions, "products.update"));
        assert!(!PermissionChecker::has_permission(&user_permissions, "orders.create"));
    }
}