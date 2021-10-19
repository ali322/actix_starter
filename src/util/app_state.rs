use crate::repository::DBConn;

#[derive(Debug, Clone)]
pub struct AppState{
  pub db_conn: DBConn
}