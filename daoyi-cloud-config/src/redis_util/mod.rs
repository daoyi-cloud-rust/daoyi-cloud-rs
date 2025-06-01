use crate::config::redis_config::RedisConfig;
use redis::aio::ConnectionManagerConfig;
use redis::{AsyncCommands, Client};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::OnceLock;
use std::time::Duration;

type Redis = redis::aio::ConnectionManager;
static REDIS_POOL: OnceLock<Redis> = OnceLock::new();

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
    // let result = pool().set::<&str, &str, String>("test", "test123").await;
    let result = pool()
        .set_ex::<&str, &str, String>("test", "test123", 10)
        .await;
    println!("{:?}", result);
    let result1 = pool().get::<String, String>("test".to_string()).await;
    println!("{:?}", result1);
}

fn pool() -> Redis {
    REDIS_POOL.get().expect("redis pool should set").to_owned()
}

static CACHED_REDIS_KEY: &str = "cached_keys";
async fn generate_redis_key_by_method(method_name: &str, suffix: &str) -> String {
    let key = format!("{}:{}", method_name, suffix);
    cache_keys(key.to_owned()).await;
    key
}

pub async fn clear_cached_keys() {
    let result = pool().get::<&str, String>(CACHED_REDIS_KEY).await;
    if let Ok(res_str) = result {
        let result: Result<Vec<String>, _> = serde_json::from_str(&res_str);
        if let Ok(list) = result {
            for key in list {
                let _ = pool().del::<&str, String>(key.as_str()).await;
            }
        }
    }
}

pub async fn cache_keys(key: String) {
    let result = pool().get::<&str, String>(CACHED_REDIS_KEY).await;
    if let Ok(res_str) = result {
        let result: Result<Vec<String>, _> = serde_json::from_str(&res_str);
        if let Ok(mut list) = result {
            list.push(key);
            pool()
                .set::<&str, String, String>(
                    CACHED_REDIS_KEY,
                    serde_json::to_string(&list).unwrap(),
                )
                .await
                .expect("redis set error");
            return;
        }
    }
    pool()
        .set::<&str, String, String>(
            CACHED_REDIS_KEY,
            serde_json::to_string(&Vec::<String>::new()).unwrap(),
        )
        .await
        .expect("redis set error");
}

pub async fn get_json_value<T>(redis_key: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let result = pool().get::<&str, String>(redis_key).await;
    if let Ok(res_str) = result {
        let result = serde_json::from_str::<T>(&res_str);
        if let Ok(list) = result {
            return Some(list);
        }
    }
    None
}

pub async fn get_method_cached<T>(method_name: &str, suffix: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    let redis_key = generate_redis_key_by_method(method_name, suffix).await;
    get_json_value::<T>(&redis_key).await
}

pub async fn set_json_value<T>(redis_key: &str, seconds: Option<u64>, value: &T)
where
    T: Serialize,
{
    pool()
        .set_ex::<&str, String, String>(
            redis_key,
            serde_json::to_string(value).unwrap(),
            seconds.unwrap_or(60 * 60 * 24 * 30 * 12), // 1 year
        )
        .await
        .expect("redis set error");
}

pub async fn set_method_cache<T>(method_name: &str, suffix: &str, seconds: Option<u64>, value: &T)
where
    T: Serialize,
{
    let redis_key = generate_redis_key_by_method(method_name, suffix).await;
    set_json_value::<T>(&redis_key, seconds, value).await;
}
