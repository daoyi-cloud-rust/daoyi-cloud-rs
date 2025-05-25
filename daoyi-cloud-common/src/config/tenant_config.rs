use serde::Deserialize;
use spring::config::Configurable;

#[derive(Debug, Configurable, Deserialize)]
#[config_prefix = "tenant"]
pub struct TenantConfig {
    pub header_key: Option<String>,
    pub ignore_urls: Vec<String>,
}
