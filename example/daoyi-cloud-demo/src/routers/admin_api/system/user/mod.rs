use axum::{Router, routing};
use daoyi_cloud_api::api::admin_api::system::user::{get_simple_user_list, get_user_page};
use daoyi_cloud_common::models::app_server::AppState;

pub(crate) fn routers() -> Router<AppState> {
    Router::new()
        .route("/list-all-simple", routing::get(get_simple_user_list))
        .route("/simple-list", routing::get(get_simple_user_list))
        .route("/page", routing::get(get_user_page))
}
