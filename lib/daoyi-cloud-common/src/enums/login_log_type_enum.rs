use crate::enums::{EnumItem, EnumItemExt};
use sea_orm::{DeriveActiveEnum, EnumIter, Iterable};
use serde::{Deserialize, Serialize};

/// 登录日志的类型枚举（对应 Java LoginLogTypeEnum）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i16", db_type = "Integer")]
pub enum LoginLogTypeEnum {
    /// 使用账号登录
    #[sea_orm(num_value = 100)]
    LoginUsername,
    /// 使用社交登录
    #[sea_orm(num_value = 101)]
    LoginSocial,
    /// 使用手机登陆
    #[sea_orm(num_value = 103)]
    LoginMobile,
    /// 使用短信登陆
    #[sea_orm(num_value = 104)]
    LoginSms,
    /// 使用邮箱登陆
    #[sea_orm(num_value = 105)]
    LoginEmail,
    /// 自己主动登出
    #[sea_orm(num_value = 200)]
    LogoutSelf,
    /// 强制退出
    #[sea_orm(num_value = 202)]
    LogoutDelete,
}

// 实现自定义反序列化
impl<'de> Deserialize<'de> for LoginLogTypeEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // 先解析为 i8
        let value = i16::deserialize(deserializer)?;

        // 映射到对应枚举值
        match value {
            101 => Ok(LoginLogTypeEnum::LoginUsername),
            103 => Ok(LoginLogTypeEnum::LoginMobile),
            104 => Ok(LoginLogTypeEnum::LoginSms),
            105 => Ok(LoginLogTypeEnum::LoginEmail),
            100 => Ok(LoginLogTypeEnum::LoginSocial),
            200 => Ok(LoginLogTypeEnum::LogoutSelf),
            202 => Ok(LoginLogTypeEnum::LogoutDelete),
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
impl EnumItemExt<i16> for LoginLogTypeEnum {
    fn item(&self) -> EnumItem<i16> {
        match self {
            LoginLogTypeEnum::LoginUsername => EnumItem::new(100, "使用账号登录"),
            LoginLogTypeEnum::LoginSocial => EnumItem::new(101, "使用社交登录"),
            LoginLogTypeEnum::LoginMobile => EnumItem::new(103, "使用手机登陆"),
            LoginLogTypeEnum::LoginSms => EnumItem::new(104, "使用短信登陆"),
            LoginLogTypeEnum::LoginEmail => EnumItem::new(105, "使用邮箱登陆"),
            LoginLogTypeEnum::LogoutSelf => EnumItem::new(200, "自己主动登出"),
            LoginLogTypeEnum::LogoutDelete => EnumItem::new(202, "强制退出"),
        }
    }

    fn value_items() -> Vec<i16> {
        LoginLogTypeEnum::iter()
            .map(|item| item.item().value)
            .collect()
    }

    fn items() -> Vec<Self>
    where
        Self: Sized,
    {
        LoginLogTypeEnum::iter().collect()
    }
}
