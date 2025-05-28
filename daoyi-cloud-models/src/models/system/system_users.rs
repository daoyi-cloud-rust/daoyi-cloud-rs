use crate::models::common_result::to_common_response;
use crate::models::mask_utils::*;
use daoyi_cloud_entities::entities::system::system_users::Model;
use salvo::oapi;
use salvo::prelude::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
pub struct SystemUsersModel {
    #[serde(serialize_with = "mask_username")]
    pub username: String,
    pub nickname: String,
    pub remark: Option<String>,
    pub dept_id: Option<i64>,
    pub post_ids: Option<String>,
    #[serde(serialize_with = "mask_email")]
    pub email: String,
    #[serde(serialize_with = "mask_phone")]
    pub mobile: String,
    pub sex: Option<i8>,
    pub avatar: Option<String>,
    pub status: i8,
    pub login_date: Option<DateTime>,
}

impl From<Model> for SystemUsersModel {
    fn from(m: Model) -> Self {
        Self {
            username: m.username,
            nickname: m.nickname,
            remark: m.remark,
            dept_id: m.dept_id,
            post_ids: m.post_ids,
            email: m.email.unwrap_or_else(|| "".to_string()),
            mobile: m.mobile.unwrap_or_else(|| "".to_string()),
            sex: m.sex,
            avatar: m.avatar,
            status: m.status,
            login_date: m.login_date,
        }
    }
}

impl EndpointOutRegister for SystemUsersModel {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for SystemUsersModel {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
