use std::sync::{Arc, OnceLock};
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{Error, Result};
use crate::base::config::Config;

const MAX_CONNECTIONS: u32 = 100;
const MIN_CONNECTIONS: u32 = 10;
const CONNECT_TIMEOUT_SEC: Duration = Duration::from_secs(10); // 10s
const CONNECT_IDLE_TIMEOUT_SEC: Duration = Duration::from_secs(8);

#[derive(Clone)]
pub struct DbPool(pub Arc<DatabaseConnection>);

static INSTANCE: OnceLock<DbPool> = OnceLock::new();

impl DbPool {
    async fn new(cfg: &Config) -> Result<Self> {
        let dburl: String = format!(
            "mysql://{}:{}@{}:{}/{}",
            cfg.database.username,
            cfg.database.password,
            &cfg.database.host,
            &cfg.database.port,
            cfg.database.dbname
        );
        tracing::info!("connect to database, url: {}", dburl);
        let mut opt = ConnectOptions::new(dburl);
        opt.max_connections(MAX_CONNECTIONS)
            .min_connections(MIN_CONNECTIONS)
            .connect_timeout(cfg.database.timeout.unwrap_or(CONNECT_TIMEOUT_SEC))
            .idle_timeout(CONNECT_IDLE_TIMEOUT_SEC)
            .max_lifetime(CONNECT_IDLE_TIMEOUT_SEC)
            .acquire_timeout(CONNECT_IDLE_TIMEOUT_SEC)
            .sqlx_logging(false)
            .sqlx_logging_level(tracing::log::LevelFilter::Info);
        let conn = Arc::new(Database::connect(opt).await.map_err(|e| Error::DatabaseError(e))?);
        Ok(DbPool(conn))
    }

    pub async fn get_instance(cfg: &Config) -> Result<Self> {
        // static mut INSTANCE: Option<Mutex<DbPool>> = None;
        // static ONCE = std::sync::Once::new();
        // unsafe {
        //     ONCE.call_once(async || {
        //         let instance = DbPool::new(cfg).await?;
        //         INSTANCE = Some(Mutex::new(instance));
        //     });
        //     Ok(INSTANCE.as_ref().unwrap().clone())
        // }
        match INSTANCE.get() {
            None => {
                let instance = DbPool::new(cfg).await?;
                _ = INSTANCE.set(instance.clone());
                Ok(instance)
            }
            Some(v) => Ok(v.clone()),
        }
    }
}
