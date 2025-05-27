mod system_users;

use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use salvo::prelude::*;

pub fn routers() -> Router {
    Router::with_path("system")
        .get(root_handler)
        .push(Router::with_path("user").push(Router::with_path("get").get(system_users::get_by_id)))
}

/// 系统管理模块 根路由
#[endpoint(tags("管理后台 - 系统管理"))]
pub fn root_handler() -> JsonResult<String> {
    json_ok(String::from("daoyi-cloud-system"))
}
