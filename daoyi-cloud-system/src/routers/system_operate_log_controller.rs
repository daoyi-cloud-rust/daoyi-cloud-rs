use daoyi_cloud_models::models::common_result::{JsonResult, json_ok};
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::operate_log_page_req_vo::OperateLogPageReqVo;
use daoyi_cloud_models::models::system::operate_log_resp_vo::OperateLogRespVo;
use daoyi_cloud_service::service::get_current_user;
use daoyi_cloud_service::service::system::system_operate_log_service;
use salvo::oapi::endpoint;
use salvo::oapi::extract::JsonBody;
use salvo::{Depot, Writer};

/// 查看操作日志分页列表
#[endpoint(tags("管理后台 - 系统管理 - 操作日志"))]
pub async fn page_operate_log(
    params: JsonBody<OperateLogPageReqVo>,
    depot: &mut Depot,
) -> JsonResult<PageResult<OperateLogRespVo>> {
    let login_user = get_current_user(depot);
    let params = params.into_inner();
    let list = system_operate_log_service::get_operate_log_page(login_user, params).await?;
    json_ok(list)
}
