use daoyi_cloud_common::utils;
use daoyi_cloud_config::config;
use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use daoyi_cloud_models::models::error::AppError;
use daoyi_cloud_models::models::system::system_users::SystemUsersModel;
use daoyi_cloud_service::service::system::system_users_service::get_system_users_by_id;
use salvo::oapi::endpoint;
use salvo::oapi::extract::QueryParam;
use salvo::{Depot, Writer};

/// 获得用户详情
#[endpoint(tags("管理后台 - 系统管理 - 用户管理"))]
pub async fn get_by_id(
    id: QueryParam<i64, true>,
    depot: &mut Depot,
) -> JsonResult<SystemUsersModel> {
    let tenant_middleware_config = &config::get().tenant;
    let Ok(tenant_id) = depot.get::<i64>(tenant_middleware_config.header_name.as_str()) else {
        return Err(AppError::internal("租户ID不存在.".to_string()));
    };
    let res = get_system_users_by_id(id.into_inner(), Some(tenant_id)).await?;
    json_ok(res)
}

/// 密码生成器
#[endpoint(tags("管理后台 - 系统管理 - 用户管理"))]
pub async fn hash_password(password: QueryParam<&str, true>) -> JsonResult<String> {
    let res = utils::hash_password(password.into_inner())?;
    json_ok(res)
}
