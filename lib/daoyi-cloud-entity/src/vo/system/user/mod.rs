use daoyi_cloud_common::models::page_param::PageParam;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserPageReqVO {
    /// @Schema(description = "手机号码/用户账号/用户昵称/用户邮箱，模糊匹配", example = "daoyi")
    pub keyword: Option<String>,
    /// @Schema(description = "用户账号，模糊匹配", example = "daoyi")
    pub username: Option<String>,
    /// @Schema(description = "手机号码，模糊匹配", example = "daoyi")
    pub mobile: Option<String>,
    /// @Schema(description = "展示状态，参见 CommonStatusEnum 枚举类", example = "1")
    pub status: Option<i8>,
    /// @Schema(description = "创建时间", example = "[2022-07-01 00:00:00, 2022-07-01 23:59:59]")
    pub create_time: Option<Vec<DateTime>>,
    /// @Schema(description = "部门编号，同时筛选子部门", example = "1024")
    pub dept_id: Option<i64>,
    /// @Schema(description = "角色编号", example = "1024")
    pub role_id: Option<i64>,
    /// 分页参数
    #[validate(nested)]
    #[serde(flatten)]
    pub pagination: PageParam,
}
