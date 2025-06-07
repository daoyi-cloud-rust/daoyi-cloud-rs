use axum::{Router, routing};
use daoyi_cloud_api::api::admin_api::system::dept::create_dept;
use daoyi_cloud_common::models::app_server::AppState;

pub(crate) fn routers() -> Router<AppState> {
    Router::new()
        .route("/create", routing::post(create_dept))
        .route("/delete", routing::delete(create_dept))
        .route("/get", routing::get(create_dept))
        .route("/list", routing::get(create_dept))
        .route("/list-all-simple", routing::get(create_dept))
        .route("/list-tree", routing::get(create_dept))
        .route("/simple-list", routing::get(create_dept))
        .route("/update", routing::put(create_dept))
}
