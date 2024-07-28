use sea_orm::{ConnectOptions, DatabaseConnection, DbErr};

use super::settings::{EnvVariables, Settings};

pub struct DB;

impl DB {
    pub async fn connect() -> Result<DatabaseConnection, DbErr> {
        let mut opt = ConnectOptions::new(Settings::expect_env(EnvVariables::DatabaseUrl));
        opt.sqlx_logging(true);
        sea_orm::Database::connect(opt).await
    }
}
