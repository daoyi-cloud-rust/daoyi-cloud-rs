use crate::models::common_result::to_common_response;
use crate::models::mask_utils::DATE_TIME_FORMAT;
use crate::models::tree_utils;
use daoyi_cloud_entities::entities::system::system_menu::Model;
use salvo::oapi;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::type_name;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct MenuRespVo {
    /// 是否总是显示
    pub always_show: bool,
    /// 组件路径,仅菜单类型为菜单时，才需要传
    pub component: Option<String>,
    /// 组件名
    pub component_name: Option<String>,
    /// 创建时间
    pub create_time: String,
    /// 菜单图标,仅菜单类型为菜单或者目录时，才需要传
    pub icon: Option<String>,
    /// 菜单编号
    pub id: i64,
    /// 是否缓存
    pub keep_alive: bool,
    /// 菜单名称
    pub name: String,
    /// 父菜单 ID
    pub parent_id: i64,
    /// 路由地址,仅菜单类型为菜单或者目录时，才需要传
    pub path: Option<String>,
    /// 权限标识,仅菜单类型为按钮时，才需要传递
    pub permission: String,
    /// 显示顺序不能为空
    pub sort: i32,
    /// 状态,见 CommonStatusEnum 枚举
    pub status: i8,
    /// 类型，参见 MenuTypeEnum 枚举类
    #[serde(rename = "type")]
    pub r#type: i8,
    /// 是否可见
    pub visible: bool,
    /// 子菜单
    pub children: Vec<MenuRespVo>,
}

impl tree_utils::TreeNode<MenuRespVo> for MenuRespVo {
    fn id(&self) -> i64 {
        self.id
    }

    fn parent_id(&self) -> i64 {
        self.parent_id
    }

    fn children(&mut self, list: Vec<MenuRespVo>) {
        self.children = list;
    }

    fn sort(&self) -> i32 {
        self.sort
    }
}

impl From<Model> for MenuRespVo {
    fn from(m: Model) -> Self {
        Self {
            always_show: m.always_show,
            component: m.component,
            component_name: m.component_name,
            create_time: m.create_time.format(DATE_TIME_FORMAT).to_string(),
            icon: m.icon,
            id: m.id,
            keep_alive: m.keep_alive,
            name: m.name,
            parent_id: m.parent_id,
            path: m.path,
            permission: m.permission,
            sort: m.sort,
            status: m.status,
            r#type: m.r#type,
            visible: m.visible,
            children: vec![],
        }
    }
}

impl EndpointOutRegister for MenuRespVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for MenuRespVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
