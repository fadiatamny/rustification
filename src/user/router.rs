use actix_web::{get, post, Responder, HttpResponse, web};
use crate::AppState;

use super::models::ModelQuery;
use super::service;

#[get("/me")]
pub async fn me(data: web::Data<AppState>) -> impl Responder {
    return HttpResponse::Ok().body("me");
}

#[post("/register")]
pub async fn register(data: web::Data<AppState>) -> impl Responder {
    return HttpResponse::Ok().body("register");
}

#[post("/login")]
pub async fn login(data: web::Data<AppState>, login: web::Json<ModelQuery>) -> impl Responder {
    let d = data.db.;
    let db = match d.as_ref() {
        Ok(db) => db,
        Err(error) => return HttpResponse::InternalServerError().body("db error"),
    };

    let handler = service::query(db, login.into_inner()).await;
    return HttpResponse::Ok().body("login");
}

#[post("/logout")]
pub async fn logout(data: web::Data<AppState>) -> impl Responder {
    return HttpResponse::Ok().body("logout");
}


pub fn config(cfg:&mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(logout);
    cfg.service(me);
}