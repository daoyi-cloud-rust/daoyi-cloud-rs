use crate::enums::{EnumItem, EnumItemExt};
use sea_orm::{DeriveActiveEnum, EnumIter, Iterable};
use serde::{Deserialize, Serialize};

/// 性别的枚举值
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum SexEnum {
    #[sea_orm(num_value = 0)]
    UNKNOWN,
    #[sea_orm(num_value = 1)]
    MALE,
    #[sea_orm(num_value = 2)]
    FEMALE,
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
