use std::sync::{Arc, OnceLock};
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::base::cfg::Config;
use crate::{Error, Result};

const MAX_CONNECTIONS: u32 = 100;
const MIN_CONNECTIONS: u32 = 10;
const CONNECT_TIMEOUT_SEC: Duration = Duration::from_secs(10); // 10s
const CONNECT_IDLE_TIMEOUT_SEC: Duration = Duration::from_secs(8);

pub struct DbPool(pub DatabaseConnection);

static INSTANCE: OnceLock<DbPool> = OnceLock::new();

impl DbPool {
    async fn new(cfg: Arc<Config>) -> Result<Self> {
        let dburl: String = format!(
            "mysql://{}:{}@{}:{}/{}",
            cfg.database.uname,
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
        let conn = Database::connect(opt).await.map_err(|e| Error::DatabaseError(e.to_string()))?;
        Ok(DbPool(conn))
    }

    pub async fn get_instance(cfg: Arc<Config>) -> Result<Arc<Self>> {
        // static mut INSTANCE: Option<Mutex<DbPool>> = None;
        // static ONCE: Once = Once::new();
        // unsafe {
        //     ONCE.call_once(async || {
        //         let instance = DbPool::new(cfg).await?;
        //         INSTANCE = Some(Mutex::new(instance));
        //     });
        //     Ok(INSTANCE.as_ref().unwrap().clone())
        // }
        DbPool::new(cfg).await.map(|v| Arc::new(v))
    }
}
