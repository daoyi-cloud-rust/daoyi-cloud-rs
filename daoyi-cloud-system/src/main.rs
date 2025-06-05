use axum::{Router, routing};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let router = Router::new().route("/", routing::get(async || "Hello, world!"));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
