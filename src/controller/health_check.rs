use actix_web::{post, web, Responder};

// モジュールの宣言
use crate::structs;
use crate::services;

// インポート
use structs::health_check::{Register, Response};
use services::health_check::create_response;

#[post("/register")]
pub async fn register(form: web::Json<Register>) -> impl Responder {
    let result: Response = create_response(form.into_inner());
    return web::Json(result);
}