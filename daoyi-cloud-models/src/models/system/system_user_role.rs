use crate::models::common_result::to_common_response;
use daoyi_cloud_entities::entities::system::system_user_role::Model;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
pub struct SystemUserRoleModel {
    pub id: i64,
    pub user_id: i64,
    pub role_id: i64,
}

impl From<Model> for SystemUserRoleModel {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            user_id: m.user_id,
            role_id: m.role_id,
        }
    }
}

impl EndpointOutRegister for SystemUserRoleModel {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for SystemUserRoleModel {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
