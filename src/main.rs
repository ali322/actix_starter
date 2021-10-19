use actix_starter::{repository::init_db, util::AppState};
use actix_web::{App, HttpServer, middleware::Logger};
use dotenv::dotenv;
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "DEBUG")
    }
    // env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).init();
    dotenv().ok();
    tracing_subscriber::fmt().pretty().init();
    let port = env::var("APP_PORT").expect("environment variable APP_PORT must be set");

    let conn = init_db().await;
    let app_state = AppState{
      db_conn: conn
    };
    HttpServer::new(move || {
        App::new().wrap(Logger::default()).service(actix_starter::api::apply_routes()).data(app_state.clone())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
