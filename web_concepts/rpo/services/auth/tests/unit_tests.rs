use crate_security::{JwtManager, PasswordManager, PermissionChecker};
use uuid::Uuid;

#[test]
fn test_password_hashing_and_verification() {
    let password = "test_password_123";
    let hash = PasswordManager::hash_password(password).unwrap();
    
    assert!(PasswordManager::verify_password(password, &hash).unwrap());
    assert!(!PasswordManager::verify_password("wrong_password", &hash).unwrap());
}

#[test]
fn test_jwt_token_creation_and_verification() {
    let jwt_manager = JwtManager::new(
        "test_secret_key_should_be_long_enough",
        "test_issuer".to_string(),
        60,
        7,
    );

    let claims = crate_security::Claims {
        sub: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        roles: vec!["admin".to_string()],
        permissions: vec!["users.create".to_string(), "users.read".to_string()],
        exp: 0,
        iat: 0,
        iss: String::new(),
    };

    let token = jwt_manager.create_access_token(&claims).unwrap();
    let verified_claims = jwt_manager.verify_token(&token).unwrap();

    assert_eq!(verified_claims.sub, claims.sub);
    assert_eq!(verified_claims.email, claims.email);
    assert_eq!(verified_claims.roles, claims.roles);
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

#[test]
fn test_role_checking() {
    let user_roles = vec!["admin".to_string(), "manager".to_string()];
    let required_roles = vec!["admin".to_string()];
    let missing_roles = vec!["super_admin".to_string()];

    assert!(PermissionChecker::has_any_role(&user_roles, &required_roles));
    assert!(!PermissionChecker::has_any_role(&user_roles, &missing_roles));
    assert!(PermissionChecker::has_all_roles(&user_roles, &required_roles));
}