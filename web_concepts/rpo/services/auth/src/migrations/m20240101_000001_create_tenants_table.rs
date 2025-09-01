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
                    .table(Tenant::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tenant::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tenant::Name).string().not_null())
                    .col(ColumnDef::new(Tenant::Code).string().not_null().unique_key())
                    .col(ColumnDef::new(Tenant::Domain).string())
                    .col(ColumnDef::new(Tenant::Settings).json().not_null().default("{}"))
                    .col(ColumnDef::new(Tenant::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Tenant::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Tenant::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tenants_code")
                    .table(Tenant::Table)
                    .col(Tenant::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tenants_domain")
                    .table(Tenant::Table)
                    .col(Tenant::Domain)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tenant::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Tenant {
    Table,
    Id,
    Name,
    Code,
    Domain,
    Settings,
    IsActive,
    CreatedAt,
    UpdatedAt,
}