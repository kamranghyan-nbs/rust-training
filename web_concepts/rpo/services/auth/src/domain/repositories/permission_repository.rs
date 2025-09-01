use async_trait::async_trait;
use crate_errors::Result;
use uuid::Uuid;
use crate::domain::entities::Permission;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn create(&self, permission: Permission) -> Result<Permission>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Permission>>;
    async fn find_by_code(&self, code: &str) -> Result<Option<Permission>>;
    async fn list(&self, limit: u64, offset: u64) -> Result<Vec<Permission>>;
    async fn update(&self, id: Uuid, updates: serde_json::Value) -> Result<Permission>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}