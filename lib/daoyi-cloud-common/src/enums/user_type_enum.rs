use crate::enums::{EnumItem, EnumItemExt};
use sea_orm::{ActiveValue, DeriveActiveEnum, EnumIter, IntoActiveValue, Iterable};
use serde::{Deserialize, Serialize};

/// 全局用户类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i8", db_type = "Integer")]
pub enum UserTypeEnum {
    #[sea_orm(num_value = 1)]
    MEMBER,
    #[sea_orm(num_value = 2)]
    ADMIN,
}
// 实现自定义反序列化
impl<'de> Deserialize<'de> for UserTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // 先解析为 i8
        let value = i8::deserialize(deserializer)?;

        // 映射到对应枚举值
        match value {
            1 => Ok(UserTypeEnum::MEMBER),
            2 => Ok(UserTypeEnum::ADMIN),
            _ => {
                // 无效值处理
                Err(serde::de::Error::custom(format!(
                    "Invalid UserType value: {}.",
                    value
                )))
            }
        }
    }
}

impl IntoActiveValue<UserTypeEnum> for UserTypeEnum {
    fn into_active_value(self) -> ActiveValue<UserTypeEnum> {
        ActiveValue::Set(self)
    }
}

impl EnumItemExt<i8> for UserTypeEnum {
    fn item(&self) -> EnumItem<i8> {
        match self {
            UserTypeEnum::MEMBER => EnumItem::new(1, "会员").ext("app@"),
            UserTypeEnum::ADMIN => EnumItem::new(2, "管理员").ext("admin@"),
        }
    }

    fn value_items() -> Vec<i8> {
        UserTypeEnum::iter().map(|item| item.item().value).collect()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        UserTypeEnum::iter().collect()
    }
}
