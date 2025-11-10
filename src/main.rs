mod handlers;
mod metrics;
mod models;
mod state;

use crate::state::*;
use actix_web::{App, HttpServer, middleware::Logger, web};
use deadpool_redis::{Config, Runtime};
use dotenvy::dotenv;
use env_logger::Env;
use handlers::*;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::redirection,
        handlers::data_shorten,
    ),
    components(
        schemas(
            models::UserLongUrl,
            models::DbOutput,
            models::ShortenResponse,
            models::ErrorResponse,
        )
    ),
    tags(
        (name = "urls", description = "URL shortening and redirection endpoints")
    ),
    info(
        title = "URL Shortener API",
        version = "1.0.0",
        description = "A high-performance URL shortener with Redis caching and Prometheus metrics",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development server"),
        (url = "https://api.example.com", description = "Production server")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let  openapi = ApiDoc::openapi();

    // DB pool creation -> sqlx
    let db_uri: String = std::env::var("DATABASE_URL").expect("Invalid database uri");
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
    let redis_url: String = std::env::var("REDIS_URL").expect("Invalid redis url");
    let cfg = Config::from_url(redis_url);
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

    // Spawn the server
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
            .service(prom)
            .service(metrics)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
    })
    .bind(("0.0.0.0", 8080))?
    .workers(2)
    .run()
    .await
}
