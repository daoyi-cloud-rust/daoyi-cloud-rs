use spring::{App, auto_config};
use spring_sea_orm::SeaOrmPlugin;
use spring_web::{get, WebConfigurator, WebPlugin};
use std::path::PathBuf;
use spring_web::axum::response::IntoResponse;

#[auto_config(WebConfigurator)]
#[tokio::main]
async fn main() {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/app.toml");
    App::new()
        .use_config_file(config_path.to_str().unwrap())
        .add_plugin(SeaOrmPlugin)
        .add_plugin(WebPlugin)
        .run()
        .await
}

#[get("/")]
async fn hello_word() -> impl IntoResponse {
    "hello word"
}