use crate::enums::{EnumItem, EnumItemExt};
use sea_orm::{ActiveValue, DeriveActiveEnum, EnumIter, IntoActiveValue, Iterable};
use serde::{Deserialize, Serialize};

/// 性别的枚举值
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum SexEnum {
    #[sea_orm(num_value = 0)]
    UNKNOWN,
    #[sea_orm(num_value = 1)]
    MALE,
    #[sea_orm(num_value = 2)]
    FEMALE,
}
// 实现自定义反序列化
impl<'de> Deserialize<'de> for SexEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // 先解析为 i8
        let value = i8::deserialize(deserializer)?;

        // 映射到对应枚举值
        match value {
            0 => Ok(SexEnum::UNKNOWN),
            1 => Ok(SexEnum::MALE),
            2 => Ok(SexEnum::FEMALE),
            _ => {
                // 无效值处理
                Err(serde::de::Error::custom(format!(
                    "Invalid sex value: {}. Allowed values: 0 (unknown), 1 (male), 2 (female)",
                    value
                )))
            }
        }
    }
}

impl IntoActiveValue<SexEnum> for SexEnum {
    fn into_active_value(self) -> ActiveValue<SexEnum> {
        ActiveValue::Set(self)
    }
}

impl EnumItemExt<i8> for SexEnum {
    fn item(&self) -> EnumItem<i8> {
        match self {
            SexEnum::UNKNOWN => EnumItem::new(0, "未知"),
            SexEnum::MALE => EnumItem::new(1, "男"),
            SexEnum::FEMALE => EnumItem::new(2, "女"),
        }
    }

    fn value_items() -> Vec<i8> {
        SexEnum::iter().map(|item| item.item().value).collect()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        SexEnum::iter().collect()
    }
}
