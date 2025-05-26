use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::{static_embed, EmbeddedFileExt};

mod auth;
mod demo;
mod user;

use crate::{config, empty_ok, hoops, Empty, JsonResult};

#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

pub fn root() -> Router {
    let favicon = Assets::get("favicon.ico")
        .expect("favicon not found")
        .into_handler();
    let router = Router::new()
        .hoop(Logger::new())
        .get(root_handler)
        // 示例路由
        .push(
            Router::with_path("demos")
                .get(demo::hello)
                .push(Router::with_path("login").get(auth::login_page))
                .push(Router::with_path("users").get(user::list_page))
                .push(
                    Router::with_path("api")
                        .push(Router::with_path("login").post(auth::post_login))
                        .push(
                            Router::with_path("users")
                                .hoop(hoops::auth_hoop(&config::get().jwt))
                                .get(user::list_users)
                                .post(user::create_user)
                                .push(
                                    Router::with_path("{user_id}")
                                        .put(user::update_user)
                                        .delete(user::delete_user),
                                ),
                        ),
                )
                .push(Router::with_path("favicon.ico").get(favicon))
                .push(Router::with_path("assets/{**rest}").get(static_embed::<Assets>())),
        );
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(Scalar::new("/api-doc/openapi.json").into_router("scalar"))
}

/// 根路由
#[endpoint]
pub fn root_handler() -> JsonResult<Empty> {
    empty_ok()
}
