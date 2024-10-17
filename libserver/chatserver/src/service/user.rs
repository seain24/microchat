use std::sync::Arc;

use anyhow::Context;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;
use serde::{Deserialize, Serialize};
use shaku::{Component, HasComponent, Interface};
use validator::Validate;
use crate::components::get_service_factory;
use crate::components::redis::IRedisService;
use crate::db::entity::user as entity;
use crate::db::repository::user::IUserRepository;

const MOBILE_PHONE_PATTERN: &str =
    "/^1(3[0-9]|4[01456879]|5[0-35-9]|6[2567]|7[0-8]|8[0-9]|9[0-35-9])\\d{8}$/";
static MOBILE_PHONE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(MOBILE_PHONE_PATTERN).unwrap());
/// 至少8个字符，至少包含一个字母（大写或小写）、数字或者特殊字符
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

impl From<i32> for Gender {
    fn from(value: i32) -> Self {
        match value {
            1 => Gender::Male,
            2 => Gender::Female,
            _ => Gender::Unknown,
        }
    }
}

impl Into<i32> for &Gender {
    fn into(self) -> i32 {
        match self {
            Gender::Male => 1,
            Gender::Female => 2,
            Gender::Unknown => 0,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignUpRequest {
    #[validate(length(min = 6, max = 20, code = "用户名至少6个字符，最多20个字符"))]
    pub username: String,
    #[validate(length(min = 1, max = 20, code = "昵称至少6个字符，最多20个字符"))]
    pub nickname: String,
    #[validate(regex(
        path = "*PASSWORD_REGEX",
        code = "至少8个字符，其中至少包含一个字母（大写或小写）、数字或者特殊字符"
    ))]
    pub password: String,
    #[validate(regex(path = "*MOBILE_PHONE_REGEX", code = "Please provide a valid mobile phone"))]
    pub mobile: String,
    #[validate(email(code = "Please provide a valid email!"))]
    pub email: String,
    pub gender: Gender,
}

#[derive(Debug, Deserialize)]
pub enum ClientType {
    WINDOWS = 0,
    LINUX = 1,
    MAC = 2,
    ANDROID = 3,
    IOS = 4,
    IPAD = 5,
}

#[derive(Debug, Deserialize)]
pub enum OnlineStatus {
    OFFLINE = 0,
    INVISIBLE = 1,
    WIFI = 2,
    AndroidCellular = 3,
    IOSCellular = 4,
    MacCellular = 5,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SignInRequest {
    #[validate(length(min = 1, code = "用户名不为空!"))]
    pub username: String,
    #[validate(length(min = 8, code = "请提供一个合法的密码!"))]
    pub password: String,
    pub client_type: ClientType,
    pub online_status: OnlineStatus,
}

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
    pub face_type: Option<i32>,
    pub custom_face: Option<String>,
    pub custom_face_fmt: Option<String>,
    pub group_info: Option<Vec<u8>>,
    pub register_time: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub base_info: UserBaseInfo,
    pub detail_info: UserDetailInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub base_info: UserBaseInfo,
    pub detail_info: UserDetailInfo,
}

#[async_trait]
pub trait IUserService: Interface {
    async fn sign_up(&self, register_req: SignUpRequest) -> anyhow::Result<UserInfo>;
    async fn sign_in(&self, login_req: SignInRequest) -> anyhow::Result<()>;
    async fn sign_out(&self, user_id: &str) -> anyhow::Result<()>;
}

#[derive(Component)]
#[shaku(interface = IUserService)]
pub struct UserServiceImpl {
    #[shaku(inject)]
    pub repo: Arc<dyn IUserRepository>,
}

#[async_trait]
impl IUserService for UserServiceImpl {
    async fn sign_up(&self, signup_req: SignUpRequest) -> anyhow::Result<UserInfo> {
        // todo 校验用户名是否重复
        let modules = get_service_factory()?;
        let redis_service: &dyn IRedisService = modules.resolve_ref();
        

        let model = self.repo.add(signup_req.into()).await.context("user signed up failed")?;
        Ok(model.into())
    }

    async fn sign_in(&self, signin_req: SignInRequest) -> anyhow::Result<()> {
        // 校验用户是否注册
        let user = self
            .repo
            .find_by_name(&signin_req.username)
            .await
            .context("user signed in failed")?;

        let Some(u) = user else {
            return Err(anyhow::anyhow!("用户未注册"));
        };
        // 校验用户名、密码是否正确
        let mut valid = false;
        if u.user_name.eq(&signin_req.username) {
            valid = true;
        }
        // todo 校验密码，密码需要解密
        if let Some(pwd) = u.password {
            if pwd.eq(&signin_req.password) {
                valid = valid & true;
            }
        }

        if !valid {
            return Err(anyhow::anyhow!("用户名或密码不正确"));
        }
        return Ok(());
    }

    async fn sign_out(&self, user_id: &str) -> anyhow::Result<()> {
        Ok(())
    }
}

// #[async_trait]
// impl UserSvc for UserManager {
//     async fn register(&self, req: net::RegisterRequest) -> Reply<UserRegisterReply> {
//         let user = self.uc.insert_user(biz::User::from(req)).await.map_err(|e| match e {
//             Error::UserNotExist(v) => response::Error::user_not_exist(v),
//             Error::UsernameDumplicate => response::Error::username_duplicate(),
//             Error::PhoneDumplicate => response::Error::phone_duplicate(),
//             Error::EmailDumplicate => response::Error::email_duplicate(),
//             Error::DabaseError(err) => response::Error::internal_server_error(&err.to_string()),
//         })?;
//         let reply = UserRegisterReply {
//             user: UserBaseInfo {
//                 user_id: user.user_id,
//                 user_name: user.user_name,
//                 nick_name: user.nick_name,
//                 gender: user.gender.into(),
//                 phone: user.phone,
//             },
//         };
//
//         Ok(reply.into())
//     }
//
//     async fn login(&self, req: net::LoginRequest) -> Reply<UserLoginReply> {
//         todo!()
//     }
//
//     async fn logout(&self, user_id: String) -> Reply<UserLogoutReply> {
//         todo!()
//     }
// }

impl Into<entity::ActiveModel> for SignUpRequest {
    fn into(self) -> entity::ActiveModel {
        entity::ActiveModel {
            id: NotSet,
            user_id: Default::default(),
            user_name: Set(self.username),
            nick_name: Set(self.nickname),
            password: Set(Some(self.password)),
            gender: Set((&self.gender).into()),
            birthday: Default::default(),
            phone: Set(self.mobile),
            email: Set(Some(self.email)),
            address: Default::default(),
            signature: Default::default(),
            facetype: Default::default(),
            customface: Default::default(),
            customfacefmt: Default::default(),
            gropup_info: Default::default(),
            register_time: Default::default(),
        }
    }
}

impl From<entity::Model> for UserInfo {
    fn from(value: entity::Model) -> Self {
        UserInfo {
            base_info: UserBaseInfo {
                user_id: value.user_id,
                user_name: value.user_name,
                nick_name: value.nick_name,
                gender: Gender::from(value.gender),
                phone: value.phone,
            },
            detail_info: UserDetailInfo {
                birthday: value.birthday,
                email: value.email,
                address: value.address,
                signature: value.signature,
                face_type: value.facetype,
                custom_face: value.customface,
                custom_face_fmt: value.customfacefmt,
                group_info: value.gropup_info,
                register_time: value.register_time,
            },
        }
    }
}

// impl From<biz::User> for interface::User {
//     fn from(value: biz::User) -> Self {
//         interface::User {
//             base_info: UserBaseInfo {
//                 user_id: value.user_id,
//                 user_name: value.user_name,
//                 nick_name: value.nick_name,
//                 gender: value.gender.into(),
//                 phone: value.phone,
//             },
//             detail_info: UserDetailInfo {
//                 birthday: value.birthday,
//                 email: value.email,
//                 address: value.address,
//                 signature: value.signature,
//                 facetype: value.facetype,
//                 customface: value.customface,
//                 customfacefmt: value.customfacefmt,
//                 gropup_info: value.group_info,
//                 register_time: value.register_time,
//             },
//         }
//     }
// }
//
// impl From<biz::Gender> for interface::Gender {
//     fn from(value: biz::Gender) -> Self {
//         match value {
//             biz::Gender::Male => Gender::Male,
//             biz::Gender::Female => Gender::Female,
//             biz::Gender::Unknown => Gender::Unknown,
//         }
//     }
// }
