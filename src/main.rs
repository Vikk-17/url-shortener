mod handlers;
mod models;
mod state;

use handlers::*;
use crate::state::*;
use actix_web::{
    HttpServer,
    App,
    middleware::Logger,
    web,
};
use env_logger::Env;
use dotenvy::dotenv;
use sqlx::{
    Pool,
    Postgres,
    postgres::PgPoolOptions,
};
// use anyhow::{Result, Error};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let db_uri = std::env::var("DATABASE_URL")
        .expect("Invalid database uri");

    let pool:Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_uri)
        .await
    {
        Ok(pool) => {
            println!("Database connection is successfull");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database {:?}", err);
            std::process::exit(1);
        }
    };
 
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .app_data(web::Data::new(AppState{
                db: pool.clone(),
            }))
        .service(index)
        .service(data_shorten)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
