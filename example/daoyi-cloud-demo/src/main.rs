mod routers;

use daoyi_cloud_server::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 设置环境变量（只在未设置时设置）
    unsafe {
        std::env::set_var("APP_ROOT", env!("CARGO_MANIFEST_DIR"));
    }

    app::run(routers::routers()).await
}
