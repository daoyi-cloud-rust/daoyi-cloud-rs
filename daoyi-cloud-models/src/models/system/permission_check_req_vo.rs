use crate::models::common_result::to_common_response;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::{Deserialize, Serialize};
use std::any::type_name;

/// 管理后台 - 权限校验 Request VO
#[derive(Serialize, Deserialize, ToSchema, Default, Debug, Clone)]
pub struct PermissionCheckReqVO {
    /// 用户编号,示例: 1
    pub user_id: i64,
    /// 权限，示例: ["user:read","user:write"]
    pub permissions: Vec<String>,
}
impl EndpointOutRegister for PermissionCheckReqVO {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for PermissionCheckReqVO {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
