use std::sync::Arc;

use async_trait::async_trait;
use fred::error::RedisErrorKind;
use fred::prelude::{HashesInterface, RedisResult};
use fred::types::RedisMap;
use library::utils;
use shaku::{Component, Interface};

use crate::components::redis::IRedisService;

#[async_trait]
pub trait ICheckService: Interface {
    async fn is_duplicate(&self, key: &str, field: &str) -> RedisResult<bool>;
}

#[derive(Component)]
#[shaku(interface = ICheckService)]
pub struct CheckServiceImpl {
    pub redis_cli: Arc<dyn IRedisService>,
}

#[async_trait]
impl ICheckService for CheckServiceImpl {
    async fn is_duplicate(&self, key: &str, field: &str) -> RedisResult<bool> {
        let redis_cli = self.redis_cli.get_conn();
        let value = redis_cli.hexists::<u128, &str, &str>(key, field).await;
        match value {
            Ok(v) => {
                tracing::info!("{} is in the {}, value is {}", field, key, v);
                Ok(true)
            }
            Err(err) => {
                tracing::error!("find {} in {} failed, {err:#}", field, key);
                match err.kind() {
                    RedisErrorKind::NotFound => {
                        // (key, field, value)存储到redis中
                        let now = utils::time::now_timestamp_nanos();
                        let field = RedisMap::try_from(vec![(field, now)])?;
                        redis_cli.hset::<u128, &str, RedisMap>(key, field).await?;
                        Ok(false)
                    }
                    _ => Err(err),
                }
            }
        }
    }
}
