use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct CreatePermissionRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(min = 3, max = 50))]
    pub code: String,
    
    #[validate(length(min = 1, max = 50))]
    pub resource: String,
    
    #[validate(length(min = 1, max = 50))]
    pub action: String,
    
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PermissionResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Permission {
    pub fn new(request: CreatePermissionRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: request.name,
            code: request.code.to_lowercase(),
            resource: request.resource.to_lowercase(),
            action: request.action.to_lowercase(),
            description: request.description,
            is_system: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn create_system_permission(
        name: String,
        code: String,
        resource: String,
        action: String,
        description: Option<String>
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            code: code.to_lowercase(),
            resource: resource.to_lowercase(),
            action: action.to_lowercase(),
            description,
            is_system: true,
            created_at: now,
            updated_at: now,
        }
    }
}