use async_trait::async_trait;
use sea_orm::*;
use crate_errors::{AppError, Result};
use uuid::Uuid;
use crate::domain::entities::Role;
use crate::domain::repositories::RoleRepository;
use crate::infrastructure::database::Database;
use crate::infrastructure::entities::roles;

pub struct RoleRepositoryImpl {
    db: Database,
}

impl RoleRepositoryImpl {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl RoleRepository for RoleRepositoryImpl {
    async fn create(&self, role: Role) -> Result<Role> {
        let db = self.db.get_connection();
        
        let role_model: roles::Model = role.into();
        let active_model: roles::ActiveModel = role_model.into();
        
        let result = roles::Entity::insert(active_model)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        let created_role = roles::Entity::find_by_id(result.last_insert_id)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::Internal("Failed to retrieve created role".to_string()))?;
            
        Ok(created_role.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>> {
        let db = self.db.get_connection();
        
        let role = roles::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(role.map(|r| r.into()))
    }

    async fn find_by_code(&self, code: &str, tenant_id: Uuid) -> Result<Option<Role>> {
        let db = self.db.get_connection();
        
        let role = roles::Entity::find()
            .filter(roles::Column::Code.eq(code))
            .filter(roles::Column::TenantId.eq(tenant_id))
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(role.map(|r| r.into()))
    }

    async fn list_by_tenant(&self, tenant_id: Uuid, limit: u64, offset: u64) -> Result<Vec<Role>> {
        let db = self.db.get_connection();
        
        let roles_list = roles::Entity::find()
            .filter(roles::Column::TenantId.eq(tenant_id))
            .limit(limit)
            .offset(offset)
            .order_by_asc(roles::Column::CreatedAt)
            .all(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(roles_list.into_iter().map(|r| r.into()).collect())
    }

    async fn update(&self, id: Uuid, updates: serde_json::Value) -> Result<Role> {
        let db = self.db.get_connection();
        
        let mut role: roles::ActiveModel = roles::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound("Role not found".to_string()))?
            .into();

        // Update fields based on the updates JSON
        if let Some(name) = updates.get("name").and_then(|v| v.as_str()) {
            role.name = Set(name.to_string());
        }
        if let Some(description) = updates.get("description").and_then(|v| v.as_str()) {
            role.description = Set(Some(description.to_string()));
        }
        if let Some(is_active) = updates.get("is_active").and_then(|v| v.as_bool()) {
            role.is_active = Set(is_active);
        }
        
        role.updated_at = Set(chrono::Utc::now());

        let updated_role = role.update(db)
            .await
            .map_err(AppError::Database)?;

        Ok(updated_role.into())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let db = self.db.get_connection();
        
        roles::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(())
    }
}