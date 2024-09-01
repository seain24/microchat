use std::sync::Arc;

use async_trait::async_trait;

use crate::base;
use crate::interface::response::Result;
use crate::interface::user_mgr::{UserLoginReply, UserLogoutReply, UserRegisterReply, UserSvc};

pub struct UserManager {
    cfg: Arc<base::cfg::Config>,
}

impl UserManager {
    pub fn new(cfg: Arc<base::cfg::Config>) -> Arc<Self> {
        Arc::new(UserManager { cfg })
    }
}

#[async_trait]
impl UserSvc for UserManager {
    async fn register(&self) -> Result<UserRegisterReply> {
        todo!()
    }

    async fn login(&self) -> Result<UserLoginReply> {
        todo!()
    }

    async fn logout(&self, user_id: String) -> Result<UserLogoutReply> {
        todo!()
    }
}
