use crate::server;
use axum::Router;
use daoyi_cloud_common::id;
use daoyi_cloud_common::models::app_server::AppState;
use daoyi_cloud_config::config;
use daoyi_cloud_config::config::database;
use daoyi_cloud_logger::logger;

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init(Some("debug"));
    id::init()?;
    logger::info!("Starting app server...");
    logger::debug!("router: {:#?}", router);

    database::init().await?;
    let db = database::pool0();
    let state = AppState::new(db);
    let server = server::Server::new(config::get().server());

    server.start(state, router).await
}
