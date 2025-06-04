use crate::models::common_result::to_common_response;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginLogPageReqVo {
    /// 页码，从 1 开始,示例值(1)
    pub page_no: Option<i32>,
    /// 每页条数，最大值为 100，-1 不分页，查询所有数据,示例值(10)
    pub page_size: Option<i32>,
    /// 日志编号
    pub id: Option<i64>,
    /// 日志类型，参见 LoginLogTypeEnum 枚举类
    pub log_type: Option<i64>,
    /// 登录结果，参见 LoginResultEnum 枚举类
    pub result: Option<i8>,
    /// 浏览器 UserAgent
    pub user_agent: Option<String>,
    /// 用户编号
    pub user_id: Option<i64>,
    /// 用户 IP
    pub user_ip: Option<String>,
    /// 用户账号
    pub username: Option<String>,
    /// 用户类型，参见 UserTypeEnum 枚举
    pub user_type: Option<i8>,
    /// 开始时间,示例值([2022-07-01 00:00:00,2022-07-01 23:59:59])
    pub create_time: Option<Vec<String>>,
    /// 链路追踪编号
    pub trace_id: Option<String>,
    /// 排序字段: sort, createTime, updateTime,示例值(sort)
    pub order_by: Option<String>,
}

impl EndpointOutRegister for LoginLogPageReqVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for LoginLogPageReqVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
