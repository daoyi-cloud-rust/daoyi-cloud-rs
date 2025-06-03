mod permission_api;
mod redis_api;
mod system_dept_controller;
mod system_oauth2_access_token;
mod system_users;

use daoyi_cloud_hoops::hoops::auth_middleware::SS;
use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use salvo::prelude::*;

pub fn routers() -> Router {
    Router::new()
        .push(
            Router::with_path("rpc-api").push(
                Router::with_path("system")
                    .get(root_handler)
                    .push(
                        Router::with_path("oauth2").push(
                            Router::with_path("token").push(
                                Router::with_path("check")
                                    .get(system_oauth2_access_token::check_access_token),
                            ),
                        ),
                    )
                    .push(
                        Router::with_path("permission").push(
                            Router::with_path("has-any-permissions")
                                .post(permission_api::has_any_permission),
                        ),
                    ),
            ),
        )
        .push(
            Router::with_path("admin-api").push(
                Router::with_path("system")
                    .get(root_handler)
                    .push(
                        Router::with_path("dept")
                            .push(
                                Router::with_path("create")
                                    .hoop(SS::has_permission("system:dept:create".to_string()))
                                    .post(system_dept_controller::create_dept),
                            )
                            .push(
                                Router::with_path("delete")
                                    .hoop(SS::has_permission("system:dept:delete".to_string()))
                                    .delete(system_dept_controller::delete_dept),
                            )
                            .push(
                                Router::with_path("get")
                                    .hoop(SS::has_permission("system:dept:query".to_string()))
                                    .get(system_dept_controller::get_dept),
                            )
                            .push(
                                Router::with_path("list")
                                    .hoop(SS::has_permission("system:dept:query".to_string()))
                                    .post(system_dept_controller::dept_list),
                            )
                            .push(
                                Router::with_path("list-tree")
                                    .hoop(SS::has_permission("system:dept:query".to_string()))
                                    .post(system_dept_controller::dept_list_tree),
                            ),
                    )
                    .push(
                        Router::with_path("user")
                            .push(
                                Router::with_path("get")
                                    .hoop(SS::has_permission("system:user:query".to_string()))
                                    .get(system_users::get_by_id),
                            )
                            .push(
                                Router::with_path("gen-password").get(system_users::hash_password),
                            ),
                    )
                    .push(
                        Router::with_path("auth").push(
                            Router::with_path("login").post(system_oauth2_access_token::login),
                        ),
                    )
                    .push(
                        Router::with_path("redis").push(
                            Router::with_path("clear")
                                .hoop(SS::has_permission("system:redis:clear".to_string()))
                                .post(redis_api::clear_cache),
                        ),
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
