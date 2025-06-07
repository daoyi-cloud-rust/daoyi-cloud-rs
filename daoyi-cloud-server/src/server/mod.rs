use axum::Router;
use daoyi_cloud_common::models::app_server::AppState;
use daoyi_cloud_config::config::ServerConfig;
use daoyi_cloud_logger::logger;
use std::net::SocketAddr;
use tokio::net::TcpListener;

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
        Router::new().merge(router).with_state(state)
    }
}
