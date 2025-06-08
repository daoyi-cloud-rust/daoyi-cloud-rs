use crate::enums::{EnumItem, EnumItemExt};
use serde::{Deserialize, Serialize};

/// 通用状态枚举（对应 Java CommonStatusEnum）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommonStatusEnum {
    Enable,
    Disable,
}

impl EnumItemExt<i8> for CommonStatusEnum {
    fn item(&self) -> EnumItem<i8> {
        match self {
            CommonStatusEnum::Enable => EnumItem::new(0, "启用"),
            CommonStatusEnum::Disable => EnumItem::new(1, "禁用"),
        }
    }
}
