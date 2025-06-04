use crate::models::common_result::to_common_response;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::Deserialize;
use std::any::type_name;

/// 管理后台 - 角色分页列表查询 Request VO
#[derive(Deserialize, ToSchema, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RolePageReqVo {
    /// 页码，从 1 开始,示例值(1)
    pub page_no: Option<i32>,
    /// 每页条数，最大值为 100，-1 不分页，查询所有数据,示例值(10)
    pub page_size: Option<i32>,
    /// 角色编码，模糊匹配
    pub code: Option<String>,
    /// 角色名称，模糊匹配
    pub name: Option<String>,
    /// 状态,见 CommonStatusEnum 枚举
    pub status: Option<i8>,
    /// 开始时间,示例值([2022-07-01 00:00:00,2022-07-01 23:59:59])
    pub create_time: Option<Vec<String>>,
    /// 排序字段: sort, createTime, updateTime,示例值(sort)
    pub order_by: Option<String>,
}
impl EndpointOutRegister for RolePageReqVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for RolePageReqVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
