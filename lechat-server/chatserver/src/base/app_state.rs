use std::sync::Arc;

use crate::db::Data;

pub struct AppState {
    pub config: Arc<super::config::Config>,
    pub data: Arc<Data>,
}

impl AppState {
    pub fn new(config: Arc<super::config::Config>, data: Arc<Data>) -> Self {
        AppState { config, data }
    }
}
