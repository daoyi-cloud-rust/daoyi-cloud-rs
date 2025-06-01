pub mod permission_service;
pub mod system_oauth2_access_token_service;
pub mod system_users_service;

pub fn generate_redis_key_by_table_name(table_name: &str, suffix: &str) -> String {
    format!("{}:{}", table_name, suffix)
}
