use crate::service::system::system_users_service::get_system_users_by_id;
use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::prelude::SystemOauth2AccessToken;
use daoyi_cloud_entities::entities::system::system_oauth2_access_token;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use salvo::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

pub async fn check_access_token(token: &str) -> AppResult<OAuth2AccessTokenCheckRespDTO> {
    match SystemOauth2AccessToken::find()
        .filter(system_oauth2_access_token::Column::AccessToken.eq(token))
        .one(db::pool())
        .await?
    {
        Some(mut v) => {
            if v.expires_time.lt(&chrono::Local::now().naive_local()) {
                v = reset_expires_time(&v.id).await?;
            }
            let system_users_model = get_system_users_by_id(v.user_id).await?;
            let mut resp_dto = OAuth2AccessTokenCheckRespDTO::from(v);
            resp_dto.user_info = Some(system_users_model);
            Ok(resp_dto)
        }
        None => Err(AppError::HttpStatus(
            StatusError::from_code(StatusCode::UNAUTHORIZED)
                .unwrap()
                .brief("访问令牌不存在"),
        )),
    }
}

pub async fn reset_expires_time(id: &i64) -> AppResult<system_oauth2_access_token::Model> {
    let Some(v) = SystemOauth2AccessToken::find_by_id(id.to_owned())
        .one(db::pool())
        .await?
    else {
        return Err(AppError::HttpStatus(
            StatusError::from_code(StatusCode::UNAUTHORIZED)
                .unwrap()
                .brief("访问令牌不存在"),
        ));
    };
    let mut v: system_oauth2_access_token::ActiveModel = v.into();
    v.expires_time = Set(chrono::Local::now().naive_local() + chrono::Duration::days(1000));
    let v = v.update(db::pool()).await?;
    Ok(v)
}
