use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "manager")]
    Manager,
    #[sea_orm(string_value = "user")]
    User,
}

impl UserRole {
    pub fn can_create(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Manager)
    }

    pub fn can_read(&self) -> bool {
        true // All roles can read
    }

    pub fn can_update(&self) -> bool {
        matches!(self, UserRole::Admin | UserRole::Manager)
    }

    pub fn can_delete(&self) -> bool {
        matches!(self, UserRole::Admin)
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        match permission {
            Permission::Create => self.can_create(),
            Permission::Read => self.can_read(),
            Permission::Update => self.can_update(),
            Permission::Delete => self.can_delete(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Permission {
    Create,
    Read,
    Update,
    Delete,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::product::Entity")]
    Products,
}

impl Related<super::product::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Products.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
