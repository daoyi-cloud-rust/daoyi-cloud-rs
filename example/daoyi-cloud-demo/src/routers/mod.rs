use crate::web::{index_handler, static_assets_handler};
use axum::{Router, routing};
use daoyi_cloud_common::models::app_server::AppState;
use tower_http::compression::CompressionLayer;

mod admin_api;

pub(crate) fn routers() -> Router<AppState> {
    Router::new().nest("/admin-api", admin_api::routers()).nest(
        "/static",
        Router::new()
            .route("/{*file}", routing::get(static_assets_handler))
            .route_layer(CompressionLayer::new())
            .fallback(routing::get(index_handler)),
    )
}
