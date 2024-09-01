//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user_relation_ship")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id1: i64,
    pub user_id2: i64,
    pub user1_group_name: String,
    pub user1_mark_name: Option<String>,
    pub user2_group_name: String,
    pub user2_mark_name: Option<String>,
    pub update_time: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
