use async_trait::async_trait;
use crate_errors::Result;
use uuid::Uuid;
use crate::domain::entities::Session;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: Session) -> Result<Session>;
    async fn find_by_token(&self, refresh_token: &str) -> Result<Option<Session>>;
    async fn update(&self, session: Session) -> Result<Session>;
    async fn delete_expired(&self) -> Result<u64>;
    async fn delete_by_user(&self, user_id: Uuid) -> Result<u64>;
}