use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use daoyi_cloud_service::service::system::system_oauth2_access_token_service;
use salvo::Writer;
use salvo::oapi::endpoint;
use salvo::oapi::extract::QueryParam;

/// 校验访问令牌
#[endpoint(tags("RPC 服务 - OAuth2.0 令牌"))]
pub async fn check_access_token(
    token: QueryParam<&str, true>,
) -> JsonResult<OAuth2AccessTokenCheckRespDTO> {
    let res = system_oauth2_access_token_service::check_access_token(token.into_inner()).await?;
    json_ok(res)
}
