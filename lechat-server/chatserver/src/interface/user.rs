use actix_web::web::ServiceConfig;
use actix_web::{get, post, web};
use serde::Serialize;
use shaku::HasComponent;
use validator::Validate;

use crate::base::response::{Error, Reply, Response};
use crate::service;
use crate::service::user::{IUserService, SignInRequest, SignUpRequest, UserInfo};

#[derive(Debug, Serialize)]
pub struct SignUpReply {
    pub user: UserInfo,
}

#[derive(Debug, Default, Serialize)]
pub struct SignInReply;

#[derive(Debug, Default, Serialize)]
pub struct SignOutReply;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/user").service(sign_up).service(sign_in).service(sign_out));
}

#[post("/signup")]
async fn sign_up(body: web::Json<SignUpRequest>) -> Reply<SignUpReply> {
    // 校验参数
    let req = body.into_inner();
    if let Err(err) = req.validate() {
        for (_, v) in err.field_errors() {
            if v.is_empty() {
                continue;
            }

            if let Some(msg) = v.first().unwrap().message.as_ref() {
                return Err(Error::ParamInvalid(msg.to_string()));
            }
            return Err(Error::ParamInvalid("参数不合法".to_string()));
        }
    }

    let modules = service::service_factory()?;
    let user_service: &dyn IUserService = modules.resolve_ref();
    let user_info = user_service.sign_up(req).await.map_err(|err| {
        tracing::error!("{err:#}");
        Error::InternalServerError
    })?;
    let reply = SignUpReply { user: user_info };
    Ok(Response::ok(reply))
}

#[post("/signin")]
async fn sign_in(body: web::Json<SignInRequest>) -> Reply<SignInReply> {
    // 校验参数
    let req = body.into_inner();
    if let Err(err) = req.validate() {
        for (_, v) in err.field_errors() {
            if v.is_empty() {
                continue;
            }

            if let Some(msg) = v.first().unwrap().message.as_ref() {
                return Err(Error::ParamInvalid(msg.to_string()));
            }
            return Err(Error::ParamInvalid("参数不合法".to_string()));
        }
    }

    let modules = service::service_factory()?;
    let user_service: &dyn IUserService = modules.resolve_ref();
    user_service.sign_in(req).await.map_err(|err| {
        tracing::error!("{err:#}");
        Error::InternalServerError
    })?;

    Ok(Response::ok(SignInReply::default()))
}

#[get("/signout/{user_id}")]
async fn sign_out(user_id: web::Path<String>) -> Reply<SignOutReply> {
    if user_id.is_empty() {
        return Err(Error::ParamInvalid("用户名不能为空".to_string()));
    }
    let modules = service::service_factory()?;
    let user_service: &dyn IUserService = modules.resolve_ref();
    user_service.sign_out(&user_id.into_inner()).await.map_err(|err| {
        tracing::error!("{err:#}");
        Error::InternalServerError
    })?;

    Ok(Response::ok(SignOutReply::default()))
}

// async fn find_friend(cond: web::Json<FindFriendRequest>) -> Reply<> {
//
// }
