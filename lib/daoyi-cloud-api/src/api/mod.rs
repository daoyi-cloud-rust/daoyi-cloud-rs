pub mod admin_api;
pub mod app_api;
pub mod rpc_api;

use axum::{Router, debug_handler, routing};
use daoyi_cloud_common::models::app_server::AppState;
use daoyi_cloud_config::config;

pub fn routers(app_routers: Router<AppState>) -> Router<AppState> {
    Router::new()
        .route("/", routing::get(hello_world))
        .merge(app_routers)
}

#[debug_handler]
async fn hello_world() -> String {
    format!("Hello, {} ~", config::get().app_name())
}
