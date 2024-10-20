use std::sync::Arc;

use crate::base::config::Config;
use crate::base::singleton::db as singleton;
use crate::base::transaction;

pub mod entity;
pub mod repository;

pub struct Data {
    cfg: Arc<Config>,
    db: singleton::DbPool,
    tx: Arc<transaction::DefaultTxProvider>,
}

impl Data {
    pub async fn new(cfg: Arc<Config>) -> crate::Result<Self> {
        tracing::info!("start to init database.");
        let db_instance = singleton::DbPool::get_instance(&cfg).await?;
        let tx = Arc::new(transaction::new_tx_provider(db_instance.clone()));
        Ok(Data {
            cfg,
            tx,
            db: db_instance,
        })
    }

    pub async fn db(&self) -> transaction::Db {
        if let Some(tx) = self.tx.fetch_tx().await {
            transaction::Db::Tx(tx)
        } else {
            transaction::Db::Db(&self.db)
        }
    }
}
