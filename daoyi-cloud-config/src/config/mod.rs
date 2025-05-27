use std::env;
use std::sync::OnceLock;

use serde::Deserialize;
use tracing::log;

mod log_config;
pub use log_config::LogConfig;
pub mod auth_middleware_config;
mod db_config;
pub mod redis_config;
pub mod tenant_middleware_config;

use crate::config::auth_middleware_config::AuthMiddlewareConfig;
use crate::config::redis_config::RedisConfig;
use crate::config::tenant_middleware_config::TenantMiddlewareConfig;
use crate::{db, redis_util};
use daoyi_cloud_utils::utils::env as EnvUtils;
use daoyi_cloud_utils::utils::toml::{ConfigRegistry, Configurable, TomlConfigRegistry};
pub use db_config::DbConfig;

static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

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
    let web_config = registry
        .get_config::<WebConfig>()
        .expect("web config is required.");
    let db_config = registry
        .get_config::<DbConfig>()
        .expect("db config is required.");
    db::init(&db_config).await;
    let jwt_config = registry
        .get_config::<JwtConfig>()
        .expect("jwt config is required.");
    let tls_config = registry.get_config::<TlsConfig>();
    let redis_config = registry
        .get_config::<RedisConfig>()
        .expect("redis config is required.");
    redis_util::init(&redis_config).await;
    let tenant_middleware_config = registry
        .get_config::<TenantMiddlewareConfig>()
        .expect("tenant middleware config is required.");
    let auth_middleware_config = registry
        .get_config::<AuthMiddlewareConfig>()
        .expect("auth middleware config is required.");

    let config = ServerConfig {
        web: web_config,
        db: db_config,
        jwt: jwt_config,
        tls: tls_config.ok(),
        redis: redis_config,
        tenant: tenant_middleware_config,
        auth: auth_middleware_config,
    };
    log::debug!("config {:#?}", config);
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
    pub tenant: TenantMiddlewareConfig,
    pub auth: AuthMiddlewareConfig,
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

pub fn default_true_str() -> String {
    true.to_string()
}

fn default_listen_addr() -> String {
    "127.0.0.1:8008".into()
}
