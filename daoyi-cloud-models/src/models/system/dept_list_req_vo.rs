use crate::models::common_result::to_common_response;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::Deserialize;
use std::any::type_name;

/// 管理后台 - 部门列表查询 Request VO
#[derive(Deserialize, ToSchema, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeptListReqVo {
    /// 部门名称
    pub name: Option<String>,
    /// 状态,见 CommonStatusEnum 枚举
    pub status: Option<i8>,
}
impl EndpointOutRegister for DeptListReqVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for DeptListReqVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
