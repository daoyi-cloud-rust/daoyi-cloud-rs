use idgenerator::{IdGeneratorOptions, IdInstance};
use sea_orm::prelude::Date;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn init() -> anyhow::Result<()> {
    let _options = IdGeneratorOptions::new()
        .base_time(
            Date::from_ymd_opt(2025, 6, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_millis(),
        )
        .worker_id(1)
        .worker_id_bit_len(4);
    Ok(())
}

pub fn next_str_id() -> String {
    next_id().to_string()
}
pub fn next_id() -> i64 {
    IdInstance::next_id()
}

/// IdParamsReqVO，管理后台 - 带ID请求 Request VO
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct IdParamsReqVO {
    /// id
    pub id: i64,
}
