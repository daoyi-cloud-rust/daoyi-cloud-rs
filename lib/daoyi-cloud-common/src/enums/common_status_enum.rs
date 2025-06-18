use crate::enums::{EnumItem, EnumItemExt};
use sea_orm::{DeriveActiveEnum, EnumIter, Iterable};
use serde::{Deserialize, Serialize};

/// 通用状态枚举（对应 Java CommonStatusEnum）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum CommonStatusEnum {
    #[sea_orm(num_value = 0)]
    Enable,
    #[sea_orm(num_value = 1)]
    Disable,
}

// 实现自定义反序列化
impl<'de> Deserialize<'de> for CommonStatusEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // 先解析为 i8
        let value = i8::deserialize(deserializer)?;

        // 映射到对应枚举值
        match value {
            0 => Ok(CommonStatusEnum::Enable),
            1 => Ok(CommonStatusEnum::Disable),
            _ => {
                // 无效值处理
                Err(serde::de::Error::custom(format!(
                    "Invalid status value: {}. Allowed values: 0 (Enable), 1 (Disable)",
                    value
                )))
            }
        }
    }
}
impl EnumItemExt<i8> for CommonStatusEnum {
    fn item(&self) -> EnumItem<i8> {
        match self {
            CommonStatusEnum::Enable => EnumItem::new(0, "启用"),
            CommonStatusEnum::Disable => EnumItem::new(1, "禁用"),
        }
    }

    fn value_items() -> Vec<i8> {
        CommonStatusEnum::iter()
            .map(|item| item.item().value)
            .collect()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        CommonStatusEnum::iter().collect()
    }
}
