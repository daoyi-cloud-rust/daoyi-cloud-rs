mod permission_api;
mod redis_api;
mod system_area_controller;
mod system_dept_controller;
mod system_login_log_controller;
mod system_menu_controller;
mod system_oauth2_access_token;
mod system_operate_log_controller;
mod system_post_controller;
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
                            )
                            .push(
                                Router::with_path("update")
                                    .hoop(SS::has_permission("system:dept:update".to_string()))
                                    .put(system_dept_controller::update_dept),
                            ),
                    )
                    .push(
                        Router::with_path("menu")
                            .push(
                                Router::with_path("create")
                                    .hoop(SS::has_permission("system:menu:create".to_string()))
                                    .post(system_menu_controller::create_menu),
                            )
                            .push(
                                Router::with_path("delete")
                                    .hoop(SS::has_permission("system:menu:delete".to_string()))
                                    .delete(system_menu_controller::delete_menu),
                            )
                            .push(
                                Router::with_path("get")
                                    .hoop(SS::has_permission("system:menu:query".to_string()))
                                    .get(system_menu_controller::get_menu),
                            )
                            .push(
                                Router::with_path("list")
                                    .hoop(SS::has_permission("system:menu:query".to_string()))
                                    .post(system_menu_controller::menu_list),
                            )
                            .push(
                                Router::with_path("list-tree")
                                    .hoop(SS::has_permission("system:menu:query".to_string()))
                                    .post(system_menu_controller::menu_list_tree),
                            )
                            .push(
                                Router::with_path("update")
                                    .hoop(SS::has_permission("system:menu:update".to_string()))
                                    .put(system_menu_controller::update_menu),
                            ),
                    )
                    .push(
                        Router::with_path("operate-log").push(
                            Router::with_path("page")
                                .hoop(SS::has_permission("system:operate-log:query".to_string()))
                                .post(system_operate_log_controller::page_operate_log),
                        ),
                    )
                    .push(
                        Router::with_path("area")
                            .push(
                                Router::with_path("tree")
                                    .get(system_area_controller::get_area_tree),
                            )
                            .push(
                                Router::with_path("get-by-ip")
                                    .get(system_area_controller::get_area_by_ip),
                            ),
                    )
                    .push(
                        Router::with_path("login-log").push(
                            Router::with_path("page")
                                .hoop(SS::has_permission("system:login-log:query".to_string()))
                                .post(system_login_log_controller::page_login_log),
                        ),
                    )
                    .push(
                        Router::with_path("post")
                            .push(
                                Router::with_path("create")
                                    .hoop(SS::has_permission("system:post:create".to_string()))
                                    .post(system_post_controller::create_post),
                            )
                            .push(
                                Router::with_path("delete")
                                    .hoop(SS::has_permission("system:post:delete".to_string()))
                                    .delete(system_post_controller::delete_post),
                            )
                            .push(
                                Router::with_path("get")
                                    .hoop(SS::has_permission("system:post:query".to_string()))
                                    .get(system_post_controller::get_post),
                            )
                            .push(
                                Router::with_path("page")
                                    .hoop(SS::has_permission("system:post:query".to_string()))
                                    .post(system_post_controller::post_list),
                            )
                            .push(
                                Router::with_path("update")
                                    .hoop(SS::has_permission("system:post:update".to_string()))
                                    .put(system_post_controller::update_post),
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
