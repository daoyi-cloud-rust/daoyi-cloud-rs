use daoyi_cloud_utils::utils::toml::Configurable;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RpcServiceConfig {
    #[serde(default = "default_check_access_token_url")]
    pub check_access_token: String,
    #[serde(default = "default_has_any_permission_url")]
    pub has_any_permission: String,
}

impl Configurable for RpcServiceConfig {
    fn config_prefix() -> &'static str {
        "rpc"
    }
}

fn default_check_access_token_url() -> String {
    "http://127.0.0.1:11021/rpc-api/system/oauth2/token/check".into()
}

fn default_has_any_permission_url() -> String {
    "http://127.0.0.1:11021/rpc-api/system/permission/has-any-permissions".into()
}
