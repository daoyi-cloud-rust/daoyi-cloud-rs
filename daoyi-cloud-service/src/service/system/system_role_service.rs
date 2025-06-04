use daoyi_cloud_common::constants::redis_key_constants;
use daoyi_cloud_common::enums::data_scope_enum::DataScopeEnum;
use daoyi_cloud_common::enums::role_code_enum::RoleCodeEnum;
use daoyi_cloud_common::enums::role_type_enum::RoleTypeEnum;
use daoyi_cloud_config::{db, redis_util};
use daoyi_cloud_entities::entities::system::prelude::SystemRole;
use daoyi_cloud_entities::entities::system::system_role;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::mask_utils::adjust_time_range;
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::role_page_req_vo::RolePageReqVo;
use daoyi_cloud_models::models::system::role_resp_vo::RoleRespVo;
use daoyi_cloud_models::models::system::role_save_req_vo::RoleSaveReqVo;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use daoyi_cloud_models::models::{biz_error, page_param};
use sea_orm::*;

pub async fn create_role(
    login_user: OAuth2AccessTokenCheckRespDTO,
    req_vo: RoleSaveReqVo,
) -> AppResult<system_role::Model> {
    // 校验正确性
    let _ = validate_role_duplicate(
        &req_vo.id,
        &req_vo.name,
        &req_vo.code,
        &login_user.tenant_id,
    )
    .await?;
    // 插入角色
    let model = system_role::ActiveModel {
        code: Set(req_vo.code),
        name: Set(req_vo.name),
        remark: Set(req_vo.remark),
        sort: Set(req_vo.sort),
        status: Set(req_vo.status),
        data_scope: Set(DataScopeEnum::All.code()),
        creator: Set(Some(login_user.user_id.to_string())),
        updater: Set(Some(login_user.user_id.to_string())),
        tenant_id: Set(login_user.tenant_id),
        ..Default::default()
    };
    let model = model.insert(db::pool()).await?;
    Ok(model)
}

async fn validate_role_duplicate(
    id: &Option<i64>,
    name: &String,
    code: &String,
    tenant_id: &i64,
) -> AppResult<system_role::Model> {
    // 0. 超级管理员，不允许创建
    if RoleCodeEnum::is_super_admin(code) {
        return biz_error::ROLE_ADMIN_CODE_ERROR.to_app_result_args(vec![code]);
    }
    // 校验自己存在
    let model = validate_role_exists(id, tenant_id).await?;
    // 1. 该 name 名字被其它角色所使用
    let _ = validate_role_name_unique(id, name, tenant_id).await?;
    // 2. 是否存在相同编码的角色
    let _ = validate_role_code_unique(id, code, tenant_id).await?;
    // 校验角色是否可以被更新
    if id.is_some() && RoleTypeEnum::is_system(model.r#type) {
        return biz_error::ROLE_CAN_NOT_UPDATE_SYSTEM_TYPE_ROLE.to_app_result();
    }
    Ok(model)
}

pub async fn delete_role(login_user: OAuth2AccessTokenCheckRespDTO, id: i64) -> AppResult<()> {
    // 校验是否存在
    let model = validate_role_exists(&Some(id), &login_user.tenant_id).await?;
    // 校验角色是否可以被更新
    if RoleTypeEnum::is_system(model.r#type) {
        return biz_error::ROLE_CAN_NOT_UPDATE_SYSTEM_TYPE_ROLE.to_app_result();
    }
    // 删除角色
    let mut model = model.into_active_model();
    model.deleted = Set(true);
    model.update(db::pool()).await?;
    redis_util::clear_cached_key(&format!("{}:{}", &login_user.tenant_id, id)).await;
    Ok(())
}

pub async fn get_role(login_user: OAuth2AccessTokenCheckRespDTO, id: i64) -> AppResult<RoleRespVo> {
    let result = redis_util::get_method_cached::<RoleRespVo>(
        redis_key_constants::ROLE,
        &format!("{}:{}", &login_user.tenant_id, id),
    )
    .await;
    if let Some(vo) = result {
        return Ok(vo);
    }
    // 校验是否存在
    let model = validate_role_exists(&Some(id), &login_user.tenant_id).await?;
    let resp_vo = model.into();
    redis_util::set_method_cache::<RoleRespVo>(
        redis_key_constants::ROLE,
        &format!("{}:{}", &login_user.tenant_id, id),
        None,
        &resp_vo,
    )
    .await;
    Ok(resp_vo)
}

pub async fn role_list(
    login_user: OAuth2AccessTokenCheckRespDTO,
    params: RolePageReqVo,
) -> AppResult<PageResult<RoleRespVo>> {
    let page_no = params.page_no.unwrap_or(page_param::PAGE_NO);
    let page_size = params.page_size.unwrap_or(page_param::PAGE_SIZE);
    let mut select = SystemRole::find()
        .filter(system_role::Column::Deleted.eq(false))
        .filter(system_role::Column::TenantId.eq(login_user.tenant_id));
    if params.code.is_some() {
        select =
            select.filter(system_role::Column::Code.like(format!("%{}%", params.code.unwrap())));
    }
    if params.name.is_some() {
        select =
            select.filter(system_role::Column::Name.like(format!("%{}%", params.name.unwrap())));
    }
    if params.status.is_some() {
        select = select.filter(system_role::Column::Status.eq(params.status.unwrap()));
    }
    if params.create_time.is_some() {
        let create_time = params.create_time.unwrap();
        if create_time.len() == 2 {
            let create_time = adjust_time_range(create_time);
            select = select
                .filter(system_role::Column::CreateTime.between(create_time[0], create_time[1]));
        }
    }
    select = select.order_by_desc(system_role::Column::CreateTime);
    // Get total count
    let total = select.clone().count(db::pool()).await?;
    select = select
        .offset(((page_no - 1) * page_size) as u64)
        .limit(page_size as u64);
    let result = select
        .all(db::pool())
        .await?
        .into_iter()
        .map(|model| RoleRespVo::from(model))
        .collect::<Vec<_>>();
    Ok(PageResult::build(result, total, page_no, page_size))
}

pub async fn update_role(
    login_user: OAuth2AccessTokenCheckRespDTO,
    req_vo: RoleSaveReqVo,
) -> AppResult<system_role::Model> {
    if req_vo.id.is_none() {
        return Err(biz_error::ROLE_NOT_EXISTS.to_app_error());
    }
    // 校验正确性
    let model = validate_role_duplicate(
        &req_vo.id,
        &req_vo.name,
        &req_vo.code,
        &login_user.tenant_id,
    )
    .await?;
    // 插入角色
    let mut model = model.into_active_model();
    model.code = Set(req_vo.code);
    model.name = Set(req_vo.name);
    model.remark = Set(req_vo.remark);
    model.sort = Set(req_vo.sort);
    model.status = Set(req_vo.status);
    model.updater = Set(Some(login_user.user_id.to_string()));
    let model = model.update(db::pool()).await?;
    redis_util::clear_cached_key(&format!("{}:{}", &login_user.tenant_id, &model.id)).await;
    Ok(model)
}

async fn validate_role_exists(id: &Option<i64>, tenant_id: &i64) -> AppResult<system_role::Model> {
    if id.is_none() {
        return Ok(system_role::Model::default());
    }
    let option = SystemRole::find()
        .filter(system_role::Column::Deleted.eq(false))
        .filter(system_role::Column::TenantId.eq(tenant_id.to_owned()))
        .filter(system_role::Column::Id.eq(id.to_owned()))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return Err(biz_error::ROLE_NOT_EXISTS.to_app_error());
    }
    Ok(option.unwrap())
}

async fn validate_role_name_unique(
    id: &Option<i64>,
    name: &String,
    tenant_id: &i64,
) -> AppResult<()> {
    let option = SystemRole::find()
        .filter(system_role::Column::Deleted.eq(false))
        .filter(system_role::Column::TenantId.eq(tenant_id.to_owned()))
        .filter(system_role::Column::Name.eq(name))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return Ok(());
    }
    // 如果 id 为空，说明不用比较是否为相同 id 的角色
    if id.is_none() || id.unwrap() != option.unwrap().id {
        return biz_error::ROLE_NAME_DUPLICATE.to_app_result_args(vec![name]);
    }
    Ok(())
}

async fn validate_role_code_unique(
    id: &Option<i64>,
    code: &String,
    tenant_id: &i64,
) -> AppResult<()> {
    let option = SystemRole::find()
        .filter(system_role::Column::Deleted.eq(false))
        .filter(system_role::Column::TenantId.eq(tenant_id.to_owned()))
        .filter(system_role::Column::Code.eq(code))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return Ok(());
    }
    // 如果 id 为空，说明不用比较是否为相同 id 的角色
    if id.is_none() || id.unwrap() != option.unwrap().id {
        return biz_error::ROLE_CODE_DUPLICATE.to_app_result_args(vec![code]);
    }
    Ok(())
}
