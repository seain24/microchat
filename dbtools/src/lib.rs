use sea_orm_migration::*;

mod m_01_create_user;
mod m_02_create_user_relationship;
mod m_03_create_chatmsg;
mod utils;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m_01_create_user::Migration),
            Box::new(m_02_create_user_relationship::Migration),
            Box::new(m_03_create_chatmsg::Migration),
        ]
    }
}
