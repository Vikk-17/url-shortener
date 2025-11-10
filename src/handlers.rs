use crate::AppState;
use crate::metrics::*;
use crate::models::*;
use actix_web::Responder;
use actix_web::{Error, HttpResponse, Result, get, http::header::LOCATION, post, web};
use prometheus::{Encoder, TextEncoder};
use redis::AsyncCommands;
use serde_json::json;
use sqlx::Row; // <- only required to call get function
use std::time::Instant;
use utoipa;

/// GET /api/v1/shorturl
/// Return longurl for HTTP Redirection
#[utoipa::path(
    get,
    path = "/api/v1/{slug}",
    responses(
        (status= 302, description = "Redirect to long URL", headers(
            ("Location" = String, description = "The original long URL")
        )),
        (status = 404, description = "URL not found", body = ErrorResponse)
    ),
    params(
        ("slug" = String, Path, description = "Short URL slug", example="abd2A"),
    ),
    tag = "urls"
)]
#[get("/api/v1/{slug}")]
pub async fn redirection(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let slug = path.into_inner();

    // 1. get the connection object from the pool
    let mut redis_conn = state
        .redis
        .get()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let cache_key = format! {"slug:{}", slug};

    // 2. Check redis cache
    let start = Instant::now();
    let cached: Option<String> = redis_conn
        .get(&cache_key)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("Redis lookup took: {:?}", start.elapsed());

    if let Some(longurl) = cached {
        HTTP_REQUEST_TOTAL
            .with_label_values(&["GET", "/api/v1/{slud}", "302"])
            .inc();

        println!("Cache hit for slug: {}", slug);
        return Ok(HttpResponse::Found()
            .insert_header((LOCATION, longurl))
            .finish());
    }

    println!("Cache missed for the slug: {}", slug);

    // 3. If not query it into db
    let start = Instant::now();
    let existing = sqlx::query(r#"SELECT longurl FROM urls WHERE slug = $1"#)
        .bind(&slug)
        .fetch_optional(&state.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    println!("Db lookup took: {:?}", start.elapsed());

    let Some(row) = existing else {
        HTTP_REQUEST_TOTAL
            .with_label_values(&["GET", "/api/v1/{slud}", "404"])
            .inc();

        return Ok(HttpResponse::NotFound().finish());
    };

    let longurl = row.get::<String, _>("longurl");

    // 4. Cache the slug with 1 hour of ttl
    let _: () = redis_conn
        .set_ex(&cache_key, &longurl, 86400)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("Cached slug: {} -> {}", slug, longurl);

    HTTP_REQUEST_TOTAL
        .with_label_values(&["GET", "/api/v1/{slug}", "302"])
        .inc();

    // 5. Redirect it to longurl
    Ok(HttpResponse::Found()
        .insert_header((LOCATION, longurl))
        .finish())
}

/// POST /api/v1/data/shorten
/// BODY JSON: {"longurl": "..."}
#[utoipa::path(
    post,
    path = "/api/v1/data/shorten",
    request_body(content = UserLongUrl, description = "Long URL to shorten", content_type = "aplication/json"),
    responses(
        (status = 200, description = "URL shortened successfully", body = ShortenResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = "urls"
)]
#[post("/api/v1/data/shorten")]
pub async fn data_shorten(
    state: web::Data<AppState>,
    user_input: web::Json<UserLongUrl>,
) -> Result<HttpResponse, Error> {
    let longurl = user_input.into_inner().longurl;
    let inserted_row = sqlx::query(
        r#"
        INSERT INTO urls (longurl)
        VALUES ($1)
        ON CONFLICT (longurl) DO NOTHING
        RETURNING id
        "#,
    )
    .bind(&longurl)
    .fetch_optional(&state.db)
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(row) = inserted_row {
        let id = row.get("id");
        let slug = base62::encode(id as u128);

        let _ = sqlx::query(
            r#"
                UPDATE urls SET slug = $1 WHERE id = $2 RETURNING slug
            "#,
        )
        .bind(&slug)
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let mut redis_conn = state
            .redis
            .get()
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let cache_key = format! {"slug:{}", slug};

        let _: () = redis_conn
            .set_ex(&cache_key, &longurl, 86400)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let output = DbOutput { id, slug, longurl };

        HTTP_REQUEST_TOTAL
            .with_label_values(&["POST", "/api/v1/data/shorten/", "200"])
            .inc();

        // use of return to avoid the error of "Missing else block"
        return Ok(HttpResponse::Ok().json(json!({
            "message": "Data inserted",
            "output": output
        })));
    };

    // Existing longurl — just fetch id + slug
    let existing = sqlx::query(r#"SELECT id, slug FROM urls WHERE longurl = $1"#)
        .bind(&longurl)
        .fetch_one(&state.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let id: i64 = existing.get("id");
    let slug: String = existing.get("slug");

    let out = DbOutput { id, slug, longurl };

    HTTP_REQUEST_TOTAL
        .with_label_values(&["POST", "/api/v1/data/shorten/", "200"])
        .inc();

    Ok(HttpResponse::Ok().json(json!({
        "message": "Already existed",
        "output": out
    })))
}

#[get("/prom")]
pub async fn prom() -> impl Responder {
    HTTP_REQUEST_TOTAL
        .with_label_values(&["GET", "/prom", "200"])
        .inc();

    HttpResponse::Ok().json(json!({
        "Message": "Prometheus testing",
    }))
}

#[get("/metrics")]
pub async fn metrics() -> Result<HttpResponse, Error> {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];
    let metric_families = prometheus::gather();

    encoder
        .encode(&metric_families, &mut buffer)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(buffer))
}
