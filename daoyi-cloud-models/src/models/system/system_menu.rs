use crate::models::common_result::to_common_response;
use daoyi_cloud_entities::entities::system::system_menu::Model;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
pub struct SystemMenuModel {
    pub id: i64,
    pub name: String,
    pub permission: String,
    pub r#type: i8,
    pub sort: i32,
    pub parent_id: i64,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub component: Option<String>,
    pub component_name: Option<String>,
    pub status: i8,
    pub visible: bool,
    pub keep_alive: bool,
    pub always_show: bool,
}

impl From<Model> for SystemMenuModel {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            permission: m.permission,
            r#type: m.r#type,
            sort: m.sort,
            parent_id: m.parent_id,
            path: m.path,
            icon: m.icon,
            component: m.component,
            component_name: m.component_name,
            status: m.status,
            visible: m.visible,
            keep_alive: m.keep_alive,
            always_show: m.always_show,
        }
    }
}

impl EndpointOutRegister for SystemMenuModel {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for SystemMenuModel {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
