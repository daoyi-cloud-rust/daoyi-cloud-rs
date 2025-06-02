use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum RoleCodeEnum {
    SuperAdmin,  // 超级管理员
    TenantAdmin, // 租户管理员
    CrmAdmin,    // CRM 系统专用
}

impl RoleCodeEnum {
    // 获取角色编码
    pub fn code(&self) -> String {
        match self {
            RoleCodeEnum::SuperAdmin => "super_admin".to_string(),
            RoleCodeEnum::TenantAdmin => "tenant_admin".to_string(),
            RoleCodeEnum::CrmAdmin => "crm_admin".to_string(),
        }
    }

    // 获取角色名称
    pub fn name(&self) -> String {
        match self {
            RoleCodeEnum::SuperAdmin => "超级管理员".to_string(),
            RoleCodeEnum::TenantAdmin => "租户管理员".to_string(),
            RoleCodeEnum::CrmAdmin => "CRM 管理员".to_string(),
        }
    }

    // 检查是否为超级管理员
    pub fn is_super_admin(code: &str) -> bool {
        code == RoleCodeEnum::SuperAdmin.code()
    }
}

// 可选：实现 Display trait 方便打印
impl fmt::Display for RoleCodeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.code())
    }
}
