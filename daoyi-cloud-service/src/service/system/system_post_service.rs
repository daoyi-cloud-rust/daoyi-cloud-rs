use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::prelude::SystemPost;
use daoyi_cloud_entities::entities::system::system_post;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::mask_utils::adjust_time_range;
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::post_page_req_vo::PostPageReqVo;
use daoyi_cloud_models::models::system::post_resp_vo::PostRespVo;
use daoyi_cloud_models::models::system::post_save_req_vo::PostSaveReqVo;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use daoyi_cloud_models::models::{biz_error, page_param};
use sea_orm::*;

pub async fn create_post(
    login_user: OAuth2AccessTokenCheckRespDTO,
    req_vo: PostSaveReqVo,
) -> AppResult<system_post::Model> {
    // 校验正确性
    let _ = validate_post_for_create_or_update(
        &req_vo.id,
        &req_vo.name,
        &req_vo.code,
        &login_user.tenant_id,
    )
    .await?;
    // 插入岗位
    let model = system_post::ActiveModel {
        code: Set(req_vo.code),
        name: Set(req_vo.name),
        remark: Set(req_vo.remark),
        sort: Set(req_vo.sort),
        status: Set(req_vo.status),
        creator: Set(Some(login_user.user_id.to_string())),
        updater: Set(Some(login_user.user_id.to_string())),
        tenant_id: Set(login_user.tenant_id),
        ..Default::default()
    };
    let model = model.insert(db::pool()).await?;
    Ok(model)
}

async fn validate_post_for_create_or_update(
    id: &Option<i64>,
    name: &String,
    code: &String,
    tenant_id: &i64,
) -> AppResult<system_post::Model> {
    // 校验自己存在
    let model = validate_post_exists(id, tenant_id).await?;
    // 校验岗位名的唯一性
    let _ = validate_post_name_unique(id, name, tenant_id).await?;
    // 校验岗位编码的唯一性
    let _ = validate_post_code_unique(id, code, tenant_id).await?;
    Ok(model)
}

pub async fn delete_post(login_user: OAuth2AccessTokenCheckRespDTO, id: i64) -> AppResult<()> {
    // 校验是否存在
    let model = validate_post_exists(&Some(id), &login_user.tenant_id).await?;
    // 删除岗位
    let mut model = model.into_active_model();
    model.deleted = Set(true);
    model.update(db::pool()).await?;
    Ok(())
}

pub async fn get_post(login_user: OAuth2AccessTokenCheckRespDTO, id: i64) -> AppResult<PostRespVo> {
    // 校验是否存在
    let model = validate_post_exists(&Some(id), &login_user.tenant_id).await?;
    Ok(model.into())
}

pub async fn post_list(
    login_user: OAuth2AccessTokenCheckRespDTO,
    params: PostPageReqVo,
) -> AppResult<PageResult<PostRespVo>> {
    let page_no = params.page_no.unwrap_or(page_param::PAGE_NO);
    let page_size = params.page_size.unwrap_or(page_param::PAGE_SIZE);
    let mut select = SystemPost::find()
        .filter(system_post::Column::Deleted.eq(false))
        .filter(system_post::Column::TenantId.eq(login_user.tenant_id));
    if params.code.is_some() {
        select =
            select.filter(system_post::Column::Code.like(format!("%{}%", params.code.unwrap())));
    }
    if params.name.is_some() {
        select =
            select.filter(system_post::Column::Name.like(format!("%{}%", params.name.unwrap())));
    }
    if params.status.is_some() {
        select = select.filter(system_post::Column::Status.eq(params.status.unwrap()));
    }
    if params.create_time.is_some() {
        let create_time = params.create_time.unwrap();
        if create_time.len() == 2 {
            let create_time = adjust_time_range(create_time);
            select = select
                .filter(system_post::Column::CreateTime.between(create_time[0], create_time[1]));
        }
    }
    select = select.order_by_desc(system_post::Column::CreateTime);
    // Get total count
    let total = select.clone().count(db::pool()).await?;
    select = select
        .offset(((page_no - 1) * page_size) as u64)
        .limit(page_size as u64);
    let result = select
        .all(db::pool())
        .await?
        .into_iter()
        .map(|model| PostRespVo::from(model))
        .collect::<Vec<_>>();
    Ok(PageResult::build(result, total, page_no, page_size))
}

pub async fn update_post(
    login_user: OAuth2AccessTokenCheckRespDTO,
    req_vo: PostSaveReqVo,
) -> AppResult<system_post::Model> {
    if req_vo.id.is_none() {
        return Err(biz_error::POST_NOT_FOUND.to_app_error());
    }
    // 校验正确性
    let model = validate_post_for_create_or_update(
        &req_vo.id,
        &req_vo.name,
        &req_vo.code,
        &login_user.tenant_id,
    )
    .await?;
    // 插入岗位
    let mut model = model.into_active_model();
    model.code = Set(req_vo.code);
    model.name = Set(req_vo.name);
    model.remark = Set(req_vo.remark);
    model.sort = Set(req_vo.sort);
    model.status = Set(req_vo.status);
    model.updater = Set(Some(login_user.user_id.to_string()));
    let model = model.update(db::pool()).await?;
    Ok(model)
}

async fn validate_post_exists(id: &Option<i64>, tenant_id: &i64) -> AppResult<system_post::Model> {
    if id.is_none() {
        return Ok(system_post::Model::default());
    }
    let option = SystemPost::find()
        .filter(system_post::Column::Deleted.eq(false))
        .filter(system_post::Column::TenantId.eq(tenant_id.to_owned()))
        .filter(system_post::Column::Id.eq(id.to_owned()))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return Err(biz_error::POST_NOT_FOUND.to_app_error());
    }
    Ok(option.unwrap())
}

async fn validate_post_name_unique(
    id: &Option<i64>,
    name: &String,
    tenant_id: &i64,
) -> AppResult<()> {
    let option = SystemPost::find()
        .filter(system_post::Column::Deleted.eq(false))
        .filter(system_post::Column::TenantId.eq(tenant_id.to_owned()))
        .filter(system_post::Column::Name.eq(name))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return Ok(());
    }
    // 如果 id 为空，说明不用比较是否为相同 id 的岗位
    if id.is_none() || id.unwrap() != option.unwrap().id {
        return biz_error::POST_NAME_DUPLICATE.to_app_result();
    }
    Ok(())
}

async fn validate_post_code_unique(
    id: &Option<i64>,
    code: &String,
    tenant_id: &i64,
) -> AppResult<()> {
    let option = SystemPost::find()
        .filter(system_post::Column::Deleted.eq(false))
        .filter(system_post::Column::TenantId.eq(tenant_id.to_owned()))
        .filter(system_post::Column::Code.eq(code))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return Ok(());
    }
    // 如果 id 为空，说明不用比较是否为相同 id 的岗位
    if id.is_none() || id.unwrap() != option.unwrap().id {
        return biz_error::POST_CODE_DUPLICATE.to_app_result();
    }
    Ok(())
}
