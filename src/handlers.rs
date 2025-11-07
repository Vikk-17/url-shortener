use crate::models::*;
use actix_web::{
    get,
    post,
    HttpResponse,
    Responder,
    web,
};
// use serde_json::json;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("ALl good")
}


#[post("/api/v1/data/shorten")]
pub async fn data_shorten(user_input: web::Json<UserLongUrl>) -> impl Responder {
    let user_input = user_input.into_inner();
    HttpResponse::Ok().json(user_input.url)
}

