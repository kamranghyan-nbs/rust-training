use async_trait::async_trait;
use crate_errors::Result;
use uuid::Uuid;
use crate::domain::entities::Role;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, role: Role) -> Result<Role>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>>;
    async fn find_by_code(&self, code: &str, tenant_id: Uuid) -> Result<Option<Role>>;
    async fn list_by_tenant(&self, tenant_id: Uuid, limit: u64, offset: u64) -> Result<Vec<Role>>;
    async fn update(&self, id: Uuid, updates: serde_json::Value) -> Result<Role>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}