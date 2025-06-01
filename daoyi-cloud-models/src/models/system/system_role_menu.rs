use crate::models::common_result::to_common_response;
use daoyi_cloud_entities::entities::system::system_role_menu::Model;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
pub struct SystemRoleMenuModel {
    pub id: i64,
    pub role_id: i64,
    pub menu_id: i64,
}

impl From<Model> for SystemRoleMenuModel {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            role_id: m.role_id,
            menu_id: m.menu_id,
        }
    }
}

impl EndpointOutRegister for SystemRoleMenuModel {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for SystemRoleMenuModel {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
