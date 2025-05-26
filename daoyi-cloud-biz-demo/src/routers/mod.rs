use daoyi_cloud_config::config;
use daoyi_cloud_hoops::hoops;
use daoyi_cloud_models::models::common_result::{json_ok, JsonResult};
use salvo::prelude::*;

mod demos;

pub fn routers() -> Router {
    Router::with_path("demos").get(root_handler).push(
        Router::with_path("api")
            .push(Router::with_path("login").post(demos::auth::post_login))
            .push(
                Router::with_path("users")
                    .hoop(hoops::auth_hoop(&config::get().jwt))
                    .get(demos::user::list_users)
                    .post(demos::user::create_user)
                    .push(
                        Router::with_path("{user_id}")
                            .put(demos::user::update_user)
                            .delete(demos::user::delete_user),
                    ),
            ),
    )
}

/// demo 根路由
#[endpoint(tags("示例"))]
pub fn root_handler() -> JsonResult<String> {
    json_ok(String::from("daoyi-cloud-biz-demo"))
}
