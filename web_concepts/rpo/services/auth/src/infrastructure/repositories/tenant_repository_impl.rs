use async_trait::async_trait;
use sea_orm::*;
use crate_errors::{AppError, Result};
use uuid::Uuid;
use crate::domain::entities::{Tenant, CreateTenantRequest};
use crate::domain::repositories::TenantRepository;
use crate::infrastructure::database::Database;
use crate::infrastructure::entities::tenants;

pub struct TenantRepositoryImpl {
    db: Database,
}

impl TenantRepositoryImpl {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TenantRepository for TenantRepositoryImpl {
    async fn create(&self, tenant: Tenant) -> Result<Tenant> {
        let db = self.db.get_connection();
        
        let tenant_model: tenants::Model = tenant.into();
        let active_model: tenants::ActiveModel = tenant_model.into();
        
        let result = tenants::Entity::insert(active_model)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        let created_tenant = tenants::Entity::find_by_id(result.last_insert_id)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::Internal("Failed to retrieve created tenant".to_string()))?;
            
        Ok(created_tenant.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>> {
        let db = self.db.get_connection();
        
        let tenant = tenants::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(tenant.map(|t| t.into()))
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<Tenant>> {
        let db = self.db.get_connection();
        
        let tenant = tenants::Entity::find()
            .filter(tenants::Column::Code.eq(code))
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(tenant.map(|t| t.into()))
    }

    async fn find_by_domain(&self, domain: &str) -> Result<Option<Tenant>> {
        let db = self.db.get_connection();
        
        let tenant = tenants::Entity::find()
            .filter(tenants::Column::Domain.eq(domain))
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(tenant.map(|t| t.into()))
    }

    async fn update(&self, id: Uuid, updates: serde_json::Value) -> Result<Tenant> {
        let db = self.db.get_connection();
        
        let mut tenant: tenants::ActiveModel = tenants::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound("Tenant not found".to_string()))?
            .into();

        // Update fields based on the updates JSON
        if let Some(name) = updates.get("name").and_then(|v| v.as_str()) {
            tenant.name = Set(name.to_string());
        }
        if let Some(domain) = updates.get("domain").and_then(|v| v.as_str()) {
            tenant.domain = Set(Some(domain.to_string()));
        }
        if let Some(settings) = updates.get("settings") {
            tenant.settings = Set(settings.clone());
        }
        if let Some(is_active) = updates.get("is_active").and_then(|v| v.as_bool()) {
            tenant.is_active = Set(is_active);
        }
        
        tenant.updated_at = Set(chrono::Utc::now());

        let updated_tenant = tenant.update(db)
            .await
            .map_err(AppError::Database)?;

        Ok(updated_tenant.into())
    }

    async fn list(&self, limit: u64, offset: u64) -> Result<Vec<Tenant>> {
        let db = self.db.get_connection();
        
        let tenants_list = tenants::Entity::find()
            .limit(limit)
            .offset(offset)
            .order_by_asc(tenants::Column::CreatedAt)
            .all(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(tenants_list.into_iter().map(|t| t.into()).collect())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let db = self.db.get_connection();
        
        tenants::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(())
    }
}