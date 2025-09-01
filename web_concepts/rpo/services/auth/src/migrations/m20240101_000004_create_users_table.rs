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
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::TenantId).uuid().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::PasswordHash).string().not_null())
                    .col(ColumnDef::new(User::FirstName).string().not_null())
                    .col(ColumnDef::new(User::LastName).string().not_null())
                    .col(ColumnDef::new(User::Phone).string())
                    .col(ColumnDef::new(User::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(User::IsVerified).boolean().not_null().default(false))
                    .col(ColumnDef::new(User::LastLoginAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::FailedLoginAttempts).integer().not_null().default(0))
                    .col(ColumnDef::new(User::LockedUntil).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_tenant_id")
                            .from(User::Table, User::TenantId)
                            .to(Tenant::Table, Tenant::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        // Create composite unique index for email + tenant_id
        manager
            .create_index(
                Index::create()
                    .name("idx_users_email_tenant")
                    .table(User::Table)
                    .col(User::Email)
                    .col(User::TenantId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create composite unique index for username + tenant_id
        manager
            .create_index(
                Index::create()
                    .name("idx_users_username_tenant")
                    .table(User::Table)
                    .col(User::Username)
                    .col(User::TenantId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    TenantId,
    Email,
    Username,
    PasswordHash,
    FirstName,
    LastName,
    Phone,
    IsActive,
    IsVerified,
    LastLoginAt,
    FailedLoginAttempts,
    LockedUntil,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Tenant {
    Table,
    Id,
}