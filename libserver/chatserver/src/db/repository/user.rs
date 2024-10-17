use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter};
use shaku::{Component, Interface};

use crate::components::mysql::IMysqlService;
use crate::db::entity::user as entity;
use crate::db::entity::user::{ActiveModel, Model};

#[async_trait]
pub trait IUserRepository: Interface {
    async fn find_by_id(&self, id: i64) -> Result<Option<Model>, DbErr>;
    async fn find_by_user_id(&self, uid: &str) -> Result<Option<Model>, DbErr>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Model>, DbErr>;
    async fn find_by_phone(&self, phone: &str) -> Result<Option<Model>, DbErr>;
    async fn find_by_email(&self, email: &str) -> Result<Option<Model>, DbErr>;
    async fn add(&self, user: entity::ActiveModel) -> Result<Model, DbErr>;
    async fn update(&self, user: ActiveModel) -> Result<Model, DbErr>;
}

#[derive(Component)]
#[shaku(interface = IUserRepository)]
pub struct UserRepositoryImpl {
    db_conn: Arc<dyn IMysqlService>,
}

#[async_trait]
impl IUserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: i64) -> Result<Option<Model>, DbErr> {
        entity::Entity::find_by_id(id).one(self.db_conn.get_conn().as_ref()).await
    }

    async fn find_by_user_id(&self, uid: &str) -> Result<Option<Model>, DbErr> {
        entity::Entity::find()
            .filter(entity::Column::UserId.eq(uid))
            .one(self.db_conn.get_conn().as_ref())
            .await
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Model>, DbErr> {
        entity::Entity::find()
            .filter(entity::Column::UserName.contains(name))
            .one(self.db_conn.get_conn().as_ref())
            .await
    }

    async fn find_by_phone(&self, phone: &str) -> Result<Option<Model>, DbErr> {
        entity::Entity::find()
            .filter(entity::Column::Phone.eq(phone))
            .one(self.db_conn.get_conn().as_ref())
            .await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<Model>, DbErr> {
        entity::Entity::find()
            .filter(entity::Column::Email.eq(email))
            .one(self.db_conn.get_conn().as_ref())
            .await
    }

    async fn add(&self, user: ActiveModel) -> Result<Model, DbErr> {
        user.insert(self.db_conn.get_conn().as_ref()).await
    }

    async fn update(&self, user: ActiveModel) -> Result<Model, DbErr> {
        user.update(self.db_conn.get_conn().as_ref()).await
    }
}
