use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub refresh_token: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub is_active: bool,
    pub expires_at: DateTimeUtc,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::users::Entity", from = "Column::UserId", to = "super::users::Column::Id")]
    User,
    #[sea_orm(belongs_to = "super::tenants::Entity", from = "Column::TenantId", to = "super::tenants::Column::Id")]
    Tenant,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::tenants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tenant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// impl From<Model> for ActiveModel {
//     fn from(model: Model) -> Self {
//         Self {
//             id: Set(model.id),
//             user_id: Set(model.user_id),
//             tenant_id: Set(model.tenant_id),
//             refresh_token: Set(model.refresh_token),
//             ip_address: Set(model.ip_address),
//             user_agent: Set(model.user_agent),
//             is_active: Set(model.is_active),
//             expires_at: Set(model.expires_at),
//             created_at: Set(model.created_at),
//             updated_at: Set(model.updated_at),
//         }
//     }
// }

// Conversions
impl From<crate::domain::entities::Session> for Model {
    fn from(session: crate::domain::entities::Session) -> Self {
        Self {
            id: session.id,
            user_id: session.user_id,
            tenant_id: session.tenant_id,
            refresh_token: session.refresh_token,
            ip_address: session.ip_address,
            user_agent: session.user_agent,
            is_active: session.is_active,
            expires_at: session.expires_at,
            created_at: session.created_at,
            updated_at: session.updated_at,
        }
    }
}

impl From<Model> for crate::domain::entities::Session {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            tenant_id: model.tenant_id,
            refresh_token: model.refresh_token,
            ip_address: model.ip_address,
            user_agent: model.user_agent,
            is_active: model.is_active,
            expires_at: model.expires_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}