use cookie::Cookie;
use salvo::oapi;
use salvo::oapi::extract::*;
use salvo::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::any::type_name;

use daoyi_cloud_common::utils;
use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::demos::users::Model;
use daoyi_cloud_entities::entities::demos::{prelude::Users, users};
use daoyi_cloud_hoops::hoops::jwt;
use daoyi_cloud_models::models::common_result::{JsonResult, json_ok, to_common_response};

#[derive(Deserialize, ToSchema, Default, Debug)]
pub struct LoginInData {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, ToSchema, Default, Debug)]
pub struct LoginOutData {
    pub id: String,
    pub username: String,
    pub token: String,
    pub exp: i64,
}

impl EndpointOutRegister for LoginOutData {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for LoginOutData {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}

#[endpoint(tags("示例"))]
pub async fn post_login(
    idata: JsonBody<LoginInData>,
    res: &mut Response,
) -> JsonResult<LoginOutData> {
    let idata = idata.into_inner();
    let conn = db::pool();
    let Some(Model {
        id,
        username,
        password,
    }) = Users::find()
        .filter(users::Column::Username.eq(idata.username))
        .one(conn)
        .await?
    else {
        return Err(StatusError::unauthorized()
            .brief("User does not exist.")
            .into());
    };

    if utils::verify_password(&idata.password, &password).is_err() {
        return Err(StatusError::unauthorized()
            .brief("Addount not exist or password is incorrect.")
            .into());
    }

    let (token, exp) = jwt::get_token(&id)?;
    let odata = LoginOutData {
        id,
        username,
        token,
        exp,
    };
    let cookie = Cookie::build(("jwt_token", odata.token.clone()))
        .path("/")
        .http_only(true)
        .build();
    res.add_cookie(cookie);
    json_ok(odata)
}
