pub mod admin_api;
pub mod app_api;
pub mod rpc_api;

use axum::debug_handler;
use daoyi_cloud_config::config;

#[debug_handler]
pub async fn hello_world() -> String {
    format!("Hello, {} ~", config::get().app_name())
}
