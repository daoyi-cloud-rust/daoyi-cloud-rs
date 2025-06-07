use axum::Router;
use daoyi_cloud_common::models::app_server::AppState;

mod infra;

pub fn routers() -> Router<AppState> {
    Router::new().nest("/infra", infra::routers())
}
