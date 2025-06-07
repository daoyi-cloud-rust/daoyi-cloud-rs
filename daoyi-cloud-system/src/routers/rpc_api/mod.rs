mod system;

use axum::Router;
use daoyi_cloud_common::models::app_server::AppState;

pub fn routers() -> Router<AppState> {
    Router::new().nest("/system", system::routers())
}
