use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::prelude::SystemUsers;
use daoyi_cloud_entities::entities::system::system_users;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::system::system_users::SystemUsersModel;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

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
