use async_trait::async_trait;
pub use sea_orm_migration::prelude::*;

pub mod m20240101_000001_create_tenants_table;
pub mod m20240101_000002_create_permissions_table;
pub mod m20240101_000003_create_roles_table;
pub mod m20240101_000004_create_users_table;
pub mod m20240101_000005_create_sessions_table;
pub mod m20240101_000006_create_user_roles_table;
pub mod m20240101_000007_create_role_permissions_table;
pub mod m20240101_000008_seed_default_permissions;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_tenants_table::Migration),
            Box::new(m20240101_000002_create_permissions_table::Migration),
            Box::new(m20240101_000003_create_roles_table::Migration),
            Box::new(m20240101_000004_create_users_table::Migration),
            Box::new(m20240101_000005_create_sessions_table::Migration),
            Box::new(m20240101_000006_create_user_roles_table::Migration),
            Box::new(m20240101_000007_create_role_permissions_table::Migration),
            Box::new(m20240101_000008_seed_default_permissions::Migration),
        ]
    }
}
