use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum RoleTypeEnum {
    System, // 内置角色
    Custom, // 自定义角色
}

impl RoleTypeEnum {
    // 获取角色编码
    pub fn code(&self) -> i8 {
        match self {
            RoleTypeEnum::System => 1,
            RoleTypeEnum::Custom => 2,
        }
    }

    // 获取角色名称
    pub fn name(&self) -> String {
        match self {
            RoleTypeEnum::System => "内置角色".to_string(),
            RoleTypeEnum::Custom => "自定义角色".to_string(),
        }
    }

    // 检查是否为目录
    pub fn is_system(code: i8) -> bool {
        code == RoleTypeEnum::System.code()
    }
}

// 可选：实现 Display trait 方便打印
impl fmt::Display for RoleTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.code())
    }
}
