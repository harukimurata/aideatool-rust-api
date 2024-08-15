use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};

// モジュールの宣言
mod structs;

// インポート
use structs::health_check::{Register, Response};

#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/res-json")]
async fn res_json() -> impl Responder {
    web::Json(Measurement { temperature: 42.3 })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/register")]
async fn register(form: web::Json<Register>) -> impl Responder {
    let Register { username, country } = form.into_inner();
    web::Json(Response { id: 1, username: username, country: country })
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(res_json)
            .service(register)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}