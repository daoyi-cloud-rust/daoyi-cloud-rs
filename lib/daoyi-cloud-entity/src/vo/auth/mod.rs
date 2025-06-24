use daoyi_cloud_common::enums::social_type_enum::SocialTypeEnum;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub trait CaptchaVerificationReqVO {
    fn get_captcha_verification(&self) -> Option<&str>;
}

/// AuthLoginReqVO，管理后台 - 账号密码登录 Request VO，如果登录并绑定社交用户，需要传递 social 开头的参数
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginReqVo {
    /// 验证码，验证码开启时，需要传递
    pub captcha_verification: Option<String>,
    /// 密码
    pub password: String,
    /// 授权码
    pub social_code: Option<String>,
    pub social_code_valid: Option<bool>,
    /// state
    pub social_state: Option<String>,
    /// 社交平台的类型，参见 SocialTypeEnum 枚举值
    pub social_type: Option<SocialTypeEnum>,
    /// 账号
    pub username: String,
}
impl CaptchaVerificationReqVO for AuthLoginReqVo {
    fn get_captcha_verification(&self) -> Option<&str> {
        self.captcha_verification.as_deref()
    }
}
/// AuthLoginRespVO，管理后台 - 登录 Response VO
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginRespVo {
    /// 访问令牌
    pub access_token: String,
    /// 过期时间
    pub expires_time: String,
    /// 刷新令牌
    pub refresh_token: String,
    /// 终端编号
    pub terminal_id: String,
    /// 用户编号
    pub user_id: i64,
}

/// AuthPermissionInfoRespVO，管理后台 - 登录用户的权限信息 Response VO，额外包括用户信息和角色列表
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthPermissionInfoRespVo {
    /// 菜单树
    pub menus: Vec<MenuVo>,
    /// 操作权限数组
    pub permissions: Vec<String>,
    /// 角色标识数组
    pub roles: Vec<String>,
    pub user: UserVo,
}

/// MenuVO，管理后台 - 登录用户的菜单信息 Response VO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuVo {
    /// 是否总是显示
    pub always_show: Option<bool>,
    pub children: Option<Vec<MenuVo>>,
    /// 组件路径,仅菜单类型为菜单时，才需要传
    pub component: Option<String>,
    /// 组件名
    pub component_name: Option<String>,
    /// 菜单图标,仅菜单类型为菜单或者目录时，才需要传
    pub icon: Option<String>,
    /// 菜单名称
    pub id: i64,
    /// 是否缓存
    pub keep_alive: bool,
    /// 菜单名称
    pub name: String,
    /// 父菜单 ID
    pub parent_id: i64,
    /// 路由地址,仅菜单类型为菜单或者目录时，才需要传
    pub path: Option<String>,
    /// 是否可见
    pub visible: bool,
}

/// UserVO，用户信息 VO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserVo {
    /// 用户头像
    pub avatar: String,
    /// 部门编号
    pub dept_id: i64,
    /// 用户编号
    pub id: i64,
    /// 用户昵称
    pub nickname: String,
    /// 用户名
    pub username: String,
}
