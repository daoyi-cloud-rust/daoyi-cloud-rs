use std::env;
use std::sync::OnceLock;

use serde::Deserialize;
use tracing::log;

mod log_config;
pub use log_config::LogConfig;
mod db_config;
pub mod redis_config;

use crate::config::redis_config::RedisConfig;
use crate::{db, redis_util};
use daoyi_cloud_utils::utils::env as EnvUtils;
use daoyi_cloud_utils::utils::toml::{ConfigRegistry, Configurable, TomlConfigRegistry};
pub use db_config::DbConfig;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

pub async fn init(env_path: Option<String>) {
    let env_path = env_path.unwrap_or_else(|| env!("CARGO_MANIFEST_DIR").to_string());
    let _env0 = EnvUtils::Env::init(Some(format!("{}/.env", env_path).as_str()));
    let config_path_buf =
        env::var("APP_CONFIG").unwrap_or_else(|_| format!("{}/resources/app.toml", env_path));
    let registry =
        TomlConfigRegistry::new(config_path_buf.as_str()).expect("config registry error.");
    let log_config = registry
        .get_config::<LogConfig>()
        .expect("log config is required.");
    let _guard = log_config.guard();
    log::info!("log level: {}", &log_config.filter_level);
    let web_config = registry
        .get_config::<WebConfig>()
        .expect("web config is required.");
    log::debug!("web {:#?}", web_config);
    let db_config = registry
        .get_config::<DbConfig>()
        .expect("db config is required.");
    db::init(&db_config).await;
    log::debug!("db {:#?}", db_config);
    let jwt_config = registry
        .get_config::<JwtConfig>()
        .expect("jwt config is required.");
    log::debug!("jwt {:#?}", jwt_config);
    let tls_config = registry.get_config::<TlsConfig>();
    log::debug!("tls {:#?}", tls_config);
    let redis_config = registry
        .get_config::<RedisConfig>()
        .expect("redis config is required.");
    log::debug!("redis {:#?}", redis_config);
    redis_util::init(&redis_config).await;

    let config = ServerConfig {
        web: web_config,
        db: db_config,
        jwt: jwt_config,
        tls: tls_config.ok(),
        redis: redis_config,
    };
    CONFIG.set(config).expect("config should be set");
}
pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    pub web: WebConfig,
    pub db: DbConfig,
    pub jwt: JwtConfig,
    pub tls: Option<TlsConfig>,
    pub redis: RedisConfig,
}

#[derive(Deserialize, Clone, Debug)]
pub struct WebConfig {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
}

impl Configurable for WebConfig {
    fn config_prefix() -> &'static str {
        "web"
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry: String,
}

impl Configurable for JwtConfig {
    fn config_prefix() -> &'static str {
        "jwt"
    }
}
#[derive(Deserialize, Clone, Debug)]
pub struct TlsConfig {
    pub cert: String,
    pub key: String,
}

impl Configurable for TlsConfig {
    fn config_prefix() -> &'static str {
        "tls"
    }
}

#[allow(dead_code)]
pub fn default_false() -> bool {
    false
}
#[allow(dead_code)]
pub fn default_true() -> bool {
    true
}

fn default_listen_addr() -> String {
    "127.0.0.1:8008".into()
}
