use axum::extract::State;
use axum::response::IntoResponse;
use axum::{Router, debug_handler, routing};
use daoyi_cloud_config::config;
use daoyi_cloud_entity::entity::demo::prelude::*;
use daoyi_cloud_entity::entity::demo::system_users;
use daoyi_cloud_server::app;
use daoyi_cloud_server::app::AppState;
use sea_orm::Condition;
use sea_orm::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 设置环境变量（只在未设置时设置）
    unsafe {
        std::env::set_var("APP_ROOT", env!("CARGO_MANIFEST_DIR"));
    }
    let router = Router::new()
        .route("/", routing::get(hello_world))
        .route("/users", routing::get(get_users));
    app::run(router).await
}

#[debug_handler]
async fn get_users(State(AppState { db }): State<AppState>) -> impl IntoResponse {
    let users = SystemUsers::find()
        .filter(system_users::Column::Deleted.eq(false))
        .filter(
            Condition::all()
                // .add(system_users::Column::Sex.eq(1))
                .add(system_users::Column::TenantId.eq(1)), // .add(
                                                            //     Condition::any()
                                                            //         .add(system_users::Column::Username.contains("张"))
                                                            //         .add(system_users::Column::Nickname.contains("李")),
                                                            // ),
        )
        .all(db)
        .await
        .unwrap();
    axum::Json(users)
}

#[debug_handler]
async fn hello_world() -> String {
    format!("Hello, {} ~", config::get().app_name())
}
