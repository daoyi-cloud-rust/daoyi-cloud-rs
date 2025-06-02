use bcrypt::{DEFAULT_COST, hash, verify};
use std::str::FromStr;

/// 密码编码器trait
pub trait PasswordEncoder {
    /// 编码原始密码，返回哈希字符串
    fn encode(&self, raw_password: &str) -> String;

    /// 验证原始密码是否与存储的哈希匹配
    fn matches(&self, raw_password: &str, encoded_password: &str) -> bool;

    /// 检查存储的哈希是否需要升级（例如，成本因子低于当前设置）
    fn upgrade_encoding(&self, encoded_password: &str) -> bool;
}

/// BCrypt密码编码器
pub struct BCryptPasswordEncoder {
    strength: u32, // 成本因子，范围在4~31（通常使用10~12）
}

impl BCryptPasswordEncoder {
    /// 创建一个新的BCryptPasswordEncoder，使用默认的成本因子（DEFAULT_COST，目前为12）
    pub fn new() -> Self {
        Self::with_strength(DEFAULT_COST)
    }

    /// 指定成本因子创建
    pub fn with_strength(strength: u32) -> Self {
        Self { strength }
    }
}

impl Default for BCryptPasswordEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordEncoder for BCryptPasswordEncoder {
    fn encode(&self, raw_password: &str) -> String {
        // 使用bcrypt库进行哈希，使用指定的成本因子
        hash(raw_password, self.strength).expect("Failed to hash password")
    }

    fn matches(&self, raw_password: &str, encoded_password: &str) -> bool {
        // 验证密码
        verify(raw_password, encoded_password).unwrap_or(false)
    }

    fn upgrade_encoding(&self, encoded_password: &str) -> bool {
        // 尝试从哈希字符串中解析出成本因子
        // BCrypt哈希字符串格式：$2a$12$...（其中12就是成本因子）
        let parts: Vec<&str> = encoded_password.split('$').collect();
        if parts.len() < 4 {
            // 无效格式，认为需要升级
            return true;
        }

        // 解析版本和成本因子部分（例如："2a"和"12"）
        let cost_str = parts[3];
        // 成本因子是字符串的前两位（因为成本因子是两位数）
        if cost_str.len() < 2 {
            return true;
        }
        let cost = u32::from_str(&cost_str[..2]).unwrap_or(0);

        // 如果存储的成本因子小于当前设置的成本因子，则需要升级
        cost < self.strength
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcrypt_workflow() {
        let encoder = BCryptPasswordEncoder::new();
        let password = "password";

        // 测试编码
        let hashed = encoder.encode(password);
        assert!(
            hashed.starts_with("$2b$12$")
                || hashed.starts_with("$2a$12$")
                || hashed.starts_with("$2y$12$")
        );

        // 测试匹配
        assert!(encoder.matches(password, &hashed));
        assert!(!encoder.matches("wrong_password", &hashed));

        // 测试升级检查
        // 创建一个低成本的哈希（成本因子为4）
        let weak_hashed = hash("password", 4).unwrap();
        assert!(encoder.upgrade_encoding(&weak_hashed));

        // 创建一个成本因子为12的哈希，则不需要升级（如果当前成本因子为12）
        let strong_hashed = hash("password", 12).unwrap();
        assert!(!encoder.upgrade_encoding(&strong_hashed));
    }
}
