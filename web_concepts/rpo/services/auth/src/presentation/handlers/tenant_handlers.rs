use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json as ResponseJson,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::presentation::extractors::AuthUser;
use crate::presentation::routes::AppState;
use crate::domain::entities::{CreateTenantRequest, TenantResponse};
use crate_errors::Result;

#[derive(Deserialize)]
pub struct ListTenantsQuery {
    page: Option<u64>,
    page_size: Option<u64>,
}

/// Create a new tenant
#[utoipa::path(
    post,
    path = "/tenants",
    tag = "tenants",
    request_body = CreateTenantRequest,
    responses(
        (status = 201, description = "Tenant created successfully", body = TenantResponse),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Tenant already exists")
    ),
    security(
        ("bearer_auth" = ["system.settings"])
    )
)]
pub async fn create_tenant(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(request): Json<CreateTenantRequest>,
) -> Result<(StatusCode, ResponseJson<TenantResponse>)> {
    let tenant = state.tenant_service.create_tenant(request).await?;
    Ok((StatusCode::CREATED, ResponseJson(tenant)))
}

/// Get tenant by ID
#[utoipa::path(
    get,
    path = "/tenants/{id}",
    tag = "tenants",
    params(
        ("id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "Tenant found", body = TenantResponse),
        (status = 404, description = "Tenant not found")
    ),
    security(
        ("bearer_auth" = ["system.settings"])
    )
)]
pub async fn get_tenant(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<TenantResponse>> {
    let tenant = state.tenant_service.get_tenant(id).await?;
    Ok(ResponseJson(tenant))
}

/// List tenants
#[utoipa::path(
    get,
    path = "/tenants",
    tag = "tenants",
    params(
        ("page" = Option<u64>, Query, description = "Page number (default: 0)"),
        ("page_size" = Option<u64>, Query, description = "Page size (default: 20)")
    ),
    responses(
        (status = 200, description = "Tenants list", body = Vec<TenantResponse>)
    ),
    security(
        ("bearer_auth" = ["system.settings"])
    )
)]
pub async fn list_tenants(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(params): Query<ListTenantsQuery>,
) -> Result<ResponseJson<Vec<TenantResponse>>> {
    let page = params.page.unwrap_or(0);
    let page_size = params.page_size.unwrap_or(20).min(100);
    
    let tenants = state.tenant_service.list_tenants(page, page_size).await?;
    Ok(ResponseJson(tenants))
}

/// Update tenant
#[utoipa::path(
    put,
    path = "/tenants/{id}",
    tag = "tenants",
    params(
        ("id" = Uuid, Path, description = "Tenant ID")
    ),
    responses(
        (status = 200, description = "Tenant updated successfully", body = TenantResponse),
        (status = 404, description = "Tenant not found")
    ),
    security(
        ("bearer_auth" = ["system.settings"])
    )
)]
pub async fn update_tenant(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(updates): Json<serde_json::Value>,
) -> Result<ResponseJson<TenantResponse>> {
    let tenant = state.tenant_service.update_tenant(id, updates).await?;
    Ok(ResponseJson(tenant))
}