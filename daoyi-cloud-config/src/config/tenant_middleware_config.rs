use crate::config::default_true_str;
use daoyi_cloud_common::utils::toml::Configurable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TenantMiddlewareConfig {
    #[serde(default = "default_true_str")]
    pub enabled: String,
    #[serde(default = "default_tenant_header_name")]
    pub header_name: String,
    #[serde(default = "default_ignore_urls")]
    pub ignore_urls: Vec<String>,
    #[serde(default = "default_ignore_tables")]
    pub ignore_tables: Vec<String>,
}

impl TenantMiddlewareConfig {
    pub fn enabled(&self) -> bool {
        self.enabled == "true"
    }
}

impl Configurable for TenantMiddlewareConfig {
    fn config_prefix() -> &'static str {
        "tenant"
    }
}

fn default_tenant_header_name() -> String {
    "tenant-id".into()
}

fn default_ignore_urls() -> Vec<String> {
    vec![String::from("/health")]
}

fn default_ignore_tables() -> Vec<String> {
    vec![]
}
