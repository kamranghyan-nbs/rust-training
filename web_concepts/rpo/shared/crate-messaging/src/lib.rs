use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Event types for inter-service communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthEvent {
    UserCreated {
        user_id: Uuid,
        tenant_id: Uuid,
        email: String,
        username: String,
        timestamp: DateTime<Utc>,
    },
    UserUpdated {
        user_id: Uuid,
        tenant_id: Uuid,
        changes: Vec<String>,
        timestamp: DateTime<Utc>,
    },
    UserDeleted {
        user_id: Uuid,
        tenant_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    UserLoggedIn {
        user_id: Uuid,
        tenant_id: Uuid,
        ip_address: Option<String>,
        timestamp: DateTime<Utc>,
    },
    UserLoggedOut {
        user_id: Uuid,
        tenant_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    TenantCreated {
        tenant_id: Uuid,
        tenant_code: String,
        tenant_name: String,
        timestamp: DateTime<Utc>,
    },
}

// Internal service communication structs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateTokenRequest {
    pub token: String,
    pub required_permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateTokenResponse {
    pub valid: bool,
    pub user_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRequest {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

// Message publisher trait
#[async_trait::async_trait]
pub trait MessagePublisher: Send + Sync {
    async fn publish(&self, topic: &str, message: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn publish_event(&self, event: &AuthEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let message = serde_json::to_vec(event)?;
        self.publish("auth.events", &message).await
    }
}

// Internal service client for other services to communicate with auth service
#[derive(Clone)]
pub struct AuthServiceClient {
    base_url: String,
    client: reqwest::Client,
}

impl AuthServiceClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn validate_token(&self, request: ValidateTokenRequest) -> Result<ValidateTokenResponse, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .post(&format!("{}/internal/validate-token", self.base_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let result: ValidateTokenResponse = response.json().await?;
            Ok(result)
        } else {
            Ok(ValidateTokenResponse {
                valid: false,
                user_id: None,
                tenant_id: None,
                permissions: vec![],
                roles: vec![],
                error: Some("Token validation failed".to_string()),
            })
        }
    }

    pub async fn get_user(&self, request: GetUserRequest) -> Result<Option<GetUserResponse>, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .post(&format!("{}/internal/get-user", self.base_url))
            .json(&request)
            .send()
            .await?;

        if response.status().is_success() {
            let result: GetUserResponse = response.json().await?;
            Ok(Some(result))
        } else if response.status() == 404 {
            Ok(None)
        } else {
            Err("Failed to get user".into())
        }
    }
}

// Helper trait for services that need authentication
#[async_trait::async_trait]
pub trait AuthenticationProvider {
    async fn validate_token(&self, token: &str, required_permissions: Vec<String>) -> Result<ValidateTokenResponse, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_user(&self, user_id: Uuid, tenant_id: Uuid) -> Result<Option<GetUserResponse>, Box<dyn std::error::Error + Send + Sync>>;
}

#[async_trait::async_trait]
impl AuthenticationProvider for AuthServiceClient {
    async fn validate_token(&self, token: &str, required_permissions: Vec<String>) -> Result<ValidateTokenResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.validate_token(ValidateTokenRequest {
            token: token.to_string(),
            required_permissions,
        }).await
    }

    async fn get_user(&self, user_id: Uuid, tenant_id: Uuid) -> Result<Option<GetUserResponse>, Box<dyn std::error::Error + Send + Sync>> {
        self.get_user(GetUserRequest { user_id, tenant_id }).await
    }
}