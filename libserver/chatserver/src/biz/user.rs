use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDateTime;

use crate::base::config;

#[repr(u8)]
pub enum Gender {
    Unknown = 0,
    Male = 1,
    Female = 2,
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

pub struct User {
    pub user_id: String,
    pub user_name: String,
    pub nick_name: String,
    pub password: Option<String>,
    pub gender: Gender,
    pub phone: String,
    pub birthday: Option<i64>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub signature: Option<String>,
    pub facetype: Option<i32>,
    pub customface: Option<String>,
    pub customfacefmt: Option<String>,
    pub group_info: Option<Vec<u8>>,
    pub register_time: NaiveDateTime,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn add_user(&self) -> super::Result<()>;
    async fn find_user_by_id(&self, id: i64) -> super::Result<User>;
    async fn find_user_by_user_id(&self, uid: &str) -> super::Result<User>;
    async fn find_user_by_name(&self, name: &str) -> super::Result<User>;
    async fn find_user_by_phone(&self, phone: &str) -> super::Result<User>;
    async fn find_user_by_email(&self, email: &str) -> super::Result<User>;
}

pub struct UserUsecase {
    cfg: Arc<config::Config>,
    repo: Arc<dyn UserRepository>,
}

impl UserUsecase {
    pub fn new(cfg: Arc<config::Config>, repo: Arc<dyn UserRepository>) -> Self {
        UserUsecase { cfg, repo }
    }

    pub async fn find_by_uid(&self, uid: &str) -> super::Result<User> {
        self.repo.find_user_by_user_id(uid).await
    }

    pub async fn insert_user(&self, mut user: User) -> super::Result<User> {
        // 校验用户名，手机号，邮箱有没有被注册，确保不会重复注册
        if let Ok(_) = self.repo.find_user_by_name(&user.user_name).await {
            return Err(super::Error::UsernameDumplicate);
        }
        if let Ok(_) = self.repo.find_user_by_phone(&user.phone).await {
            return Err(super::Error::PhoneDumplicate);
        }
        if let Some(email) = user.email {
            if let Ok(_) = self.repo.find_user_by_email(&email).await {
                return Err(super::Error::EmailDumplicate);
            }
        }

        // 生成唯一user_id
        let uid = uuid::Uuid::new_v4();
        user.user_id = uid.to_string();
        // user.register_time = Utils::current_naive_datetime();
        self.repo.add_user().await?;

        todo!()
    }
}
