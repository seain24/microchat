use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use sea_orm::{DbBackend, DbErr, ExecResult, QueryResult, Statement, TransactionTrait};

use crate::base::singleton::db as singleton;
use crate::Error;

/// transaction provider for sea-orm.
pub type TxProvider = super::TxProvider<SeaTx>;

/// Transaction for sea-orm which can be cloned
#[derive(Clone)]
pub struct SeaTx {
    tx: Arc<Mutex<Option<sea_orm::DatabaseTransaction>>>,
}

impl SeaTx {
    pub fn new(tx: sea_orm::DatabaseTransaction) -> Self {
        SeaTx {
            tx: Arc::new(Mutex::new(Some(tx))),
        }
    }

    pub fn tx(&self) -> &sea_orm::DatabaseTransaction {
        let ptr = self.tx.lock().unwrap().as_ref().unwrap() as *const sea_orm::DatabaseTransaction;
        unsafe { &*ptr }
    }
}

#[async_trait]
impl super::Tx for SeaTx {
    async fn commit(&self) -> crate::Result<()> {
        let tx = self.tx.lock().unwrap().take().unwrap();
        Ok(tx.commit().await.map_err(|e| Error::DatabaseError(e))?)
    }

    async fn rollback(&self) -> crate::Result<()> {
        let tx = self.tx.lock().unwrap().take().unwrap();
        Ok(tx.rollback().await.map_err(|e| Error::DatabaseError(e))?)
    }
}

/// DB instance of sea-orm
/// wrap transaction or connection,
/// and implemented `sea_orm::ConnectionTrait`
pub enum EitherDb<'a> {
    Db(&'a singleton::DbPool),
    Tx(SeaTx),
}

#[allow(unused_variables)]
#[async_trait::async_trait]
impl<'a> sea_orm::ConnectionTrait for EitherDb<'a> {
    fn get_database_backend(&self) -> DbBackend {
        match self {
            EitherDb::Db(db) => db.0.get_database_backend(),
            EitherDb::Tx(tx) => tx.tx().get_database_backend(),
        }
    }

    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        todo!()
    }

    async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
        todo!()
    }


    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        todo!()
    }

    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        todo!()
    }
}

/// Transaction factory for sea-orm.
struct TxFactory {
    data: singleton::DbPool,
}

impl TxFactory {
    fn new(data: singleton::DbPool) -> Self {
        TxFactory { data }
    }
}

#[async_trait]
impl super::TxFactory<SeaTx> for TxFactory {
    async fn tx(&self) -> crate::Result<SeaTx> {
        Ok(SeaTx::new(
            self.data.0.begin().await.map_err(|e| Error::DatabaseError(e))?,
        ))
    }
}

pub fn new_tx_provider(data: singleton::DbPool) -> TxProvider {
    TxProvider::new(Box::new(TxFactory::new(data)))
}
