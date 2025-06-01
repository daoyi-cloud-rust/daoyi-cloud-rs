use crate::models::common_result::to_common_response;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::Deserialize;
use std::any::type_name;

/// 管理后台 - 账号密码登录 Request VO
#[derive(Deserialize, ToSchema, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptSaveReqVo {
    /// 邮箱
    pub email: Option<String>,
    /// 部门编号
    pub id: Option<i64>,
    /// 负责人的用户编号
    pub leader_user_id: Option<i64>,
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    pub parent_id: Option<i64>,
    /// 联系电话
    pub phone: Option<String>,
    /// 显示顺序不能为空
    pub sort: i32,
    /// 状态,见 CommonStatusEnum 枚举
    pub status: i8,
}
impl EndpointOutRegister for DeptSaveReqVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for DeptSaveReqVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
