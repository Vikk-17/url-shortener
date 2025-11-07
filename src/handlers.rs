use crate::AppState;
use crate::models::*;
use actix_web::{Error, HttpResponse, Responder, Result, get, post, web};
use serde_json::json;
use sqlx::Row; // <- only required to call get function

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("ALl good")
}

/// POST /api/v1/data/shorten
/// BODY JSON: {"url": "..."}
#[post("/api/v1/data/shorten")]
pub async fn data_shorten(
    data: web::Data<AppState>,
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
    .fetch_optional(&data.db)
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
        .fetch_optional(&data.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let output = DbOutput { id, slug, longurl };

        // use of return to avoid the error of "Missing else block"
        return Ok(HttpResponse::Ok().json(json!({
            "message": "Data inserted",
            "output": output
        })));
    };

    // Existing longurl — just fetch id + slug
    let existing = sqlx::query(r#"SELECT id, slug FROM urls WHERE longurl = $1"#)
        .bind(&longurl)
        .fetch_one(&data.db)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let id: i64 = existing.get("id");
    let slug: String = existing.get("slug");

    let out = DbOutput { id, slug, longurl };

    Ok(HttpResponse::Ok().json(json!({
        "message": "Already existed",
        "output": out
    })))
}
