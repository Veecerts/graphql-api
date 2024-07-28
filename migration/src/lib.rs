pub use sea_orm_migration::prelude::*;

mod m20240728_105913_create_user_table;
mod m20240728_155049_create_file_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240728_105913_create_user_table::Migration),
            Box::new(m20240728_155049_create_file_table::Migration),
        ]
    }
}
