use async_trait::async_trait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Define default permissions
        let default_permissions = vec![
            // User management
            ("Users - Create", "users.create", "users", "create", "Create new users"),
            ("Users - Read", "users.read", "users", "read", "View user information"),
            ("Users - Update", "users.update", "users", "update", "Update user information"),
            ("Users - Delete", "users.delete", "users", "delete", "Delete users"),
            
            // Role management
            ("Roles - Create", "roles.create", "roles", "create", "Create new roles"),
            ("Roles - Read", "roles.read", "roles", "read", "View roles"),
            ("Roles - Update", "roles.update", "roles", "update", "Update roles"),
            ("Roles - Delete", "roles.delete", "roles", "delete", "Delete roles"),
            
            // Product management
            ("Products - Create", "products.create", "products", "create", "Create new products"),
            ("Products - Read", "products.read", "products", "read", "View products"),
            ("Products - Update", "products.update", "products", "update", "Update products"),
            ("Products - Delete", "products.delete", "products", "delete", "Delete products"),
            
            // Order management
            ("Orders - Create", "orders.create", "orders", "create", "Create new orders"),
            ("Orders - Read", "orders.read", "orders", "read", "View orders"),
            ("Orders - Update", "orders.update", "orders", "update", "Update orders"),
            ("Orders - Delete", "orders.delete", "orders", "delete", "Delete orders"),
            
            // Inventory management
            ("Inventory - Read", "inventory.read", "inventory", "read", "View inventory"),
            ("Inventory - Update", "inventory.update", "inventory", "update", "Update inventory"),
            
            // Reports
            ("Reports - Sales", "reports.sales", "reports", "sales", "View sales reports"),
            ("Reports - Inventory", "reports.inventory", "reports", "inventory", "View inventory reports"),
            ("Reports - Users", "reports.users", "reports", "users", "View user reports"),
            
            // System administration
            ("System - Settings", "system.settings", "system", "settings", "Manage system settings"),
            ("System - Logs", "system.logs", "system", "logs", "View system logs"),
        ];

        for (name, code, resource, action, description) in default_permissions {
            let query = Query::insert()
                .into_table(Permission::Table)
                .columns([
                    Permission::Id,
                    Permission::Name,
                    Permission::Code,
                    Permission::Resource,
                    Permission::Action,
                    Permission::Description,
                    Permission::IsSystem,
                    Permission::CreatedAt,
                    Permission::UpdatedAt,
                ])
                .values_panic([
                    uuid::Uuid::new_v4().into(),
                    name.into(),
                    code.into(),
                    resource.into(),
                    action.into(),
                    description.into(),
                    true.into(),
                    chrono::Utc::now().into(),
                    chrono::Utc::now().into(),
                ])
                .to_owned();

            manager.exec_stmt(query).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_query = Query::delete()
            .from_table(Permission::Table)
            .and_where(Expr::col(Permission::IsSystem).eq(true))
            .to_owned();

        manager.exec_stmt(delete_query).await?;
        Ok(())
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