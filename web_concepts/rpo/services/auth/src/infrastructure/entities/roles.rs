use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::tenants::Entity", from = "Column::TenantId", to = "super::tenants::Column::Id")]
    Tenant,
    #[sea_orm(has_many = "super::user_roles::Entity")]
    UserRoles,
    #[sea_orm(has_many = "super::role_permissions::Entity")]
    RolePermissions,
}

impl Related<super::tenants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tenant.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_roles::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_roles::Relation::Role.def().rev())
    }
}

impl Related<super::permissions::Entity> for Entity {
    fn to() -> RelationDef {
        super::role_permissions::Relation::Permission.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::role_permissions::Relation::Role.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

// impl From<Model> for ActiveModel {
//     fn from(model: Model) -> Self {
//         Self {
//             id: Set(model.id),
//             tenant_id: Set(model.tenant_id),
//             name: Set(model.name),
//             code: Set(model.code),
//             description: Set(model.description),
//             is_system: Set(model.is_system),
//             is_active: Set(model.is_active),
//             created_at: Set(model.created_at),
//             updated_at: Set(model.updated_at),
//         }
//     }
// }

// Conversions
impl From<crate::domain::entities::Role> for Model {
    fn from(role: crate::domain::entities::Role) -> Self {
        Self {
            id: role.id,
            tenant_id: role.tenant_id,
            name: role.name,
            code: role.code,
            description: role.description,
            is_system: role.is_system,
            is_active: role.is_active,
            created_at: role.created_at,
            updated_at: role.updated_at,
        }
    }
}

impl From<Model> for crate::domain::entities::Role {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            tenant_id: model.tenant_id,
            name: model.name,
            code: model.code,
            description: model.description,
            is_system: model.is_system,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}