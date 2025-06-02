use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::prelude::SystemDept;
use daoyi_cloud_entities::entities::system::system_dept;
use daoyi_cloud_models::models::biz_error;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::system::dept_resp_vo::DeptRespVo;
use daoyi_cloud_models::models::system::dept_save_req_vo::DeptSaveReqVo;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use sea_orm::*;

pub async fn create_dept(
    login_user: OAuth2AccessTokenCheckRespDTO,
    req_vo: DeptSaveReqVo,
) -> AppResult<system_dept::Model> {
    // 校验父部门的有效性
    let _ = validate_parent_dept(&req_vo.id, &req_vo.parent_id).await?;
    // 校验部门名的唯一性
    let _ = validate_dept_name_unique(&req_vo.id, &req_vo.parent_id, &req_vo.name).await?;
    // 插入部门
    let model = system_dept::ActiveModel {
        email: Set(req_vo.email),
        leader_user_id: Set(req_vo.leader_user_id),
        name: Set(req_vo.name),
        parent_id: Set(req_vo.parent_id.unwrap_or(system_dept::PARENT_ID_ROOT)),
        phone: Set(req_vo.phone),
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

pub async fn delete_dept(_login_user: OAuth2AccessTokenCheckRespDTO, id: i64) -> AppResult<()> {
    // 校验是否存在
    let model = validate_dept_exists(&id).await?;
    // 校验是否有子部门
    let res = validate_dept_has_children(&id).await?;
    if res {
        return biz_error::DEPT_EXITS_CHILDREN.to_app_result();
    }
    // 删除部门
    let mut model = model.into_active_model();
    model.deleted = Set(true);
    model.update(db::pool()).await?;
    Ok(())
}

pub async fn get_dept(
    _login_user: OAuth2AccessTokenCheckRespDTO,
    id: i64,
) -> AppResult<DeptRespVo> {
    // 校验是否存在
    let model = validate_dept_exists(&id).await?;
    Ok(model.into())
}

async fn validate_dept_has_children(id: &i64) -> AppResult<bool> {
    let list = SystemDept::find()
        .filter(system_dept::Column::Deleted.eq(false))
        .filter(system_dept::Column::ParentId.eq(id.to_owned()))
        .all(db::pool())
        .await?;
    Ok(!list.is_empty())
}

async fn validate_dept_exists(id: &i64) -> AppResult<system_dept::Model> {
    let option = SystemDept::find()
        .filter(system_dept::Column::Deleted.eq(false))
        .filter(system_dept::Column::Id.eq(id.to_owned()))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return Err(biz_error::DEPT_NOT_FOUND.to_app_error());
    }
    Ok(option.unwrap())
}

async fn validate_dept_name_unique(
    id: &Option<i64>,
    parent_id: &Option<i64>,
    name: &String,
) -> AppResult<()> {
    let parent_id = parent_id.unwrap_or(system_dept::PARENT_ID_ROOT);
    let option = SystemDept::find()
        .filter(system_dept::Column::Deleted.eq(false))
        .filter(system_dept::Column::Name.eq(name))
        .filter(system_dept::Column::ParentId.eq(parent_id))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return Ok(());
    }
    // 如果 id 为空，说明不用比较是否为相同 id 的部门
    if id.is_none() || id.unwrap() != option.unwrap().id {
        return biz_error::DEPT_NAME_DUPLICATE.to_app_result();
    }
    Ok(())
}

async fn validate_parent_dept(id: &Option<i64>, parent_id: &Option<i64>) -> AppResult<()> {
    if parent_id.is_none() || parent_id.unwrap() == system_dept::PARENT_ID_ROOT {
        return Ok(());
    }
    // 1. 不能设置自己为父部门
    if id.is_some() && id.unwrap() == parent_id.unwrap() {
        return biz_error::DEPT_PARENT_ERROR.to_app_result();
    }
    // 2. 父部门不存在
    let option = SystemDept::find()
        .filter(system_dept::Column::Deleted.eq(false))
        .filter(system_dept::Column::Id.eq(parent_id.unwrap()))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return biz_error::DEPT_PARENT_NOT_EXITS.to_app_result();
    }
    // 3. 递归校验父部门，如果父部门是自己的子部门，则报错，避免形成环路
    if id.is_none() {
        return Ok(());
    }
    let mut parent = option.unwrap();
    loop {
        // 3.1 校验环路
        let parent_id = parent.parent_id;
        if parent_id == id.unwrap() {
            return biz_error::DEPT_PARENT_IS_CHILD.to_app_result();
        }
        // 3.2 继续递归下一级父部门
        if parent_id == system_dept::PARENT_ID_ROOT {
            break;
        }
        let option = SystemDept::find()
            .filter(system_dept::Column::Deleted.eq(false))
            .filter(system_dept::Column::Id.eq(parent_id))
            .one(db::pool())
            .await?;
        if option.is_none() {
            break;
        }
        parent = option.unwrap();
    }

    Ok(())
}
