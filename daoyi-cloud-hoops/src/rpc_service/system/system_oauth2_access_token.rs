use daoyi_cloud_config::config;
use daoyi_cloud_models::models::common_result::{CommonResult, JsonResult};
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use reqwest::StatusCode;
use salvo::http::StatusError;
use tracing::log;

// pub async fn check_access_token_redis(token: &str) -> JsonResult<OAuth2AccessTokenCheckRespDTO> {
//     let redis = redis_util::pool().to_owned();
//     let result: String = redis.get(token).await.context("获取访问令牌失败");
//
//     json_ok(OAuth2AccessTokenCheckRespDTO::from(v))
// }

pub async fn check_access_token(token: &str) -> JsonResult<OAuth2AccessTokenCheckRespDTO> {
    let check_access_token_url = &config::get().rpc.check_access_token;
    let request_url = format!("{check_access_token_url}?token={token}");
    let response = reqwest::get(request_url).await.map_err(|e| {
        log::error!("RPC请求失败: {}", e);
        AppError::HttpStatus(
            StatusError::from_code(StatusCode::SERVICE_UNAVAILABLE)
                .unwrap()
                .brief("认证服务不可用."),
        )
    })?;
    if !response.status().is_success() {
        let status = response.status();
        let error_msg = match status {
            StatusCode::UNAUTHORIZED => "无效访问令牌",
            StatusCode::BAD_REQUEST => "请求参数错误",
            _ => "认证服务内部错误",
        };
        return Err(AppError::HttpStatus(
            StatusError::from_code(StatusCode::SERVICE_UNAVAILABLE)
                .unwrap()
                .brief(error_msg),
        ));
    }
    let json: serde_json::Value = response.json().await.map_err(|e| {
        log::error!("RPC请求失败: {}", e);
        AppError::HttpStatus(
            StatusError::from_code(StatusCode::SERVICE_UNAVAILABLE)
                .unwrap()
                .brief("认证服务不可用."),
        )
    })?;
    let resp: CommonResult<OAuth2AccessTokenCheckRespDTO> =
        serde_json::from_value(json).map_err(|e| {
            log::error!("数据反序列化失败: {}", e);
            AppError::HttpStatus(
                StatusError::from_code(StatusCode::UNAUTHORIZED)
                    .unwrap()
                    .brief("认证信息错误."),
            )
        })?;
    if resp.is_success() {
        return Ok(resp);
    }
    Err(AppError::HttpStatus(
        StatusError::from_code(StatusCode::from_u16(resp.code()).unwrap())
            .unwrap()
            .brief(resp.msg()),
    ))
}
