use sea_orm::prelude::DateTime;
use serde::Serializer;

pub static DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// 手机号脱敏函数
pub fn mask_phone<S>(phone: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if phone.len() < 7 {
        return serializer.serialize_str(phone);
    }
    let masked = format!("{}****{}", &phone[0..3], &phone[7..]);
    serializer.serialize_str(&masked)
}

pub fn mask_email<S>(email: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return serializer.serialize_str(email);
    }

    let (username, domain) = (parts[0], parts[1]);
    let masked_username = if username.len() <= 4 {
        // 短用户名全替换
        "*".repeat(username.len())
    } else {
        // 保留前2字符，后保留@前最后一个字符
        let visible = 2;
        let end_char = &username[username.len() - 1..];
        format!("{}*{}", &username[0..visible], end_char)
    };

    let masked = format!("{}@{}", masked_username, domain);
    serializer.serialize_str(&masked)
}

pub fn mask_username<S>(username: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let masked_username = if username.len() <= 4 {
        // 短用户名全替换
        "*".repeat(username.len())
    } else {
        // 保留前1字符，最后一个字符
        let visible = 1;
        let end_char = &username[username.len() - 1..];
        format!("{}*{}", &username[0..visible], end_char)
    };

    serializer.serialize_str(&masked_username)
}

pub fn serializer_datetime_opt<S>(date: &Option<DateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if date.is_none() {
        return serializer.serialize_none();
    }
    let formatted = date.unwrap().format(DATE_TIME_FORMAT); // 可动态修改格式[3](@ref)
    serializer.serialize_str(&formatted.to_string())
}

pub fn serializer_datetime<S>(date: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer_datetime_opt(&Some(date.clone()), serializer)
}
