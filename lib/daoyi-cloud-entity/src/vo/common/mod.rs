use serde::{Deserialize, Serialize};
use validator::Validate;

/// UserSaveReqVO，管理后台 - 用户创建/修改 Request VO
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdReqVo {
    /// ID参数
    pub id: i64,
}
