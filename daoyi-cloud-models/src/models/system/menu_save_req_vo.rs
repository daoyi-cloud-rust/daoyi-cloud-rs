use crate::models::common_result::to_common_response;
use salvo::http::StatusCode;
use salvo::oapi;
use salvo::oapi::{EndpointOutRegister, ToResponse, ToSchema};
use serde::Deserialize;
use std::any::type_name;
use validator::Validate;

/// 管理后台 - 菜单创建/修改 Request VO
#[derive(Deserialize, ToSchema, Default, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MenuSaveReqVo {
    /// 是否总是显示
    pub always_show: Option<bool>,
    /// 组件路径,仅菜单类型为菜单时，才需要传
    pub component: Option<String>,
    /// 组件名
    pub component_name: Option<String>,
    /// 菜单图标,仅菜单类型为菜单或者目录时，才需要传
    pub icon: Option<String>,
    /// 菜单编号
    pub id: Option<i64>,
    /// 是否缓存
    pub keep_alive: Option<bool>,
    /// 菜单名称
    pub name: String,
    /// 父菜单 ID
    pub parent_id: Option<i64>,
    /// 路由地址,仅菜单类型为菜单或者目录时，才需要传
    pub path: Option<String>,
    /// 权限标识,仅菜单类型为按钮时，才需要传递
    pub permission: Option<String>,
    /// 显示顺序不能为空
    pub sort: i32,
    /// 状态,见 CommonStatusEnum 枚举
    pub status: i8,
    /// 类型，参见 MenuTypeEnum 枚举类
    pub r#type: i8,
    /// 是否可见
    pub visible: Option<bool>,
}
impl EndpointOutRegister for MenuSaveReqVo {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert(StatusCode::OK.as_str(), Self::to_response(components));
    }
}

impl ToResponse for MenuSaveReqVo {
    fn to_response(components: &mut oapi::Components) -> oapi::RefOr<oapi::response::Response> {
        let schema_ref = Self::to_schema(components);
        let type_name = type_name::<Self>();
        to_common_response(components, type_name, schema_ref)
    }
}
