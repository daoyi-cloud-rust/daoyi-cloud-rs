use daoyi_cloud_config::db;
use daoyi_cloud_entities::entities::system::prelude::SystemDept;
use daoyi_cloud_entities::entities::system::system_dept;
use daoyi_cloud_models::models::biz_error;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::dept_list_req_vo::DeptListReqVo;
use daoyi_cloud_models::models::system::dept_resp_vo::DeptRespVo;
use daoyi_cloud_models::models::system::dept_save_req_vo::DeptSaveReqVo;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use sea_orm::*;
use std::collections::HashMap;

pub async fn create_dept(
    login_user: OAuth2AccessTokenCheckRespDTO,
    req_vo: DeptSaveReqVo,
) -> AppResult<system_dept::Model> {
    // 校验父部门的有效性
    let _ = validate_parent_dept(&req_vo.id, &req_vo.parent_id, &login_user.tenant_id).await?;
    // 校验部门名的唯一性
    let _ = validate_dept_name_unique(
        &req_vo.id,
        &req_vo.parent_id,
        &req_vo.name,
        &login_user.tenant_id,
    )
    .await?;
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

pub async fn delete_dept(login_user: OAuth2AccessTokenCheckRespDTO, id: i64) -> AppResult<()> {
    // 校验是否存在
    let model = validate_dept_exists(&id, &login_user.tenant_id).await?;
    // 校验是否有子部门
    let res = validate_dept_has_children(&id, &login_user.tenant_id).await?;
    if res {
        return biz_error::DEPT_EXITS_CHILDREN.to_app_result();
    }
    // 删除部门
    let mut model = model.into_active_model();
    model.deleted = Set(true);
    model.update(db::pool()).await?;
    Ok(())
}

pub async fn get_dept(login_user: OAuth2AccessTokenCheckRespDTO, id: i64) -> AppResult<DeptRespVo> {
    // 校验是否存在
    let model = validate_dept_exists(&id, &login_user.tenant_id).await?;
    Ok(model.into())
}

pub async fn dept_list(
    login_user: OAuth2AccessTokenCheckRespDTO,
    params: DeptListReqVo,
) -> AppResult<PageResult<DeptRespVo>> {
    let mut select = SystemDept::find()
        .filter(system_dept::Column::Deleted.eq(false))
        .filter(system_dept::Column::TenantId.eq(login_user.tenant_id));
    if params.name.is_some() {
        select =
            select.filter(system_dept::Column::Name.like(format!("%{}%", params.name.unwrap())));
    }
    if params.status.is_some() {
        select = select.filter(system_dept::Column::Status.eq(params.status.unwrap()));
    }
    let mut result = select
        .all(db::pool())
        .await?
        .into_iter()
        .map(|model| DeptRespVo::from(model))
        .collect::<Vec<_>>();
    let total = result.len() as u64;
    result.sort_by_key(|a| a.sort);
    Ok(PageResult::new(result, total, 1, total as u32))
}

pub async fn dept_list_tree(
    login_user: OAuth2AccessTokenCheckRespDTO,
    params: DeptListReqVo,
) -> AppResult<PageResult<DeptRespVo>> {
    let mut result = dept_list(login_user, params).await?;
    let list: Vec<DeptRespVo> = result.list().to_vec();
    result.set_list(build_dept_tree(list));
    Ok(result)
}

fn build_dept_tree(list: Vec<DeptRespVo>) -> Vec<DeptRespVo> {
    // 创建哈希映射以便快速查找节点
    let mut map: HashMap<i64, DeptRespVo> = HashMap::new();
    let mut root_nodes = Vec::new();
    // 先构建所有节点并将其添加到映射中
    for mut dept in list.into_iter() {
        // 清空子节点列表（准备重新构建）
        dept.children = Vec::new();
        map.insert(dept.id, dept);
    }
    // 创建父节点 ID 与子节点列表的映射
    let mut parent_children: HashMap<i64, Vec<i64>> = HashMap::new();
    for id in map.keys() {
        let dept = map.get(id).unwrap();
        parent_children
            .entry(dept.parent_id)
            .or_insert_with(Vec::new)
            .push(*id);
    }
    // 递归构建树形结构
    fn build_children(
        parent_id: i64,
        map: &mut HashMap<i64, DeptRespVo>,
        parent_children: &HashMap<i64, Vec<i64>>,
    ) -> Vec<DeptRespVo> {
        let mut children = Vec::new();
        if let Some(child_ids) = parent_children.get(&parent_id) {
            for &child_id in child_ids {
                let mut child = map.remove(&child_id).unwrap();
                // 递归添加子节点
                child.children = build_children(child_id, map, parent_children);
                children.push(child);
            }
        }
        // 按排序值排序
        children.sort_by_key(|c| c.sort);
        children
    }
    // 处理根节点（父节点为 0 的节点）
    if let Some(root_ids) = parent_children.get(&system_dept::PARENT_ID_ROOT) {
        for &root_id in root_ids {
            let mut root = map.remove(&root_id).unwrap();
            // 构建根节点的子节点
            root.children = build_children(root_id, &mut map, &parent_children);
            root_nodes.push(root);
        }
    }
    // 按排序值对根节点排序
    root_nodes.sort_by_key(|r| r.sort);
    root_nodes
}

async fn validate_dept_has_children(id: &i64, tenant_id: &i64) -> AppResult<bool> {
    let list = SystemDept::find()
        .filter(system_dept::Column::Deleted.eq(false))
        .filter(system_dept::Column::TenantId.eq(tenant_id.to_owned()))
        .filter(system_dept::Column::ParentId.eq(id.to_owned()))
        .all(db::pool())
        .await?;
    Ok(!list.is_empty())
}

async fn validate_dept_exists(id: &i64, tenant_id: &i64) -> AppResult<system_dept::Model> {
    let option = SystemDept::find()
        .filter(system_dept::Column::Deleted.eq(false))
        .filter(system_dept::Column::TenantId.eq(tenant_id.to_owned()))
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
    tenant_id: &i64,
) -> AppResult<()> {
    let parent_id = parent_id.unwrap_or(system_dept::PARENT_ID_ROOT);
    let option = SystemDept::find()
        .filter(system_dept::Column::Deleted.eq(false))
        .filter(system_dept::Column::TenantId.eq(tenant_id.to_owned()))
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

async fn validate_parent_dept(
    id: &Option<i64>,
    parent_id: &Option<i64>,
    tenant_id: &i64,
) -> AppResult<()> {
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
        .filter(system_dept::Column::TenantId.eq(tenant_id.to_owned()))
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
            .filter(system_dept::Column::TenantId.eq(tenant_id.to_owned()))
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
