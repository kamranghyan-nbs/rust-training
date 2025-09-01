use async_trait::async_trait;
use crate_errors::Result;
use uuid::Uuid;
use crate::domain::entities::{User, CreateUserRequest, UpdateUserRequest};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn find_by_email(&self, email: &str, tenant_id: Uuid) -> Result<Option<User>>;
    async fn find_by_username(&self, username: &str, tenant_id: Uuid) -> Result<Option<User>>;
    async fn update(&self, id: Uuid, updates: UpdateUserRequest) -> Result<User>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn list_by_tenant(&self, tenant_id: Uuid, limit: u64, offset: u64) -> Result<Vec<User>>;
    async fn assign_roles(&self, user_id: Uuid, role_ids: Vec<Uuid>) -> Result<()>;
    async fn get_user_permissions(&self, user_id: Uuid) -> Result<Vec<String>>;
    async fn get_user_roles(&self, user_id: Uuid) -> Result<Vec<String>>;
}