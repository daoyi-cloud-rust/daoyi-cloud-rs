use crate::config::jwt::JwtConfig;
use axum::http::header;
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
    #[serde(default = "default_ignore_urls")]
    pub ignore_urls: Vec<String>,
}

fn default_header() -> String {
    header::AUTHORIZATION.to_string()
}

fn default_prefix() -> String {
    "Bearer ".to_string()
}

fn default_jwt_config() -> JwtConfig {
    JwtConfig::default()
}

fn default_ignore_urls() -> Vec<String> {
    vec![String::from("/health")]
}
