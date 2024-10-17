use std::sync::Arc;

use fred::prelude::RedisClient;
use once_cell::sync::OnceCell;
use sea_orm::{DatabaseConnection, DatabaseTransaction};

use crate::components::mysql::{MysqlServiceImpl, MysqlServiceImplParameters};
use crate::components::redis::{RedisServiceImpl, RedisServiceImplParameters};
use crate::db::repository::user::UserRepositoryImpl;
use crate::service::user::UserServiceImpl;

pub mod mysql;
pub mod redis;

static COMPONENT_FACTORY: OnceCell<Arc<Modules>> = OnceCell::new();

shaku::module! {
    pub Modules {
        components = [
            // basic components
            RedisServiceImpl,
            MysqlServiceImpl,

            // biz components
            UserRepositoryImpl,
            UserServiceImpl,
        ],
        providers = []
    }
}

/// 注册服务组件
pub async fn register_components(
    db_conn: Arc<DatabaseConnection>,
    db_tx: Arc<DatabaseTransaction>,
    redis_cli: Arc<RedisClient>,
) -> anyhow::Result<Arc<Modules>> {
    let modules = Modules::builder()
        .with_component_parameters::<MysqlServiceImpl>(MysqlServiceImplParameters { db_conn, db_tx })
        .with_component_parameters::<RedisServiceImpl>(RedisServiceImplParameters { redis_cli })
        .build();

    let res = Arc::new(modules);
    COMPONENT_FACTORY
        .set(res.clone())
        .map_err(|_| anyhow::Error::msg("component init error..."))?;
    tracing::info!("Init component factory success!");

    Ok(res)
}

/// 获取服务组件工厂
pub fn get_service_factory() -> anyhow::Result<Arc<Modules>> {
    COMPONENT_FACTORY
        .get()
        .map_or(Err(anyhow::anyhow!("components not init")), |modules| {
            Ok(modules.clone())
        })
}
