use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum CusErr {
    #[error("Something wrong with Request parameter: {0}")]
    ReqParamError(String),

    #[error("Something wrong when Delete: {0}")]
    ReqDeleteFail(String),

    #[error("App rule: {0}")]
    AppRuleError(String),
    
    #[error("认证失败：{0}")]
    AuthError(String),

    #[error("{1}")]
    ServiceException(u64, String),
}
