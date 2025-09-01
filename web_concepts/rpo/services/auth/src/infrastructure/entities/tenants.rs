use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "tenants")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub domain: Option<String>,
    pub settings: Json,
    pub is_active: bool,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::users::Entity")]
    Users,
    #[sea_orm(has_many = "super::roles::Entity")]
    Roles,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// impl From<Model> for ActiveModel {
//     fn from(model: Model) -> Self {
//         Self {
//             id: Set(model.id),
//             name: Set(model.name),
//             code: Set(model.code),
//             domain: Set(model.domain),
//             settings: Set(model.settings),
//             is_active: Set(model.is_active),
//             created_at: Set(model.created_at),
//             updated_at: Set(model.updated_at),
//         }
//     }
// }

// Conversions
impl From<crate::domain::entities::Tenant> for Model {
    fn from(tenant: crate::domain::entities::Tenant) -> Self {
        Self {
            id: tenant.id,
            name: tenant.name,
            code: tenant.code,
            domain: tenant.domain,
            settings: tenant.settings,
            is_active: tenant.is_active,
            created_at: tenant.created_at,
            updated_at: tenant.updated_at,
        }
    }
}

impl From<Model> for crate::domain::entities::Tenant {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            code: model.code,
            domain: model.domain,
            settings: model.settings,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}