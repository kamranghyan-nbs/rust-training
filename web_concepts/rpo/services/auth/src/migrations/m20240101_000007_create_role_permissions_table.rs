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
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RolePermission::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(RolePermission::RoleId).uuid().not_null())
                    .col(ColumnDef::new(RolePermission::PermissionId).uuid().not_null())
                    .col(ColumnDef::new(RolePermission::CreatedAt).timestamp_with_time_zone().not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permissions_role_id")
                            .from(RolePermission::Table, RolePermission::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_role_permissions_permission_id")
                            .from(RolePermission::Table, RolePermission::PermissionId)
                            .to(Permission::Table, Permission::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_role_permissions_role_permission")
                    .table(RolePermission::Table)
                    .col(RolePermission::RoleId)
                    .col(RolePermission::PermissionId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RolePermission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RolePermission {
    Table,
    Id,
    RoleId,
    PermissionId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Role {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Permission {
    Table,
    Id,
}