use crate::config::redis_config::RedisConfig;
use redis::aio::ConnectionManagerConfig;
use redis::{AsyncCommands, Client};
use std::sync::OnceLock;
use std::time::Duration;

pub type Redis = redis::aio::ConnectionManager;
pub static REDIS_POOL: OnceLock<Redis> = OnceLock::new();

pub async fn init(config: &RedisConfig) {
    let url = config.uri.to_owned();
    let client =
        Client::open(url.clone()).expect(format!("redis connect failed:{}", url.clone()).as_str());

    let mut conn_config = ConnectionManagerConfig::new();

    if let Some(exponent_base) = config.exponent_base {
        conn_config = conn_config.set_exponent_base(exponent_base);
    }
    if let Some(factor) = config.factor {
        conn_config = conn_config.set_factor(factor);
    }
    if let Some(number_of_retries) = config.number_of_retries {
        conn_config = conn_config.set_number_of_retries(number_of_retries);
    }
    if let Some(max_delay) = config.max_delay {
        conn_config = conn_config.set_max_delay(max_delay);
    }
    if let Some(response_timeout) = config.response_timeout {
        conn_config = conn_config.set_response_timeout(Duration::from_millis(response_timeout));
    }
    if let Some(connection_timeout) = config.connection_timeout {
        conn_config = conn_config.set_connection_timeout(Duration::from_millis(connection_timeout));
    }

    let connection_manager = client
        .get_connection_manager_with_config(conn_config)
        .await
        .expect(format!("redis connect failed:{}", url.clone()).as_str());
    match REDIS_POOL.set(connection_manager) {
        Ok(x) => x,
        Err(_) => panic!("redis init failed:{}", url.clone()),
    };
    let result = pool().set::<&str, &str, String>("test", "test123").await;
    println!("{:?}", result);
    let result1 = pool().get::<String, String>("test".to_string()).await;
    println!("{:?}", result1);
}

pub fn pool() -> Redis {
    REDIS_POOL.get().expect("redis pool should set").to_owned()
}
