#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonStatusEnum {
    Enable,
    Disable,
}

impl CommonStatusEnum {
    // 获取状态值
    pub fn status(&self) -> u8 {
        match self {
            Self::Enable => 0,
            Self::Disable => 1,
        }
    }

    // 获取状态名称
    pub fn name(&self) -> &'static str {
        match self {
            Self::Enable => "开启",
            Self::Disable => "关闭",
        }
    }

    // 静态方法：判断是否为 Enable
    pub fn is_enable(status: u8) -> bool {
        status == CommonStatusEnum::Enable.status()
    }

    // 静态方法：判断是否为 Disable
    pub fn is_disable(status: u8) -> bool {
        status == CommonStatusEnum::Disable.status()
    }

    // 返回所有状态值数组
    pub fn array() -> Vec<u8> {
        vec![Self::Enable.status(), Self::Disable.status()]
    }

    // 返回所有枚举实例，可用于遍历等操作
    pub fn values() -> Vec<CommonStatusEnum> {
        vec![Self::Enable, Self::Disable]
    }
}
