use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::role_permissions::Entity")]
    RolePermissions,
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_permissions::Relation::Role.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_permissions::Relation::Permission.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

// impl From<Model> for ActiveModel {
//     fn from(model: Model) -> Self {
//         Self {
//             id: Set(model.id),
//             name: Set(model.name),
//             code: Set(model.code),
//             resource: Set(model.resource),
//             action: Set(model.action),
//             description: Set(model.description),
//             is_system: Set(model.is_system),
//             created_at: Set(model.created_at),
//             updated_at: Set(model.updated_at),
//         }
//     }
// }

// Conversions
impl From<crate::domain::entities::Permission> for Model {
    fn from(permission: crate::domain::entities::Permission) -> Self {
        Self {
            id: permission.id,
            name: permission.name,
            code: permission.code,
            resource: permission.resource,
            action: permission.action,
            description: permission.description,
            is_system: permission.is_system,
            created_at: permission.created_at,
            updated_at: permission.updated_at,
        }
    }
}

impl From<Model> for crate::domain::entities::Permission {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            code: model.code,
            resource: model.resource,
            action: model.action,
            description: model.description,
            is_system: model.is_system,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}