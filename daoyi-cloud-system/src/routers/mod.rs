mod system_oauth2_access_token;
mod system_users;

use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use salvo::prelude::*;

pub fn routers() -> Router {
    Router::new()
        .push(
            Router::with_path("rpc-api").push(Router::with_path("system").get(root_handler).push(
                Router::with_path("oauth2").push(Router::with_path("token").push(
                    Router::with_path("check").get(system_oauth2_access_token::check_access_token),
                )),
            )),
        )
        .push(
            Router::with_path("admin-api").push(
                Router::with_path("system")
                    .get(root_handler)
                    .push(
                        Router::with_path("user")
                            .push(Router::with_path("get").get(system_users::get_by_id)),
                    )
                    .push(
                        Router::with_path("auth")
                            .push(Router::with_path("refresh-token").post(system_users::get_by_id)),
                    ),
            ),
        )
    // .push(
    //     Router::with_path("app-api").push(
    //         Router::with_path("system").get(root_handler).push(
    //             Router::with_path("user")
    //                 .push(Router::with_path("get").get(system_users::get_by_id)),
    //         ),
    //     ),
    // )
}

/// 系统管理模块 根路由
#[endpoint(tags("管理后台 - 系统管理"))]
pub fn root_handler() -> JsonResult<String> {
    json_ok(String::from("daoyi-cloud-system"))
}
