use crate::service::system::user::admin_user_service::AdminUserService;
use axum::debug_handler;
use axum::extract::State;
use axum::response::IntoResponse;
use daoyi_cloud_common::enums::EnumItemExt;
use daoyi_cloud_common::enums::common_status_enum::CommonStatusEnum;
use daoyi_cloud_common::models::app_server::AppState;

#[debug_handler]
pub async fn get_simple_user_list(State(AppState { db }): State<AppState>) -> impl IntoResponse {
    let users = AdminUserService::get_user_list_by_status(db, CommonStatusEnum::Enable.value())
        .await
        .unwrap();
    axum::Json(users)
}
