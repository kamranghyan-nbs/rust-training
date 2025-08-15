// use crate::{
//     entities::{prelude::*, user},
//     error::AppError,
//     models::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
//     utils::{create_jwt, hash_password, verify_password},
// };
// use sea_orm::{prelude::*, ActiveModelTrait, Set};
// use std::sync::Arc;

// pub struct AuthService;

// impl AuthService {
//     pub async fn login(
//         db: &DatabaseConnection,
//         config: Arc<crate::config::Config>,
//         request: LoginRequest,
//     ) -> Result<AuthResponse, AppError> {
//         let user = User::find()
//             .filter(user::Column::Username.eq(&request.username))
//             .one(db)
//             .await?
//             .ok_or(AppError::Unauthorized)?;

//         let is_valid = verify_password(&request.password, &user.password_hash)?;
//         if !is_valid {
//             return Err(AppError::Unauthorized);
//         }

//         let token = create_jwt(
//             &user.id.to_string(),
//             &user.username,
//             &config.jwt_secret,
//             config.jwt_expiration,
//         )?;

//         Ok(AuthResponse {
//             token,
//             user: UserResponse {
//                 id: user.id,
//                 username: user.username,
//                 email: user.email,
//             },
//         })
//     }

//     pub async fn register(
//         db: &DatabaseConnection,
//         config: Arc<crate::config::Config>,
//         request: RegisterRequest,
//     ) -> Result<AuthResponse, AppError> {
//         // Check if user already exists
//         let existing_user = User::find()
//             .filter(
//                 user::Column::Username
//                     .eq(&request.username)
//                     .or(user::Column::Email.eq(&request.email)),
//             )
//             .one(db)
//             .await?;

//         if existing_user.is_some() {
//             return Err(AppError::Conflict("User already exists".to_string()));
//         }

//         let password_hash = hash_password(&request.password)?;
//         let user_id = uuid::Uuid::new_v4();
//         let now = chrono::Utc::now();

//         let new_user = user::ActiveModel {
//             id: Set(user_id),
//             username: Set(request.username.clone()),
//             email: Set(request.email.clone()),
//             password_hash: Set(password_hash),
//             created_at: Set(now),
//             updated_at: Set(now),
//         };

//         let user = new_user.insert(db).await?;

//         let token = create_jwt(
//             &user.id.to_string(),
//             &user.username,
//             &config.jwt_secret,
//             config.jwt_expiration,
//         )?;

//         Ok(AuthResponse {
//             token,
//             user: UserResponse {
//                 id: user.id,
//                 username: user.username,
//                 email: user.email,
//             },
//         })
//     }
// }

use crate::{
    entities::{prelude::*, user},
    error::AppError,
    models::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
    utils::{create_jwt, hash_password, verify_password},
};
use sea_orm::{prelude::*, ActiveModelTrait, Set};
use std::sync::Arc;

pub struct AuthService;

impl AuthService {
    #[cfg(not(feature = "mock"))]
    pub async fn login(
        db: &DatabaseConnection,
        config: Arc<crate::config::Config>,
        request: LoginRequest,
    ) -> Result<AuthResponse, AppError> {
        let user = User::find()
            .filter(user::Column::Username.eq(&request.username))
            .one(db)
            .await?
            .ok_or(AppError::Unauthorized)?;

        let is_valid = verify_password(&request.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::Unauthorized);
        }

        let token = create_jwt(
            &user.id.to_string(),
            &user.username,
            &config.jwt_secret,
            config.jwt_expiration,
        )?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        })
    }

    #[cfg(feature = "mock")]
    pub async fn login(
        _db: &(),
        config: Arc<crate::config::Config>,
        request: LoginRequest,
    ) -> Result<AuthResponse, AppError> {
        if request.username == "admin" && request.password == "password" {
            Ok(AuthResponse {
                token: create_jwt("1", "admin", &config.jwt_secret, config.jwt_expiration)?,
                user: UserResponse {
                    id: uuid::Uuid::new_v4(),
                    username: "admin".to_string(),
                    email: "admin@example.com".to_string(),
                },
            })
        } else {
            Err(AppError::Unauthorized)
        }
    }

    #[cfg(not(feature = "mock"))]
    pub async fn register(
        db: &DatabaseConnection,
        config: Arc<crate::config::Config>,
        request: RegisterRequest,
    ) -> Result<AuthResponse, AppError> {
        let existing_user = User::find()
            .filter(
                user::Column::Username
                    .eq(&request.username)
                    .or(user::Column::Email.eq(&request.email)),
            )
            .one(db)
            .await?;

        if existing_user.is_some() {
            return Err(AppError::Conflict("User already exists".to_string()));
        }

        let password_hash = hash_password(&request.password)?;
        let user_id = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();

        let new_user = user::ActiveModel {
            id: Set(user_id),
            username: Set(request.username.clone()),
            email: Set(request.email.clone()),
            password_hash: Set(password_hash),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let user = new_user.insert(db).await?;

        let token = create_jwt(
            &user.id.to_string(),
            &user.username,
            &config.jwt_secret,
            config.jwt_expiration,
        )?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        })
    }

    #[cfg(feature = "mock")]
    pub async fn register(
        _db: &(),
        config: Arc<crate::config::Config>,
        request: RegisterRequest,
    ) -> Result<AuthResponse, AppError> {
        Ok(AuthResponse {
            token: create_jwt("2", &request.username, &config.jwt_secret, config.jwt_expiration)?,
            user: UserResponse {
                id: uuid::Uuid::new_v4(),
                username: request.username.clone(),
                email: request.email.clone(),
            },
        })
    }
}

