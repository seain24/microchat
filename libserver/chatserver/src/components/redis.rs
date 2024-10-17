use std::sync::Arc;

use async_trait::async_trait;
use fred::prelude::*;
use fred::types::RespVersion;
use shaku::{Component, Interface};

use crate::base::config::RedisConfig;

pub async fn init_redis(redis_cfg: &RedisConfig) -> Result<RedisClient, RedisError> {
    // 初始化Redis-connection
    let config = fred::types::RedisConfig {
        fail_fast: true,
        server: ServerConfig::new_centralized(redis_cfg.host.as_str(), redis_cfg.port),
        blocking: Blocking::Block,
        username: redis_cfg.username.clone(),
        password: redis_cfg.password.clone(),
        version: RespVersion::RESP2,
        database: Some(redis_cfg.database),
    };
    let conn_config = ConnectionConfig::default();
    let perf = PerformanceConfig::default();
    let policy = ReconnectPolicy::default();
    let client = RedisClient::new(config, Some(perf), Some(conn_config), Some(policy));
    // spawn tasks that listen for connection close or reconnect events
    let mut error_rx = client.error_rx();
    let mut reconnect_rx = client.reconnect_rx();

    tokio::spawn(async move {
        while let Ok(error) = error_rx.recv().await {
            tracing::error!("Client disconnected with error: {:?}", error);
        }
    });
    tokio::spawn(async move {
        while reconnect_rx.recv().await.is_ok() {
            tracing::info!("Client reconnected");
        }
    });
    client.init().await?;
    client.quit().await?;

    tracing::info!("Init redis client successfully!");
    Ok(client)
}

pub trait IRedisService: Interface {
    fn get_conn(&self) -> Arc<RedisClient>;
}

#[derive(Component)]
#[shaku(interface = IRedisService)]
pub struct RedisServiceImpl {
    redis_cli: Arc<RedisClient>,
}

impl IRedisService for RedisServiceImpl {
    fn get_conn(&self) -> Arc<RedisClient> {
        self.redis_cli.clone()
    }
}
