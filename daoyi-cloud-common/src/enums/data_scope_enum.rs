use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum DataScopeEnum {
    All,          // 全部数据权限
    DeptCustom,   // 指定部门数据权限
    DeptOnly,     // 部门数据权限
    DeptAndChild, // 部门及以下数据权限
    OnlySelf,     // 仅本人数据权限
}

impl DataScopeEnum {
    // 获取角色编码
    pub fn code(&self) -> i8 {
        match self {
            DataScopeEnum::All => 1,
            DataScopeEnum::DeptCustom => 2,
            DataScopeEnum::DeptOnly => 3,
            DataScopeEnum::DeptAndChild => 4,
            DataScopeEnum::OnlySelf => 5,
        }
    }

    // 获取角色名称
    pub fn name(&self) -> String {
        match self {
            DataScopeEnum::All => "全部数据权限".to_string(),
            DataScopeEnum::DeptCustom => "指定部门数据权限".to_string(),
            DataScopeEnum::DeptOnly => "部门数据权限".to_string(),
            DataScopeEnum::DeptAndChild => "部门及以下数据权限".to_string(),
            DataScopeEnum::OnlySelf => "仅本人数据权限".to_string(),
        }
    }
}

// 可选：实现 Display trait 方便打印
impl fmt::Display for DataScopeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.code())
    }
}
