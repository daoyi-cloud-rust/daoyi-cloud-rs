use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::prelude::SystemUsers;
use daoyi_cloud_entities::entities::system::system_users;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::system::system_users::SystemUsersModel;
use salvo::http::{StatusCode, StatusError};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

pub async fn get_system_users_by_id(
    id: i64,
    tenant_id: Option<&i64>,
) -> AppResult<SystemUsersModel> {
    let mut select = SystemUsers::find()
        .filter(system_users::Column::Deleted.eq(false))
        .filter(system_users::Column::Id.eq(id));
    if let Some(tenant_id) = tenant_id {
        select = select.filter(system_users::Column::TenantId.eq(tenant_id.to_owned()));
    }
    match select.one(db::pool()).await? {
        Some(v) => Ok(SystemUsersModel::from(v)),
        None => Err(AppError::internal(format!("用户不存在 id = {id}"))),
    }
}

pub async fn get_system_users_by_username(
    username: &str,
    tenant_id: Option<&i64>,
) -> AppResult<system_users::Model> {
    let mut select = SystemUsers::find()
        .filter(system_users::Column::Deleted.eq(false))
        .filter(system_users::Column::Username.eq(username));
    if let Some(tenant_id) = tenant_id {
        select = select.filter(system_users::Column::TenantId.eq(tenant_id.to_owned()));
    }
    match select.one(db::pool()).await? {
        Some(v) => Ok(v),
        None => Err(AppError::internal("用户名或密码错误.".to_string())),
    }
}
pub async fn reset_login_time(id: &i64) -> AppResult<()> {
    let Some(v) = SystemUsers::find_by_id(id.to_owned())
        .one(db::pool())
        .await?
    else {
        return Err(AppError::HttpStatus(
            StatusError::from_code(StatusCode::UNAUTHORIZED)
                .unwrap()
                .brief("用户不存在."),
        ));
    };
    let mut v: system_users::ActiveModel = v.into();
    v.login_date = Set(Some(chrono::Local::now().naive_local()));
    let _v = v.update(db::pool()).await?;
    Ok(())
}
