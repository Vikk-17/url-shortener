mod handlers;
mod models;
mod state;

use crate::state::*;
use actix_web::{App, HttpServer, middleware::Logger, web};
use deadpool_redis::{Config, Runtime};
use dotenvy::dotenv;
use env_logger::Env;
use handlers::*;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let db_uri = std::env::var("DATABASE_URL").expect("Invalid database uri");

    // db pool creation
    let pool: Pool<Postgres> = match PgPoolOptions::new()
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

    // Redis pool creation
    let cfg = Config::from_url("redis://127.0.0.1/");
    let redis_pool = match cfg.create_pool(Some(Runtime::Tokio1)) {
        Ok(pool) => match pool.get().await {
            Ok(_) => {
                println!("Redis connection successfull");
                pool
            }
            Err(e) => {
                eprintln!("Failed to connect to redis: {e}");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Failed to connect to the redis {e}");
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                redis: redis_pool.clone(),
            }))
            .service(redirection)
            .service(data_shorten)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(2)
    .run()
    .await
}
