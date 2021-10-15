use actix_web::{web, Scope};

macro_rules! reject {
  ($e: expr) => {
      crate::util::APIError::Custom($e.to_string())
  };
}

macro_rules! reply {
  ($t: tt) => {
    actix_web::HttpResponse::Ok().json(serde_json::json!({"code":0, "data": $t}))
  };
}

pub mod v1;

pub fn apply_routes() -> Scope {
    web::scope("/api").service(v1::apply_routes())
}
