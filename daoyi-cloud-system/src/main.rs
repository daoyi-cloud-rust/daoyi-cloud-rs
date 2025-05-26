use daoyi_cloud_app::app;
use daoyi_cloud_config::config;

mod routers;

#[tokio::main]
async fn main() {
    config::init(Some(String::from(env!("CARGO_MANIFEST_DIR")))).await;
    app::start(routers::root()).await;
}
