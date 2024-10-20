use std::sync::Arc;

use async_trait::async_trait;
use shaku::Interface;

use crate::base;
use crate::base::response;
use crate::base::response::Reply;
use crate::biz::{Error, user as biz};
use crate::interface::user::RegisterRequest;
use crate::network::stubs::user as net;

#[async_trait]
pub trait IUserService: Interface {
    async fn register(&self, register_req: RegisterRequest) -> anyhow::Result<bool>;
    async fn login(&self, user: ActiveModel) -> anyhow::Result<bool>;
    async fn logout(&self, user_id: &str) -> anyhow::Result<bool>;
}

pub struct UserManager {
    cfg: Arc<base::config::Config>,
    uc: Arc<biz::UserUsecase>,
}

impl UserManager {
    pub fn new(cfg: Arc<base::config::Config>, uc: Arc<biz::UserUsecase>) -> Arc<Self> {
        Arc::new(UserManager { cfg, uc })
    }
}

#[async_trait]
impl UserSvc for UserManager {
    async fn register(&self, req: net::RegisterRequest) -> Reply<UserRegisterReply> {
        let user = self.uc.insert_user(biz::User::from(req)).await.map_err(|e| match e {
            Error::UserNotExist(v) => response::Error::user_not_exist(v),
            Error::UsernameDumplicate => response::Error::username_duplicate(),
            Error::PhoneDumplicate => response::Error::phone_duplicate(),
            Error::EmailDumplicate => response::Error::email_duplicate(),
            Error::DabaseError(err) => response::Error::internal_server_error(&err.to_string()),
        })?;
        let reply = UserRegisterReply {
            user: UserBaseInfo {
                user_id: user.user_id,
                user_name: user.user_name,
                nick_name: user.nick_name,
                gender: user.gender.into(),
                phone: user.phone,
            },
        };

        Ok(reply.into())
    }

    async fn login(&self, req: net::LoginRequest) -> Reply<UserLoginReply> {
        todo!()
    }

    async fn logout(&self, user_id: String) -> Reply<UserLogoutReply> {
        todo!()
    }
}

impl From<net::RegisterRequest> for biz::User {
    fn from(value: net::RegisterRequest) -> Self {
        biz::User {
            user_id: "".to_string(),
            user_name: value.username,
            nick_name: value.nickname,
            password: Some(value.password),
            gender: (&value.gender.enum_value_or_default()).into(),
            phone: value.mobile,
            birthday: None,
            email: None,
            address: None,
            signature: None,
            facetype: None,
            customface: None,
            customfacefmt: None,
            group_info: None,
            register_time: Default::default(),
        }
    }
}

impl From<&net::Gender> for biz::Gender {
    fn from(value: &net::Gender) -> Self {
        match value {
            net::Gender::Male => biz::Gender::Male,
            net::Gender::Female => biz::Gender::Female,
            _ => biz::Gender::Unknown,
        }
    }
}

impl From<biz::User> for interface::User {
    fn from(value: biz::User) -> Self {
        interface::User {
            base_info: UserBaseInfo {
                user_id: value.user_id,
                user_name: value.user_name,
                nick_name: value.nick_name,
                gender: value.gender.into(),
                phone: value.phone,
            },
            detail_info: UserDetailInfo {
                birthday: value.birthday,
                email: value.email,
                address: value.address,
                signature: value.signature,
                facetype: value.facetype,
                customface: value.customface,
                customfacefmt: value.customfacefmt,
                gropup_info: value.group_info,
                register_time: value.register_time,
            },
        }
    }
}

impl From<biz::Gender> for interface::Gender {
    fn from(value: biz::Gender) -> Self {
        match value {
            biz::Gender::Male => Gender::Male,
            biz::Gender::Female => Gender::Female,
            biz::Gender::Unknown => Gender::Unknown,
        }
    }
}
