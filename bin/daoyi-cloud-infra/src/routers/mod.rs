use axum::Router;
use daoyi_cloud_common::models::app_server::AppState;

mod admin_api;
mod app_api;
mod rpc_api;

pub fn routers() -> Router<AppState> {
    Router::new()
        .nest("/admin-api", admin_api::routers())
        .nest("/app-api", app_api::routers())
        .nest("/rpc-api", rpc_api::routers())
}
