#![allow(dead_code)]

pub mod common_status_enum;
pub mod login_log_type_enum;
pub mod login_result_enum;
pub mod sex_enum;
pub mod social_type_enum;
pub mod user_type_enum;

use serde::{Deserialize, Serialize, Serializer};
use validator::ValidationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumItem<T> {
    pub name: &'static str,
    pub value: T,
    pub ext: Option<&'static str>,
}

impl<T> EnumItem<T> {
    pub fn new(value: T, name: &'static str) -> Self {
        Self {
            name,
            value,
            ext: None,
        }
    }

    pub fn ext(mut self, ext_info: &'static str) -> Self {
        self.ext = Some(ext_info);
        self
    }
}

pub trait EnumItemExt<T> {
    fn item(&self) -> EnumItem<T>;
    /// 获取枚举值
    fn value(&self) -> T {
        self.item().value
    }

    fn ext(&self) -> Option<&'static str> {
        self.item().ext
    }

    /// 获取枚举名称
    fn name(&self) -> &'static str {
        self.item().name
    }

    fn value_items() -> Vec<T>;
    fn items() -> Vec<Self>
    where
        Self: Sized;
    fn by_value(value: T) -> Option<Self>
    where
        T: PartialEq,
        Self: Sized,
    {
        Self::items().into_iter().find(|item| item.value() == value)
    }

    fn validate_option_value(value: Option<T>) -> Result<(), ValidationError>
    where
        T: PartialEq,
    {
        if value.is_none() {
            return Ok(());
        }
        Self::validate_value(value.unwrap())
    }
    fn validate_value(value: T) -> Result<(), ValidationError>
    where
        T: PartialEq,
    {
        if Self::value_items().contains(&value) {
            return Ok(());
        }
        Err(ValidationError::new("数据不合法."))
    }
}

// 通用枚举序列化工具
pub fn serialize_enum<S, T, E>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: EnumItemExt<E>,
    T: Serialize,
    E: Serialize,
{
    EnumItem {
        name: value.name(),
        value: value.value(),
        ext: value.ext(),
    }
    .serialize(serializer)
}
pub fn serialize_opt_enum<S, T, E>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: EnumItemExt<E>,
    T: Serialize,
    E: Serialize,
{
    match value {
        None => serializer.serialize_none(),
        Some(v) => serialize_enum(v, serializer),
    }
}
