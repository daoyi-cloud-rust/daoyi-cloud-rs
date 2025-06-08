#![allow(dead_code)]
use crate::utils::serde_util::deserialize_number;
use serde::{Deserialize, Serialize};

/**
 * 每页条数 - 不分页
 * <p>
 * 例如说，导出接口，可以设置 {@link #pageSize} 为 -1 不分页，查询所有数据。
*/
pub const PAGE_SIZE_NONE: i64 = -1;
const PAGE_NO: u64 = 1;
const PAGE_SIZE: i64 = 10;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageParam {
    /// @Schema(description = "页码，从 1 开始", requiredMode = Schema.RequiredMode.REQUIRED, example = "1")
    #[serde(default = "default_page_no", deserialize_with = "deserialize_number")]
    pub page_no: u64,
    /// @Schema(description = "每页条数，最大值为 100，-1 不分页，查询所有数据", requiredMode = Schema.RequiredMode.REQUIRED, example = "10")
    #[serde(default = "default_page_size", deserialize_with = "deserialize_number")]
    pub page_size: i64,
    /// @Schema(description = "排序字段: sort, createTime, updateTime", example = "sort")
    pub order_by: Option<String>,
}

fn default_page_no() -> u64 {
    PAGE_NO
}

fn default_page_size() -> i64 {
    PAGE_SIZE
}
