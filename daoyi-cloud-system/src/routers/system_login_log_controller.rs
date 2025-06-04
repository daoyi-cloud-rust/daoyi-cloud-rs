use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::login_log_page_req_vo::LoginLogPageReqVo;
use daoyi_cloud_models::models::system::login_log_resp_vo::LoginLogRespVo;
use daoyi_cloud_service::service::get_current_user;
use daoyi_cloud_service::service::system::system_login_log_service;
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;
use salvo::{Depot, Writer};

/// 查看登录日志分页列表
#[endpoint(tags("管理后台 - 系统管理 - 登录日志"))]
pub async fn page_login_log(
    params: JsonBody<LoginLogPageReqVo>,
    depot: &mut Depot,
) -> JsonResult<PageResult<LoginLogRespVo>> {
    let login_user = get_current_user(depot);
    let params = params.into_inner();
    let list = system_login_log_service::get_login_log_page(login_user, params).await?;
    json_ok(list)
}
