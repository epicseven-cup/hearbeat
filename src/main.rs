mod models;
mod service;

use std::collections::HashMap;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get};
use actix_web::rt::time::interval;
use actix_web::web::head;

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


    let share_state = web::Data::new(AppState {
        resources: Mutex::new(HashMap::new())
    });






    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
