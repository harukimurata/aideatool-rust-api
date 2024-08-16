use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};

// モジュールの宣言
mod structs;
mod controller;
mod services;

#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

#[get("/")]
async fn hello() -> impl Responder {
    return HttpResponse::Ok().body("Hello world!");
}

#[get("/res-json")]
async fn res_json() -> impl Responder {
    return web::Json(Measurement { temperature: 42.3 });
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    return HttpResponse::Ok().body(req_body);
}

async fn manual_hello() -> impl Responder {
    return HttpResponse::Ok().body("Hey there!");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(res_json)
            .service(controller::health_check::register)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}