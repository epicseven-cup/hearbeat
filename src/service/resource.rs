use actix_web::{post, web, HttpResponse, Responder};
use crate::AppState;
use crate::models::resource;

async fn add_resource(state: web::Data<AppState>, payload: web::Json<resource::Resource>) -> impl Responder {
    let mut resources = state.resources.lock().unwrap();

    resources.insert(payload.uri.clone(), true);
    HttpResponse::Ok().body(format!("Resource added {}", payload.uri.clone()))
}
