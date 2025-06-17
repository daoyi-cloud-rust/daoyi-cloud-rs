use daoyi_cloud_common::models::page_param::PAGE_SIZE_NONE;
use daoyi_cloud_common::models::page_result::PageResult;
use daoyi_cloud_entity::entity::system::prelude::SystemUsers;
use daoyi_cloud_entity::entity::system::system_users;
use daoyi_cloud_entity::entity::system::system_users::ActiveModel;
use daoyi_cloud_entity::vo::system::user::{UserPageReqVO, UserSaveReqVo};
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

    pub async fn get_user_page(
        db: &DatabaseConnection,
        params: UserPageReqVO,
    ) -> anyhow::Result<PageResult<system_users::Model>> {
        let select = SystemUsers::find()
            .apply_if(params.keyword.as_ref(), |query, keyword| {
                query.filter(
                    Condition::any()
                        .add(system_users::Column::Username.contains(keyword))
                        .add(system_users::Column::Nickname.contains(keyword))
                        .add(system_users::Column::Email.contains(keyword))
                        .add(system_users::Column::Mobile.contains(keyword)),
                )
            })
            .apply_if(params.username, |query, username| {
                query.filter(system_users::Column::Username.contains(username))
            })
            .apply_if(params.mobile, |query, mobile| {
                query.filter(system_users::Column::Mobile.contains(mobile))
            })
            .apply_if(params.status, |query, status| {
                query.filter(system_users::Column::Status.eq(status))
            })
            .apply_if(params.create_time, |query, create_time| {
                if create_time.len() == 2 {
                    let start = create_time[0];
                    let end = create_time[1];
                    return query.filter(system_users::Column::CreateTime.between(start, end));
                }
                query
            })
            .order_by_desc(system_users::Column::Id);
        if params.pagination.page_size == PAGE_SIZE_NONE {
            let vec = select.all(db).await?;
            return Ok(PageResult::from_pagination(
                params.pagination,
                vec.len() as u64,
                vec,
            ));
        }
        let page_size = params.pagination.page_size as u64;
        let paginator = select.paginate(db, page_size);
        let total = paginator.num_items().await?;
        let items = paginator.fetch_page(params.pagination.page_no - 1).await?;
        let result = PageResult::from_pagination(params.pagination, total, items);
        Ok(result)
    }

    pub async fn create_user(
        db: &DatabaseConnection,
        params: UserSaveReqVo,
    ) -> anyhow::Result<i64> {
        let active_model: ActiveModel = params.into();
        let model = active_model.insert(db).await?;
        let id = model.id;
        Ok(id)
    }
}
