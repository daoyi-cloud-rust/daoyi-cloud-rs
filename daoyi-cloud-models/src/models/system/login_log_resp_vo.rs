use crate::models::common_result::to_common_response;
use crate::models::mask_utils::*;
use daoyi_cloud_entities::entities::system::system_login_log::Model;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginLogRespVo {
    /// 登录时间
    pub create_time: String,
    /// 日志编号
    pub id: i64,
    /// 日志类型，参见 LoginLogTypeEnum 枚举类
    pub log_type: i64,
    /// 登录结果，参见 LoginResultEnum 枚举类
    pub result: i8,
    /// 链路追踪编号
    pub trace_id: String,
    /// 浏览器 UserAgent
    pub user_agent: String,
    /// 用户编号
    pub user_id: i64,
    /// 用户 IP
    pub user_ip: String,
    /// 用户账号
    pub username: String,
    /// 用户类型，参见 UserTypeEnum 枚举
    pub user_type: i8,
}

impl From<Model> for LoginLogRespVo {
    fn from(m: Model) -> Self {
        Self {
            create_time: m.create_time.format(DATE_TIME_FORMAT).to_string(),
            id: m.id,
            log_type: m.log_type,
            result: m.result,
            trace_id: m.trace_id,
            user_agent: m.user_agent,
            user_id: m.user_id,
            user_ip: m.user_ip,
            username: m.username,
            user_type: m.user_type,
        }
    }
}

impl EndpointOutRegister for LoginLogRespVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for LoginLogRespVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
