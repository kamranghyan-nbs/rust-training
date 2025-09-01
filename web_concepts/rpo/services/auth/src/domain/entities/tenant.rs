use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub domain: Option<String>,
    pub settings: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateTenantRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(min = 3, max = 20))]
    pub code: String,
    
    pub domain: Option<String>,
    
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct TenantResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub domain: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tenant {
    pub fn new(request: CreateTenantRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: request.name,
            code: request.code.to_lowercase(),
            domain: request.domain,
            settings: request.settings.unwrap_or_default(),
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}