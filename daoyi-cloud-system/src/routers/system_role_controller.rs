use daoyi_cloud_models::models::biz_error;
use daoyi_cloud_models::models::common_result::{Empty, JsonResult, empty_json_ok, json_ok};
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::role_page_req_vo::RolePageReqVo;
use daoyi_cloud_models::models::system::role_resp_vo::RoleRespVo;
use daoyi_cloud_models::models::system::role_save_req_vo::RoleSaveReqVo;
use daoyi_cloud_service::service::get_current_user;
use daoyi_cloud_service::service::system::system_role_service;
use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, QueryParam};
use salvo::{Depot, Writer};
use validator::Validate;

/// 创建角色
#[endpoint(tags("管理后台 - 系统管理 - 角色"))]
pub async fn create_role(params: JsonBody<RoleSaveReqVo>, depot: &mut Depot) -> JsonResult<String> {
    let login_user = get_current_user(depot);
    let vo = params.into_inner();
    vo.validate()?;
    let model = system_role_service::create_role(login_user, vo).await?;
    json_ok(model.id.to_string())
}

/// 删除角色
#[endpoint(tags("管理后台 - 系统管理 - 角色"))]
pub async fn delete_role(id: QueryParam<i64>, depot: &mut Depot) -> JsonResult<Empty> {
    let login_user = get_current_user(depot);
    let id = id.into_inner();
    let _ = system_role_service::delete_role(login_user, id).await?;
    empty_json_ok()
}

/// 获得角色信息
#[endpoint(tags("管理后台 - 系统管理 - 角色"))]
pub async fn get_role(id: QueryParam<i64>, depot: &mut Depot) -> JsonResult<RoleRespVo> {
    let login_user = get_current_user(depot);
    let id = id.into_inner();
    let vo = system_role_service::get_role(login_user, id).await?;
    json_ok(vo)
}

/// 获取角色分页列表
#[endpoint(tags("管理后台 - 系统管理 - 角色"))]
pub async fn role_list(
    params: JsonBody<RolePageReqVo>,
    depot: &mut Depot,
) -> JsonResult<PageResult<RoleRespVo>> {
    let login_user = get_current_user(depot);
    let params = params.into_inner();
    let list = system_role_service::role_list(login_user, params).await?;
    json_ok(list)
}

/// 更新角色
#[endpoint(tags("管理后台 - 系统管理 - 角色"))]
pub async fn update_role(params: JsonBody<RoleSaveReqVo>, depot: &mut Depot) -> JsonResult<String> {
    let login_user = get_current_user(depot);
    let vo = params.into_inner();
    vo.validate()?;
    if vo.id.is_none() {
        return biz_error::DEPT_NOT_FOUND.to_app_result();
    }
    let model = system_role_service::update_role(login_user, vo).await?;
    json_ok(model.id.to_string())
}
