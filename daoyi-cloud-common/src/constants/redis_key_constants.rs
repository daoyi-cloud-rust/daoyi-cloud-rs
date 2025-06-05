/**
 * 指定部门的所有子部门编号数组的缓存
 * <p>
 * KEY 格式：dept_children_ids:{id}
 * VALUE 数据类型：String 子部门编号集合
*/
pub const DEPT_CHILDREN_ID_LIST: &str = "dept_children_ids";

/**
 * 角色的缓存
 * <p>
 * KEY 格式：role:{id}
 * VALUE 数据类型：String 角色信息
*/
pub const ROLE: &str = "role";

/**
 * 用户拥有的角色编号的缓存
 * <p>
 * KEY 格式：user_role_ids:{userId}
 * VALUE 数据类型：String 角色编号集合
*/
pub const USER_ROLE_ID_LIST: &str = "user_role_ids";

/**
 * 拥有指定菜单的角色编号的缓存
 * <p>
 * KEY 格式：user_role_ids:{menuId}
 * VALUE 数据类型：String 角色编号集合
*/
pub const MENU_ROLE_ID_LIST: &str = "menu_role_ids";

/// 拥有指定权限的缓存
pub const HAS_ANY_PERMISSION: &str = "has_any_permission";
/**
 * 拥有权限对应的菜单编号数组的缓存
 * <p>
 * KEY 格式：permission_menu_ids:{permission}
 * VALUE 数据类型：String 菜单编号数组
*/
pub const PERMISSION_MENU_ID_LIST: &str = "permission_menu_ids";

/**
 * OAuth2 客户端的缓存
 * <p>
 * KEY 格式：oauth_client:{id}
 * VALUE 数据类型：String 客户端信息
*/
pub const OAUTH_CLIENT: &str = "oauth_client";

/**
 * 访问令牌的缓存
 * <p>
 * KEY 格式：oauth2_access_token:{token}
 * VALUE 数据类型：String 访问令牌信息 {@link OAuth2AccessTokenDO}
 * <p>
 * 由于动态过期时间，使用 RedisTemplate 操作
*/
pub const OAUTH2_ACCESS_TOKEN: &str = "oauth2_access_token";

/**
 * 站内信模版的缓存
 * <p>
 * KEY 格式：notify_template:{code}
 * VALUE 数据格式：String 模版信息
*/
pub const NOTIFY_TEMPLATE: &str = "notify_template";

/**
 * 邮件账号的缓存
 * <p>
 * KEY 格式：mail_account:{id}
 * VALUE 数据格式：String 账号信息
*/
pub const MAIL_ACCOUNT: &str = "mail_account";

/**
 * 邮件模版的缓存
 * <p>
 * KEY 格式：mail_template:{code}
 * VALUE 数据格式：String 模版信息
*/
pub const MAIL_TEMPLATE: &str = "mail_template";

/**
 * 短信模版的缓存
 * <p>
 * KEY 格式：sms_template:{id}
 * VALUE 数据格式：String 模版信息
*/
pub const SMS_TEMPLATE: &str = "sms_template";

/**
 * 小程序订阅模版的缓存
 * <p>
 * KEY 格式：wxa_subscribe_template:{userType}
 * VALUE 数据格式 String, 模版信息
*/
pub const WXA_SUBSCRIBE_TEMPLATE: &str = "wxa_subscribe_template";
