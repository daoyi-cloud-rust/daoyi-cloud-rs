use axum::{Router, routing};
use daoyi_cloud_api::api::admin_api::system::auth::{encode_password, get_permission_info, login};
use daoyi_cloud_common::models::app_server::AppState;

pub(crate) fn routers() -> Router<AppState> {
    Router::new()
        .route("/encode-password/{passwd}", routing::get(encode_password))
        .route("/login", routing::post(login))
        .route("/get-permission-info", routing::get(get_permission_info))
}
