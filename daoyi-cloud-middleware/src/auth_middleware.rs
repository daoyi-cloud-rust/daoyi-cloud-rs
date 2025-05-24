use spring_web::{
    axum::{http, middleware, response},
    extractor,
};

pub async fn auth_middleware(
    // Component(db): Component<DbConn>,
    req: extractor::Request,
    next: middleware::Next,
) -> Result<response::Response, http::StatusCode> {
    // 校验 Token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // 假的校验
    if token != "123" {
        return Err(http::StatusCode::UNAUTHORIZED);
    }

    let user_id = "123"; // 调用认证服务

    // 将用户信息注入请求扩展
    let mut req = req;
    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
