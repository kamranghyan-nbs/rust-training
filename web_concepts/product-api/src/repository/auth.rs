use crate::{
    entities::{prelude::*, user},
    error::AppError,
};
use async_trait::async_trait;
use sea_orm::{prelude::*, ActiveModelTrait, Set};
use uuid::Uuid;

#[async_trait]
pub trait AuthRepositoryTrait {
    async fn find_by_username(&self, username: &str) -> Result<Option<user::Model>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<user::Model>, AppError>;
    async fn find_by_username_or_email(&self, username: &str, email: &str) -> Result<Option<user::Model>, AppError>;
    async fn create_user(&self, username: String, email: String, password_hash: String) -> Result<user::Model, AppError>;
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<user::Model>, AppError>;
}

#[derive(Clone)]
pub struct AuthRepository {
    db: DatabaseConnection,
}

impl AuthRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthRepositoryTrait for AuthRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<user::Model>, AppError> {
        let user = User::find()
            .filter(user::Column::Username.eq(username))
            .one(&self.db)
            .await?;
        
        Ok(user)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<user::Model>, AppError> {
        let user = User::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await?;
        
        Ok(user)
    }

    async fn find_by_username_or_email(&self, username: &str, email: &str) -> Result<Option<user::Model>, AppError> {
        let user = User::find()
            .filter(
                user::Column::Username
                    .eq(username)
                    .or(user::Column::Email.eq(email)),
            )
            .one(&self.db)
            .await?;
        
        Ok(user)
    }

    async fn create_user(&self, username: String, email: String, password_hash: String) -> Result<user::Model, AppError> {
        let user_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let new_user = user::ActiveModel {
            id: Set(user_id),
            username: Set(username),
            email: Set(email),
            password_hash: Set(password_hash),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let user = new_user.insert(&self.db).await?;
        Ok(user)
    }

    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<user::Model>, AppError> {
        let user = User::find_by_id(user_id)
            .one(&self.db)
            .await?;
        
        Ok(user)
    }
}