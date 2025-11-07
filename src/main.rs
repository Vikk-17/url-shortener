mod handlers;
mod models;

use handlers::*;
use actix_web::{
    HttpServer,
    App,
    middleware::Logger,
    web,
};
use env_logger::Env;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

// use anyhow::{Result, Error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let db_uri = std::env::var("DATABASE_URI")
        .expect("Invalid database uri");

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_uri)
        .await
        .expect("Pool connection failed");

    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .app_data(pool_data.clone())
        .service(index)
        .service(data_shorten)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
