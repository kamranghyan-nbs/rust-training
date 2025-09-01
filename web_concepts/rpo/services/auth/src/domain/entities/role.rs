use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Role {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, utoipa::ToSchema)]
pub struct CreateRoleRequest {
    pub tenant_id: Uuid,
    
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(length(min = 3, max = 50))]
    pub code: String,
    
    pub description: Option<String>,
    
    pub permission_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RoleResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Role {
    pub fn new(request: CreateRoleRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id: request.tenant_id,
            name: request.name,
            code: request.code.to_lowercase(),
            description: request.description,
            is_system: false,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn create_system_role(tenant_id: Uuid, name: String, code: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            name,
            code: code.to_lowercase(),
            description,
            is_system: true,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}