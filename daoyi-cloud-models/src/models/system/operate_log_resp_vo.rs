use crate::models::common_result::to_common_response;
use crate::models::mask_utils::*;
use daoyi_cloud_entities::entities::system::system_operate_log::Model;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct OperateLogRespVo {
    /// 操作明细
    pub action: String,
    /// 操作模块业务编号
    pub biz_id: i64,
    /// 创建时间
    pub create_time: String,
    /// 拓展字段
    pub extra: String,
    /// 日志编号
    pub id: i64,
    /// 请求方法名
    pub request_method: Option<String>,
    /// 请求地址
    pub request_url: Option<String>,
    /// 操作名
    pub sub_type: String,
    /// 链路追踪编号
    pub trace_id: String,
    /// 操作模块类型
    pub r#type: String,
    /// 浏览器 UserAgent
    pub user_agent: Option<String>,
    /// 用户编号
    pub user_id: i64,
    /// 用户 IP
    pub user_ip: Option<String>,
    /// 用户昵称
    pub user_name: Option<String>,
}

impl From<Model> for OperateLogRespVo {
    fn from(m: Model) -> Self {
        Self {
            action: m.action,
            biz_id: m.biz_id,
            create_time: m.create_time.format(DATE_TIME_FORMAT).to_string(),
            extra: m.extra,
            id: m.id,
            request_method: m.request_method,
            request_url: m.request_url,
            sub_type: m.sub_type,
            trace_id: m.trace_id,
            r#type: m.r#type,
            user_agent: m.user_agent,
            user_id: m.user_id,
            user_ip: m.user_ip,
            user_name: None,
        }
    }
}

impl EndpointOutRegister for OperateLogRespVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for OperateLogRespVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
