use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .unique_key()
                            .comment("自增ID"),
                    )
                    .col(
                        ColumnDef::new(User::UserId)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(User::UserName)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("用户名"),
                    )
                    .col(
                        ColumnDef::new(User::NickName)
                            .string()
                            .string_len(64)
                            .not_null()
                            .comment("用户昵称"),
                    )
                    .col(ColumnDef::new(User::Password).string().string_len(64).comment("用户密码"))
                    .col(ColumnDef::new(User::Gender).integer().default(0).comment("性别"))
                    .col(ColumnDef::new(User::Birthday).big_integer().default(19900101).comment("生日"))
                    .col(ColumnDef::new(User::Phone).string().string_len(64).comment("电话"))
                    .col(ColumnDef::new(User::Email).string().string_len(256).comment("邮箱"))
                    .col(ColumnDef::new(User::Address).string().string_len(256).comment("地址"))
                    .col(ColumnDef::new(User::Signature).string().string_len(64).comment("个性签名"))
                    .col(ColumnDef::new(User::Facetype).integer().default(0).comment("用户头像类型"))
                    .col(ColumnDef::new(User::Customface).string().string_len(32).comment("自定义头像名"))
                    .col(
                        ColumnDef::new(User::Customfacefmt)
                            .string()
                            .string_len(6)
                            .comment("自定义头像格式"),
                    )
                    .col(ColumnDef::new(User::GropupInfo).blob().comment("好友分组信息"))
                    .col(ColumnDef::new(User::RegisterTime).date_time().not_null().comment("注册时间"))
                    .to_owned(),
            )
            .await
    }
}

/// 用户表
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    UserId,
    UserName,
    NickName,
    Password,
    Gender,
    Birthday,
    Phone,
    Email,
    Address,
    Signature,
    Facetype,
    Customface,
    Customfacefmt,
    GropupInfo,
    RegisterTime,
}
