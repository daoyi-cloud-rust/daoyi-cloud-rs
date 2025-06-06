use axum::response::IntoResponse;
use axum::{Router, debug_handler, routing};
use daoyi_cloud_config::config;
use daoyi_cloud_config::config::database;
use daoyi_cloud_entity::entity::demo::prelude::*;
use daoyi_cloud_entity::entity::demo::system_users;
use daoyi_cloud_logger::logger;
use sea_orm::Condition;
use sea_orm::prelude::*;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 设置环境变量（只在未设置时设置）
    unsafe {
        std::env::set_var("APP_ROOT", env!("CARGO_MANIFEST_DIR"));
    }
    logger::init(Some("debug"));
    database::init().await?;
    let router = Router::new()
        .route("/", routing::get(hello_world))
        .route("/users", routing::get(get_users));
    let port = config::get().server().port();
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    logger::info!("Listening on {}://{}", "http", listener.local_addr()?);
    axum::serve(listener, router).await?;
    Ok(())
}

#[debug_handler]
async fn get_users() -> impl IntoResponse {
    let users = SystemUsers::find()
        .filter(system_users::Column::Deleted.eq(true))
        .filter(
            Condition::all()
                .add(system_users::Column::Sex.eq(1))
                .add(system_users::Column::TenantId.gt(18))
                .add(
                    Condition::any()
                        .add(system_users::Column::Username.contains("张"))
                        .add(system_users::Column::Nickname.contains("李")),
                ),
        )
        .all(database::pool1())
        .await
        .unwrap();
    axum::Json(users)
}

#[debug_handler]
async fn hello_world() -> String {
    format!("Hello, {} ~", config::get().app_name())
}
