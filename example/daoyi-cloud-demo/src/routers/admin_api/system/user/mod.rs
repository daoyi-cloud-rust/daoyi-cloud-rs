use axum::{Router, routing};
use daoyi_cloud_api::api::admin_api::system::user::*;
use daoyi_cloud_common::models::app_server::AppState;

pub(crate) fn routers() -> Router<AppState> {
    Router::new()
        .route("/list-all-simple", routing::get(get_simple_user_list))
        .route("/simple-list", routing::get(get_simple_user_list))
        .route("/page", routing::post(get_user_page))
        .route("/create", routing::post(create_user))
        .route("/update/{id}", routing::put(update_user))
        .route("/delete/{id}", routing::delete(delete_user))
        .route("/get/{id}", routing::get(get_user))
}
