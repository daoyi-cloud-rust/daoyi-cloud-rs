use crate::models::common_result::to_common_response;
use crate::models::mask_utils::DATE_TIME_FORMAT;
use crate::models::system::system_users::SystemUsersModel;
use daoyi_cloud_entities::entities::system::system_oauth2_access_token::Model;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::{Deserialize, Serialize};
use std::any::type_name;
use std::string::String;

/// RPC 服务 - OAuth2 访问令牌的校验 Response DTO
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
pub struct OAuth2AccessTokenCheckRespDTO {
    /// 用户编号
    pub user_id: i64,
    /// 用户类型，参见 UserTypeEnum 枚举
    pub user_type: i8,
    /// 用户信息
    pub user_info: Option<SystemUsersModel>,
    /// 授权范围的数组
    pub scopes: Vec<String>,
    /// 过期时间
    pub expires_time: String,
    /// 终端编号
    pub terminal_id: Option<String>,
    /// 租户编号
    pub tenant_id: i64,
}

impl From<Model> for OAuth2AccessTokenCheckRespDTO {
    fn from(m: Model) -> Self {
        Self {
            user_id: m.user_id,
            user_type: m.user_type,
            user_info: None,
            scopes: serde_json::from_str(&m.scopes.unwrap_or_else(|| String::from("[]"))).unwrap(),
            expires_time: m.expires_time.format(DATE_TIME_FORMAT).to_string(),
            terminal_id: m.terminal_id,
            tenant_id: m.tenant_id,
        }
    }
}

impl EndpointOutRegister for OAuth2AccessTokenCheckRespDTO {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for OAuth2AccessTokenCheckRespDTO {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
