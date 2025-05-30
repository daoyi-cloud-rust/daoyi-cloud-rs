use daoyi_cloud_config::{config, redis_util};
use daoyi_cloud_models::models::common_result::{CommonResult, JsonResult};
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::system::permission_check_req_vo::PermissionCheckReqVO;
use redis::AsyncCommands;
use reqwest::StatusCode;
use salvo::http::StatusError;
use tracing::log;

fn gen_redis_key(user_id: &i64, permissions: &Vec<String>) -> String {
    let mut vec = permissions.to_owned();
    vec.sort();
    format!("has_any_permission:{}:{}", user_id, vec.join("|"))
}
pub async fn has_any_permission(check_req_vo: PermissionCheckReqVO) -> bool {
    let result = redis_util::pool()
        .get::<&str, String>(
            gen_redis_key(&(check_req_vo.user_id), &(check_req_vo.permissions)).as_str(),
        )
        .await;
    if let Ok(json_str) = result {
        let dto: Result<&str, _> = serde_json::from_str(&json_str);
        if let Ok(dto) = dto {
            return "true".eq(dto);
        }
    }
    let result = check_permission(check_req_vo).await;
    if result.is_ok() {
        return "true".eq(result
            .unwrap()
            .data()
            .unwrap_or_else(|| "false".to_string())
            .as_str());
    }
    false
}

async fn check_permission(check_req_vo: PermissionCheckReqVO) -> JsonResult<String> {
    let check_access_token_url = &config::get().rpc.has_any_permission;
    let request_url = format!("{check_access_token_url}");
    let vo = check_req_vo.clone();
    let response = reqwest::Client::new()
        .post(request_url)
        .json(&vo)
        .send()
        .await
        .map_err(|e| {
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
    let resp: CommonResult<String> = serde_json::from_str(&json_str).map_err(|e| {
        log::error!("数据反序列化失败: {}", e);
        AppError::HttpStatus(
            StatusError::from_code(StatusCode::UNAUTHORIZED)
                .unwrap()
                .brief("认证信息错误."),
        )
    })?;
    if resp.is_success() {
        if let Some(dto) = resp.clone().data() {
            redis_util::pool()
                .set_ex::<&str, String, String>(
                    gen_redis_key(&(check_req_vo.user_id), &(check_req_vo.permissions)).as_str(),
                    serde_json::to_string(&dto).unwrap(),
                    60,
                )
                .await
                .expect("redis set error");
        }
        return Ok(resp);
    }
    Err(AppError::HttpStatus(
        StatusError::from_code(StatusCode::from_u16(resp.code()).unwrap())
            .unwrap()
            .brief(resp.msg()),
    ))
}
