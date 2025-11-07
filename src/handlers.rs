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
        RETURNING id, longurl, slug
        "#,
    )
    .bind(&input.longurl)
    .fetch_one(&data.db)
    .await;

    match row {
        Ok(r) => {
            let output = DbOutput {
                id: r.get("id"),
                longurl: r.get("longurl"),
                slug: r.get("slug"),
            };
            Ok(HttpResponse::Ok().json(json!({
                "message": "Data inserted",
                "output": output})))
        },

        Err(e) => {
            eprintln!("Db insertion error {e}");
            Ok(HttpResponse::InternalServerError()
                .json(json!({"error": format!("Db error{}", e)})),
            )
        }
    }
}

