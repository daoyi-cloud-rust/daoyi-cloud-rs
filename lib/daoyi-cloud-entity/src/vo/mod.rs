pub mod common;
pub mod system;

use sea_orm::prelude::DateTime;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

#[derive(Deserialize, Debug)]
#[serde(untagged)] // 处理多种可能的输入格式
enum DateTimeVec {
    Array(Vec<String>),
    Single(String),
}

// 为 DateTime 类型实现自定义序列化
pub fn serialize_datetime<S>(dt: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", dt.format("%Y-%m-%d %H:%M:%S"));
    serializer.serialize_str(&s)
}
pub fn serialize_opt_datetime<S>(dt: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match dt {
        Some(dt) => serialize_datetime(dt, serializer),
        None => serializer.serialize_none(),
    }
}

// 自定义反序列化函数
pub fn deserialize_optional_datetime_vec<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<DateTime>>, D::Error>
where
    D: Deserializer<'de>,
{
    // 尝试解析为 DateTimeVec 枚举
    let input = match DateTimeVec::deserialize(deserializer)? {
        DateTimeVec::Array(v) => v,
        DateTimeVec::Single(s) => vec![s],
    };

    // 转换每个元素为 NaiveDateTime
    let mut result = Vec::new();
    for s in input {
        // 尝试多种可能的日期时间格式
        match DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
            Ok(dt) => result.push(dt),
            Err(_) => {
                // 尝试其他可能的格式变体
                let alt_formats = [
                    "%Y-%m-%dT%H:%M:%S",    // ISO 8601
                    "%Y-%m-%d %H:%M:%S%.f", // 带毫秒
                    "%Y-%m-%d",             // 只有日期
                ];

                let mut parsed = false;
                for fmt in &alt_formats {
                    if let Ok(dt) = DateTime::parse_from_str(&s, fmt) {
                        result.push(dt);
                        parsed = true;
                        break;
                    }
                }

                if !parsed {
                    return Err(D::Error::custom(format!(
                        "Failed to parse datetime: '{}'",
                        s
                    )));
                }
            }
        }
    }

    // 如果结果为空返回 None，否则返回 Some
    if result.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}
