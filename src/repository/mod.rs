use sea_orm::{DatabaseConnection, Database, DbErr};
use std::env;

pub mod entity;
pub mod dto;
pub mod vo;

pub type DBError = DbErr;
pub type DBConn = DatabaseConnection;

pub async fn init_db() -> DBConn {
  let database_url =
      env::var("DATABASE_URL").expect("environment variable DATABASE_URL must be set");
  Database::connect(&database_url).await.expect("failed to connect databse")
}

// lazy_static!{
//   static ref CONN: DatabaseConnection =  async_std::task::block_on(async { init_db().await });
// }