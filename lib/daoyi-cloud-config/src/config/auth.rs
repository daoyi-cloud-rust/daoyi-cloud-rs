use daoyi_cloud_common::auth::JwtConfig;
use serde::Deserialize;

/// Auth 配置信息
#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(default = "default_header")]
    pub header: String,
    #[serde(default = "default_prefix")]
    pub prefix: String,
    #[serde(default = "default_jwt_config")]
    pub jwt: JwtConfig,
}

fn default_header() -> String {
    "Authorization".to_string()
}

fn default_prefix() -> String {
    "Bearer ".to_string()
}

fn default_jwt_config() -> JwtConfig {
    JwtConfig::default()
}
