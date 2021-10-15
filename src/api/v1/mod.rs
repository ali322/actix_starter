use actix_web::{Scope, web, get, Responder};

#[get("/ping")]
async fn index() -> impl Responder {
  "pong"
}

mod user;

pub fn apply_routes() -> Scope {
  let mut scope = web::scope("/v1").service(index);
  scope = user::apply_routes(scope);
  scope
}