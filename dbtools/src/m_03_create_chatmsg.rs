use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ChatMsg::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChatMsg::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .comment("自增ID"),
                    )
                    .col(ColumnDef::new(ChatMsg::SenderId).big_integer().not_null().comment("发送者id"))
                    .col(ColumnDef::new(ChatMsg::TargetId).big_integer().not_null().comment("接收者id"))
                    .col(ColumnDef::new(ChatMsg::MsgContent).blob().not_null().comment("聊天内容"))
                    .col(
                        ColumnDef::new(ChatMsg::CreateTime)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .to_owned(),
            )
            .await
    }
}

/// 聊天消息记录表
#[derive(Iden)]
pub enum ChatMsg {
    Table,
    Id,
    SenderId,
    TargetId,
    MsgContent,
    CreateTime,
}
