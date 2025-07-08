use crate::service::system::auth::admin_auth_service::AdminAuthService;
use crate::service::system::user::admin_user_service::AdminUserService;
use axum::extract::{ConnectInfo, State};
use axum::{Extension, debug_handler};
use daoyi_cloud_common::error::ApiResult;
use daoyi_cloud_common::models::api_extract::path::Path;
use daoyi_cloud_common::models::api_extract::valid::ValidJson;
use daoyi_cloud_common::models::app_server::AppState;
use daoyi_cloud_common::response::ApiResponse;
use daoyi_cloud_common::utils::base64_util;
use daoyi_cloud_config::config::jwt::Principal;
use daoyi_cloud_entity::vo::auth::{AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo};
use std::net::SocketAddr;

/// 加密密码
#[debug_handler]
pub async fn encode_password(Path(passwd): Path<String>) -> ApiResult<String> {
    let res = base64_util::encode_password(&passwd).await?;
    ApiResponse::okk(Some(res))
}

/// 使用账号密码登录
#[debug_handler]
#[tracing::instrument(name = "login", skip_all, fields(account = %params.username, ip = %addr))]
pub async fn login(
    State(AppState { db }): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<AuthLoginReqVo>,
) -> ApiResult<AuthLoginRespVo> {
    tracing::debug!("开始处理登录请求");
    let res = AdminAuthService::login(db, params).await?;
    tracing::debug!("登录成功，生成token {}", res.access_token);
    ApiResponse::okk(Some(res))
}

/// 获取登录用户的权限信息
#[debug_handler]
pub async fn get_permission_info(
    State(AppState { db }): State<AppState>,
    Extension(principal): Extension<Principal>,
) -> ApiResult<AuthPermissionInfoRespVo> {
    // 1.1 获得用户信息
    let user = AdminUserService::validate_user_exists(db, Some(&principal.id)).await?;
    if user.is_none() {
        return ApiResponse::okk(None);
    }
    // 1.2 获得角色列表
    // 1.3 获得菜单列表
    // 2. 拼接结果返回
    ApiResponse::okk(Some(AuthPermissionInfoRespVo {
        menus: vec![],
        permissions: vec![],
        roles: vec![],
        user: user.unwrap().into(),
    }))
}
