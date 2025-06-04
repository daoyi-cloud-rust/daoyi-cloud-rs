use daoyi_cloud_common::constants::redis_key_constants;
use daoyi_cloud_common::enums::menu_type_enum;
use daoyi_cloud_config::{db, redis_util};
use daoyi_cloud_entities::entities::system::prelude::SystemMenu;
use daoyi_cloud_entities::entities::system::system_menu;
use daoyi_cloud_models::models::common_result::AppResult;
use daoyi_cloud_models::models::page_result::PageResult;
use daoyi_cloud_models::models::system::menu_list_req_vo::MenuListReqVo;
use daoyi_cloud_models::models::system::menu_resp_vo::MenuRespVo;
use daoyi_cloud_models::models::system::menu_save_req_vo::MenuSaveReqVo;
use daoyi_cloud_models::models::system::system_oauth2_access_token::OAuth2AccessTokenCheckRespDTO;
use daoyi_cloud_models::models::{biz_error, tree_utils};
use itertools::Itertools;
use sea_orm::*;

pub async fn create_menu(
    login_user: OAuth2AccessTokenCheckRespDTO,
    req_vo: MenuSaveReqVo,
) -> AppResult<system_menu::Model> {
    // 校验父菜单的有效性
    let _ = validate_parent_menu(&req_vo.id, &req_vo.parent_id, &login_user.tenant_id).await?;
    // 校验菜单名的唯一性
    let _ = validate_menu_name_unique(
        &req_vo.id,
        &req_vo.parent_id,
        &req_vo.name,
        &login_user.tenant_id,
    )
    .await?;
    // 插入菜单
    let model = system_menu::ActiveModel {
        always_show: Set(req_vo.always_show.unwrap_or(true)),
        component: Set(req_vo.component),
        component_name: Set(req_vo.component_name),
        icon: Set(req_vo.icon),
        keep_alive: Set(req_vo.keep_alive.unwrap_or(true)),
        name: Set(req_vo.name),
        parent_id: Set(req_vo.parent_id.unwrap_or(system_menu::ID_ROOT)),
        path: Set(req_vo.path),
        permission: Set(req_vo.permission.unwrap_or("".to_string())),
        sort: Set(req_vo.sort),
        status: Set(req_vo.status),
        r#type: Set(req_vo.r#type),
        visible: Set(req_vo.visible.unwrap_or(true)),
        creator: Set(Some(login_user.user_id.to_string())),
        updater: Set(Some(login_user.user_id.to_string())),
        ..Default::default()
    };
    let model = model.insert(db::pool()).await?;
    redis_util::clear_cache_by_prefix(&format!(
        "{}:{}",
        redis_key_constants::PERMISSION_MENU_ID_LIST,
        &login_user.tenant_id
    ))
    .await;
    Ok(model)
}

pub async fn delete_menu(login_user: OAuth2AccessTokenCheckRespDTO, id: i64) -> AppResult<()> {
    // 校验是否存在
    let model = validate_menu_exists(&id, &login_user.tenant_id).await?;
    // 校验是否有子菜单
    let res = validate_menu_has_children(&id, &login_user.tenant_id).await?;
    if res {
        return biz_error::MENU_EXISTS_CHILDREN.to_app_result();
    }
    // 删除菜单
    let mut model = model.into_active_model();
    model.deleted = Set(true);
    model.update(db::pool()).await?;
    redis_util::clear_cache_by_prefix(&format!(
        "{}:{}",
        redis_key_constants::PERMISSION_MENU_ID_LIST,
        &login_user.tenant_id
    ))
    .await;
    Ok(())
}

pub async fn get_menu(login_user: OAuth2AccessTokenCheckRespDTO, id: i64) -> AppResult<MenuRespVo> {
    // 校验是否存在
    let model = validate_menu_exists(&id, &login_user.tenant_id).await?;
    Ok(model.into())
}

pub async fn menu_list(
    _login_user: OAuth2AccessTokenCheckRespDTO,
    params: MenuListReqVo,
) -> AppResult<PageResult<MenuRespVo>> {
    let mut select = SystemMenu::find().filter(system_menu::Column::Deleted.eq(false));
    if params.name.is_some() {
        select =
            select.filter(system_menu::Column::Name.like(format!("%{}%", params.name.unwrap())));
    }
    if params.status.is_some() {
        select = select.filter(system_menu::Column::Status.eq(params.status.unwrap()));
    }
    let mut result = select
        .all(db::pool())
        .await?
        .into_iter()
        .map(|model| MenuRespVo::from(model))
        .collect::<Vec<_>>();
    let total = result.len() as u64;
    result.sort_by_key(|a| a.sort);
    Ok(PageResult::new(result, total, 1, total as u32))
}

pub async fn menu_list_tree(
    login_user: OAuth2AccessTokenCheckRespDTO,
    params: MenuListReqVo,
) -> AppResult<PageResult<MenuRespVo>> {
    let mut result = menu_list(login_user, params).await?;
    let list: Vec<MenuRespVo> = result.list().to_vec();
    result.set_list(tree_utils::TreeUtil::<MenuRespVo>::build(list).build_tree());
    Ok(result)
}

pub async fn update_menu(
    login_user: OAuth2AccessTokenCheckRespDTO,
    req_vo: MenuSaveReqVo,
) -> AppResult<system_menu::Model> {
    // 校验是否存在
    let model = validate_menu_exists(&req_vo.id.unwrap(), &login_user.tenant_id).await?;
    // 校验父菜单的有效性
    let _ = validate_parent_menu(&req_vo.id, &req_vo.parent_id, &login_user.tenant_id).await?;
    // 校验菜单名的唯一性
    let _ = validate_menu_name_unique(
        &req_vo.id,
        &req_vo.parent_id,
        &req_vo.name,
        &login_user.tenant_id,
    )
    .await?;
    // 插入菜单
    let mut model = model.into_active_model();
    model.always_show = Set(req_vo.always_show.unwrap_or(true));
    model.component = Set(req_vo.component);
    model.component_name = Set(req_vo.component_name);
    model.icon = Set(req_vo.icon);
    model.keep_alive = Set(req_vo.keep_alive.unwrap_or(true));
    model.name = Set(req_vo.name);
    model.parent_id = Set(req_vo.parent_id.unwrap_or(system_menu::ID_ROOT));
    model.path = Set(req_vo.path);
    model.permission = Set(req_vo.permission.unwrap_or("".to_string()));
    model.sort = Set(req_vo.sort);
    model.status = Set(req_vo.status);
    model.r#type = Set(req_vo.r#type);
    model.visible = Set(req_vo.visible.unwrap_or(true));
    model.updater = Set(Some(login_user.user_id.to_string()));
    let model = model.update(db::pool()).await?;
    redis_util::clear_cache_by_prefix(&format!(
        "{}:{}",
        redis_key_constants::PERMISSION_MENU_ID_LIST,
        &login_user.tenant_id
    ))
    .await;
    Ok(model)
}

pub async fn get_child_menu_id_list_from_cache(id: &i64, tenant_id: &i64) -> AppResult<Vec<i64>> {
    let result = redis_util::get_method_cached::<Vec<i64>>(
        redis_key_constants::PERMISSION_MENU_ID_LIST,
        &format!("{}{}", tenant_id, id),
    )
    .await;
    if let Some(list) = result {
        return Ok(list);
    }
    let list = get_child_menu_list(&vec![id.to_owned()], tenant_id).await?;
    let mut list = list.iter().map(|model| model.id).collect::<Vec<_>>();
    list.sort();
    redis_util::set_method_cache::<Vec<i64>>(
        redis_key_constants::PERMISSION_MENU_ID_LIST,
        &format!("{}{}", tenant_id, id),
        None,
        &list,
    )
    .await;
    Ok(list)
}

async fn get_child_menu_list(
    ids: &Vec<i64>,
    _tenant_id: &i64,
) -> AppResult<Vec<system_menu::Model>> {
    let mut children = Vec::new();
    let mut parent_ids = ids.to_vec();
    loop {
        let mut list = SystemMenu::find()
            .filter(system_menu::Column::Deleted.eq(false))
            .filter(system_menu::Column::ParentId.is_in(parent_ids))
            .all(db::pool())
            .await?;
        if list.is_empty() {
            break;
        }
        parent_ids = list
            .iter()
            .map(|model| model.id)
            .unique()
            .collect::<Vec<_>>();
        children.append(&mut list);
    }
    Ok(children)
}

async fn validate_menu_has_children(id: &i64, _tenant_id: &i64) -> AppResult<bool> {
    let list = SystemMenu::find()
        .filter(system_menu::Column::Deleted.eq(false))
        .filter(system_menu::Column::ParentId.eq(id.to_owned()))
        .all(db::pool())
        .await?;
    Ok(!list.is_empty())
}

async fn validate_menu_exists(id: &i64, _tenant_id: &i64) -> AppResult<system_menu::Model> {
    let option = SystemMenu::find()
        .filter(system_menu::Column::Deleted.eq(false))
        .filter(system_menu::Column::Id.eq(id.to_owned()))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return Err(biz_error::MENU_NOT_EXISTS.to_app_error());
    }
    Ok(option.unwrap())
}

async fn validate_menu_name_unique(
    id: &Option<i64>,
    parent_id: &Option<i64>,
    name: &String,
    _tenant_id: &i64,
) -> AppResult<()> {
    let parent_id = parent_id.unwrap_or(system_menu::ID_ROOT);
    let option = SystemMenu::find()
        .filter(system_menu::Column::Deleted.eq(false))
        .filter(system_menu::Column::Name.eq(name))
        .filter(system_menu::Column::ParentId.eq(parent_id))
        .one(db::pool())
        .await?;
    if option.is_none() {
        return Ok(());
    }
    // 如果 id 为空，说明不用比较是否为相同 id 的菜单
    if id.is_none() || id.unwrap() != option.unwrap().id {
        return biz_error::MENU_NAME_DUPLICATE.to_app_result();
    }
    Ok(())
}

async fn validate_parent_menu(
    id: &Option<i64>,
    parent_id: &Option<i64>,
    tenant_id: &i64,
) -> AppResult<()> {
    if parent_id.is_none() || parent_id.unwrap() == system_menu::ID_ROOT {
        return Ok(());
    }
    // 1. 不能设置自己为父菜单
    if id.is_some() && id.unwrap() == parent_id.unwrap() {
        return biz_error::MENU_PARENT_ERROR.to_app_result();
    }
    // 2. 父菜单不存在
    let option = validate_menu_exists(&parent_id.to_owned().unwrap(), tenant_id).await;
    if option.is_err() {
        return biz_error::MENU_PARENT_NOT_EXISTS.to_app_result();
    }
    // 3. 递归校验父菜单，如果父菜单是自己的子菜单，则报错，避免形成环路
    let mut parent = option?;
    // 3.0 父菜单必须是目录或者菜单类型
    if parent.r#type != menu_type_enum::MenuTypeEnum::Dir.code()
        && parent.r#type != menu_type_enum::MenuTypeEnum::Menu.code()
    {
        return biz_error::MENU_PARENT_NOT_DIR_OR_MENU.to_app_result();
    }
    if id.is_none() {
        return Ok(());
    }
    loop {
        // 3.1 校验环路
        let parent_id = parent.parent_id;
        if parent_id == id.unwrap() {
            return biz_error::DEPT_PARENT_IS_CHILD.to_app_result();
        }
        // 3.2 继续递归下一级父菜单
        if parent_id == system_menu::ID_ROOT {
            break;
        }
        let option = validate_menu_exists(&parent_id, tenant_id).await;
        if option.is_err() {
            break;
        }
        parent = option?;
    }

    Ok(())
}
