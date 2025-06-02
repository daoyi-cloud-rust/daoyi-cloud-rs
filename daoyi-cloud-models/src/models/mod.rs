pub mod biz_error;
pub mod common_result;
pub mod error;
mod mask_utils;
pub mod system;

use crate::models::common_result::to_common_response;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::Serialize;
use std::any::type_name;

#[derive(Serialize, ToSchema, Debug)]
pub struct SafeUser {
    pub id: String,
    pub username: String,
}

impl EndpointOutRegister for SafeUser {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for SafeUser {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
