use std::sync::Arc;

use crate::base::singleton::db::DbPool;

pub struct AppState {
    pub config: Arc<super::cfg::Config>,
    pub conn: Arc<DbPool>,
}

impl AppState {
    pub fn new(config: Arc<super::cfg::Config>, conn: Arc<DbPool>) -> Self {
        AppState { config, conn }
    }
}
