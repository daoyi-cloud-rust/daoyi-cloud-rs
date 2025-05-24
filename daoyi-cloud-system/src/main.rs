use daoyi_cloud_middleware::router_middleware;
use spring::App;
use spring_sea_orm::SeaOrmPlugin;
use spring_web::axum::response::IntoResponse;
use spring_web::{get, WebConfigurator, WebPlugin};
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/app.toml");
    App::new()
        .use_config_file(config_path.to_str().unwrap())
        .add_plugin(SeaOrmPlugin)
        .add_plugin(WebPlugin)
        .add_router(router_middleware::root_router())
        .run()
        .await
}

#[get("/")]
async fn hello_word() -> impl IntoResponse {
    "hello word"
}
