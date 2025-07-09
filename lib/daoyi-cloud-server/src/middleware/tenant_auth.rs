use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use daoyi_cloud_api::api::feign_client::system::tenant::TenantApi;
use daoyi_cloud_common::error::ApiError;
use daoyi_cloud_common::utils::path_matches::path_any_matches;
use daoyi_cloud_config::config;
use daoyi_cloud_config::config::redis_config::RedisUtils;

// 租户上下文（将注入请求）
#[derive(Debug, Clone, Default)]
pub struct TenantContext {
    id: Option<i64>,
    // 可以添加更多租户相关数据，如权限等
}

impl TenantContext {
    pub fn id(&self) -> i64 {
        self.id.unwrap_or(0)
    }
}

// 中间件：验证租户ID并注入上下文
pub async fn tenant_auth_middleware(request: Request, next: Next) -> Result<Response, ApiError> {
    // 1. 获取请求头
    let headers = request.headers().clone();
    let path = request.uri().path();
    let tenant_config = config::get().tenant();
    let ignore_urls = &tenant_config.ignore_urls;
    let header_key = &tenant_config.header;
    let enable = &tenant_config.enable;
    if !enable {
        // 继续处理请求
        return Ok(next.run(request).await);
    }
    // 2. 从头部提取租户ID
    let tenant_in_header = headers.get(header_key);
    if path_any_matches(ignore_urls, path) {
        if tenant_in_header.is_none() {
            // 继续处理请求
            return Ok(next.run(request).await);
        }
    }

    let tenant_id = tenant_in_header
        .map(|value| -> Result<_, ApiError> {
            let tenant_id = value
                .to_str()
                .map_err(|_| ApiError::Unauthenticated(format!("{} 请求头无效", header_key)))?;
            Ok(tenant_id)
        })
        .transpose()?
        .ok_or_else(|| ApiError::Unauthenticated(format!("{}请求头不能为空", header_key)))?;

    let tenant_id = tenant_id
        .parse::<i64>()
        .map_err(|_| ApiError::Unauthenticated(format!("{}请求头值无效", header_key)))?;

    // 3. 验证租户有效性（包含缓存和数据库检查）
    if !is_valid_tenant(tenant_id).await {
        return Err(ApiError::Unauthenticated(format!(
            "租户 {} 不存在",
            tenant_id
        )));
    }

    // 4. 创建租户上下文
    let tenant_ctx = TenantContext {
        id: Some(tenant_id),
    };

    // 5. 克隆请求并注入上下文
    let mut request = request;
    request.extensions_mut().insert(tenant_ctx);

    // 6. 继续处理请求
    Ok(next.run(request).await)
}

async fn is_valid_tenant(tenant_id: i64) -> bool {
    let valid: Option<bool> = RedisUtils::get_method_cached("is_valid_tenant", &tenant_id).await;
    if valid.is_some() {
        return valid.unwrap();
    }
    let result = TenantApi::valid_tenant(tenant_id).await;
    if result.is_ok() {
        let result = result.unwrap();
        let result = result.data();
        if result {
            RedisUtils::set_method_cache("is_valid_tenant", &tenant_id, Some(10), &result).await;
        }
        return result;
    }
    false
}
