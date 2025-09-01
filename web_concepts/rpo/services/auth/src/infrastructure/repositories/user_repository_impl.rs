use async_trait::async_trait;
use sea_orm::*;
use crate_errors::{AppError, Result};
use uuid::Uuid;
use crate::domain::entities::{User, CreateUserRequest, UpdateUserRequest};
use crate::domain::repositories::UserRepository;
use crate::infrastructure::database::Database;
use crate::infrastructure::entities::{users, user_roles, roles, role_permissions, permissions};

pub struct UserRepositoryImpl {
    db: Database,
}

impl UserRepositoryImpl {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<User> {
        let db = self.db.get_connection();
        
        let user_model: users::Model = user.into();
        let active_model: users::ActiveModel = user_model.into();
        
        let result = users::Entity::insert(active_model)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        let created_user = users::Entity::find_by_id(result.last_insert_id)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::Internal("Failed to retrieve created user".to_string()))?;
            
        Ok(created_user.into())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let db = self.db.get_connection();
        
        let user = users::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(user.map(|u| u.into()))
    }

    async fn find_by_email(&self, email: &str, tenant_id: Uuid) -> Result<Option<User>> {
        let db = self.db.get_connection();
        
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .filter(users::Column::TenantId.eq(tenant_id))
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(user.map(|u| u.into()))
    }

    async fn find_by_username(&self, username: &str, tenant_id: Uuid) -> Result<Option<User>> {
        let db = self.db.get_connection();
        
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .filter(users::Column::TenantId.eq(tenant_id))
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(user.map(|u| u.into()))
    }

    async fn update(&self, id: Uuid, updates: UpdateUserRequest) -> Result<User> {
        let db = self.db.get_connection();
        
        let mut user: users::ActiveModel = users::Entity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?
            .into();

        // Update fields if provided
        if let Some(email) = updates.email {
            user.email = Set(email);
        }
        if let Some(username) = updates.username {
            user.username = Set(username);
        }
        if let Some(first_name) = updates.first_name {
            user.first_name = Set(first_name);
        }
        if let Some(last_name) = updates.last_name {
            user.last_name = Set(last_name);
        }
        if let Some(phone) = updates.phone {
            user.phone = Set(Some(phone));
        }
        if let Some(is_active) = updates.is_active {
            user.is_active = Set(is_active);
        }
        
        user.updated_at = Set(chrono::Utc::now());

        let updated_user = user.update(db)
            .await
            .map_err(AppError::Database)?;

        // Handle role assignments if provided
        if let Some(role_ids) = updates.role_ids {
            self.assign_roles(id, role_ids).await?;
        }

        Ok(updated_user.into())
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let db = self.db.get_connection();
        
        users::Entity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(())
    }

    async fn list_by_tenant(&self, tenant_id: Uuid, limit: u64, offset: u64) -> Result<Vec<User>> {
        let db = self.db.get_connection();
        
        let users_list = users::Entity::find()
            .filter(users::Column::TenantId.eq(tenant_id))
            .limit(limit)
            .offset(offset)
            .order_by_asc(users::Column::CreatedAt)
            .all(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(users_list.into_iter().map(|u| u.into()).collect())
    }

    async fn assign_roles(&self, user_id: Uuid, role_ids: Vec<Uuid>) -> Result<()> {
        let db = self.db.get_connection();
        
        // Start transaction
        let txn = db.begin().await.map_err(AppError::Database)?;
        
        // Delete existing role assignments
        user_roles::Entity::delete_many()
            .filter(user_roles::Column::UserId.eq(user_id))
            .exec(&txn)
            .await
            .map_err(AppError::Database)?;
        
        // Insert new role assignments
        let user_role_models: Vec<user_roles::ActiveModel> = role_ids
            .into_iter()
            .map(|role_id| user_roles::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user_id),
                role_id: Set(role_id),
                created_at: Set(chrono::Utc::now()),
            })
            .collect();

        if !user_role_models.is_empty() {
            user_roles::Entity::insert_many(user_role_models)
                .exec(&txn)
                .await
                .map_err(AppError::Database)?;
        }
        
        txn.commit().await.map_err(AppError::Database)?;
        Ok(())
    }

    async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<String>> {
        let db = self.db.get_connection();
        
        let permissions_list = permissions::Entity::find()
            .join(JoinType::InnerJoin, permissions::Relation::RolePermissions.def())
            .join(JoinType::InnerJoin, role_permissions::Relation::Role.def())
            .join(JoinType::InnerJoin, roles::Relation::UserRoles.def())
            .filter(user_roles::Column::UserId.eq(user_id))
            .filter(roles::Column::IsActive.eq(true))
            .distinct()
            .all(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(permissions_list.into_iter().map(|p| p.code).collect())
    }

    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>> {
        let db = self.db.get_connection();
        
        let roles_list = roles::Entity::find()
            .join(JoinType::InnerJoin, roles::Relation::UserRoles.def())
            .filter(user_roles::Column::UserId.eq(user_id))
            .filter(roles::Column::IsActive.eq(true))
            .all(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(roles_list.into_iter().map(|r| r.code).collect())
    }
}