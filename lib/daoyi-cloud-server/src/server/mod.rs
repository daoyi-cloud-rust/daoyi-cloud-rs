use crate::latency::LatencyOnResponse;
use crate::middleware::get_auth_layer;
use axum::extract::{DefaultBodyLimit, Request};
use axum::{Router, routing};
use bytesize::ByteSize;
use daoyi_cloud_api::api;
use daoyi_cloud_common::error::{ApiError, ApiResult};
use daoyi_cloud_common::models::app_server::AppState;
use daoyi_cloud_config::config::ServerConfig;
use daoyi_cloud_logger::logger;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors;
use tower_http::cors::CorsLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let router = self.build_router(state, router);

        let port = self.config.port();
        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

        logger::info!("Listening on {}://{}", "http", listener.local_addr()?);

        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        let timeout = TimeoutLayer::new(Duration::from_secs(120));
        let body_limit = DefaultBodyLimit::max(ByteSize::mib(10).as_u64() as usize);
        let cors = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_headers(cors::Any)
            .allow_methods(cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600 * 12)); // 12 hours
        let normalize_path = NormalizePathLayer::trim_trailing_slash();

        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();
                tracing::info_span!("Api Request", id = %id, method = %method, path = %path)
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);
        Router::new()
            .route("/", routing::get(api::hello_world))
            .merge(router)
            .route_layer(get_auth_layer())
            .fallback(async || -> ApiResult<()> {
                logger::warn!("Not Found.");
                Err(ApiError::NotFound)
            })
            .method_not_allowed_fallback(async || -> ApiResult<()> {
                logger::warn!("Method Not Allowed.");
                Err(ApiError::MethodNotAllowed)
            })
            .layer(timeout)
            .layer(body_limit)
            .layer(tracing)
            .layer(cors)
            .layer(normalize_path)
            .with_state(state)
    }
}
