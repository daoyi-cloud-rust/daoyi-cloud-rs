use axum::{Router, routing};
use daoyi_cloud_api::api::admin_api::system::auth::{get_permission_info, login};
use daoyi_cloud_common::models::app_server::AppState;

pub(crate) fn routers() -> Router<AppState> {
    Router::new()
        .route("/login", routing::post(login))
        .route("/get-permission-info", routing::post(get_permission_info))
}
