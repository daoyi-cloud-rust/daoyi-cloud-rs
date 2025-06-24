use crate::config::default_boolean;
use serde::Deserialize;

/// Captcha 配置信息
#[derive(Debug, Deserialize, Default)]
pub struct CaptchaConfig {
    #[serde(default = "default_boolean")]
    pub enable: bool,
}
