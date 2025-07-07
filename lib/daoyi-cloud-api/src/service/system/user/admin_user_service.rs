use daoyi_cloud_common::error::biz_error::{
    PASSWORD_INVALID_HASH, USER_NOT_EXISTS, USER_USERNAME_EXISTS,
};
use daoyi_cloud_common::models::page_param::PAGE_SIZE_NONE;
use daoyi_cloud_common::models::page_result::PageResult;
use daoyi_cloud_common::utils::base64_util::check_password;
use daoyi_cloud_entity::entity::system::prelude::SystemUsers;
use daoyi_cloud_entity::entity::system::system_users;
use daoyi_cloud_entity::entity::system::system_users::ActiveModel;
use daoyi_cloud_entity::vo::system::user::{UserPageReqVO, UserSaveReqVo};
use daoyi_cloud_logger::logger;
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
        Self::validate_user_for_create_or_update(
            db,
            None,
            Some(&params.username),
            Option::from(&params.mobile),
            Option::from(&params.email),
            Option::from(&params.dept_id),
            Option::from(&params.post_ids),
        )
        .await?;
        let mut active_model: ActiveModel = params.into_active_model();
        active_model.id = NotSet;
        let model = active_model.insert(db).await?;
        let id = model.id;
        Ok(id)
    }

    pub async fn update_user(
        db: &DatabaseConnection,
        id: i64,
        params: UserSaveReqVo,
    ) -> anyhow::Result<()> {
        let model = Self::validate_user_for_create_or_update(
            db,
            Some(&id),
            Some(&params.username),
            Option::from(&params.mobile),
            Option::from(&params.email),
            Option::from(&params.dept_id),
            Option::from(&params.post_ids),
        )
        .await?
        .unwrap();
        let mut active_model: ActiveModel = params.into_active_model();
        active_model.id = Unchanged(id);
        active_model.password = Unchanged(model.password); // 不修改密码
        active_model.update(db).await?;
        Ok(())
    }

    pub async fn delete_user(db: &DatabaseConnection, id: i64) -> anyhow::Result<()> {
        Self::validate_user_exists(db, Some(&id)).await?;
        SystemUsers::delete_by_id(id).exec(db).await?;
        Ok(())
    }

    pub async fn validate_user_for_create_or_update(
        db: &DatabaseConnection,
        id: Option<&i64>,
        username: Option<&String>,
        mobile: Option<&String>,
        email: Option<&String>,
        _dept_id: Option<&i64>,
        _post_ids: Option<&String>,
    ) -> anyhow::Result<Option<system_users::Model>> {
        // 校验用户存在
        let model = Self::validate_user_exists(db, id).await?;
        // 校验用户名唯一
        Self::validate_username_unique(db, id, username).await?;
        // 校验手机号唯一
        Self::validate_mobile_unique(db, id, mobile).await?;
        // 校验邮箱唯一
        Self::validate_email_unique(db, id, email).await?;
        // 校验部门处于开启状态 @todo
        // 校验岗位处于开启状态 @todo
        Ok(model)
    }

    pub async fn validate_email_unique(
        db: &DatabaseConnection,
        id: Option<&i64>,
        email: Option<&String>,
    ) -> anyhow::Result<()> {
        if email.is_none() || email.unwrap().is_empty() {
            return Ok(());
        }
        let model = Self::select_by_email(db, email.unwrap()).await?;
        if model.is_none() {
            return Ok(());
        }
        // 如果 id 为空，说明不用比较是否为相同 id 的用户
        if id.is_none() {
            return Err(anyhow::Error::from(USER_USERNAME_EXISTS.to_app_error()));
        }
        if model.unwrap().id != id.unwrap().to_owned() {
            return Err(anyhow::Error::from(USER_USERNAME_EXISTS.to_app_error()));
        }
        Ok(())
    }

    /**
     * 判断密码是否匹配
     *
     * @param raw_password     未加密的密码
     * @param encoded_password 加密后的密码
     * @return 是否匹配
     */
    pub async fn is_password_match(
        raw_password: &String,
        encoded_password: &String,
    ) -> anyhow::Result<bool> {
        let encoded_password = check_password(raw_password, encoded_password)
            .await
            .map_err(|err| {
                logger::error!("密码匹配失败: {:?}", err);
                anyhow::Error::from(PASSWORD_INVALID_HASH.to_app_error())
            })?;
        Ok(encoded_password)
    }

    pub async fn select_by_email(
        db: &DatabaseConnection,
        email: &String,
    ) -> anyhow::Result<Option<system_users::Model>> {
        let model = SystemUsers::find()
            .filter(system_users::Column::Email.eq(email))
            .one(db)
            .await?;
        Ok(model)
    }

    pub async fn validate_mobile_unique(
        db: &DatabaseConnection,
        id: Option<&i64>,
        mobile: Option<&String>,
    ) -> anyhow::Result<()> {
        if mobile.is_none() || mobile.unwrap().is_empty() {
            return Ok(());
        }
        let model = Self::select_by_mobile(db, mobile.unwrap()).await?;
        if model.is_none() {
            return Ok(());
        }
        // 如果 id 为空，说明不用比较是否为相同 id 的用户
        if id.is_none() {
            return Err(anyhow::Error::from(USER_USERNAME_EXISTS.to_app_error()));
        }
        if model.unwrap().id != id.unwrap().to_owned() {
            return Err(anyhow::Error::from(USER_USERNAME_EXISTS.to_app_error()));
        }
        Ok(())
    }

    pub async fn select_by_mobile(
        db: &DatabaseConnection,
        mobile: &String,
    ) -> anyhow::Result<Option<system_users::Model>> {
        let model = SystemUsers::find()
            .filter(system_users::Column::Mobile.eq(mobile))
            .one(db)
            .await?;
        Ok(model)
    }

    pub async fn validate_username_unique(
        db: &DatabaseConnection,
        id: Option<&i64>,
        username: Option<&String>,
    ) -> anyhow::Result<()> {
        if username.is_none() || username.unwrap().is_empty() {
            return Ok(());
        }
        let model = Self::select_by_username(db, username.unwrap()).await?;
        if model.is_none() {
            return Ok(());
        }
        // 如果 id 为空，说明不用比较是否为相同 id 的用户
        if id.is_none() {
            return Err(anyhow::Error::from(USER_USERNAME_EXISTS.to_app_error()));
        }
        if model.unwrap().id != id.unwrap().to_owned() {
            return Err(anyhow::Error::from(USER_USERNAME_EXISTS.to_app_error()));
        }
        Ok(())
    }

    pub async fn select_by_username(
        db: &DatabaseConnection,
        username: &String,
    ) -> anyhow::Result<Option<system_users::Model>> {
        let model = SystemUsers::find()
            .filter(system_users::Column::Username.eq(username))
            .one(db)
            .await?;
        Ok(model)
    }

    pub async fn validate_user_exists(
        db: &DatabaseConnection,
        id: Option<&i64>,
    ) -> anyhow::Result<Option<system_users::Model>> {
        if id.is_none() {
            return Ok(None);
        }
        let model = SystemUsers::find_by_id(id.unwrap().to_owned())
            .one(db)
            .await?
            .ok_or_else(|| USER_NOT_EXISTS.to_app_error())?;
        Ok(Some(model))
    }
}
