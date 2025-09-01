// Internal endpoints for service-to-service communication
use axum::{
    extract::State,
    response::Json as ResponseJson,
    Json,
};
use crate::presentation::routes::AppState;
use crate_messaging::{ValidateTokenRequest, ValidateTokenResponse, GetUserRequest, GetUserResponse};
use crate_errors::Result;

/// Internal endpoint for token validation (used by other services)
pub async fn validate_token_internal(
    State(state): State<AppState>,
    Json(request): Json<ValidateTokenRequest>,
) -> Result<ResponseJson<ValidateTokenResponse>> {
    match state.auth_service.validate_token(&request.token).await {
        Ok(validation_result) => {
            if validation_result.valid {
                // Check if user has required permissions
                let has_permissions = if request.required_permissions.is_empty() {
                    true
                } else {
                    request.required_permissions.iter().all(|perm| {
                        crate_security::PermissionChecker::has_permission(&validation_result.permissions, perm)
                    })
                };

                Ok(ResponseJson(ValidateTokenResponse {
                    valid: has_permissions,
                    user_id: validation_result.user_id,
                    tenant_id: validation_result.tenant_id,
                    permissions: validation_result.permissions,
                    roles: validation_result.roles,
                    error: if has_permissions { None } else { Some("Insufficient permissions".to_string()) },
                }))
            } else {
                Ok(ResponseJson(ValidateTokenResponse {
                    valid: false,
                    user_id: None,
                    tenant_id: None,
                    permissions: vec![],
                    roles: vec![],
                    error: Some("Invalid token".to_string()),
                }))
            }
        },
        Err(e) => Ok(ResponseJson(ValidateTokenResponse {
            valid: false,
            user_id: None,
            tenant_id: None,
            permissions: vec![],
            roles: vec![],
            error: Some(e.to_string()),
        }))
    }
}

/// Internal endpoint for getting user information (used by other services)
pub async fn get_user_internal(
    State(state): State<AppState>,
    Json(request): Json<GetUserRequest>,
) -> Result<ResponseJson<Option<GetUserResponse>>> {
    match state.user_service.get_user(request.user_id).await {
        Ok(user_response) => {
            if user_response.tenant_id == request.tenant_id {
                Ok(ResponseJson(Some(GetUserResponse {
                    user_id: user_response.id,
                    tenant_id: user_response.tenant_id,
                    email: user_response.email,
                    username: user_response.username,
                    first_name: user_response.first_name,
                    last_name: user_response.last_name,
                    is_active: user_response.is_active,
                    roles: user_response.roles,
                    permissions: user_response.permissions,
                })))
            } else {
                Ok(ResponseJson(None))
            }
        },
        Err(_) => Ok(ResponseJson(None))
    }
}