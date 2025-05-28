use daoyi_cloud_utils::utils::toml::Configurable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AuthMiddlewareConfig {
    #[serde(default = "default_auth_header_prefix")]
    pub prefix: String,
    #[serde(default = "default_auth_header_name")]
    pub header_name: String,
    #[serde(default = "default_login_user_key")]
    pub login_user_key: String,
    #[serde(default = "default_ignore_urls")]
    pub ignore_urls: Vec<String>,
}

impl Configurable for AuthMiddlewareConfig {
    fn config_prefix() -> &'static str {
        "auth"
    }
}

fn default_auth_header_prefix() -> String {
    "Bearer ".into()
}

fn default_auth_header_name() -> String {
    "Authorization".into()
}

fn default_login_user_key() -> String {
    "current_user".into()
}

fn default_ignore_urls() -> Vec<String> {
    vec![String::from("/health")]
}
