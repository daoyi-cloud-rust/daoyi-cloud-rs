#![allow(dead_code)]

use daoyi_cloud_logger::logger;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};
use serde::Deserialize;
use std::cmp::max;
use std::sync::OnceLock;
use std::time::Duration;

static SEA_ORM_POOL_0: OnceLock<DatabaseConnection> = OnceLock::new();
static SEA_ORM_POOL_1: OnceLock<DatabaseConnection> = OnceLock::new();
static SEA_ORM_POOL_2: OnceLock<DatabaseConnection> = OnceLock::new();
static SEA_ORM_POOL_3: OnceLock<DatabaseConnection> = OnceLock::new();

#[derive(Debug, Deserialize, Default)]
pub struct DatabaseConfig {
    url: Option<String>,
    protocol: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    database: Option<String>,
    schema: Option<String>,
}

impl DatabaseConfig {
    pub fn url(&self) -> String {
        self.url.as_deref().map(String::from).unwrap_or_else(|| {
            format!(
                "{}://{}:{}@{}:{}/{}",
                self.protocol(),
                self.user(),
                self.password(),
                self.host(),
                self.port(),
                self.database()
            )
        })
    }

    pub fn protocol(&self) -> &str {
        self.protocol.as_deref().unwrap_or("postgres")
    }
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(5432)
    }
    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("postgres")
    }
    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("postgres")
    }
    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("postgres")
    }
    pub fn schema(&self) -> Option<&str> {
        self.schema.as_deref()
    }
}

async fn init_datasource(database_config: &DatabaseConfig) -> anyhow::Result<DatabaseConnection> {
    let mut options = ConnectOptions::new(database_config.url());
    let cpus = num_cpus::get() as u32;
    options
        .min_connections(max(cpus * 4, 10))
        .max_connections(max(cpus * 8, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600 * 24))
        .sqlx_logging(false);
    // 设置schema（如果存在）
    if let Some(schema) = database_config.schema() {
        options.set_schema_search_path(schema);
    }
    let db = Database::connect(options).await?;
    db.ping().await?;
    logger::debug!("Database [{}] connected.", database_config.url());
    log_database_info(&db).await?;
    Ok(db)
}

pub async fn init() -> anyhow::Result<()> {
    let config = crate::config::get();
    let database_config = config.datasource_0();
    let db = init_datasource(&database_config).await?;
    SEA_ORM_POOL_0.set(db).expect("sea-orm pool should be set");
    if let Some(database_config) = config.datasource_1() {
        let db = init_datasource(&database_config).await?;
        SEA_ORM_POOL_1.set(db).expect("sea-orm pool should be set");
    }
    if let Some(database_config) = config.datasource_2() {
        let db = init_datasource(&database_config).await?;
        SEA_ORM_POOL_2.set(db).expect("sea-orm pool should be set");
    }
    if let Some(database_config) = config.datasource_3() {
        let db = init_datasource(&database_config).await?;
        SEA_ORM_POOL_3.set(db).expect("sea-orm pool should be set");
    }
    Ok(())
}

pub fn pool0() -> &'static DatabaseConnection {
    SEA_ORM_POOL_0.get().expect("sea-orm pool should set")
}
pub fn pool1() -> &'static DatabaseConnection {
    SEA_ORM_POOL_1.get().expect("sea-orm pool should set")
}
pub fn pool2() -> &'static DatabaseConnection {
    SEA_ORM_POOL_2.get().expect("sea-orm pool should set")
}
pub fn pool3() -> &'static DatabaseConnection {
    SEA_ORM_POOL_3.get().expect("sea-orm pool should set")
}

async fn log_database_info(db: &DatabaseConnection) -> anyhow::Result<()> {
    // 获取数据库类型
    let db_type = match db.get_database_backend() {
        sea_orm::DatabaseBackend::MySql => "MySQL",
        sea_orm::DatabaseBackend::Postgres => "PostgreSQL",
        sea_orm::DatabaseBackend::Sqlite => "SQLite",
    };

    // 获取版本
    let stmt = Statement::from_string(db.get_database_backend(), "SELECT version()".to_string());

    let result = db.query_one(stmt).await?;
    let version = result
        .and_then(|row| row.try_get_by_index::<String>(0).ok())
        .unwrap_or_else(|| "unknown".to_string());

    logger::debug!("Connected to {} version: {}", db_type, version);
    Ok(())
}
