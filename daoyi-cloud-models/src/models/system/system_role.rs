use crate::models::common_result::to_common_response;
use daoyi_cloud_entities::entities::system::system_role::Model;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
pub struct SystemRoleModel {
    pub id: i64,
    pub name: String,
    pub code: String,
    pub sort: i32,
    pub data_scope: i8,
    pub data_scope_dept_ids: String,
    pub status: i8,
    pub r#type: i8,
    pub remark: Option<String>,
}

impl From<Model> for SystemRoleModel {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            code: m.code,
            sort: m.sort,
            data_scope: m.data_scope,
            data_scope_dept_ids: m.data_scope_dept_ids,
            status: m.status,
            r#type: m.r#type,
            remark: m.remark,
        }
    }
}

impl EndpointOutRegister for SystemRoleModel {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for SystemRoleModel {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
