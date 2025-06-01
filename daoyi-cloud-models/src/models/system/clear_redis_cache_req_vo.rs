use crate::models::common_result::to_common_response;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::{Deserialize, Serialize};
use std::any::type_name;

/// 管理后台 - 清理Redis缓存 Request VO
#[derive(Serialize, Deserialize, ToSchema, Default, Debug, Clone)]
pub struct ClearRedisCacheReqVO {
    /// 键，示例: ["user:read","user:write"]
    pub keys: Option<Vec<String>>,
}
impl EndpointOutRegister for ClearRedisCacheReqVO {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for ClearRedisCacheReqVO {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
