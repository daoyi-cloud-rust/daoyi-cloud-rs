use regex::Regex;
use std::borrow::Cow;
use std::cell::LazyCell;
use std::collections::HashMap;
use validator::ValidationError;

const MOBILE_PHONE_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(
        r#"(?x)          # 启用注释和空白忽略模式
        ^                                # 字符串起始
        (?:                              # 分组开始（非捕获）
          1[3-9]\d{9}                   # 手机号（13-19开头的11位数字）
          |                              # 或
          $?0\d{2,3}$?              # 区号：可选的括号包裹2-3位数字（0开头）
            [\s-]?                      # 可选的分隔符（空格或短横）
            \d{7,8}                     # 本地号码：7-8位数字
            (?:[\s-]?\d{1,4})?
        )                                # 分组结束
        $                                # 字符串结尾
    "#,
    )
    .expect("手机和座机号码正则表达式解析失败")
});

pub fn is_mobile_phone(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() {
        return Ok(());
    }
    if MOBILE_PHONE_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(build_validation_error("手机号码格式错误"))
    }
}

fn build_validation_error(msg: &'static str) -> ValidationError {
    ValidationError {
        code: Cow::from("invalid"),
        message: Some(Cow::from(msg)),
        params: HashMap::new(),
    }
}
