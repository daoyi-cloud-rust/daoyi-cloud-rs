use daoyi_cloud_models::models::common_result::{EmptyJsonResult, empty_json_ok};
use daoyi_cloud_models::models::system::permission_assign_role_data_scope_req_vo::PermissionAssignRoleDataScopeReqVo;
use daoyi_cloud_service::service::get_current_user;
use daoyi_cloud_service::service::system::system_permission_service;
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;
use salvo::{Depot, Writer};
use validator::Validate;

/// 赋予角色数据权限
#[endpoint(tags("管理后台 - 系统管理 - 权限"))]
pub async fn assign_role_data_scope(
    params: JsonBody<PermissionAssignRoleDataScopeReqVo>,
    depot: &mut Depot,
) -> EmptyJsonResult {
    let login_user = get_current_user(depot);
    let vo = params.into_inner();
    vo.validate()?;
    let _ = system_permission_service::assign_role_data_scope(login_user, vo).await?;
    empty_json_ok()
}
