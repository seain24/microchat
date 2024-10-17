use std::sync::Arc;

use async_trait::async_trait;
use fred::prelude::{HashesInterface, KeysInterface, RedisResult};
use shaku::{Component, Interface};

use crate::components::redis::IRedisService;

#[async_trait]
pub trait ICheckService: Interface {
    async fn is_duplicate(&self, key: &str, field: &str) -> bool;
}

#[derive(Component)]
#[shaku(interface = ICheckService)]
pub struct CheckServiceImpl {
    pub redis_cli: Arc<dyn IRedisService>,
}

#[async_trait]
impl ICheckService for CheckServiceImpl {
    async fn is_duplicate(&self, key: &str, field: &str) -> bool {
        let redis_cli = self.redis_cli.get_conn();
        let value = redis_cli
            .hexists::<u32, String, String>(key.to_string(), field.to_string())
            .await;
        match value {
            Ok(_) => true,
            Err(err) => {
                tracing::error!("field {} is not in the {}, {err:#}", field, key);
                // (key, field)存储到redis中
                let t = redis_cli.hset(key.to_string(), field.to_string()).await;
                false
            }
        }
    }
}
