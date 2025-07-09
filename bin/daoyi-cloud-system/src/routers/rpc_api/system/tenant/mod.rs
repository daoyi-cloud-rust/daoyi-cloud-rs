use axum::{Router, routing};
use daoyi_cloud_api::api::rpc_api::tenant::valid_tenant;
use daoyi_cloud_common::models::app_server::AppState;

pub(crate) fn routers() -> Router<AppState> {
    Router::new().route("/valid", routing::get(valid_tenant))
}
