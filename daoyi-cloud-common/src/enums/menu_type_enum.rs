use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum MenuTypeEnum {
    Dir,    // 目录
    Menu,   // 菜单
    Button, // 按钮
}

impl MenuTypeEnum {
    // 获取角色编码
    pub fn code(&self) -> i8 {
        match self {
            MenuTypeEnum::Dir => 1,
            MenuTypeEnum::Menu => 2,
            MenuTypeEnum::Button => 3,
        }
    }

    // 获取角色名称
    pub fn name(&self) -> String {
        match self {
            MenuTypeEnum::Dir => "目录".to_string(),
            MenuTypeEnum::Menu => "菜单".to_string(),
            MenuTypeEnum::Button => "按钮".to_string(),
        }
    }

    // 检查是否为目录
    pub fn is_dir(code: i8) -> bool {
        code == MenuTypeEnum::Dir.code()
    }
}

// 可选：实现 Display trait 方便打印
impl fmt::Display for MenuTypeEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name(), self.code())
    }
}
