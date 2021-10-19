use actix_web::{HttpResponse, Scope, get, web};
use validator::Validate;

use crate::{repository::dto::QueryUser, util::{APIError, AppState}};

#[get("/user")]
async fn users(q: web::Query<QueryUser>, app_state: web::Data<AppState>) -> Result<HttpResponse, APIError>{
  q.validate()?;
  let conn = &app_state.db_conn;
  let (count, rows) = q.find_all(conn).await?;
  Ok(reply!({
    "count": count, "rows": rows,
  }))
}

pub fn apply_routes(scope: Scope) -> Scope {
  scope.service(users)
}