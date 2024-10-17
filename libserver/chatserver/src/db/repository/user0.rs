use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::sea_query::{Expr, Func};

use crate::biz;
use crate::biz::{error as biz_error, user as biz_user};
use crate::db::Data;

use crate::db::entity::user as entity_user;

pub struct UserRepo {
    data: Arc<Data>,
}

impl UserRepo {
    pub fn repo(data: Arc<Data>) -> Arc<Self> {
        Arc::new(UserRepo { data })
    }
}

#[async_trait]
impl biz_user::UserRepository for UserRepo {
    async fn add_user(&self) -> biz::Result<()> {
        // todo 待实现
        Ok(())
    }

    async fn find_user_by_id(&self, id: i64) -> biz::Result<biz_user::User> {
        let user = entity_user::Entity::find()
            .filter(entity_user::Column::Id.eq(id))
            .one(&self.data.db().await)
            .await
            .map_err(|e| biz_error::Error::DabaseError(e))?
            .ok_or(biz_error::Error::UserNotExist(id.to_string()))?;
        Ok(biz_user::User::from(user))
    }

    async fn find_user_by_user_id(&self, uid: &str) -> biz::Result<biz_user::User> {
        let user = entity_user::Entity::find_by_id(uid)
            .one(&self.data.db().await)
            .await
            .map_err(|e| biz_error::Error::DabaseError(e))?
            .ok_or(biz_error::Error::UserNotExist(uid.to_string()))?;
        Ok(biz_user::User::from(user))
    }

    /// Find user by name ignore upper
    ///
    /// # Example
    ///
    /// ```
    /// use chat_server::db::entity::user;
    /// use sea_orm::sea_query::{Expr, Func, Query};
    /// let query = Query::select()
    ///     .column(user::Column::UserName)
    ///     .from(user::Entity)
    ///     .and_where(
    ///         Expr::expr(Func::lower(Expr::col(user::Column::UserName)))
    ///             .eq("jason".trim().to_lowercase()),
    ///     )
    ///     .take();
    /// ```
    async fn find_user_by_name(&self, name: &str) -> biz::Result<biz_user::User> {
        let cond =
            Expr::expr(Func::lower(Expr::col(entity_user::Column::UserName))).eq(name.trim().to_lowercase());
        let user = entity_user::Entity::find()
            .filter(cond)
            .one(&self.data.db().await)
            .await
            .map_err(|e| biz_error::Error::DabaseError(e))?
            .ok_or(biz_error::Error::UserNotExist(name.to_string()))?;
        Ok(biz_user::User::from(user))
    }

    async fn find_user_by_phone(&self, phone: &str) -> biz::Result<biz_user::User> {
        let user = entity_user::Entity::find()
            .filter(entity_user::Column::Phone.eq(phone))
            .one(&self.data.db().await)
            .await
            .map_err(|e| biz_error::Error::DabaseError(e))?
            .ok_or(biz_error::Error::UserNotExist(phone.to_string()))?;
        Ok(biz_user::User::from(user))
    }

    async fn find_user_by_email(&self, email: &str) -> biz::Result<biz_user::User> {
        let user = entity_user::Entity::find()
            .filter(entity_user::Column::Email.eq(email))
            .one(&self.data.db().await)
            .await
            .map_err(|e| biz_error::Error::DabaseError(e))?
            .ok_or(biz_error::Error::UserNotExist(email.to_string()))?;
        Ok(biz_user::User::from(user))
    }
}

impl From<entity_user::Model> for biz_user::User {
    fn from(value: entity_user::Model) -> Self {
        biz_user::User {
            user_id: value.user_id,
            user_name: value.user_name,
            nick_name: value.nick_name,
            password: None,
            gender: value.gender.into(),
            phone: value.phone,
            birthday: value.birthday,
            email: value.email,
            address: value.address,
            signature: value.signature,
            facetype: value.facetype,
            customface: value.customface,
            customfacefmt: value.customfacefmt,
            group_info: value.gropup_info,
            register_time: Default::default(),
        }
    }
}

impl From<Option<i32>> for biz_user::Gender {
    fn from(value: Option<i32>) -> Self {
        match value {
            None => biz_user::Gender::Unknown,
            Some(v) => biz_user::Gender::from(v),
        }
    }
}
