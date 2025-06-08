use daoyi_cloud_entity::entity::system::prelude::SystemUsers;
use daoyi_cloud_entity::entity::system::system_users;
use sea_orm::prelude::*;
use sea_orm::*;

pub struct AdminUserService;
impl AdminUserService {
    pub async fn get_user_list_by_status(
        db: &DatabaseConnection,
        status: i8,
    ) -> anyhow::Result<Vec<system_users::Model>> {
        let vec = SystemUsers::find()
            .filter(system_users::Column::Status.eq(status))
            .order_by_desc(system_users::Column::Id)
            .all(db)
            .await?;
        Ok(vec)
    }
}
