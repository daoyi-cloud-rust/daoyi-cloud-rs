use crate::service::system::auth::admin_auth_service::AdminAuthService;
use axum::debug_handler;
use axum::extract::State;
use daoyi_cloud_common::error::ApiResult;
use daoyi_cloud_common::models::api_extract::valid::ValidJson;
use daoyi_cloud_common::models::app_server::AppState;
use daoyi_cloud_common::response::ApiResponse;
use daoyi_cloud_entity::vo::auth::{AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo};

/// 使用账号密码登录
#[debug_handler]
pub async fn login(
    State(AppState { db }): State<AppState>,
    ValidJson(params): ValidJson<AuthLoginReqVo>,
) -> ApiResult<AuthLoginRespVo> {
    let res = AdminAuthService::login(db, params).await?;
    ApiResponse::okk(Some(res))
}

/// 获取登录用户的权限信息
#[debug_handler]
pub async fn get_permission_info(
    State(AppState { db }): State<AppState>,
) -> ApiResult<AuthPermissionInfoRespVo> {
    // let id = AdminUserService::create_user(db, params).await?;
    // ApiResponse::okk(Some(id))
    todo!()
}
