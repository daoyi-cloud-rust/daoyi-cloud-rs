#![allow(dead_code)]

pub mod common_status_enum;

use serde::{Deserialize, Serialize};

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
}
