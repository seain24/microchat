use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRelationShip::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRelationShip::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(UserRelationShip::UserId1)
                            .big_integer()
                            .not_null()
                            .comment("第一个用户id"),
                    )
                    .col(
                        ColumnDef::new(UserRelationShip::UserId2)
                            .big_integer()
                            .not_null()
                            .comment("第二个用户id"),
                    )
                    .col(
                        ColumnDef::new(UserRelationShip::User1GroupName)
                            .string()
                            .string_len(32)
                            .not_null()
                            .default("我的好友")
                            .comment("用户2在用户1的好友分组名称"),
                    )
                    .col(
                        ColumnDef::new(UserRelationShip::User1MarkName)
                            .string()
                            .string_len(32)
                            .comment("用户2在用户1的备注名称"),
                    )
                    .col(
                        ColumnDef::new(UserRelationShip::User2GroupName)
                            .string()
                            .string_len(32)
                            .not_null()
                            .default("我的好友")
                            .comment("用户1在用户2的好友分组名称"),
                    )
                    .col(
                        ColumnDef::new(UserRelationShip::User2MarkName)
                            .string()
                            .string_len(32)
                            .comment("用户1在用户2的备注名称"),
                    )
                    .col(
                        ColumnDef::new(UserRelationShip::UpdateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await
    }
}

/// 用户关系表
#[derive(Iden)]
pub enum UserRelationShip {
    Table,
    Id,
    UserId1,
    UserId2,
    User1GroupName,
    User1MarkName,
    User2GroupName,
    User2MarkName,
    UpdateTime,
}
