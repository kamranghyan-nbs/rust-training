use async_trait::async_trait;
use sea_orm::*;
use crate_errors::{AppError, Result};
use uuid::Uuid;
use crate::domain::entities::Session;
use crate::domain::repositories::SessionRepository;
use crate::infrastructure::database::Database;
use crate::infrastructure::entities::sessions;

pub struct SessionRepositoryImpl {
    db: Database,
}

impl SessionRepositoryImpl {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {
    async fn create(&self, session: Session) -> Result<Session> {
        let db = self.db.get_connection();
        
        let session_model: sessions::Model = session.into();
        let active_model: sessions::ActiveModel = session_model.into();
        
        let result = sessions::Entity::insert(active_model)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        let created_session = sessions::Entity::find_by_id(result.last_insert_id)
            .one(db)
            .await
            .map_err(AppError::Database)?
            .ok_or_else(|| AppError::Internal("Failed to retrieve created session".to_string()))?;
            
        Ok(created_session.into())
    }

    async fn find_by_token(&self, refresh_token: &str) -> Result<Option<Session>> {
        let db = self.db.get_connection();
        
        let session = sessions::Entity::find()
            .filter(sessions::Column::RefreshToken.eq(refresh_token))
            .one(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(session.map(|s| s.into()))
    }

    async fn update(&self, session: Session) -> Result<Session> {
        let db = self.db.get_connection();
        
        let session_model: sessions::Model = session.into();
        let active_model: sessions::ActiveModel = session_model.into();
        
        let updated_session = active_model.update(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(updated_session.into())
    }

    async fn delete_expired(&self) -> Result<u64> {
        let db = self.db.get_connection();
        
        let result = sessions::Entity::delete_many()
            .filter(sessions::Column::ExpiresAt.lt(chrono::Utc::now()))
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(result.rows_affected)
    }

    async fn delete_by_user(&self, user_id: Uuid) -> Result<u64> {
        let db = self.db.get_connection();
        
        let result = sessions::Entity::delete_many()
            .filter(sessions::Column::UserId.eq(user_id))
            .exec(db)
            .await
            .map_err(AppError::Database)?;
            
        Ok(result.rows_affected)
    }
}