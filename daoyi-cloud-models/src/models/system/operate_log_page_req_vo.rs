use crate::models::common_result::to_common_response;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct OperateLogPageReqVo {
    /// 页码，从 1 开始,示例值(1)
    pub page_no: Option<i32>,
    /// 每页条数，最大值为 100，-1 不分页，查询所有数据,示例值(10)
    pub page_size: Option<i32>,
    /// 用户编号
    pub user_id: Option<i64>,
    /// 操作模块业务编号
    pub biz_id: Option<i64>,
    /// 操作模块类型
    pub r#type: Option<String>,
    /// 操作名
    pub sub_type: Option<String>,
    /// 操作明细
    pub action: Option<String>,
    /// 开始时间,示例值([2022-07-01 00:00:00,2022-07-01 23:59:59])
    pub create_time: Option<Vec<String>>,
    /// 链路追踪编号
    pub trace_id: Option<String>,
    /// 排序字段: sort, createTime, updateTime,示例值(sort)
    pub order_by: Option<String>,
}

impl EndpointOutRegister for OperateLogPageReqVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for OperateLogPageReqVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
