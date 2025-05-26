use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use salvo::prelude::*;

pub fn routers() -> Router {
    Router::with_path("system").get(root_handler)
}

/// 系统管理模块 根路由
#[endpoint]
pub fn root_handler() -> JsonResult<String> {
    json_ok(String::from("daoyi-cloud-system"))
}
