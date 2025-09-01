use async_trait::async_trait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Permission::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Permission::Name).string().not_null())
                    .col(ColumnDef::new(Permission::Code).string().not_null().unique_key())
                    .col(ColumnDef::new(Permission::Resource).string().not_null())
                    .col(ColumnDef::new(Permission::Action).string().not_null())
                    .col(ColumnDef::new(Permission::Description).string())
                    .col(ColumnDef::new(Permission::IsSystem).boolean().not_null().default(false))
                    .col(ColumnDef::new(Permission::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Permission::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_permissions_code")
                    .table(Permission::Table)
                    .col(Permission::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_permissions_resource_action")
                    .table(Permission::Table)
                    .col(Permission::Resource)
                    .col(Permission::Action)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Permission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Permission {
    Table,
    Id,
    Name,
    Code,
    Resource,
    Action,
    Description,
    IsSystem,
    CreatedAt,
    UpdatedAt,
}