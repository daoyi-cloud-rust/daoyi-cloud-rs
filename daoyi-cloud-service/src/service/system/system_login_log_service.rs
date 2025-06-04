use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::prelude::SystemLoginLog;
use daoyi_cloud_entities::entities::system::system_login_log;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::mask_utils::adjust_time_range;
use daoyi_cloud_models::models::page_param;
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::login_log_page_req_vo::LoginLogPageReqVo;
use daoyi_cloud_models::models::system::login_log_resp_vo::LoginLogRespVo;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use sea_orm::*;

pub async fn get_login_log_page(
    login_user: OAuth2AccessTokenCheckRespDTO,
    params: LoginLogPageReqVo,
) -> AppResult<PageResult<LoginLogRespVo>> {
    let page_no = params.page_no.unwrap_or(page_param::PAGE_NO);
    let page_size = params.page_size.unwrap_or(page_param::PAGE_SIZE);
    let mut select = SystemLoginLog::find()
        .filter(system_login_log::Column::Deleted.eq(false))
        .filter(system_login_log::Column::TenantId.eq(login_user.tenant_id));
    if params.user_id.is_some() {
        select = select.filter(system_login_log::Column::UserId.eq(params.user_id.unwrap()));
    }
    if params.user_ip.is_some() {
        select = select.filter(system_login_log::Column::UserIp.eq(params.user_ip.unwrap()));
    }
    if params.username.is_some() {
        select = select.filter(system_login_log::Column::Username.eq(params.username.unwrap()));
    }
    if params.result.is_some() {
        select = select.filter(system_login_log::Column::Result.eq(params.result.unwrap()));
    }
    if params.create_time.is_some() {
        let create_time = params.create_time.unwrap();
        if create_time.len() == 2 {
            let create_time = adjust_time_range(create_time);
            select = select.filter(
                system_login_log::Column::CreateTime.between(create_time[0], create_time[1]),
            );
        }
    }
    select = select.order_by_desc(system_login_log::Column::CreateTime);
    // Get total count
    let total = select.clone().count(db::pool()).await?;
    select = select
        .offset(((page_no - 1) * page_size) as u64)
        .limit(page_size as u64);
    let result = select
        .all(db::pool())
        .await?
        .into_iter()
        .map(|model| LoginLogRespVo::from(model))
        .collect::<Vec<_>>();
    Ok(PageResult::build(result, total, page_no, page_size))
}
