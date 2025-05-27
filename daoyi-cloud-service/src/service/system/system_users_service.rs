use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::prelude::SystemUsers;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::system::system_users::SystemUsersModel;
use sea_orm::EntityTrait;

pub async fn get_system_users_by_id(id: i64) -> AppResult<SystemUsersModel> {
    match SystemUsers::find_by_id(id).one(db::pool()).await? {
        Some(v) => Ok(SystemUsersModel::from(v)),
        None => Err(AppError::internal(format!(
            "system_users not found by id {id}"
        ))),
    }
}
