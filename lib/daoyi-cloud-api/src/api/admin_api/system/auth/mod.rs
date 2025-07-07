use crate::service::system::auth::admin_auth_service::AdminAuthService;
use axum::debug_handler;
use axum::extract::State;
use daoyi_cloud_common::error::ApiResult;
use daoyi_cloud_common::models::api_extract::path::Path;
use daoyi_cloud_common::models::api_extract::valid::ValidJson;
use daoyi_cloud_common::models::app_server::AppState;
use daoyi_cloud_common::response::ApiResponse;
use daoyi_cloud_common::utils::base64_util;
use daoyi_cloud_entity::vo::auth::{AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo};

/// 加密密码
#[debug_handler]
pub async fn encode_password(Path(passwd): Path<String>) -> ApiResult<String> {
    let res = base64_util::encode_password(&passwd).await?;
    ApiResponse::okk(Some(res))
}

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
