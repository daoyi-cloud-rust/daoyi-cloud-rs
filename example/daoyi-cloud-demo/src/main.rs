mod routers;
mod web;

use daoyi_cloud_server::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(routers::routers()).await
}
