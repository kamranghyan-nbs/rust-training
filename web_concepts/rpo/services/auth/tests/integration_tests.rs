use auth_service::{
    config::AuthConfig,
    infrastructure::database::Database,
    presentation::routes::create_router,
};
use axum_test::TestServer;
use sea_orm_migration::MigratorTrait;
use serde_json::json;
use uuid::Uuid;

struct TestContext {
    server: TestServer,
    tenant_id: Uuid,
}

impl TestContext {
    async fn new() -> Self {
        // Load test configuration
        let config = AuthConfig {
            database: crate_config::DatabaseConfig {
                url: "postgres://test:test@localhost:5432/auth_test".to_string(),
                max_connections: 5,
                min_connections: 1,
                acquire_timeout: 30,
                idle_timeout: 300,
            },
            jwt: crate_config::JwtConfig {
                secret: "test_secret_key_for_testing_only".to_string(),
                access_token_expire_minutes: 60,
                refresh_token_expire_days: 7,
                issuer: "test_issuer".to_string(),
            },
            server: crate_config::ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 0,
                workers: 1,
            },
            logging: crate_config::LoggingConfig {
                level: "debug".to_string(),
                format: "pretty".to_string(),
                jaeger_endpoint: None,
            },
            redis: None,
        };

        // Setup test database
        let database = Database::new(&config.database).await.unwrap();
        
        // Run migrations
        auth_service::infrastructure::migrations::Migrator::up(database.get_connection(), None)
            .await
            .unwrap();

        // Create test app
        let app = create_router(config, database);
        let server = TestServer::new(app).unwrap();

        // Create test tenant
        let tenant_response = server
            .post("/tenants")
            .json(&json!({
                "name": "Test Tenant",
                "code": "test_tenant",
                "domain": "test.example.com"
            }))
            .await;

        let tenant_id = tenant_response.json::<serde_json::Value>()["id"]
            .as_str()
            .unwrap()
            .parse::<Uuid>()
            .unwrap();

        Self { server, tenant_id }
    }
}

#[tokio::test]
async fn test_tenant_creation() {
    let ctx = TestContext::new().await;

    let response = ctx.server
        .post("/tenants")
        .json(&json!({
            "name": "New Tenant",
            "code": "new_tenant",
            "domain": "new.example.com"
        }))
        .await;

    response.assert_status_ok();
    
    let tenant: serde_json::Value = response.json();
    assert_eq!(tenant["name"], "New Tenant");
    assert_eq!(tenant["code"], "new_tenant");
}

#[tokio::test]
async fn test_user_registration_and_login() {
    let ctx = TestContext::new().await;

    // Create a user
    let create_user_response = ctx.server
        .post("/users")
        .json(&json!({
            "tenant_id": ctx.tenant_id,
            "email": "test@example.com",
            "username": "testuser",
            "password": "password123",
            "first_name": "Test",
            "last_name": "User",
            "role_ids": []
        }))
        .await;

    create_user_response.assert_status_ok();

    // Login
    let login_response = ctx.server
        .post("/auth/login")
        .json(&json!({
            "email": "test@example.com",
            "password": "password123",
            "tenant_code": "test_tenant"
        }))
        .await;

    login_response.assert_status_ok();
    
    let login_data: serde_json::Value = login_response.json();
    assert!(login_data["access_token"].is_string());
    assert!(login_data["refresh_token"].is_string());
    assert_eq!(login_data["token_type"], "Bearer");
}

#[tokio::test]
async fn test_token_validation() {
    let ctx = TestContext::new().await;

    // First create and login a user
    ctx.server
        .post("/users")
        .json(&json!({
            "tenant_id": ctx.tenant_id,
            "email": "validate@example.com",
            "username": "validateuser",
            "password": "password123",
            "first_name": "Validate",
            "last_name": "User",
            "role_ids": []
        }))
        .await
        .assert_status_ok();

    let login_response = ctx.server
        .post("/auth/login")
        .json(&json!({
            "email": "validate@example.com",
            "password": "password123",
            "tenant_code": "test_tenant"
        }))
        .await;

    let login_data: serde_json::Value = login_response.json();
    let access_token = login_data["access_token"].as_str().unwrap();

    // Validate the token
    let validation_response = ctx.server
        .post("/auth/validate")
        .json(&json!(access_token))
        .await;

    validation_response.assert_status_ok();
    
    let validation_data: serde_json::Value = validation_response.json();
    assert_eq!(validation_data["valid"], true);
}

#[tokio::test]
async fn test_invalid_login() {
    let ctx = TestContext::new().await;

    let response = ctx.server
        .post("/auth/login")
        .json(&json!({
            "email": "nonexistent@example.com",
            "password": "wrongpassword",
            "tenant_code": "test_tenant"
        }))
        .await;

    response.assert_status(401);
}

#[tokio::test]
async fn test_refresh_token() {
    let ctx = TestContext::new().await;

    // Create and login user
    ctx.server
        .post("/users")
        .json(&json!({
            "tenant_id": ctx.tenant_id,
            "email": "refresh@example.com",
            "username": "refreshuser",
            "password": "password123",
            "first_name": "Refresh",
            "last_name": "User",
            "role_ids": []
        }))
        .await
        .assert_status_ok();

    let login_response = ctx.server
        .post("/auth/login")
        .json(&json!({
            "email": "refresh@example.com",
            "password": "password123",
            "tenant_code": "test_tenant"
        }))
        .await;

    let login_data: serde_json::Value = login_response.json();
    let refresh_token = login_data["refresh_token"].as_str().unwrap();

    // Use refresh token to get new access token
    let refresh_response = ctx.server
        .post("/auth/refresh")
        .json(&json!({
            "refresh_token": refresh_token
        }))
        .await;

    refresh_response.assert_status_ok();
    
    let refresh_data: serde_json::Value = refresh_response.json();
    assert!(refresh_data["access_token"].is_string());
}