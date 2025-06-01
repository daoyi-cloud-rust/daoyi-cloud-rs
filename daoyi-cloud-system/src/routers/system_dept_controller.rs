use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use daoyi_cloud_models::models::system::dept_save_req_vo::DeptSaveReqVo;
use daoyi_cloud_service::service::get_current_user;
use daoyi_cloud_service::service::system::system_dept_service::create_dept;
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;
use salvo::{Depot, Writer};

/// 创建部门
#[endpoint(tags("管理后台 - 系统管理 - 部门"))]
pub async fn create(params: JsonBody<DeptSaveReqVo>, depot: &mut Depot) -> JsonResult<String> {
    let login_user = get_current_user(depot);
    let vo = params.into_inner();
    let model = create_dept(login_user, vo).await?;
    json_ok(model.id.to_string())
}
