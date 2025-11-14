mod models;
mod service;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpServer, Responder, HttpResponse, post, get};
use actix_web::rt::time::interval;
use actix_web::web::head;
use tokio::task;

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


    let share_state = Arc::new(AppState {
        resources: Mutex::new(HashMap::new())
    });

    share_state.resources.lock().unwrap().insert(String::from("https://lat.sh"), true);

    println!("Starting http server at: http://localhost:8080/");


    let background = share_state.clone();


    task::spawn(async move {
        service::background::heartbeat(background).await;
    });
    println!("After background");


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(share_state.clone()))
            .service(hello)
            .service(echo)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
