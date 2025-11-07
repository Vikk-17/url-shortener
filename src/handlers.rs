use crate::models::*;
use crate::AppState;
use sqlx::Row; // <- only required to call get function
use actix_web::{
    get,
    post,
    HttpResponse,
    Responder,
    web,
    Error,
    Result,
};
use serde_json::json;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("ALl good")
}

/// POST /api/v1/data/shorten
/// BODY JSON: {"url": "..."}
#[post("/api/v1/data/shorten")]
pub async fn data_shorten(data: web::Data<AppState>, user_input: web::Json<UserLongUrl>) -> Result<HttpResponse, Error> {
    let input = user_input.into_inner();
    let row = sqlx::query(
        r#"
        INSERT INTO urls (longurl)
        VALUES ($1)
        RETURNING id, longurl
        "#,
    )
    .bind(&input.longurl)
    .fetch_one(&data.db)
    .await;

    match row {
        Ok(r) => {
            let id = r.get("id");
            let slug = base62::encode(id as u128);
            let longurl= r.get("longurl");

            let updated = sqlx::query(
                r#"
                UPDATE urls SET slug = $1 WHERE id = $2 RETURNING slug
                "#,
            )
            .bind(&slug)
            .bind(id)
            .execute(&data.db)
            .await;

            match updated {
                Ok(_) => {
                    let output = DbOutput { id, slug, longurl };
                    Ok(HttpResponse::Ok().json(json!({
                        "message": "Data inserted",
                        "output": output
                    })))
                }
                Err(e) => {
                    eprintln!("Update error: {e}");
                    Ok(HttpResponse::InternalServerError().json(json!({
                        "error": format!("Db update error: {e}"),
                })))
                }
            }
        }
        Err(e) => {
            eprintln!("Db insertion error {e}");
            Ok(HttpResponse::InternalServerError()
                .json(json!({"error": format!("Db error{}", e)})),
            )
        }
    }
}

