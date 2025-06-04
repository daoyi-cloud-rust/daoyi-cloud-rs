use crate::models::common_result::to_common_response;
use crate::models::mask_utils::*;
use daoyi_cloud_entities::entities::system::system_post::Model;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostRespVo {
    /// 岗位编码
    pub code: String,
    /// 创建时间
    pub create_time: String,
    /// 岗位序号
    pub id: i64,
    /// 岗位名称
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 显示顺序
    pub sort: i32,
    /// 状态，参见 CommonStatusEnum 枚举类
    pub status: i8,
}

impl From<Model> for PostRespVo {
    fn from(m: Model) -> Self {
        Self {
            code: m.code,
            create_time: m.create_time.format(DATE_TIME_FORMAT).to_string(),
            id: m.id,
            name: m.name,
            remark: m.remark,
            sort: m.sort,
            status: m.status,
        }
    }
}

impl EndpointOutRegister for PostRespVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for PostRespVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
