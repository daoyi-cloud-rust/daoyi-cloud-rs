use axum::{Router, debug_handler, routing};
use daoyi_cloud_logger::logger;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    logger::init(Some("debug"));
    logger::debug!("Hello, world!");
    let router = Router::new().route("/", routing::get(hello_world));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    logger::debug!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

#[debug_handler]
async fn hello_world() -> &'static str {
    "Hello, world!"
}
