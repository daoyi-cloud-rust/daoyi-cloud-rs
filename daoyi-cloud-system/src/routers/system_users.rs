use daoyi_cloud_models::models::common_result::{json_ok, JsonResult};
use daoyi_cloud_models::models::system::system_users::SystemUsersModel;
use daoyi_cloud_service::service::system::system_users_service::get_system_users_by_id;
use salvo::oapi::endpoint;
use salvo::oapi::extract::QueryParam;
use salvo::Writer;

/// 获得用户详情
#[endpoint(tags("管理后台 - 系统管理 - 用户管理"))]
pub async fn get_by_id(id: QueryParam<i64, true>) -> JsonResult<SystemUsersModel> {
    let res = get_system_users_by_id(id.into_inner()).await?;
    json_ok(res)
}
