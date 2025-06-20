use crate::entity::system::system_users::ActiveModel;
use daoyi_cloud_common::enums::common_status_enum::CommonStatusEnum;
use daoyi_cloud_common::enums::sex_enum::SexEnum;
use daoyi_cloud_common::models::page_param::PageParam;
use daoyi_cloud_common::utils::serde_util::{
    deserialize_optional_datetime_vec, deserialize_optional_id_vec,
};
use daoyi_cloud_common::validation::is_mobile_phone;
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// UserPageReqVO，管理后台 - 用户列表 Request VO
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
    pub status: Option<CommonStatusEnum>,
    /// @Schema(description = "创建时间", example = "[2022-07-01 00:00:00, 2022-07-01 23:59:59]")
    #[serde(deserialize_with = "deserialize_optional_datetime_vec", default)]
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

/// UserSaveReqVO，管理后台 - 用户创建/修改 Request VO
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Validate, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct UserSaveReqVo {
    /// 用户头像
    pub avatar: Option<String>,
    /// 部门编号
    pub dept_id: Option<i64>,
    /// 用户邮箱
    #[validate(
        email(message = "邮箱格式不正确"),
        length(max = 50, message = "邮箱长度不能超过50个字符")
    )]
    pub email: Option<String>,
    // /// 用户编号
    // pub id: i64,
    /// 手机号码
    #[validate(custom(function = "is_mobile_phone"))]
    pub mobile: Option<String>,
    /// 用户昵称
    #[validate(length(max = 30, message = "用户昵称长度不能超过30个字符"))]
    pub nickname: String,
    /// 密码
    pub password: Option<String>,
    /// 岗位编号数组
    #[serde(deserialize_with = "deserialize_optional_id_vec", default)]
    pub post_ids: Option<String>,
    /// 备注
    pub remark: Option<String>,
    /// 用户性别，参见 SexEnum 枚举类
    pub sex: Option<SexEnum>,
    /// 用户账号
    #[validate(length(min = 4, max = 30, message = "用户账号长度为 4-30 个字符"))]
    pub username: String,
}
