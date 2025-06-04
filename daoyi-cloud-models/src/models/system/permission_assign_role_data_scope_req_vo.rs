use crate::models::common_result::to_common_response;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::Deserialize;
use std::any::type_name;
use validator::Validate;

/// 管理后台 - 菜单创建/修改 Request VO
#[derive(Deserialize, ToSchema, Default, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionAssignRoleDataScopeReqVo {
    /// 数据范围，参见 DataScopeEnum 枚举类
    pub data_scope: i8,
    /// 部门编号列表，只有范围类型为 DEPT_CUSTOM 时，该字段才需要
    pub data_scope_dept_ids: Option<Vec<i64>>,
    /// 角色编号
    pub role_id: i64,
}

impl EndpointOutRegister for PermissionAssignRoleDataScopeReqVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for PermissionAssignRoleDataScopeReqVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
