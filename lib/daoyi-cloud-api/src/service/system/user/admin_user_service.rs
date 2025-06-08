use daoyi_cloud_entity::entity::system::prelude::SystemUsers;
use daoyi_cloud_entity::entity::system::system_users;
use sea_orm::DatabaseConnection;
use sea_orm::prelude::*;

pub struct AdminUserService;
impl AdminUserService {
    pub async fn get_user_list_by_status(
        db: &DatabaseConnection,
        status: i8,
    ) -> anyhow::Result<Vec<system_users::Model>> {
        let vec = SystemUsers::find()
            .filter(system_users::Column::Status.eq(status))
            .all(db)
            .await?;
        Ok(vec)
    }
}
