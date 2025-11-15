use std::os::macos::raw::stat;
use std::sync::Arc;
use actix_web::{post, web, HttpResponse, Responder};
use crate::AppState;
use crate::models::resource;

pub async fn add_resource(state: web::Data<Arc<AppState>>, payload: web::Json<resource::Resource>) -> impl Responder {
    let mut resources = state.resources.lock().unwrap();

    resources.insert(payload.uri.clone(), true);
    HttpResponse::Ok().body(format!("Resource added {}", payload.uri.clone()))
}

pub async fn list_resource(state: web::Data<Arc<AppState>>) -> impl Responder {
    let health = state.resources.lock().unwrap();
    HttpResponse::Ok().json(health.clone())
}