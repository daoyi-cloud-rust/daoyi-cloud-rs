use daoyi_cloud_middleware::auth_middleware::auth_middleware;
use spring::App;
use spring_sea_orm::SeaOrmPlugin;
use spring_web::axum::response::IntoResponse;
use spring_web::axum::middleware;
use spring_web::middleware::timeout::TimeoutLayer;
use spring_web::{get, Router, WebConfigurator, WebPlugin};
use std::path::PathBuf;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/app.toml");
    App::new()
        .use_config_file(config_path.to_str().unwrap())
        .add_plugin(SeaOrmPlugin)
        .add_plugin(WebPlugin)
        .add_router(router())
        .run()
        .await
}

fn router() -> Router {
    Router::new()
        .merge(spring_web::handler::auto_router())
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        .layer(middleware::from_fn(auth_middleware))
}

#[get("/")]
async fn hello_word() -> impl IntoResponse {
    "hello word"
}
