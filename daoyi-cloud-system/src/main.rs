use daoyi_cloud_middleware::router_middleware;
use spring::App;
use spring_sea_orm::SeaOrmPlugin;
use spring_web::axum::response::IntoResponse;
use spring_web::{get, WebConfigurator, WebPlugin};
use std::path::PathBuf;
use anyhow::Error;
use daoyi_cloud_common::error::CusErr;
use daoyi_cloud_common::res::Res;

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
    let res = test_service().await;

    match res {
        Ok(data) => Res::success(data),
        Err(err) => Res::error(err),
    }
}


pub async fn test_service() -> anyhow::Result<String> {
    Err(Error::from(CusErr::AppRuleError(
        "对不起，触犯天条了！".into(),
    )))
}
