use axum::Router;
use daoyi_cloud_common::models::app_server::AppState;

mod dept;
mod user;

pub fn routers() -> Router<AppState> {
    Router::new()
        .nest("/dept", dept::routers())
        .nest("/user", user::routers())
}
