use crate::service::system::tenant::tenant_service::TenantService;
use axum::debug_handler;
use axum::extract::State;
use daoyi_cloud_common::error::ApiResult;
use daoyi_cloud_common::id::IdParamsReqVO;
use daoyi_cloud_common::models::api_extract::valid::ValidQuery;
use daoyi_cloud_common::models::app_server::AppState;
use daoyi_cloud_common::response::ApiResponse;

/// 校验租户是否合法
#[debug_handler]
pub async fn valid_tenant(
    State(AppState { db }): State<AppState>,
    ValidQuery(id): ValidQuery<IdParamsReqVO>,
) -> ApiResult<bool> {
    TenantService::valid_tenant(db, id.id).await?;
    ApiResponse::okk(Some(true))
}
