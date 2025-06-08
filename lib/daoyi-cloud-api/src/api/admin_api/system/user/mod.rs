#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused)]
use crate::service::system::user::admin_user_service::AdminUserService;
use axum::debug_handler;
use axum::extract::State;
use daoyi_cloud_common::enums::EnumItemExt;
use daoyi_cloud_common::enums::common_status_enum::CommonStatusEnum;
use daoyi_cloud_common::error::ApiResult;
use daoyi_cloud_common::models::api_extract::query::Query;
use daoyi_cloud_common::models::api_extract::valid::Valid;
use daoyi_cloud_common::models::app_server::AppState;
use daoyi_cloud_common::models::page_result::PageResult;
use daoyi_cloud_common::response::ApiResponse;
use daoyi_cloud_entity::entity::system::system_users;
use daoyi_cloud_entity::vo::system::user::UserPageReqVO;

// #[tracing::instrument(name = "get_simple_user_list", skip_all, fields(pay_method = "alipay"))]
#[debug_handler]
pub async fn get_simple_user_list(
    State(AppState { db }): State<AppState>,
) -> ApiResult<Vec<system_users::Model>> {
    // logger::warn!("出了点小错误。。。");
    let users =
        AdminUserService::get_user_list_by_status(db, CommonStatusEnum::Enable.value()).await?;
    ApiResponse::okk(Some(users))
}

#[debug_handler]
pub async fn get_user_page(
    State(AppState { db }): State<AppState>,
    Valid(Query(params)): Valid<Query<UserPageReqVO>>,
) -> ApiResult<PageResult<system_users::Model>> {
    let users = AdminUserService::get_user_page(db, params).await?;
    ApiResponse::okk(Some(users))
}
