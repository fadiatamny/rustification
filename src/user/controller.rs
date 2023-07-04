use actix_web::{get, post, Responder, HttpResponse, web};
use crate::AppState;
use serde::Deserialize;

use super::models::ModelQuery;
use super::service;

#[derive(Deserialize, Debug)]
enum UserInfo {

}

#[get("/")]
pub async fn list(data: web::Data<AppState>) -> impl Responder {
    let list = service::list(&data.db).await;
    match list {
        Ok(list) => {
            return HttpResponse::Ok().json(list);
        }
        Err(err) => {
            return HttpResponse::Ok().body(err.to_string());
        }
    }
}

#[get("/{id}")]
pub async fn get(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    return HttpResponse::Ok().body("get " + id);
}

#[post("/")]
pub async fn create(data: web::Data<AppState>, create_model: web::Json<ModelQuery>) -> impl Responder {
    return HttpResponse::Ok().body("create");
}

#[patch("/{id}")]
pub async fn update(data: web::Data<AppState>, path: web::Path<String>, patch_model: web::Json<ModelQuery>) -> impl Responder {
    return HttpResponse::Ok().body("update");
}

#[delete("/{id}")]
pub async fn delete(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    return HttpResponse::Ok().body("delete");
}

#[post("/query")]
pub async fn create(data: web::Data<AppState>, body: web::Json<UserInfo>) -> impl Responder {
    let query = body.into_inner();
    let res = service::query(&data.db, query).await;
    return HttpResponse::Ok().body("create");
}

pub fn config(cfg:&mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(list)
            .service(get)
            .service(create)
            .service(update)
            .service(delete)
    );
}