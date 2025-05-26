use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use salvo::prelude::*;

pub fn root() -> Router {
    let router = Router::new().hoop(Logger::new()).get(root_handler);
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}

/// 根路由
#[endpoint]
pub fn root_handler() -> JsonResult<String> {
    json_ok(String::from("daoyi-cloud-system"))
}
