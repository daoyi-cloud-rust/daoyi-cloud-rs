use crate::service::system::user::admin_user_service::AdminUserService;
use axum::debug_handler;
use axum::extract::State;
use daoyi_cloud_common::enums::EnumItemExt;
use daoyi_cloud_common::enums::common_status_enum::CommonStatusEnum;
use daoyi_cloud_common::error::ApiResult;
use daoyi_cloud_common::models::app_server::AppState;
use daoyi_cloud_common::response::ApiResponse;
use daoyi_cloud_entity::entity::system::system_users;

#[debug_handler]
pub async fn get_simple_user_list(
    State(AppState { db }): State<AppState>,
) -> ApiResult<Vec<system_users::Model>> {
    let users =
        AdminUserService::get_user_list_by_status(db, CommonStatusEnum::Enable.value()).await?;
    ApiResponse::okk(Some(users))
}
