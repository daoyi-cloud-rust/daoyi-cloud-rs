use daoyi_cloud_entities::entities::system::system_users::Model;
use salvo::prelude::*;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
pub struct SystemUsersModel {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub remark: Option<String>,
    pub dept_id: Option<i64>,
    pub post_ids: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub sex: Option<i8>,
    pub avatar: Option<String>,
    pub status: i8,
    pub login_ip: Option<String>,
    pub login_date: Option<DateTime>,
    pub creator: Option<String>,
    pub create_time: DateTime,
    pub updater: Option<String>,
    pub update_time: DateTime,
    pub deleted: bool,
    pub tenant_id: i64,
}

impl From<Model> for SystemUsersModel {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            username: m.username,
            password: m.password,
            nickname: m.nickname,
            remark: m.remark,
            dept_id: m.dept_id,
            post_ids: m.post_ids,
            email: m.email,
            mobile: m.mobile,
            sex: m.sex,
            avatar: m.avatar,
            status: m.status,
            login_ip: m.login_ip,
            login_date: m.login_date,
            creator: m.creator,
            create_time: m.create_time,
            updater: m.updater,
            update_time: m.update_time,
            deleted: m.deleted,
            tenant_id: m.tenant_id,
        }
    }
}
