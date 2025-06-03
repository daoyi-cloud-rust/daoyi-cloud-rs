use crate::models::common_result::to_common_response;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

/// 分页结果
#[derive(Debug, Serialize, ToSchema, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    /// 数据
    list: Vec<T>,
    /// 总量
    total: u64,
    /// 页码，从 1 开始
    page_no: u32,
    /// 每页条数，最大值为 100，-1 不分页，查询所有数据
    page_size: u32,
}

impl<T> PageResult<T> {}

impl<T: ToSchema + 'static> EndpointOutRegister for PageResult<T> {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl<T: ToSchema + 'static> ToResponse for PageResult<T> {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
