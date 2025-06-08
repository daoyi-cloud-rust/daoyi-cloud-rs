#![allow(dead_code)]

pub mod common_status_enum;

use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumItem<T> {
    pub name: &'static str,
    pub value: T,
}

impl<T> EnumItem<T> {
    pub fn new(value: T, name: &'static str) -> Self {
        Self { name, value }
    }
}

pub trait EnumItemExt<T> {
    fn item(&self) -> EnumItem<T>;
    /// 获取枚举值
    fn value(&self) -> T {
        self.item().value
    }

    /// 获取枚举名称
    fn name(&self) -> &'static str {
        self.item().name
    }

    fn value_items() -> Vec<T>;

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
