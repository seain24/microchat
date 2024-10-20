use std::sync::Arc;

use crate::base::response::{Error, Result};
use crate::components::{get_service_factory, Modules};

pub mod checker;
pub mod user;

#[inline]
pub fn service_factory() -> Result<Arc<Modules>> {
    get_service_factory().map_err(|err| {
        tracing::error!("get service factory failed {err:#}");
        Error::InternalServerError
    })
}
