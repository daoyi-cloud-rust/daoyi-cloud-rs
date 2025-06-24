use crate::enums::{EnumItem, EnumItemExt};
use sea_orm::{DeriveActiveEnum, EnumIter, Iterable};
use serde::{Deserialize, Serialize};

/// LoginLogTypeEnum（对应 Java LoginResultEnum）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "Integer")]
pub enum LoginResultEnum {
    /// 成功
    #[sea_orm(num_value = 0)]
    Success,
    /// 账号或密码不正确
    #[sea_orm(num_value = 10)]
    BadCredentials,
    /// 用户被禁用
    #[sea_orm(num_value = 20)]
    UserDisabled,
    /// 图片验证码不存在
    #[sea_orm(num_value = 30)]
    CaptchaNotFound,
    /// 图片验证码不正确
    #[sea_orm(num_value = 31)]
    CaptchaCodeError,
}

// 实现自定义反序列化
impl<'de> Deserialize<'de> for LoginResultEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // 先解析为 i8
        let value = i16::deserialize(deserializer)?;

        // 映射到对应枚举值
        match value {
            0 => Ok(LoginResultEnum::Success),
            10 => Ok(LoginResultEnum::BadCredentials),
            20 => Ok(LoginResultEnum::UserDisabled),
            30 => Ok(LoginResultEnum::CaptchaNotFound),
            31 => Ok(LoginResultEnum::CaptchaCodeError),
            _ => {
                // 无效值处理
                Err(serde::de::Error::custom(format!(
                    "Invalid LoginResult value: {}.",
                    value
                )))
            }
        }
    }
}
impl EnumItemExt<i16> for LoginResultEnum {
    fn item(&self) -> EnumItem<i16> {
        match self {
            LoginResultEnum::Success => EnumItem::new(0, "成功"),
            LoginResultEnum::BadCredentials => EnumItem::new(10, "账号或密码不正确"),
            LoginResultEnum::UserDisabled => EnumItem::new(20, "用户被禁用"),
            LoginResultEnum::CaptchaNotFound => EnumItem::new(30, "图片验证码不存在"),
            LoginResultEnum::CaptchaCodeError => EnumItem::new(31, "图片验证码不正确"),
        }
    }

    fn value_items() -> Vec<i16> {
        LoginResultEnum::iter()
            .map(|item| item.item().value)
            .collect()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        LoginResultEnum::iter().collect()
    }
}
