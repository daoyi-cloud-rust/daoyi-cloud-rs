use crate::service::system::generate_redis_key_by_table_name;
use daoyi_cloud_config::{db, redis_util};
use daoyi_cloud_entities::entities::system::prelude::{
    SystemMenu, SystemRole, SystemRoleMenu, SystemUserRole,
};
use daoyi_cloud_entities::entities::system::{
    system_menu, system_role, system_role_menu, system_user_role,
};
use daoyi_cloud_models::models::system::system_role::SystemRoleModel;
use daoyi_cloud_utils::enums::role_code_enum::RoleCodeEnum;
use itertools::Itertools;
use redis::AsyncCommands;
use sea_orm::*;

pub async fn has_any_permissions(user_id: i64, permissions: Vec<String>) -> bool {
    // 如果为空，说明已经有权限
    if permissions.is_empty() {
        return true;
    }
    // 获得当前登录的角色。如果为空，说明没有权限
    let roles = get_enable_role_list_by_user_id(user_id).await;
    if roles.is_empty() {
        return false;
    }
    // 情况一：遍历判断每个权限，如果有一满足，说明有权限
    for permission in permissions {
        if roles_has_permission(&roles, &permission).await {
            return true;
        }
    }
    // 情况二：如果是超管，也说明有权限
    roles
        .into_iter()
        .any(|x| RoleCodeEnum::is_super_admin(x.code.as_str()))
}

/// 判断指定角色，是否拥有该 permission 权限
pub async fn roles_has_permission(role_list: &Vec<SystemRoleModel>, permission: &str) -> bool {
    // 如果角色为空，说明没有权限
    if role_list.is_empty() {
        return false;
    }
    let menu_ids = get_menu_id_list_by_permission(permission).await;
    // 采用严格模式，如果权限找不到对应的 Menu 的话，也认为没有权限
    if menu_ids.is_empty() {
        return false;
    }
    // 判断是否有权限
    let role_ids: Vec<i64> = role_list.into_iter().map(|x| x.id).unique().collect();
    for menu_id in menu_ids {
        // 获得拥有该菜单的角色编号集合
        let menu_role_ids = get_role_id_list_by_menu_id(menu_id).await;
        // 如果有交集，说明有权限
        if menu_role_ids.iter().any(|x| role_ids.contains(x)) {
            return true;
        }
    }
    false
}

pub async fn get_role_id_list_by_menu_id(menu_id: i64) -> Vec<i64> {
    let redis_key = generate_redis_key_by_table_name(
        "get_role_id_list_by_menu_id",
        menu_id.to_string().as_str(),
    );
    let result = redis_util::pool()
        .get::<String, String>(redis_key.to_owned())
        .await;
    if let Ok(res_str) = result {
        let result: Result<Vec<i64>, _> = serde_json::from_str(&res_str);
        if let Ok(list) = result {
            if !list.is_empty() {
                return list;
            }
        }
    }
    let select = SystemRoleMenu::find()
        .filter(system_role_menu::Column::Deleted.eq(false))
        .filter(system_role_menu::Column::MenuId.eq(menu_id));
    let result = select.all(db::pool()).await;
    if let Ok(list) = result {
        let menu_ids: Vec<i64> = list.into_iter().map(|x| x.menu_id).unique().collect();
        if !menu_ids.is_empty() {
            redis_util::pool()
                .set_ex::<&str, String, String>(
                    redis_key.as_str(),
                    serde_json::to_string(&menu_ids).unwrap(),
                    60,
                )
                .await
                .expect("redis set error");
        }
    }
    vec![]
}

pub async fn get_menu_id_list_by_permission(permission: &str) -> Vec<i64> {
    let redis_key = generate_redis_key_by_table_name("get_menu_id_list_by_permission", permission);
    let result = redis_util::pool()
        .get::<String, String>(redis_key.to_owned())
        .await;
    if let Ok(res_str) = result {
        let result: Result<Vec<i64>, _> = serde_json::from_str(&res_str);
        if let Ok(list) = result {
            if !list.is_empty() {
                return list;
            }
        }
    }
    let mut menu_id_list: Vec<i64> = Vec::new();
    let select = SystemMenu::find()
        .filter(system_menu::Column::Deleted.eq(false))
        .filter(system_menu::Column::Permission.eq(permission));
    let result = select.all(db::pool()).await;
    if let Ok(list) = result {
        let menu_ids: Vec<i64> = list.into_iter().map(|x| x.id).collect();
        if !menu_ids.is_empty() {
            menu_id_list = menu_ids;
        }
    }
    redis_util::pool()
        .set_ex::<&str, String, String>(
            redis_key.as_str(),
            serde_json::to_string(&menu_id_list).unwrap(),
            60,
        )
        .await
        .expect("redis set error");
    menu_id_list
}

pub async fn get_enable_role_list_by_user_id(user_id: i64) -> Vec<SystemRoleModel> {
    let redis_key = generate_redis_key_by_table_name(
        "get_enable_role_list_by_user_id",
        user_id.to_string().as_str(),
    );
    let result = redis_util::pool()
        .get::<String, String>(redis_key.to_owned())
        .await;
    if let Ok(res_str) = result {
        let result: Result<Vec<SystemRoleModel>, _> = serde_json::from_str(&res_str);
        if let Ok(list) = result {
            if !list.is_empty() {
                return list;
            }
        }
    }
    let mut roles: Vec<SystemRoleModel> = Vec::new();
    let select = SystemUserRole::find()
        .filter(system_user_role::Column::Deleted.eq(false))
        .filter(system_user_role::Column::UserId.eq(user_id));
    let result = select.all(db::pool()).await;
    if let Ok(list) = result {
        let role_ids: Vec<i64> = list.into_iter().map(|x| x.role_id).unique().collect();
        if !role_ids.is_empty() {
            let select = SystemRole::find()
                .filter(system_role::Column::Deleted.eq(false))
                .filter(system_role::Column::Id.is_in(role_ids));
            let result = select.all(db::pool()).await;
            if let Ok(list) = result {
                let role_list: Vec<SystemRoleModel> =
                    list.into_iter().map(|x| SystemRoleModel::from(x)).collect();
                roles = role_list;
            }
        }
    }
    redis_util::pool()
        .set_ex::<&str, String, String>(
            redis_key.as_str(),
            serde_json::to_string(&roles).unwrap(),
            60,
        )
        .await
        .expect("redis set error");
    roles
}
