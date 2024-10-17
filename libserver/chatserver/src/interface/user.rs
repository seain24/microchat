use actix_web::web::ServiceConfig;
use actix_web::{get, post, web};
use serde::Serialize;
use shaku::HasComponent;
use validator::Validate;

use crate::base::response::{Error, Reply, Response};
use crate::components::get_service_factory;
use crate::service::user::{IUserService, SignInRequest, SignUpRequest, UserInfo};

#[derive(Debug, Serialize)]
pub struct SignUpReply {
    pub user: UserInfo,
}

#[derive(Debug, Default, Serialize)]
pub struct SignInReply;

#[derive(Debug, Default, Serialize)]
pub struct SignOutReply;

#[post("/signup")]
async fn sign_up(body: web::Json<SignUpRequest>) -> Reply<SignUpReply> {
    // 校验参数
    let req = body.into_inner();
    if let Err(err) = req.validate() {
        for (_, v) in err.field_errors() {
            if !v.is_empty() {
                return Err(Error::param_invalid(v.first().unwrap().code.as_ref()));
            }
        }
    }

    let modules = get_service_factory().map_err(|err| {
        tracing::error!("user signed up failed, {err:#}");
        Error::internal_server_error("用户注册失败")
    })?;
    let user_service: &dyn IUserService = modules.resolve_ref();
    let user_info = user_service.sign_up(req).await.map_err(|err| {
        tracing::error!("{err:#}");
        Error::internal_server_error("用户注册失败")
    })?;
    let reply = SignUpReply { user: user_info };
    Ok(Response::new(Some(reply), None))
}

#[post("/signin")]
async fn sign_in(body: web::Json<SignInRequest>) -> Reply<SignInReply> {
    // 校验参数
    let req = body.into_inner();
    if let Err(err) = req.validate() {
        for (_, v) in err.field_errors() {
            if !v.is_empty() {
                return Err(Error::param_invalid(v.first().unwrap().code.as_ref()));
            }
        }
    }

    let modules = get_service_factory().map_err(|err| {
        tracing::error!("user signed in failed, {err:#}");
        Error::internal_server_error("登录失败")
    })?;
    let user_service: &dyn IUserService = modules.resolve_ref();
    user_service.sign_in(req).await.map_err(|err| {
        tracing::error!("{err:#}");
        Error::internal_server_error("登录失败")
    })?;

    Ok(Response::new(Some(SignInReply::default()), Some("登录成功")))
}

#[get("/signout/{user_id}")]
async fn sign_out(user_id: web::Path<String>) -> Reply<SignOutReply> {
    if user_id.is_empty() {
        return Err(Error::param_invalid("用户id不能为空"));
    }

    let modules = get_service_factory().map_err(|err| {
        tracing::error!("user signed out failed, {err:#}");
        Error::internal_server_error("退出帐号失败")
    })?;
    let user_service: &dyn IUserService = modules.resolve_ref();
    user_service.sign_out(&user_id.into_inner()).await.map_err(|err| {
        tracing::error!("{err:#}");
        Error::internal_server_error("退出帐号失败")
    })?;

    Ok(Response::new(Some(SignOutReply::default()), Some("退出帐号成功")))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/user").service(sign_up).service(sign_in).service(sign_out));
}
