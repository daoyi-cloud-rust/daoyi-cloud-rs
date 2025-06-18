pub mod common;
pub mod system;

use sea_orm::prelude::DateTime;
use serde::de::{self, Error, Visitor};
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
                    return Err(Error::custom(format!("Failed to parse datetime: '{}'", s)));
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

/// 自定义反序列化器：将整数数组转换为逗号分隔的字符串
pub fn deserialize_optional_id_vec<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    // 定义中间类型，处理可能的null和空数组情况
    struct OptionalIdVecVisitor;

    impl<'de> Visitor<'de> for OptionalIdVecVisitor {
        type Value = Option<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("optional array of integers or null")
        }

        // 处理null值
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        // 处理Unit类型（某些序列化器用这种形式表示null）
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        // 处理空数组
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            if let Some(0) = seq.size_hint() {
                return Ok(None); // 空数组返回None
            }

            let mut ids = Vec::new();
            while let Some(id) = seq.next_element::<i64>()? {
                ids.push(id.to_string());
            }

            if ids.is_empty() {
                Ok(None)
            } else {
                // 使用迭代器连接避免临时分配
                let result = ids.join(",");
                Ok(Some(result))
            }
        }
    }

    // 使用该访问器进行反序列化
    deserializer.deserialize_any(OptionalIdVecVisitor)
}
