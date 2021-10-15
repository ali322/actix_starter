use actix_web::{HttpResponse, Scope, get};
use serde_json::{Value, json};

use crate::util::APIError;

#[get("/user")]
async fn users() -> Result<HttpResponse, APIError>{
  // Err(reject!("fail"))
  // Ok(ret)
  Ok(reply!("ok"))
}

pub fn apply_routes(scope: Scope) -> Scope {
  scope.service(users)
}