use std::sync::Arc;

use actix_web::{get, post, web};
use actix_web::web::ServiceConfig;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::base::response::{Reply, Result};
use crate::network::stubs::user as net;

const MOBILE_PHONE_PATTERN: &str =
    "/^1(3[0-9]|4[01456879]|5[0-35-9]|6[2567]|7[0-8]|8[0-9]|9[0-35-9])\\d{8}$/";
static MOBILE_PHONE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(MOBILE_PHONE_PATTERN).unwrap());
const PASSWORD_PATTERN: &str = r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d@$!%*#?&]{8,}$";
static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(PASSWORD_PATTERN).unwrap());

#[repr(u8)]
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    #[default]
    Unknown,
}

#[derive(Debug, Serialize)]
pub struct UserRegisterReply {
    pub user: UserBaseInfo,
}

#[derive(Debug, Serialize)]
pub struct UserLoginReply {}

#[derive(Debug, Serialize)]
pub struct UserLogoutReply {}

#[derive(Debug, Serialize)]
pub struct UserBaseInfo {
    pub user_id: String,
    pub user_name: String,
    pub nick_name: String,
    pub gender: Gender,
    pub phone: String,
}

#[derive(Debug, Serialize)]
pub struct UserDetailInfo {
    pub birthday: Option<i64>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub signature: Option<String>,
    pub facetype: Option<i32>,
    pub customface: Option<String>,
    pub customfacefmt: Option<String>,
    pub gropup_info: Option<Vec<u8>>,
    pub register_time: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub base_info: UserBaseInfo,
    pub detail_info: UserDetailInfo,
}

#[async_trait]
pub trait UserSvc {
    async fn register(&self, req: net::RegisterRequest) -> Reply<UserRegisterReply>;
    async fn login(&self, req: net::LoginRequest) -> Reply<UserLoginReply>;
    async fn logout(&self, user_id: String) -> Reply<UserLogoutReply>;
}

#[post("register")]
async fn register(svc: web::Data<dyn UserSvc>, req: web::Bytes) -> Reply<UserRegisterReply> {
    let mut req = super::deserialize::<net::RegisterRequest>(req.as_ref())?;
    req.validate()?;
    svc.register(req).await
}

#[post("login")]
async fn login(svc: web::Data<dyn UserSvc>, req: web::Bytes) -> Reply<UserLoginReply> {
    let mut req = super::deserialize::<net::LoginRequest>(req.as_ref())?;
    req.validate()?;
    svc.login(req).await
}

#[get("logout/{user_id}")]
async fn logout(svc: web::Data<dyn UserSvc>, user_id: web::Path<String>) -> Reply<UserLogoutReply> {
    if user_id.is_empty() {
        return Err(error::user_name_empty());
    }
    svc.logout(user_id.into_inner()).await
}

pub fn config(cfg: &mut ServiceConfig, svc: Arc<dyn UserSvc>) {
    cfg.app_data(web::Data::from(svc))
        .service(register)
        .service(login)
        .service(logout);
}

mod constant {
    pub const MIN_USERNAME_LENGTH: usize = 6;
    pub const MAX_USERNAME_LENGTH: usize = 20;
}

pub mod error {
    use actix_web::http::StatusCode as Code;

    use crate::base::response::Error;
    use crate::interface::user_mgr::constant;

    pub fn login_type_empty() -> Error {
        Error::new(
            Code::BAD_REQUEST.as_u16(),
            "LOGIN_TYPE_EMPTY",
            "请选择密码或验证码登录",
        )
    }

    pub fn user_name_empty() -> Error {
        Error::new(Code::BAD_REQUEST.as_u16(), "USER_NAME_EMPTY", "用户名不能为空")
    }

    pub fn user_name_too_long() -> Error {
        Error::new(
            Code::BAD_REQUEST.as_u16(),
            "USER_NAME_TOO_LONG",
            &format!("用户名不能超过{}个字符", constant::MAX_USERNAME_LENGTH),
        )
    }

    pub fn nickname_empty() -> Error {
        Error::new(Code::BAD_REQUEST.as_u16(), "NICKNAME_EMPTY", "昵称不能为空")
    }

    pub fn nickname_too_long() -> Error {
        Error::new(
            Code::BAD_REQUEST.as_u16(),
            "NICKNAME_TOO_LONG",
            &format!("昵称不能超过{}个字符", constant::MAX_USERNAME_LENGTH),
        )
    }

    pub fn mobile_empty() -> Error {
        Error::new(Code::BAD_REQUEST.as_u16(), "MOBILE_EMPTY", "手机号不能为空")
    }

    pub fn mobile_invalid() -> Error {
        Error::new(Code::BAD_REQUEST.as_u16(), "MOBILE_INVALID", "手机号码不合法")
    }

    pub fn password_empty() -> Error {
        Error::new(Code::BAD_REQUEST.as_u16(), "PASSWORD_EMPTY", "密码不能为空")
    }

    pub fn password_invalid() -> Error {
        Error::new(
            Code::BAD_REQUEST.as_u16(),
            "PASSWORD_INVALID",
            "密码不合法，至少包含一个数字和一个大写或小写字母，长度至少为8个字符",
        )
    }
}

// infrastructure

impl Validate for net::RegisterRequest {
    fn validate(&mut self) -> Result<()> {
        if self.username.is_empty() {
            return Err(error::user_name_empty());
        }
        if self.username.len() > constant::MAX_USERNAME_LENGTH {
            return Err(error::user_name_too_long());
        }

        if self.nickname.is_empty() {
            return Err(error::user_name_empty());
        }
        if self.nickname.len() > constant::MAX_USERNAME_LENGTH {
            return Err(error::user_name_too_long());
        }

        if self.mobile.is_empty() {
            return Err(error::mobile_empty());
        }

        if !utils::validator::is_mobile_valid(&self.mobile) {
            return Err(error::mobile_invalid());
        }

        if self.password.is_empty() {
            return Err(error::password_empty());
        }
        if !utils::validator::is_password_valid(&self.password) {
            return Err(error::password_invalid());
        }

        Ok(())
    }
}

impl Validate for net::LoginRequest {
    fn validate(&mut self) -> Result<()> {
        use net::login_request::Login_type as LoginType;
        match &self.login_type {
            None => return Err(error::login_type_empty()),
            Some(type_) => match type_ {
                LoginType::Custom(v) => {
                    if v.username.is_empty() {
                        return Err(error::user_name_empty());
                    }
                    if v.username.len() > constant::MAX_USERNAME_LENGTH {
                        return Err(error::user_name_too_long());
                    }
                    if v.password.is_empty() {
                        return Err(error::password_empty());
                    }
                    if utils::validator::is_password_valid(&v.password) {
                        return Err(error::password_invalid());
                    }
                }
                LoginType::Mobile(v) => {
                    if v.mobile.is_empty() {
                        return Err(error::mobile_empty());
                    }
                    if utils::validator::is_mobile_valid(&v.mobile) {
                        return Err(error::mobile_invalid());
                    }
                }
            },
        }

        Ok(())
    }
}
