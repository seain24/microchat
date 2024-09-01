use std::sync::Arc;

use actix_web::web::ServiceConfig;
use actix_web::{get, post, web};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::response::Result;

#[derive(Debug, Deserialize)]
pub struct UserRegisterRequest {}

#[derive(Debug, Serialize)]
pub struct UserRegisterReply {}

#[derive(Debug, Serialize)]
pub struct UserLoginReply {}

#[derive(Debug, Serialize)]
pub struct UserLogoutReply {}

#[async_trait]
pub trait UserSvc {
    async fn register(&self) -> Result<UserRegisterReply>;
    async fn login(&self) -> Result<UserLoginReply>;
    async fn logout(&self, user_id: String) -> Result<UserLogoutReply>;
}

#[post("register")]
async fn register(svc: web::Data<dyn UserSvc>) -> Result<UserRegisterReply> {
    // todo 登录逻辑
    svc.register().await
}

#[post("login")]
async fn login(svc: web::Data<dyn UserSvc>) -> Result<UserLoginReply> {
    // todo 登录逻辑
    svc.login().await
}

#[get("logout/{user_id}")]
async fn logout(svc: web::Data<dyn UserSvc>, user_id: web::Path<String>) -> Result<UserLogoutReply> {
    // todo 退出逻辑
    svc.logout(user_id.into_inner()).await
}

pub fn config(cfg: &mut ServiceConfig, svc: Arc<dyn UserSvc>) {
    cfg.service(register).service(login).service(logout);
}
