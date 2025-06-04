use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::prelude::SystemOperateLog;
use daoyi_cloud_entities::entities::system::system_operate_log;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::mask_utils::adjust_time_range;
use daoyi_cloud_models::models::page_param;
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::operate_log_page_req_vo::OperateLogPageReqVo;
use daoyi_cloud_models::models::system::operate_log_resp_vo::OperateLogRespVo;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use sea_orm::*;

pub async fn get_operate_log_page(
    login_user: OAuth2AccessTokenCheckRespDTO,
    params: OperateLogPageReqVo,
) -> AppResult<PageResult<OperateLogRespVo>> {
    let page_no = params.page_no.unwrap_or(page_param::PAGE_NO);
    let page_size = params.page_size.unwrap_or(page_param::PAGE_SIZE);
    let mut select = SystemOperateLog::find()
        .filter(system_operate_log::Column::Deleted.eq(false))
        .filter(system_operate_log::Column::TenantId.eq(login_user.tenant_id));
    if params.user_id.is_some() {
        select = select.filter(system_operate_log::Column::UserId.eq(params.user_id.unwrap()));
    }
    if params.biz_id.is_some() {
        select = select.filter(system_operate_log::Column::BizId.eq(params.biz_id.unwrap()));
    }
    if params.r#type.is_some() {
        select = select.filter(system_operate_log::Column::Type.eq(params.r#type.unwrap()));
    }
    if params.sub_type.is_some() {
        select = select.filter(system_operate_log::Column::SubType.eq(params.sub_type.unwrap()));
    }
    if params.action.is_some() {
        select = select.filter(
            system_operate_log::Column::Action.like(format!("%{}%", params.action.unwrap())),
        );
    }
    if params.create_time.is_some() {
        let create_time = params.create_time.unwrap();
        if create_time.len() == 2 {
            let create_time = adjust_time_range(create_time);
            select = select.filter(
                system_operate_log::Column::CreateTime.between(create_time[0], create_time[1]),
            );
        }
    }
    select = select.order_by_desc(system_operate_log::Column::CreateTime);
    // Get total count
    let total = select.clone().count(db::pool()).await?;
    select = select
        .offset(((page_no - 1) * page_size) as u64)
        .limit(page_size as u64);
    let result = select
        .all(db::pool())
        .await?
        .into_iter()
        .map(|model| OperateLogRespVo::from(model))
        .collect::<Vec<_>>();
    Ok(PageResult::build(result, total, page_no, page_size))
}
