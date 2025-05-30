use bcrypt::DEFAULT_COST;
use daoyi_cloud_utils::utils::toml::Configurable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SecurityConfig {
    #[serde(default = "default_password_strength")]
    pub strength: String,
}

impl SecurityConfig {
    pub fn strength(&self) -> u32 {
        self.strength.parse().unwrap_or(DEFAULT_COST)
    }
}

impl Configurable for SecurityConfig {
    fn config_prefix() -> &'static str {
        "security"
    }
}

fn default_password_strength() -> String {
    DEFAULT_COST.to_string()
}
