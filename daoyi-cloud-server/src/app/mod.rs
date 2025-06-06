use crate::server;
use axum::Router;
use daoyi_cloud_config::config;
use daoyi_cloud_config::config::database;
use daoyi_cloud_logger::logger;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: &'static DatabaseConnection,
}

impl AppState {
    pub fn new(db: &'static DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init(Some("debug"));
    logger::info!("Starting app server...");

    database::init().await?;
    let db = database::pool0();
    let state = AppState::new(db);
    let server = server::Server::new(config::get().server());

    server.start(state, router).await
}
