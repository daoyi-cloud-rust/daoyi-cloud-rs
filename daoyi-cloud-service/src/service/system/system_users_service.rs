use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::prelude::SystemUsers;
use daoyi_cloud_entities::entities::system::system_users;
use daoyi_cloud_models::models::biz_error;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::system::system_users::SystemUsersModel;
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
        None => Err(biz_error::USER_NOT_EXISTS.to_app_error()),
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
        None => Err(biz_error::USER_PASSWORD_FAILED.to_app_error()),
    }
}
pub async fn reset_login_time(id: &i64) -> AppResult<()> {
    let Some(v) = SystemUsers::find_by_id(id.to_owned())
        .one(db::pool())
        .await?
    else {
        return biz_error::USER_NOT_EXISTS.to_app_result();
    };
    let mut v: system_users::ActiveModel = v.into();
    v.login_date = Set(Some(chrono::Local::now().naive_local()));
    let _v = v.update(db::pool()).await?;
    Ok(())
}
