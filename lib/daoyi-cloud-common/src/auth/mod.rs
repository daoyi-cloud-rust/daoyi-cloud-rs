use humantime::parse_duration;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Deserializer, Serialize};
use std::iter::Map;
use std::time::Duration;

/// 登录用户信息
#[derive(Debug, Clone)]
pub struct Principal {
    /// 用户编号
    pub id: i64,
    /// 用户类型
    pub user_type: i8,
    /// 额外的用户信息
    pub info: Map<String, String>,
    /// 租户编号
    pub tenant_id: i64,
    /// 授权范围
    pub scopes: Vec<String>,
    /// 过期时间
    pub expires_time: DateTime,
    /// 终端编号
    pub terminal_id: String,
}

/// JWT Claims 声明
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// JWT token
    jti: String,
    /// 主体
    sub: String,
    /// 受众
    aud: String,
    /// 签发者
    iss: String,
    /// 签发时间
    iat: u64,
    /// 过期时间
    exp: u64,
}

/// JWT 配置信息
#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    /// 密钥
    pub secret: String,
    /// 过期时间
    #[serde(deserialize_with = "deserialize_duration")]
    pub expiration: Duration,
    /// 受众
    pub audience: String,
    /// 签发者
    pub issuer: String,
}

// 自定义反序列化函数
fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    // 1. 先解析为字符串
    let duration_str = String::deserialize(deserializer)?;

    // 2. 使用 humantime 解析字符串为 Duration
    parse_duration(&duration_str).map_err(|e| serde::de::Error::custom(e.to_string()))
}

impl Default for JwtConfig {
    fn default() -> Self {
        JwtConfig {
            secret: "daoyi".to_string(),
            expiration: Duration::from_secs(3600),
            audience: "daoyi".to_string(),
            issuer: "daoyi".to_string(),
        }
    }
}
