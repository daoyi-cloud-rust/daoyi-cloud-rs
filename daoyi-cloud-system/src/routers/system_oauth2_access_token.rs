use daoyi_cloud_config::config;
use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::system::auth_login_req_vo::AuthLoginReqVO;
use daoyi_cloud_models::models::system::auth_login_resp_vo::AuthLoginRespVO;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use daoyi_cloud_models::models::system::system_users::SystemUsersModel;
use daoyi_cloud_service::service::system::system_oauth2_access_token_service;
use daoyi_cloud_service::service::system::system_oauth2_access_token_service::create_token_after_login_success;
use daoyi_cloud_service::service::system::system_users_service::get_system_users_by_username;
use daoyi_cloud_utils::utils;
use salvo::http::StatusError;
use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, QueryParam};
use salvo::{Depot, Writer};

/// 校验访问令牌
#[endpoint(tags("RPC 服务 - OAuth2.0 令牌"))]
pub async fn check_access_token(
    token: QueryParam<&str, true>,
) -> JsonResult<OAuth2AccessTokenCheckRespDTO> {
    let res = system_oauth2_access_token_service::check_access_token(token.into_inner()).await?;
    json_ok(res)
}

/// 使用账号密码登录
#[endpoint(tags("管理后台 - 系统管理 - 认证"))]
pub async fn login(
    idata: JsonBody<AuthLoginReqVO>,
    depot: &mut Depot,
) -> JsonResult<AuthLoginRespVO> {
    let tenant_middleware_config = &config::get().tenant;
    let Ok(tenant_id) = depot.get::<i64>(tenant_middleware_config.header_name.as_str()) else {
        return Err(AppError::internal("租户ID不存在.".to_string()));
    };
    let res = get_system_users_by_username(idata.username.as_str(), Some(tenant_id)).await?;
    if utils::verify_password(&idata.password, &res.password).is_err() {
        return Err(StatusError::unauthorized()
            .brief("用户名或密码错误.")
            .into());
    }
    let res = create_token_after_login_success(SystemUsersModel::from(res)).await?;
    json_ok(res)
}
