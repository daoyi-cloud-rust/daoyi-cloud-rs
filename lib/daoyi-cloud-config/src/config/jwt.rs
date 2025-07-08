use crate::config;
use daoyi_cloud_common::utils::serde_util::deserialize_duration;
use daoyi_cloud_common::utils::serde_util::serialize_datetime;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp,
};
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::sync::LazyLock;
use std::time::Duration;

static DEFAULT_JWT: LazyLock<JWT> = LazyLock::new(|| JWT::new(config::get().auth.jwt.clone()));

/// 登录用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    /// 用户编号
    pub id: i64,
    /// 用户类型
    pub user_type: i8,
    /// 额外的用户信息
    pub info: String,
    /// 租户编号
    pub tenant_id: i64,
    /// 授权范围
    pub scopes: Vec<String>,
    /// 过期时间
    #[serde(serialize_with = "serialize_datetime")]
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
    /// 拓展信息
    info: String,
}

/// JWT 配置信息
#[derive(Debug, Deserialize, Clone)]
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

pub struct JWT {
    /// 密钥
    encode_secret: EncodingKey,
    decode_secret: DecodingKey,
    header: Header,
    validation: Validation,
    expiration: Duration,
    audience: String,
    issuer: String,
}

impl JWT {
    pub fn expiration(&self) -> Duration {
        self.expiration.to_owned()
    }

    pub fn new(config: JwtConfig) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_audience(&[&config.audience]);
        validation.set_issuer(&[&config.issuer]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);
        let secret = config.secret.as_bytes();
        Self {
            encode_secret: EncodingKey::from_secret(secret),
            decode_secret: DecodingKey::from_secret(secret),
            header: Header::new(Algorithm::HS256),
            validation,
            expiration: config.expiration,
            audience: config.audience,
            issuer: config.issuer,
        }
    }

    pub fn encode(&self, principal: Principal) -> anyhow::Result<String> {
        let current_timestamp = get_current_timestamp();
        let claims = Claims {
            jti: xid::new().to_string(),
            sub: format!(
                "{}:{}:{}",
                principal.tenant_id, principal.user_type, principal.id
            ),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_timestamp,
            exp: current_timestamp.saturating_add(self.expiration.as_secs()),
            info: serde_json::to_string(&principal)?,
        };
        Ok(jsonwebtoken::encode(
            &self.header,
            &claims,
            &self.encode_secret,
        )?)
    }

    pub fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        let claims = jsonwebtoken::decode::<Claims>(token, &self.decode_secret, &self.validation)?;
        let claims = claims.claims;
        let principal: Principal = serde_json::from_str(&claims.info)?;
        Ok(principal)
    }
}

impl Default for JWT {
    fn default() -> Self {
        Self::new(JwtConfig::default())
    }
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

pub fn get_jwt() -> &'static JWT {
    &DEFAULT_JWT
}
