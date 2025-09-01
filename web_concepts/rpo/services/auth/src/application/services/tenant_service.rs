use crate_errors::{AppError, Result};
use crate::domain::entities::{Tenant, CreateTenantRequest, TenantResponse};
use crate::domain::repositories::TenantRepository;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct TenantService {
    tenant_repository: Arc<dyn TenantRepository>,
}

impl TenantService {
    pub fn new(tenant_repository: Arc<dyn TenantRepository>) -> Self {
        Self { tenant_repository }
    }

    pub async fn create_tenant(&self, request: CreateTenantRequest) -> Result<TenantResponse> {
        request.validate().map_err(|e| AppError::Validation(e.to_string()))?;

        // Check if tenant with code already exists
        if let Some(_) = self.tenant_repository.find_by_code(&request.code).await? {
            return Err(AppError::Conflict("Tenant with this code already exists".to_string()));
        }

        // Check if tenant with domain already exists (if domain provided)
        if let Some(domain) = &request.domain {
            if let Some(_) = self.tenant_repository.find_by_domain(domain).await? {
                return Err(AppError::Conflict("Tenant with this domain already exists".to_string()));
            }
        }

        let tenant = Tenant::new(request);
        let created_tenant = self.tenant_repository.create(tenant).await?;

        Ok(TenantResponse {
            id: created_tenant.id,
            name: created_tenant.name,
            code: created_tenant.code,
            domain: created_tenant.domain,
            is_active: created_tenant.is_active,
            created_at: created_tenant.created_at,
            updated_at: created_tenant.updated_at,
        })
    }

    pub async fn get_tenant(&self, id: Uuid) -> Result<TenantResponse> {
        let tenant = self.tenant_repository.find_by_id(id).await?
            .ok_or_else(|| AppError::NotFound("Tenant not found".to_string()))?;

        Ok(TenantResponse {
            id: tenant.id,
            name: tenant.name,
            code: tenant.code,
            domain: tenant.domain,
            is_active: tenant.is_active,
            created_at: tenant.created_at,
            updated_at: tenant.updated_at,
        })
    }

    pub async fn get_tenant_by_code(&self, code: &str) -> Result<TenantResponse> {
        let tenant = self.tenant_repository.find_by_code(code).await?
            .ok_or_else(|| AppError::NotFound("Tenant not found".to_string()))?;

        Ok(TenantResponse {
            id: tenant.id,
            name: tenant.name,
            code: tenant.code,
            domain: tenant.domain,
            is_active: tenant.is_active,
            created_at: tenant.created_at,
            updated_at: tenant.updated_at,
        })
    }

    pub async fn list_tenants(&self, page: u64, page_size: u64) -> Result<Vec<TenantResponse>> {
        let offset = page * page_size;
        let tenants = self.tenant_repository.list(page_size, offset).await?;

        let tenant_responses = tenants
            .into_iter()
            .map(|tenant| TenantResponse {
                id: tenant.id,
                name: tenant.name,
                code: tenant.code,
                domain: tenant.domain,
                is_active: tenant.is_active,
                created_at: tenant.created_at,
                updated_at: tenant.updated_at,
            })
            .collect();

        Ok(tenant_responses)
    }

    pub async fn update_tenant(&self, id: Uuid, updates: serde_json::Value) -> Result<TenantResponse> {
        // Check if tenant exists
        if self.tenant_repository.find_by_id(id).await?.is_none() {
            return Err(AppError::NotFound("Tenant not found".to_string()));
        }

        let updated_tenant = self.tenant_repository.update(id, updates).await?;

        Ok(TenantResponse {
            id: updated_tenant.id,
            name: updated_tenant.name,
            code: updated_tenant.code,
            domain: updated_tenant.domain,
            is_active: updated_tenant.is_active,
            created_at: updated_tenant.created_at,
            updated_at: updated_tenant.updated_at,
        })
    }
}