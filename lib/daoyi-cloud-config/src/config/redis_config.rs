use daoyi_cloud_logger::logger;
use redis::aio::ConnectionManagerConfig;
use redis::{AsyncCommands, Client};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Duration;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct RedisConfig {
    /// The URI for connecting to the Redis server. For example:
    /// <redis://127.0.0.1/>
    pub uri: String,

    /// The new connection will time out operations after `response_timeout` has passed.
    pub response_timeout: Option<u64>,

    /// Each connection attempt to the server will time out after `connection_timeout`.
    pub connection_timeout: Option<u64>,

    /// number_of_retries times, with an exponentially increasing delay
    pub number_of_retries: Option<usize>,

    /// The resulting duration is calculated by taking the base to the `n`-th power,
    /// where `n` denotes the number of past attempts.
    pub exponent_base: Option<u64>,

    /// A multiplicative factor that will be applied to the retry delay.
    ///
    /// For example, using a factor of `1000` will make each delay in units of seconds.
    pub factor: Option<u64>,

    /// Apply a maximum delay between connection attempts. The delay between attempts won't be longer than max_delay milliseconds.
    pub max_delay: Option<u64>,
}

type Redis = redis::aio::ConnectionManager;
static REDIS_POOL: OnceLock<Redis> = OnceLock::new();
static CACHED_REDIS_KEY: &str = "cached_keys";
pub struct RedisUtils;

impl RedisUtils {
    async fn init(config: &RedisConfig) -> anyhow::Result<()> {
        let url = config.uri.to_owned();
        let client = Client::open(url.clone())?;

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
            conn_config =
                conn_config.set_connection_timeout(Duration::from_millis(connection_timeout));
        }

        let connection_manager = client
            .get_connection_manager_with_config(conn_config)
            .await?;
        match REDIS_POOL.set(connection_manager) {
            Ok(x) => x,
            Err(_) => panic!("redis init failed:{}", url.clone()),
        };
        // let result = pool().set::<&str, &str, String>("test", "test123").await;
        let result = Self::pool()
            .set_ex::<&str, &str, String>("test", "test123", 10)
            .await;
        logger::debug!("{:?}", result);
        let result1 = Self::pool().get::<String, String>("test".to_string()).await;
        logger::debug!("{:?}", result1);
        Ok(())
    }

    fn pool() -> Redis {
        REDIS_POOL.get().expect("redis pool should set").to_owned()
    }

    async fn generate_redis_key_by_method(method_name: &str, suffix: &str) -> String {
        let key = format!("{}:{}", method_name, suffix);
        Self::cache_keys(key.to_owned()).await;
        key
    }

    pub async fn clear_cached_key(del_key: &str) {
        let res = Self::pool().del::<&str, String>(del_key).await;
        if let Ok(_) = res {
            let result = Self::pool().get::<&str, String>(CACHED_REDIS_KEY).await;
            if let Ok(res_str) = result {
                let result: Result<Vec<String>, _> = serde_json::from_str(&res_str);
                if let Ok(mut list) = result {
                    let index = list.iter().position(|x| x == del_key);
                    if let Some(posi) = index {
                        list.remove(posi);
                        Self::pool()
                            .set::<&str, String, String>(
                                CACHED_REDIS_KEY,
                                serde_json::to_string(&list).unwrap(),
                            )
                            .await
                            .expect("redis set error");
                    }
                }
            }
        }
    }

    pub async fn clear_all_cached_keys() {
        let result = Self::pool().get::<&str, String>(CACHED_REDIS_KEY).await;
        if let Ok(res_str) = result {
            let result: Result<Vec<String>, _> = serde_json::from_str(&res_str);
            if let Ok(list) = result {
                for key in list {
                    Self::clear_cached_key(key.as_str()).await;
                }
            }
        }
    }

    pub async fn clear_cache_by_prefix(prefix: &str) {
        let result = Self::pool().get::<&str, String>(CACHED_REDIS_KEY).await;
        if let Ok(res_str) = result {
            let result: Result<Vec<String>, _> = serde_json::from_str(&res_str);
            if let Ok(list) = result {
                for key in list {
                    if key.starts_with(prefix) {
                        Self::clear_cached_key(key.as_str()).await;
                    }
                }
            }
        }
    }

    async fn cache_keys(key: String) {
        let result = Self::pool().get::<&str, String>(CACHED_REDIS_KEY).await;
        if let Ok(res_str) = result {
            let result: Result<Vec<String>, _> = serde_json::from_str(&res_str);
            if let Ok(mut list) = result {
                list.push(key);
                Self::pool()
                    .set::<&str, String, String>(
                        CACHED_REDIS_KEY,
                        serde_json::to_string(&list).unwrap(),
                    )
                    .await
                    .expect("redis set error");
                return;
            }
        }
        Self::pool()
            .set::<&str, String, String>(
                CACHED_REDIS_KEY,
                serde_json::to_string(&Vec::<String>::new()).unwrap(),
            )
            .await
            .expect("redis set error");
    }

    async fn get_json_value<T>(redis_key: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let result = Self::pool().get::<&str, String>(redis_key).await;
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
        let redis_key = Self::generate_redis_key_by_method(method_name, suffix).await;
        Self::get_json_value::<T>(&redis_key).await
    }

    async fn set_json_value<T>(redis_key: &str, seconds: Option<u64>, value: &T)
    where
        T: Serialize,
    {
        Self::pool()
            .set_ex::<&str, String, String>(
                redis_key,
                serde_json::to_string(value).unwrap(),
                seconds.unwrap_or(60 * 60 * 24 * 30 * 12), // 1 year
            )
            .await
            .expect("redis set error");
    }

    pub async fn set_method_cache<T>(
        method_name: &str,
        suffix: &str,
        seconds: Option<u64>,
        value: &T,
    ) where
        T: Serialize,
    {
        let redis_key = Self::generate_redis_key_by_method(method_name, suffix).await;
        Self::set_json_value::<T>(&redis_key, seconds, value).await;
    }
}

pub async fn init() -> anyhow::Result<()> {
    let config = crate::config::get();
    RedisUtils::init(config.redis()).await?;
    Ok(())
}
