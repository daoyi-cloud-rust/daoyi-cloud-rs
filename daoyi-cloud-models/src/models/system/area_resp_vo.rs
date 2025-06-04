use crate::models::common_result::to_common_response;
use crate::models::tree_utils;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct AreaCsvVo {
    /// 区域编号
    pub id: i64,
    /// 区域名称
    pub name: String,
    /// 父区域 ID
    pub parent_id: i64,
    /// 类型
    pub r#type: i8,
}
/// 区域返回值
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct AreaRespVo {
    /// 区域编号
    pub id: i64,
    /// 区域名称
    pub name: String,
    /// 父区域 ID
    pub parent_id: i64,
    /// 类型
    pub r#type: i8,
    /// 子区域
    pub children: Vec<AreaRespVo>,
}

impl From<AreaCsvVo> for AreaRespVo {
    fn from(area: AreaCsvVo) -> Self {
        Self {
            id: area.id,
            name: area.name,
            parent_id: area.parent_id,
            r#type: area.r#type,
            children: vec![],
        }
    }
}

impl tree_utils::TreeNode<AreaRespVo> for AreaRespVo {
    fn id(&self) -> i64 {
        self.id
    }

    fn parent_id(&self) -> i64 {
        self.parent_id
    }

    fn children(&mut self, list: Vec<AreaRespVo>) {
        self.children = list;
    }

    fn sort(&self) -> i32 {
        self.id as i32
    }
}

impl EndpointOutRegister for AreaRespVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for AreaRespVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
