use std::sync::Arc;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DatabaseTransaction, DbErr};
use shaku::{Component, Interface};

use crate::base::config::DatabaseConfig;

const DEFAULT_MAX_CONNECTION: u32 = 30;
const DEFAULT_MIN_CONNECTION: u32 = 30;
const DEFAULT_TIMEOUT: Duration = std::time::Duration::from_secs(5);

pub async fn init_db_conn(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let db_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.dbname
    );
    tracing::info!("Connecting to mysql with {}", &db_url);
    let mut opt = ConnectOptions::new(&db_url);
    opt.max_connections(config.max_connection.unwrap_or(DEFAULT_MAX_CONNECTION))
        .min_connections(config.min_connection.unwrap_or(DEFAULT_MIN_CONNECTION))
        .max_lifetime(config.timeout.unwrap_or(DEFAULT_TIMEOUT))
        .connect_timeout(config.timeout.unwrap_or(DEFAULT_TIMEOUT))
        .idle_timeout(config.timeout.unwrap_or(DEFAULT_TIMEOUT))
        .sqlx_logging(false);
    Database::connect(opt).await
}

pub trait IMysqlService: Interface {
    fn get_conn(&self) -> Arc<DatabaseConnection>;
    fn get_tx(&self) -> Arc<DatabaseTransaction>;
}

#[derive(Component)]
#[shaku(interface = IMysqlService)]
pub struct MysqlServiceImpl {
    db_conn: Arc<DatabaseConnection>,
    db_tx: Arc<DatabaseTransaction>,
}

impl IMysqlService for MysqlServiceImpl {
    fn get_conn(&self) -> Arc<DatabaseConnection> {
        self.db_conn.clone()
    }

    fn get_tx(&self) -> Arc<DatabaseTransaction> {
        self.db_tx.clone()
    }
}
