#![allow(dead_code)]
pub mod database;
pub mod server;

use anyhow::Context;
use daoyi_cloud_logger::logger;
pub use database::DatabaseConfig;
use serde::Deserialize;
pub use server::ServerConfig;
use std::path::PathBuf;
use std::sync::LazyLock;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to initialize config."));
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_app_name")]
    app_name: String,
    server: ServerConfig,
    database: DatabaseConfig,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        let app_root = find_app_root()?;
        let config_path = app_root.join("resources").join("application.yaml");
        logger::debug!("config_path: {:?}", config_path);
        config::Config::builder()
            .add_source(
                config::File::from(config_path)
                    .format(config::FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| "Failed to load config.")?
            .try_deserialize()
            .with_context(|| "Failed to deserialize config.")
    }
    pub fn app_name(&self) -> &str {
        &self.app_name
    }
    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}

fn find_app_root() -> anyhow::Result<PathBuf> {
    // 1. 尝试环境变量
    if let Ok(root) = std::env::var("APP_ROOT") {
        return Ok(PathBuf::from(root));
    }

    // 2. 尝试可执行文件追溯
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(p) = exe_path
            .parent()
            .and_then(|p| p.parent()) // target
            .and_then(|p| p.parent())
        // 项目根目录
        {
            return Ok(p.to_path_buf());
        }
    }

    // 3. 开发时使用 CARGO_MANIFEST_DIR
    #[cfg(debug_assertions)]
    return Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    // 4. 所有路径都失败时返回错误（仅在非debug模式）
    #[cfg(not(debug_assertions))]
    Err(anyhow::anyhow!("Failed to locate project root"))
}

fn default_app_name() -> String {
    "daoyi-cloud".to_string()
}
