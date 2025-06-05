use daoyi_cloud_common::constants::redis_key_constants;
use daoyi_cloud_config::{config, redis_util};
use daoyi_cloud_models::models::common_result::{CommonResult, JsonResult, json_ok};
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use reqwest::StatusCode;
use salvo::http::StatusError;
use tracing::log;

pub async fn check_access_token(token: &str) -> JsonResult<OAuth2AccessTokenCheckRespDTO> {
    let result = redis_util::get_method_cached::<OAuth2AccessTokenCheckRespDTO>(
        redis_key_constants::OAUTH2_ACCESS_TOKEN,
        token,
    )
    .await;
    if let Some(dto) = result {
        return json_ok(dto);
    }
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
    let json_str = response.text().await.map_err(|e| {
        log::error!("RPC请求失败: {}", e);
        AppError::HttpStatus(
            StatusError::from_code(StatusCode::SERVICE_UNAVAILABLE)
                .unwrap()
                .brief("认证服务不可用."),
        )
    })?;
    log::debug!("json_str: {json_str}");
    let resp: CommonResult<OAuth2AccessTokenCheckRespDTO> = serde_json::from_str(&json_str)
        .map_err(|e| {
            log::error!("数据反序列化失败: {}", e);
            AppError::HttpStatus(
                StatusError::from_code(StatusCode::UNAUTHORIZED)
                    .unwrap()
                    .brief("认证信息错误."),
            )
        })?;
    if resp.is_success() {
        if let Some(dto) = resp.clone().data() {
            redis_util::set_method_cache::<OAuth2AccessTokenCheckRespDTO>(
                redis_key_constants::OAUTH2_ACCESS_TOKEN,
                token,
                Some(60 * 60 * 24 * 365), // 一年
                &dto,
            )
            .await;
        }
        return Ok(resp);
    }
    Err(AppError::HttpStatus(
        StatusError::from_code(StatusCode::from_u16(resp.code().try_into().unwrap()).unwrap())
            .unwrap()
            .brief(resp.msg()),
    ))
}
