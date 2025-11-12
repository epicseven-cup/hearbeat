mod models;
mod service;

use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get};


struct AppState {
    resources: Mutex<HashMap<String, bool>>
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
