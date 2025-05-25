use daoyi_cloud_common::error::CusErr;
use daoyi_cloud_common::res::Res;
use spring_web::axum::response::{IntoResponse, Response};
use spring_web::{
    axum::middleware,
    extractor,
};

pub async fn auth_middleware(
    // Component(db): Component<DbConn>,
    req: extractor::Request,
    next: middleware::Next,
) -> Response {
    // 校验 Token
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // 假的校验
    if token != "123" {
        return Res::<String>::error(anyhow::Error::from(CusErr::AppRuleError("翻天调了".to_string()))).into_response();
    }

    let user_id = "123"; // 调用认证服务

    // 将用户信息注入请求扩展
    let mut req = req;
    req.extensions_mut().insert(user_id);

    next.run(req).await
}
