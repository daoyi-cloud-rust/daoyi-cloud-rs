use axum::Router;
use daoyi_cloud_common::models::app_server::AppState;

mod admin_api;

pub(crate) fn routers() -> Router<AppState> {
    Router::new().nest("/admin-api", admin_api::routers())
}
