use async_trait::async_trait;
use crate_errors::Result;
use uuid::Uuid;
use crate::domain::entities::{Tenant, CreateTenantRequest};

#[async_trait]
pub trait TenantRepository: Send + Sync {
    async fn create(&self, tenant: Tenant) -> Result<Tenant>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>>;
    async fn find_by_code(&self, code: &str) -> Result<Option<Tenant>>;
    async fn find_by_domain(&self, domain: &str) -> Result<Option<Tenant>>;
    async fn update(&self, id: Uuid, updates: serde_json::Value) -> Result<Tenant>;
    async fn list(&self, limit: u64, offset: u64) -> Result<Vec<Tenant>>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}