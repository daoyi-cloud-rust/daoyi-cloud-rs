use axum::{Router, debug_handler, routing};
use daoyi_cloud_config::config;
use daoyi_cloud_server::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 设置环境变量（只在未设置时设置）
    unsafe {
        std::env::set_var("APP_ROOT", env!("CARGO_MANIFEST_DIR"));
    }
    let router = Router::new().route("/", routing::get(hello_world));

    app::run(router).await
}

#[debug_handler]
async fn hello_world() -> String {
    format!("Hello, {} ~", config::get().app_name())
}
