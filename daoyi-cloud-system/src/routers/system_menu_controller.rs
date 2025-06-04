use daoyi_cloud_models::models::biz_error;
use daoyi_cloud_models::models::common_result::{Empty, JsonResult, empty_json_ok, json_ok};
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::menu_list_req_vo::MenuListReqVo;
use daoyi_cloud_models::models::system::menu_resp_vo::MenuRespVo;
use daoyi_cloud_models::models::system::menu_save_req_vo::MenuSaveReqVo;
use daoyi_cloud_service::service::get_current_user;
use daoyi_cloud_service::service::system::system_menu_service;
use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, QueryParam};
use salvo::{Depot, Writer};
use validator::Validate;

/// 创建菜单
#[endpoint(tags("管理后台 - 系统管理 - 菜单"))]
pub async fn create_menu(params: JsonBody<MenuSaveReqVo>, depot: &mut Depot) -> JsonResult<String> {
    let login_user = get_current_user(depot);
    let vo = params.into_inner();
    vo.validate()?;
    let model = system_menu_service::create_menu(login_user, vo).await?;
    json_ok(model.id.to_string())
}

/// 删除菜单
#[endpoint(tags("管理后台 - 系统管理 - 菜单"))]
pub async fn delete_menu(id: QueryParam<i64>, depot: &mut Depot) -> JsonResult<Empty> {
    let login_user = get_current_user(depot);
    let id = id.into_inner();
    let _ = system_menu_service::delete_menu(login_user, id).await?;
    empty_json_ok()
}

/// 获得菜单信息
#[endpoint(tags("管理后台 - 系统管理 - 菜单"))]
pub async fn get_menu(id: QueryParam<i64>, depot: &mut Depot) -> JsonResult<MenuRespVo> {
    let login_user = get_current_user(depot);
    let id = id.into_inner();
    let vo = system_menu_service::get_menu(login_user, id).await?;
    json_ok(vo)
}

/// 获取菜单列表
#[endpoint(tags("管理后台 - 系统管理 - 菜单"))]
pub async fn menu_list(
    params: JsonBody<MenuListReqVo>,
    depot: &mut Depot,
) -> JsonResult<PageResult<MenuRespVo>> {
    let login_user = get_current_user(depot);
    let params = params.into_inner();
    let list = system_menu_service::menu_list(login_user, params).await?;
    json_ok(list)
}

/// 获取菜单树列表
#[endpoint(tags("管理后台 - 系统管理 - 菜单"))]
pub async fn menu_list_tree(
    params: JsonBody<MenuListReqVo>,
    depot: &mut Depot,
) -> JsonResult<PageResult<MenuRespVo>> {
    let login_user = get_current_user(depot);
    let params = params.into_inner();
    let list = system_menu_service::menu_list_tree(login_user, params).await?;
    json_ok(list)
}

/// 更新菜单
#[endpoint(tags("管理后台 - 系统管理 - 菜单"))]
pub async fn update_menu(params: JsonBody<MenuSaveReqVo>, depot: &mut Depot) -> JsonResult<String> {
    let login_user = get_current_user(depot);
    let vo = params.into_inner();
    vo.validate()?;
    if vo.id.is_none() {
        return biz_error::DEPT_NOT_FOUND.to_app_result();
    }
    let model = system_menu_service::update_menu(login_user, vo).await?;
    json_ok(model.id.to_string())
}
