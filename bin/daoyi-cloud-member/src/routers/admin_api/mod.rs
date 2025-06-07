use axum::Router;
use daoyi_cloud_common::models::app_server::AppState;

mod member;

pub fn routers() -> Router<AppState> {
    Router::new().nest("/member", member::routers())
}
