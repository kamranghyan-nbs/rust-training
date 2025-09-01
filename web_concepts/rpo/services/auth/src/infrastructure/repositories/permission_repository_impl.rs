use async_trait::async_trait;
use sea_orm::*;
use crate_errors::{AppError, Result};
use uuid::Uuid;
use crate::domain::entities::Permission;
use crate::domain::repositories::PermissionRepository;
use crate::infrastructure::database::Database;
use crate::infrastructure::entities::permissions;

pub struct PermissionRepositoryImpl {
    db: Database,
}

impl PermissionRepositoryImpl {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PermissionRepository for PermissionRepositoryImpl {
    async fn create(&self, permission: Permission) -> Result<Permission> {
        let db = self.db.get_connection();
        
        let permission_model: permissions::Model = permission.into();
        let active_model: permissions::ActiveModel = permission_model.into();
        
        let result = permissions::Entity::insert(active_model)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        let created_permission = permissions::Entity::find_by_id(result.last_insert_id)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::Internal("Failed to retrieve created permission".to_string()))?;
            
        Ok(created_permission.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Permission>> {
        let db = self.db.get_connection();
        
        let permission = permissions::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(permission.map(|p| p.into()))
    }

    async fn find_by_code(&self, code: &str) -> Result<Option<Permission>> {
        let db = self.db.get_connection();
        
        let permission = permissions::Entity::find()
            .filter(permissions::Column::Code.eq(code))
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(permission.map(|p| p.into()))
    }

    async fn list(&self, limit: u64, offset: u64) -> Result<Vec<Permission>> {
        let db = self.db.get_connection();
        
        let permissions_list = permissions::Entity::find()
            .limit(limit)
            .offset(offset)
            .order_by_asc(permissions::Column::CreatedAt)
            .all(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(permissions_list.into_iter().map(|p| p.into()).collect())
    }

    async fn update(&self, id: Uuid, updates: serde_json::Value) -> Result<Permission> {
        let db = self.db.get_connection();
        
        let mut permission: permissions::ActiveModel = permissions::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound("Permission not found".to_string()))?
            .into();

        // Update fields based on the updates JSON
        if let Some(name) = updates.get("name").and_then(|v| v.as_str()) {
            permission.name = Set(name.to_string());
        }
        if let Some(description) = updates.get("description").and_then(|v| v.as_str()) {
            permission.description = Set(Some(description.to_string()));
        }
        
        permission.updated_at = Set(chrono::Utc::now());

        let updated_permission = permission.update(db)
            .await
            .map_err(AppError::Database)?;

        Ok(updated_permission.into())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let db = self.db.get_connection();
        
        permissions::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(())
    }
}