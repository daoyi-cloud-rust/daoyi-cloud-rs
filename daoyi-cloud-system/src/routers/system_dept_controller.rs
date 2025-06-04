use daoyi_cloud_models::models::biz_error;
use daoyi_cloud_models::models::common_result::{Empty, JsonResult, empty_ok, json_ok};
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::dept_list_req_vo::DeptListReqVo;
use daoyi_cloud_models::models::system::dept_resp_vo::DeptRespVo;
use daoyi_cloud_models::models::system::dept_save_req_vo::DeptSaveReqVo;
use daoyi_cloud_service::service::get_current_user;
use daoyi_cloud_service::service::system::system_dept_service;
use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, QueryParam};
use salvo::{Depot, Writer};
use validator::Validate;

/// 创建部门
#[endpoint(tags("管理后台 - 系统管理 - 部门"))]
pub async fn create_dept(params: JsonBody<DeptSaveReqVo>, depot: &mut Depot) -> JsonResult<String> {
    let login_user = get_current_user(depot);
    let vo = params.into_inner();
    vo.validate()?;
    let model = system_dept_service::create_dept(login_user, vo).await?;
    json_ok(model.id.to_string())
}

/// 删除部门
#[endpoint(tags("管理后台 - 系统管理 - 部门"))]
pub async fn delete_dept(id: QueryParam<i64>, depot: &mut Depot) -> JsonResult<Empty> {
    let login_user = get_current_user(depot);
    let id = id.into_inner();
    let _ = system_dept_service::delete_dept(login_user, id).await?;
    empty_ok()
}

/// 获得部门信息
#[endpoint(tags("管理后台 - 系统管理 - 部门"))]
pub async fn get_dept(id: QueryParam<i64>, depot: &mut Depot) -> JsonResult<DeptRespVo> {
    let login_user = get_current_user(depot);
    let id = id.into_inner();
    let vo = system_dept_service::get_dept(login_user, id).await?;
    json_ok(vo)
}

/// 获取部门列表
#[endpoint(tags("管理后台 - 系统管理 - 部门"))]
pub async fn dept_list(
    params: JsonBody<DeptListReqVo>,
    depot: &mut Depot,
) -> JsonResult<PageResult<DeptRespVo>> {
    let login_user = get_current_user(depot);
    let params = params.into_inner();
    let list = system_dept_service::dept_list(login_user, params).await?;
    json_ok(list)
}

/// 获取部门树列表
#[endpoint(tags("管理后台 - 系统管理 - 部门"))]
pub async fn dept_list_tree(
    params: JsonBody<DeptListReqVo>,
    depot: &mut Depot,
) -> JsonResult<PageResult<DeptRespVo>> {
    let login_user = get_current_user(depot);
    let params = params.into_inner();
    let list = system_dept_service::dept_list_tree(login_user, params).await?;
    json_ok(list)
}

/// 更新部门
#[endpoint(tags("管理后台 - 系统管理 - 部门"))]
pub async fn update_dept(params: JsonBody<DeptSaveReqVo>, depot: &mut Depot) -> JsonResult<String> {
    let login_user = get_current_user(depot);
    let vo = params.into_inner();
    vo.validate()?;
    if vo.id.is_none() {
        return biz_error::DEPT_NOT_FOUND.to_app_result();
    }
    let model = system_dept_service::update_dept(login_user, vo).await?;
    json_ok(model.id.to_string())
}
