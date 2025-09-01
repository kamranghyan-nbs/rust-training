use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add user_id column
        manager
            .alter_table(
                Table::alter()
                    .table(Todo::Table)
                    .add_column(
                        ColumnDef::new(Todo::UserId)
                            .integer()
                            .not_null()
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_todo_user")
                            .from_tbl(Todo::Table)
                            .from_col(Todo::UserId)
                            .to_tbl(User::Table)
                            .to_col(User::Id)
                            .on_delete(ForeignKeyAction::Cascade) // if user deleted, todos deleted
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Todo::Table)
                    .drop_foreign_key(Alias::new("fk_todo_user"))
                    .drop_column(Todo::UserId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum Todo {
    Table,
    UserId,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
