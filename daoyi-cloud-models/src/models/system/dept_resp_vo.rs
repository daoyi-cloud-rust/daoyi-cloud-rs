use crate::models::common_result::to_common_response;
use crate::models::mask_utils::DATE_TIME_FORMAT;
use crate::models::tree_utils;
use daoyi_cloud_entities::entities::system::system_dept::Model;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeptRespVo {
    /// 创建时间
    pub create_time: String,
    /// 邮箱
    pub email: Option<String>,
    /// 部门编号
    pub id: i64,
    /// 负责人的用户编号
    pub leader_user_id: Option<i64>,
    /// 部门名称
    pub name: String,
    /// 父部门 ID
    pub parent_id: i64,
    /// 联系电话
    pub phone: Option<String>,
    /// 显示顺序不能为空
    pub sort: i32,
    /// 状态,见 CommonStatusEnum 枚举
    pub status: i8,
    /// 子部门
    pub children: Vec<DeptRespVo>,
}

impl tree_utils::TreeNode<DeptRespVo> for DeptRespVo {
    fn id(&self) -> i64 {
        self.id
    }

    fn parent_id(&self) -> i64 {
        self.parent_id
    }

    fn children(&mut self, list: Vec<DeptRespVo>) {
        self.children = list;
    }

    fn sort(&self) -> i32 {
        self.sort
    }
}

impl From<Model> for DeptRespVo {
    fn from(m: Model) -> Self {
        Self {
            create_time: m.create_time.format(DATE_TIME_FORMAT).to_string(),
            email: m.email,
            id: m.id,
            leader_user_id: m.leader_user_id,
            name: m.name,
            parent_id: m.parent_id,
            phone: m.phone,
            sort: m.sort,
            status: m.status,
            children: Vec::new(),
        }
    }
}

impl EndpointOutRegister for DeptRespVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for DeptRespVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
