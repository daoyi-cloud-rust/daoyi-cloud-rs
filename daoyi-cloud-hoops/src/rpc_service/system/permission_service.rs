use daoyi_cloud_config::{config, redis_util};
use daoyi_cloud_models::models::common_result::{CommonResult, JsonResult};
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::system::permission_check_req_vo::PermissionCheckReqVO;
use reqwest::StatusCode;
use salvo::http::StatusError;
use tracing::log;

fn gen_redis_key(user_id: &i64, permissions: &Vec<String>) -> String {
    let mut vec = permissions.to_owned();
    vec.sort();
    format!("{}:{}", user_id, vec.join("|"))
}
pub async fn has_any_permission(check_req_vo: PermissionCheckReqVO) -> bool {
    let suffix = gen_redis_key(&(check_req_vo.user_id), &(check_req_vo.permissions));
    if let Some(json_str) =
        redis_util::get_method_cached::<String>("has_any_permission", &suffix).await
    {
        return json_str.eq("true");
    }
    let result = check_permission(check_req_vo).await;
    if result.is_ok() {
        let x = "true".eq(result
            .unwrap()
            .data()
            .unwrap_or_else(|| "false".to_string())
            .as_str());
        redis_util::set_method_cache::<String>(
            "has_any_permission",
            &suffix,
            Some(60 * 10), // 10分钟
            &x.to_string(),
        )
        .await;
        return x;
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
        return Ok(resp);
    }
    Err(AppError::HttpStatus(
        StatusError::from_code(StatusCode::from_u16(resp.code()).unwrap())
            .unwrap()
            .brief(resp.msg()),
    ))
}
