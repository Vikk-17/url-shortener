use crate::models::*;
use crate::AppState;

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
        INSERT INTO urls (longurl) VALUES ($1)
        "#,
    )
    .bind(&input.longurl)
    .fetch_all(&data.db)
    .await;

    // HttpResponse::Ok().json(user_input.url)
    match row {
        Ok(_) => Ok(HttpResponse::Ok().body("Insertion done")),
        Err(e) => Ok(HttpResponse::InternalServerError()
                    .json(json!({"error": format!("Db error{}", e)})),
            )
    }
}

