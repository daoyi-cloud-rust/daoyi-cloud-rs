use crate::models::common_result::to_common_response;
use crate::models::mask_utils::*;
use daoyi_cloud_entities::entities::system::system_role::Model;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct RoleRespVo {
    /// 角色标志
    pub code: String,
    /// 创建时间
    pub create_time: String,
    /// 数据范围，参见 DataScopeEnum 枚举类
    pub data_scope: i8,
    /// 数据范围(指定部门数组)
    pub data_scope_dept_ids: Option<Vec<i64>>,
    /// 角色编号
    pub id: i64,
    /// 角色名称
    pub name: String,
    /// 备注
    pub remark: Option<String>,
    /// 显示顺序
    pub sort: i32,
    /// 状态，参见 CommonStatusEnum 枚举类
    pub status: i8,
    /// 角色类型，参见 RoleTypeEnum 枚举类
    pub r#type: i8,
}

impl From<Model> for RoleRespVo {
    fn from(m: Model) -> Self {
        Self {
            code: m.code,
            create_time: m.create_time.format(DATE_TIME_FORMAT).to_string(),
            data_scope: m.data_scope,
            data_scope_dept_ids: serde_json::from_str(&m.data_scope_dept_ids).ok(),
            id: m.id,
            name: m.name,
            remark: m.remark,
            sort: m.sort,
            status: m.status,
            r#type: m.r#type,
        }
    }
}

impl EndpointOutRegister for RoleRespVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for RoleRespVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
