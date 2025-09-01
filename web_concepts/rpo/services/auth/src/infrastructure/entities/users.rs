use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub last_login_at: Option<DateTimeUtc>,
    pub failed_login_attempts: i32,
    pub locked_until: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::tenants::Entity", from = "Column::TenantId", to = "super::tenants::Column::Id")]
    Tenant,
    #[sea_orm(has_many = "super::sessions::Entity")]
    Sessions,
    #[sea_orm(has_many = "super::user_roles::Entity")]
    UserRoles,
}

impl Related<super::tenants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tenant.def()
    }
}

impl Related<super::sessions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Sessions.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_roles::Relation::Role.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_roles::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

// impl From<Model> for ActiveModel {
//     fn from(model: Model) -> Self {
//         Self {
//             id: Set(model.id),
//             tenant_id: Set(model.tenant_id),
//             email: Set(model.email),
//             username: Set(model.username),
//             password_hash: Set(model.password_hash),
//             first_name: Set(model.first_name),
//             last_name: Set(model.last_name),
//             phone: Set(model.phone),
//             is_active: Set(model.is_active),
//             is_verified: Set(model.is_verified),
//             last_login_at: Set(model.last_login_at),
//             failed_login_attempts: Set(model.failed_login_attempts),
//             locked_until: Set(model.locked_until),
//             created_at: Set(model.created_at),
//             updated_at: Set(model.updated_at),
//         }
//     }
// }

// Conversion between domain and infrastructure models
impl From<crate::domain::entities::User> for Model {
    fn from(user: crate::domain::entities::User) -> Self {
        Self {
            id: user.id,
            tenant_id: user.tenant_id,
            email: user.email,
            username: user.username,
            password_hash: user.password_hash,
            first_name: user.first_name,
            last_name: user.last_name,
            phone: user.phone,
            is_active: user.is_active,
            is_verified: user.is_verified,
            last_login_at: user.last_login_at,
            failed_login_attempts: user.failed_login_attempts,
            locked_until: user.locked_until,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<Model> for crate::domain::entities::User {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            tenant_id: model.tenant_id,
            email: model.email,
            username: model.username,
            password_hash: model.password_hash,
            first_name: model.first_name,
            last_name: model.last_name,
            phone: model.phone,
            is_active: model.is_active,
            is_verified: model.is_verified,
            last_login_at: model.last_login_at,
            failed_login_attempts: model.failed_login_attempts,
            locked_until: model.locked_until,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}