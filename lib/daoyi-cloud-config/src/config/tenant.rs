use crate::config::default_boolean;
use serde::Deserialize;

/// Tenant 配置信息
#[derive(Debug, Deserialize, Default)]
pub struct TenantConfig {
    #[serde(default = "default_header")]
    pub header: String,
    #[serde(default = "default_boolean")]
    pub enable: bool,
    #[serde(default = "default_ignore_urls")]
    pub ignore_urls: Vec<String>,
    #[serde(default = "default_valid_url")]
    pub valid_url: String,
}

fn default_header() -> String {
    "tenant-id".to_string()
}

fn default_prefix() -> String {
    "Bearer ".to_string()
}

fn default_ignore_urls() -> Vec<String> {
    vec!["/health".to_string()]
}
fn default_valid_url() -> String {
    "http://127.0.0.1:11021/rpc-api/system/tenant/valid".to_string()
}
