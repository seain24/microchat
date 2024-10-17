use std::future::Future;

use async_trait::async_trait;

pub use sea::{EitherDb as Db, new_tx_provider, TxProvider as DefaultTxProvider};

use crate::Result;

mod sea;

#[async_trait]
pub trait Tx: Send + Sync {
    async fn commit(&self) -> Result<()>;
    async fn rollback(&self) -> Result<()>;
}

#[async_trait]
pub trait TxFactory<T: Tx>: Send + Sync {
    async fn tx(&self) -> Result<T>;
}

pub struct TxProvider<T: Tx> {
    fact: Box<dyn TxFactory<T>>,
}

impl<T: Tx> TxProvider<T> {
    pub fn new(fact: Box<dyn TxFactory<T>>) -> Self {
        TxProvider { fact }
    }
}

impl<T: Tx + Clone + 'static> TxProvider<T> {
    /// 提交事务
    pub async fn exec(&self, fut: impl Future<Output = Result<()>>) -> Result<()> {
        // 1. 生成事物
        let tx = self.fact.tx().await?;
        // 事务id放入thread local, poll work future
        let (mut ext, res) =
            task_local_extensions::with_extensions(task_local_extensions::Extensions::new().with(tx), fut)
                .await;
        // 重新获取事务， 提交或回滚
        let tx: T = ext.remove().unwrap();
        if res.is_err() {
            // 回滚
            tx.rollback().await?;
            return res;
        }
        // 提交事务
        tx.commit().await?;

        res
    }

    pub async fn fetch_tx(&self) -> Option<T> {
        task_local_extensions::get_local_item::<T>().await
    }
}
