use daoyi_cloud_common::config::tenant_config::TenantConfig;
use daoyi_cloud_common::error::CusErr;
use daoyi_cloud_common::res::Res;
use spring_redis::Redis;
use spring_web::axum::response::{IntoResponse, Response};
use spring_web::extractor::{Component, Config};
use spring_web::{axum::middleware, extractor};

pub async fn tenant_middleware(
    Component(redis): Component<Redis>,
    Config(config): Config<TenantConfig>,
    req: extractor::Request,
    next: middleware::Next,
) -> Response {
    if config.header_key.is_none() {
        // 不启用租户
        return next.run(req).await;
    }
    if config.ignore_urls.contains(&req.uri().to_string()) {
        // 忽略租户的 URL
        return next.run(req).await;
    }
    // 校验 Token
    let tenant_id: i32 = req
        .headers()
        .get("tenant-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("0")
        .parse()
        .unwrap();
    if tenant_id <= 0 {
        return Res::<String>::error(anyhow::Error::from(CusErr::AuthError(
            "租户无效".to_string(),
        )))
        .into_response();
    }

    let user_id = "123"; // 调用认证服务

    // 将用户信息注入请求扩展
    let mut req = req;
    req.extensions_mut().insert(user_id);

    next.run(req).await
}
