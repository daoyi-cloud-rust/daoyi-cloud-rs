use crate::models::common_result::to_common_response;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::Deserialize;
use std::any::type_name;

/// 管理后台 - 账号密码登录 Request VO
#[derive(Deserialize, ToSchema, Default, Debug)]
pub struct AuthLoginReqVO {
    /// 账号,示例值(daoyi-cloudyuanma)
    pub username: String,
    /// 密码,示例值(buzhidao)
    pub password: String,
}
impl EndpointOutRegister for AuthLoginReqVO {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for AuthLoginReqVO {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
