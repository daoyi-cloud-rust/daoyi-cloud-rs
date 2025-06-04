use crate::models::common_result::to_common_response;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::Deserialize;
use std::any::type_name;
use validator::Validate;

/// 管理后台 - 岗位创建/修改 Request VO
#[derive(Deserialize, ToSchema, Default, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PostSaveReqVo {
    /// 岗位编码
    pub code: String,
    /// 岗位编号
    pub id: Option<i64>,
    /// 岗位名称
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 显示顺序
    pub sort: i32,
    /// 状态
    #[validate(range(min = 0, max = 1, message = "状态值必须在0-1之间"))]
    pub status: i8,
}
impl EndpointOutRegister for PostSaveReqVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for PostSaveReqVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
