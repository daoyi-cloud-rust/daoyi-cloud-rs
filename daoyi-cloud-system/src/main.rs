use axum::{Router, debug_handler, routing};
use daoyi_cloud_config::config;
use daoyi_cloud_logger::logger;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // 设置环境变量（只在未设置时设置）
    unsafe {
        std::env::set_var("APP_ROOT", env!("CARGO_MANIFEST_DIR"));
    }
    logger::init(Some("debug"));
    let router = Router::new().route("/", routing::get(hello_world));
    let port = config::get().server.port();
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    logger::debug!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn hello_world() -> String {
    format!("Hello, {}!", config::get().app_name)
}
