use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "app=DEBUG")
    }
    dotenv().ok();
    tracing_subscriber::fmt().pretty().init();
    let port = env::var("APP_PORT").expect("environment variable APP_PORT must be set");
    HttpServer::new(|| {
        App::new().wrap(Logger::default()).service(actix_starter::api::apply_routes())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
