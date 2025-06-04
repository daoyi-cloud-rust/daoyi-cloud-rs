use daoyi_cloud_models::models::biz_error;
use daoyi_cloud_models::models::common_result::{Empty, JsonResult, empty_json_ok, json_ok};
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::post_page_req_vo::PostPageReqVo;
use daoyi_cloud_models::models::system::post_resp_vo::PostRespVo;
use daoyi_cloud_models::models::system::post_save_req_vo::PostSaveReqVo;
use daoyi_cloud_service::service::get_current_user;
use daoyi_cloud_service::service::system::system_post_service;
use salvo::oapi::endpoint;
use salvo::oapi::extract::{JsonBody, QueryParam};
use salvo::{Depot, Writer};
use validator::Validate;

/// 创建岗位
#[endpoint(tags("管理后台 - 系统管理 - 岗位"))]
pub async fn create_post(params: JsonBody<PostSaveReqVo>, depot: &mut Depot) -> JsonResult<String> {
    let login_user = get_current_user(depot);
    let vo = params.into_inner();
    vo.validate()?;
    let model = system_post_service::create_post(login_user, vo).await?;
    json_ok(model.id.to_string())
}

/// 删除岗位
#[endpoint(tags("管理后台 - 系统管理 - 岗位"))]
pub async fn delete_post(id: QueryParam<i64>, depot: &mut Depot) -> JsonResult<Empty> {
    let login_user = get_current_user(depot);
    let id = id.into_inner();
    let _ = system_post_service::delete_post(login_user, id).await?;
    empty_json_ok()
}

/// 获得岗位信息
#[endpoint(tags("管理后台 - 系统管理 - 岗位"))]
pub async fn get_post(id: QueryParam<i64>, depot: &mut Depot) -> JsonResult<PostRespVo> {
    let login_user = get_current_user(depot);
    let id = id.into_inner();
    let vo = system_post_service::get_post(login_user, id).await?;
    json_ok(vo)
}

/// 获取岗位分页列表
#[endpoint(tags("管理后台 - 系统管理 - 岗位"))]
pub async fn post_list(
    params: JsonBody<PostPageReqVo>,
    depot: &mut Depot,
) -> JsonResult<PageResult<PostRespVo>> {
    let login_user = get_current_user(depot);
    let params = params.into_inner();
    let list = system_post_service::post_list(login_user, params).await?;
    json_ok(list)
}

/// 更新岗位
#[endpoint(tags("管理后台 - 系统管理 - 岗位"))]
pub async fn update_post(params: JsonBody<PostSaveReqVo>, depot: &mut Depot) -> JsonResult<String> {
    let login_user = get_current_user(depot);
    let vo = params.into_inner();
    vo.validate()?;
    if vo.id.is_none() {
        return biz_error::DEPT_NOT_FOUND.to_app_result();
    }
    let model = system_post_service::update_post(login_user, vo).await?;
    json_ok(model.id.to_string())
}
