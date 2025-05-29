use crate::models::common_result::to_common_response;
use crate::models::mask_utils::DATE_TIME_FORMAT;
use daoyi_cloud_entities::entities::system::system_oauth2_access_token::Model;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::Serialize;
use std::any::type_name;

/// 管理后台 - 登录 Response VO
#[derive(Serialize, ToSchema, Default, Debug)]
pub struct AuthLoginRespVO {
    /// 用户编号
    pub user_id: i64,
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: String,
    /// 过期时间
    pub expires_time: String,
    /// 终端编号
    pub terminal_id: Option<String>,
}

impl From<Model> for AuthLoginRespVO {
    fn from(m: Model) -> Self {
        Self {
            user_id: m.user_id,
            access_token: m.access_token,
            refresh_token: m.refresh_token,
            expires_time: m.expires_time.format(DATE_TIME_FORMAT).to_string(),
            terminal_id: m.terminal_id,
        }
    }
}
impl EndpointOutRegister for AuthLoginRespVO {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for AuthLoginRespVO {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
