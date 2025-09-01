pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_todo;
mod m20250826_000001_create_user;
mod m20250826_000002_add_user_fk_to_todo;
mod m20250827_000001_create_log;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_todo::Migration),
            Box::new(m20250826_000001_create_user::Migration),
            Box::new(m20250826_000002_add_user_fk_to_todo::Migration),
            Box::new(m20250827_000001_create_log::Migration),
        ]
    }
}
